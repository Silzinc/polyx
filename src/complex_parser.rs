use super::parser::Ops;
use crate::{
	errors::PolynomialError::{self, *},
	polynomial,
	traits::Primitive,
	Polynomial,
};
use num::Complex;
use num_traits::{FromPrimitive, ToPrimitive, Zero};

fn to_complexf64<T>(c: Complex<T>) -> Option<Complex<f64>>
	where T: Clone + ToPrimitive
{
	Some(Complex::new(c.re.to_f64()?, c.im.to_f64()?))
}

fn from_complexf64<T>(c: Complex<f64>) -> Option<Complex<T>>
	where T: Clone + FromPrimitive
{
	Some(Complex::new(T::from_f64(c.re)?, T::from_f64(c.im)?))
}

struct ComplexParser<T>
{
	pols_vec:  Vec<Polynomial<Complex<T>>>,
	ops_vec:   Vec<Ops>,
	reads_num: bool,
	reads_dec: bool,
	num:       u64,
	nb_decs:   u32,
	unary_min: bool,
	is_min:    bool,
	is_factor: bool,
	icount:    u32,
}

impl<T> ComplexParser<T>
	where T: Primitive,
	      Complex<T>: crate::traits::Num
{
	fn execute_bin_operator(&mut self) -> Result<(), PolynomialError>
	{
		#[allow(non_snake_case)]
		let X = polynomial![Complex::<T>::zero(), Complex::from(T::one())];
		// println!("Got {:?} and {:?}", self.pols_vec, op);
		let op: Ops = self.ops_vec.pop().ok_or(NoBinaryOperator)?;
		let p1: Polynomial<Complex<T>> = self.pols_vec.pop().ok_or(BinaryOperatorZeroOperand(op))?;
		let p2: Polynomial<Complex<T>> = self.pols_vec
		                                     .pop()
		                                     .ok_or(BinaryOperatorOneOperand(op, p1.to_string()))?;

		if op == Ops::Mul
		   && self.ops_vec.len() != 0
		   && self.ops_vec[self.ops_vec.len() - 1] == Ops::Pow
		   && self.pols_vec.len() != 0
		   && self.pols_vec[self.pols_vec.len() - 1].degree() == 0
		   && p1.degree() == 0
		   && p2 == X
		{
			if self.pols_vec[self.pols_vec.len() - 1].is_zero() {
				return Err(ImpossiblePower(p2.to_string(), p1[0].to_string()));
			}
			// This is to optimize a bit the parsing of an expression like "cX^pow"
			let c: Complex<T> = self.pols_vec.pop().unwrap()[0].clone();
			if let Some(pow) = p1[0].to_f64() {
				// Conversion only works if the imaginary part is 0
				if pow == pow.round() && pow >= 0. {
					self.ops_vec.pop();
					self.pols_vec.push(polynomial![c] << pow as usize);
					Ok(())
				} else {
					Err(ImpossiblePower(p2.to_string(), c.to_string()))
				}
			} else {
				Err(ImpossiblePower(p2.to_string(), c.to_string()))
			}
		} else {
			match op {
				Ops::Add => self.pols_vec.push(p2 + p1),
				Ops::Min => self.pols_vec.push(p2 - p1),
				Ops::Mul => self.pols_vec.push(p2 * p1),
				Ops::Div =>
					if p1.degree() == 0 {
						let c: Complex<T> = Complex::from(T::one()) / p1[0].clone();
						self.pols_vec.push(polynomial![c] * p2)
					} else {
						return Err(ImpossibleDivision(p2.to_string(), p1.to_string()));
					},
				Ops::Pow =>
					if p1.degree() == 0 {
						let c: Complex<f64> = to_complexf64(p1[0].clone()).unwrap();
						if p2.degree() == 0 {
							let c2: Complex<f64> = to_complexf64(p2[0].clone()).unwrap();
							self.pols_vec
							    .push(polynomial![from_complexf64(c2.powc(c)).unwrap()])
						} else if c.im.is_zero() && c.re == c.re.round() {
							if c.re.is_sign_negative() {
								return Err(ImpossiblePower(p2.to_string(), c.re.to_string()));
							}
							let r: Polynomial<Complex<T>> = p2.powi(c.re.to_usize().unwrap());
							self.pols_vec.push(r)
						} else {
							return Err(ImpossiblePower(p2.to_string(), p1[0].to_string()));
						}
					} else {
						return Err(ImpossiblePower2Polynomials(p2.to_string(), p1.to_string()));
					},
				Ops::Open => return Err(ImpossibleOpen),
			};
			Ok(())
		}
	}

	fn push_num(&mut self) -> Result<(), PolynomialError>
	{
		if self.reads_num {
			if self.is_factor {
				// i.e. if there was a factor before the number
				self.push_bin_operator(Ops::Mul)?;
			}
			if self.num == 0 {
				self.pols_vec.push(Polynomial::zero());
			} else {
				let reduced_num = T::from_f64(self.num.to_f64().unwrap() / 10u64.pow(self.nb_decs) as f64).expect("Cannot convert f64 to parsed type");
				let complex_num = match self.icount % 4 {
					0 => Complex::new(reduced_num, T::zero()),
					1 => Complex::new(T::zero(), reduced_num),
					2 => Complex::new(T::zero() - reduced_num, T::zero()),
					3 => Complex::new(T::zero(), T::zero() - reduced_num),
					_ => unreachable!(),
				};
				if self.is_min {
					self.pols_vec
					    .push(polynomial![Complex::<T>::zero() - complex_num]);
				} else {
					self.pols_vec.push(polynomial![complex_num]);
				}
			}
			self.icount = 0;
			self.num = 0;
			self.nb_decs = 0;
			self.reads_num = false;
			self.reads_dec = false;
			self.is_min = false;
			self.is_factor = true; // There can be a factor after the number
		}
		Ok(())
	}

	fn push_bin_operator(&mut self, op: Ops) -> Result<(), PolynomialError>
	{
		if self.is_min {
			Err(UnaryMinusFailed(op))
		} else {
			if self.ops_vec.len() != 0 {
				let p = op.prio();
				while self.ops_vec.len() != 0 && self.ops_vec[self.ops_vec.len() - 1].prio() >= p {
					self.execute_bin_operator()?;
				}
			}
			self.ops_vec.push(op);
			Ok(())
		}
	}

	fn read_digit(&mut self, n: u8)
	{
		if self.reads_dec {
			self.nb_decs += 1;
		}
		self.num = (n as u64) + 10 * (self.num);
		self.reads_num = true;
		self.unary_min = false;
	}

	fn read_i(&mut self)
	{
		self.icount += 1;
		if !self.reads_num {
			self.reads_num = true;
			self.num = 1u64;
		}
		self.unary_min = false;
	}

	fn read_char(&mut self, c: char) -> Result<(), PolynomialError>
	{
		#[allow(non_snake_case)]
		let X = polynomial![Complex::zero(), Complex::from(T::one())];
		match c {
			' ' => self.push_num()?,
			'+' => {
				self.push_num()?;
				self.push_bin_operator(Ops::Add)?;
				self.is_factor = false;
				self.unary_min = true;
			},
			'-' =>
				if self.unary_min {
					self.unary_min = false;
					self.is_min = true;
					self.is_factor = false;
				} else {
					self.push_num()?;
					self.push_bin_operator(Ops::Min)?;
					self.is_factor = false;
					self.unary_min = true;
				},
			'*' => {
				self.push_num()?;
				self.push_bin_operator(Ops::Mul)?;
				self.is_factor = false;
				self.unary_min = true;
			},
			'/' => {
				self.push_num()?;
				self.push_bin_operator(Ops::Div)?;
				self.is_factor = false;
				self.unary_min = true;
			},
			'^' => {
				self.push_num()?;
				self.push_bin_operator(Ops::Pow)?;
				self.is_factor = false;
				self.unary_min = true;
			},
			'X' | 'x' => {
				self.push_num()?;
				if self.is_factor {
					self.push_bin_operator(Ops::Mul)?;
				}
				self.is_factor = true;
				self.unary_min = false;
				if self.is_min {
					// In that case we have an expression like -X, which should only be seen at the
					// beginning of an expression
					self.is_min = false;
					self.pols_vec
					    .push(polynomial![Complex::from(T::zero() - T::one())]);
					self.push_bin_operator(Ops::Mul)?;
				}
				self.pols_vec.push(X);
			},
			'(' => {
				self.push_num()?;
				if self.is_factor {
					self.push_bin_operator(Ops::Mul)?;
				}
				self.ops_vec.push(Ops::Open);
				if self.is_min {
					// The idea is to turning any (...) into ((...)) and -(...) into (-1 * (...))
					self.is_min = false;
					self.pols_vec
					    .push(polynomial![Complex::from(T::zero() - T::one())]);
					self.push_bin_operator(Ops::Mul)?;
				}
				self.ops_vec.push(Ops::Open);
				self.is_factor = false;
				self.unary_min = true;
			},
			')' => {
				self.push_num()?;
				while self.ops_vec.len() != 0 && self.ops_vec[self.ops_vec.len() - 1] != Ops::Open {
					self.execute_bin_operator()?;
				}
				if self.ops_vec.len() == 0 || self.ops_vec.pop() != Some(Ops::Open) {
					return Err(ImpossibleClose);
				}
				// Doing it a second time because there always are two layers of parenthesis
				while self.ops_vec.len() != 0 && self.ops_vec[self.ops_vec.len() - 1] != Ops::Open {
					self.execute_bin_operator()?;
				}
				if self.ops_vec.len() == 0 || self.ops_vec.pop() != Some(Ops::Open) {
					return Err(ImpossibleClose);
				}
				self.is_min = false;
				self.is_factor = true;
				self.unary_min = false;
			},
			'.' => {
				self.reads_num = true;
				self.reads_dec = true;
				self.unary_min = false;
			},
			',' => {
				self.reads_num = true;
				self.reads_dec = true;
				self.unary_min = false;
			},
			'0' => self.read_digit(0),
			'1' => self.read_digit(1),
			'2' => self.read_digit(2),
			'3' => self.read_digit(3),
			'4' => self.read_digit(4),
			'5' => self.read_digit(5),
			'6' => self.read_digit(6),
			'7' => self.read_digit(7),
			'8' => self.read_digit(8),
			'9' => self.read_digit(9),
			'⁰' => {
				self.read_char('^')?;
				self.read_digit(0);
			},
			'¹' => {
				self.read_char('^')?;
				self.read_digit(1);
			},
			'²' => {
				self.read_char('^')?;
				self.read_digit(2);
			},
			'³' => {
				self.read_char('^')?;
				self.read_digit(3);
			},
			'⁴' => {
				self.read_char('^')?;
				self.read_digit(4);
			},
			'⁵' => {
				self.read_char('^')?;
				self.read_digit(5);
			},
			'⁶' => {
				self.read_char('^')?;
				self.read_digit(6);
			},
			'⁷' => {
				self.read_char('^')?;
				self.read_digit(7);
			},
			'⁸' => {
				self.read_char('^')?;
				self.read_digit(8);
			},
			'⁹' => {
				self.read_char('^')?;
				self.read_digit(9);
			},
			'i' | 'j' => {
				self.read_i();
			},
			_ => return Err(UnsupportedCharacter(c)),
		};
		Ok(())
	}
}

impl<T> Polynomial<Complex<T>> where T: Primitive
{
	fn parse_string_checked(s: String) -> Result<Self, PolynomialError>
	{
		let mut parser = ComplexParser::<T> { pols_vec:  Vec::new(),
		                                      ops_vec:   Vec::new(),
		                                      reads_num: false,
		                                      reads_dec: false,
		                                      num:       0u64,
		                                      nb_decs:   0u32,
		                                      unary_min: true,
		                                      is_min:    false,
		                                      is_factor: false,
		                                      icount:    0u32, };
		let s_chars: Vec<char> = s.chars().collect();
		for c in s_chars {
			parser.read_char(c)?;
		}
		parser.push_num()?;
		while parser.ops_vec.len() != 0 {
			parser.execute_bin_operator()?;
		}
		parser.pols_vec.pop().ok_or(EmptyStringInput)
	}

	pub fn parse_string(s: String) -> Result<Self, String>
	{
		match Self::parse_string_checked(s) {
			Ok(p) => Ok(p),
			Err(e) => Err(format!("{e}")),
		}
	}
}
