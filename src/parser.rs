use crate::{
	errors::PolynomialError::{self, *},
	Polynomial,
};
use num_traits::{FromPrimitive, One, ToPrimitive, Zero};
use std::{
	fmt::{self, Debug},
	ops::{Add, Div, Mul, Neg, Sub},
};

#[derive(PartialEq, Debug, Clone, Copy)]
pub(crate) enum Ops
{
	Add,
	Min,
	Mul,
	Div,
	Pow,
	Open,
}

impl fmt::Display for Ops
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		match self {
			Ops::Add => write!(f, "{}", "'+'"),
			Ops::Min => write!(f, "{}", "'-'"),
			Ops::Mul => write!(f, "{}", "'*'"),
			Ops::Div => write!(f, "{}", "'/'"),
			Ops::Pow => write!(f, "{}", "'^'"),
			Ops::Open => write!(f, "{}", "'('"),
		}
	}
}

impl Ops
{
	fn prio(&self) -> u32
	{
		match self {
			Ops::Mul => 2,
			Ops::Div => 2,
			Ops::Pow => 3,
			Ops::Open => 0,
			_ => 1,
		}
	}
}

struct Parser<T>
{
	pols_vec:  Vec<Polynomial<T>>,
	ops_vec:   Vec<Ops>,
	reads_num: bool,
	reads_dec: bool,
	num:       u64,
	nb_decs:   u32,
	unary_min: bool,
	is_min:    bool,
	is_factor: bool,
}

impl<T> Parser<T>
	where T: FromPrimitive
	        + ToPrimitive
	        + Zero
	        + One
	        + Mul<T, Output = T>
	        + Add<T, Output = T>
	        + Clone
	        + PartialEq
	        + Div<T, Output = T>
	        + Sub<T, Output = T>
	        + Neg<Output = T>
	        + Debug
{
	fn execute_bin_operator(&mut self) -> Result<(), PolynomialError<T>>
	{
		#[allow(non_snake_case)]
		let X = crate::polynomial![T::zero(), T::one()];
		// println!("Got {:?} and {:?}", self.pols_vec, op);
		let op: Ops = self.ops_vec.pop().ok_or(NoBinaryOperator)?;
		let p1: Polynomial<T> = self.pols_vec.pop().ok_or(BinaryOperatorZeroOperand(op))?;
		let p2: Polynomial<T> = self.pols_vec
		                            .pop()
		                            .ok_or(BinaryOperatorOneOperand(op, p1.as_string()))?;

		if op == Ops::Mul
		   && self.ops_vec.len() != 0
		   && self.ops_vec[self.ops_vec.len() - 1] == Ops::Pow
		   && self.pols_vec.len() != 0
		   && self.pols_vec[self.pols_vec.len() - 1].degree() == 0
		   && p1.degree() == 0
		   && p2 == X
		{
			if self.pols_vec[self.pols_vec.len() - 1].is_zero() {
				return Err(ImpossiblePower(p2.as_string(), p1[0].clone()));
			}
			// This is to optimize a bit the parsing of an expression like "cX^pow"
			let c: T = self.pols_vec.pop().unwrap()[0].clone();
			let pow: f64 = p1[0].to_f64().unwrap();
			if pow == pow.round() && pow >= 0. {
				let l: usize = pow as usize + 1;
				let mut p = crate::polynomial![T::zero(); l];
				p[l - 1] = c;
				self.ops_vec.pop();
				self.pols_vec.push(p);
				Ok(())
			} else {
				Err(ImpossiblePower(p2.as_string(), c))
			}
		} else {
			match op {
				Ops::Add => self.pols_vec.push(p2 + p1),
				Ops::Min => self.pols_vec.push(p2 - p1),
				Ops::Mul => self.pols_vec.push(p2 * p1),
				Ops::Div =>
					if p1.degree() == 0 {
						let c: T = T::one() / p1[0].clone();
						self.pols_vec.push(crate::polynomial![c] * p2)
					} else {
						return Err(ImpossibleDivision(p2.as_string(), p1.as_string()));
					},
				Ops::Pow =>
					if p1.degree() == 0 {
						let c: f64 = p1[0].to_f64().unwrap();
						if p2.degree() == 0 {
							let c2: f64 = p2[0].to_f64().unwrap();
							self.pols_vec
							    .push(crate::polynomial![T::from_f64(c2.powf(c)).unwrap()])
						} else if c == c.round() {
							if c.is_sign_negative() {
								return Err(ImpossiblePower(p2.as_string(), p1[0].clone()));
							}
							let r: Polynomial<T> = p2.powi(c.to_usize().unwrap());
							self.pols_vec.push(r)
						} else {
							return Err(ImpossiblePower(p2.as_string(), p1[0].clone()));
						}
					} else {
						return Err(ImpossiblePower2Polynomials(p2.as_string(), p1.as_string()));
					},
				Ops::Open => return Err(ImpossibleOpen),
			};
			Ok(())
		}
	}

	fn push_num(&mut self) -> Result<(), PolynomialError<T>>
	{
		if self.reads_num {
			if self.is_factor {
				// i.e. if there was a factor before the number
				self.push_bin_operator(Ops::Mul)?;
			}
			if self.num == 0 {
				self.pols_vec.push(Polynomial::zero());
			} else {
				if self.is_min {
					self.pols_vec.push(crate::polynomial![T::from_f64(
						-(self.num as f64) / 10f64.powi(self.nb_decs as i32)
					).unwrap()]);
				} else {
					self.pols_vec.push(crate::polynomial![T::from_f64(
						self.num as f64 / 10f64.powi(self.nb_decs as i32)
					).unwrap()]);
				}
			}
			self.num = 0;
			self.nb_decs = 0;
			self.reads_num = false;
			self.reads_dec = false;
			self.is_min = false;
			self.is_factor = true; // There can be a factor after the number
		}
		Ok(())
	}

	fn push_bin_operator(&mut self, op: Ops) -> Result<(), PolynomialError<T>>
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

	fn read_char(&mut self, c: char) -> Result<(), PolynomialError<T>>
	{
		#[allow(non_snake_case)]
		let X = crate::polynomial![T::zero(), T::one()];
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
					self.pols_vec.push(crate::polynomial![T::zero() - T::one()]);
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
					self.pols_vec.push(crate::polynomial![T::zero() - T::one()]);
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
			_ => return Err(UnsupportedCharacter(c)),
		};
		Ok(())
	}
}

fn _parse_string<T>(s: String) -> Result<Polynomial<T>, PolynomialError<T>>
	where T: FromPrimitive
	        + ToPrimitive
	        + Zero
	        + One
	        + Mul<T, Output = T>
	        + Add<T, Output = T>
	        + Clone
	        + PartialEq
	        + Div<T, Output = T>
	        + Sub<T, Output = T>
	        + Neg<Output = T>
	        + Debug
{
	let mut parser = Parser::<T> { pols_vec:  Vec::new(),
	                               ops_vec:   Vec::new(),
	                               reads_num: false,
	                               reads_dec: false,
	                               num:       0u64,
	                               nb_decs:   0u32,
	                               unary_min: true,
	                               is_min:    false,
	                               is_factor: false, };
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

pub fn parse_string<T>(s: String) -> Result<Polynomial<T>, String>
	where T: FromPrimitive
	        + ToPrimitive
	        + Zero
	        + One
	        + Mul<T, Output = T>
	        + Add<T, Output = T>
	        + Clone
	        + PartialEq
	        + Div<T, Output = T>
	        + Sub<T, Output = T>
	        + Neg<Output = T>
	        + Debug
{
	match _parse_string(s) {
		Ok(p) => Ok(p),
		Err(_e) => Err("".to_string()), // Err(e.description()),
	}
}

#[macro_export]
macro_rules! polynomial_expr {
	($($e:expr)*) => {
		parse_string(stringify!($($e)*).to_string())
	};
}
