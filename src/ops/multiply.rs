use crate::Polynomial;
use crate::Polynomial::{NonZero, Zero, X};
use num_traits::{Pow, PrimInt, ToPrimitive};
use rustfft::{num_complex::Complex, FftPlanner};
use std::{
	fmt::Debug,
	ops::{Mul, MulAssign},
};

// Base multiplication with the Schönhage-Strassen algorithm

impl Mul<&Polynomial> for &Polynomial
{
	type Output = Polynomial;

	fn mul(self, other: &Polynomial) -> Polynomial
	{
		match (self, other) {
			(Zero, _) => Zero,
			(_, Zero) => Zero,
			(X, X) => NonZero(vec![0f64, 0f64, 1f64]),
			(NonZero(coefs), X) => {
				let mut new_coefs = vec![0f64; coefs.len() + 1];
				for k in 1..=coefs.len() {
					new_coefs[k] = coefs[k - 1];
				}
				NonZero(new_coefs)
			}
			(X, _) => other.mul(self),
			(NonZero(coefs), NonZero(other_coefs)) => {
				if coefs.len() == 1 {
					let c: f64 = coefs[0];
					Polynomial::from(Vec::from_iter(other_coefs.iter().map(|x| c * x)))
				} else if other_coefs.len() == 1 {
					let c: f64 = other_coefs[0];
					Polynomial::from(Vec::from_iter(coefs.iter().map(|x| c * x)))
				} else {
					let n = coefs.len() + other_coefs.len() - 1;
					let mut planner = FftPlanner::new();
					let mut buffer = vec![Complex { re: 0f64, im: 0f64 }; n];

					{
						let mut other_buffer = vec![Complex { re: 0f64, im: 0f64 }; n];

						for k in 0..coefs.len() {
							buffer[k].re = coefs[k];
						}
						for k in 0..other_coefs.len() {
							other_buffer[k].re = other_coefs[k];
						}

						let fft = planner.plan_fft_forward(n);
						fft.process(&mut buffer);
						fft.process(&mut other_buffer);

						for k in 0..n {
							buffer[k] *= other_buffer[k];
						}
					}

					let fft_inv = planner.plan_fft_inverse(n);
					fft_inv.process(&mut buffer);

					let new_coefs = Vec::from_iter(buffer.iter().map(|x| x.re / n as f64));
					Polynomial::from(new_coefs)
				}
			}
		}
	}
}

// Ownership taking versions

impl Mul<&Polynomial> for Polynomial
{
	type Output = Polynomial;

	fn mul(self, other: &Polynomial) -> Polynomial { &self * other }
}
impl Mul<Polynomial> for &Polynomial
{
	type Output = Polynomial;

	fn mul(self, other: Polynomial) -> Polynomial { self * &other }
}
impl Mul<Polynomial> for Polynomial
{
	type Output = Polynomial;

	fn mul(self, other: Polynomial) -> Polynomial { &self * &other }
}

// MulAssign

impl MulAssign<&Polynomial> for Polynomial
{
	fn mul_assign(&mut self, other: &Polynomial)
	{
		match self {
			Zero => (),
			X => {
				*self = &X * other;
			}
			NonZero(coefs) => {
				*self = &NonZero((*coefs).clone()) * other;
			}
		};
	}
}
impl MulAssign<Polynomial> for Polynomial
{
	fn mul_assign(&mut self, other: Polynomial) { *self *= &other; }
}

// Multiplication by constant

impl<T: ToPrimitive + Debug> Mul<T> for &Polynomial
{
	type Output = Polynomial;

	fn mul(self, other: T) -> Polynomial
	{
		let x: f64 = match other.to_f64() {
			Some(y) => y,
			None => panic!("Error when converting {:?} to f64", other),
		};
		self * &Polynomial::from(vec![x])
	}
}
impl<T: ToPrimitive + Debug> Mul<T> for Polynomial
{
	type Output = Polynomial;

	fn mul(self, other: T) -> Polynomial
	{
		let x: f64 = match other.to_f64() {
			Some(y) => y,
			None => panic!("Error when converting {:?} to f64", other),
		};
		&self * &Polynomial::from(vec![x])
	}
}
impl Mul<Polynomial> for f64
{
	type Output = Polynomial;
	fn mul(self, other: Polynomial) -> Polynomial { other * self }
}
impl Mul<&Polynomial> for f64
{
	type Output = Polynomial;
	fn mul(self, other: &Polynomial) -> Polynomial { other * self }
}
impl Mul<Polynomial> for f32
{
	type Output = Polynomial;
	fn mul(self, other: Polynomial) -> Polynomial { other * (self as f64) }
}
impl Mul<&Polynomial> for f32
{
	type Output = Polynomial;
	fn mul(self, other: &Polynomial) -> Polynomial { other * (self as f64) }
}
impl Mul<Polynomial> for i32
{
	type Output = Polynomial;
	fn mul(self, other: Polynomial) -> Polynomial { other * (self as f64) }
}
impl Mul<&Polynomial> for i32
{
	type Output = Polynomial;
	fn mul(self, other: &Polynomial) -> Polynomial { other * (self as f64) }
}
impl Mul<Polynomial> for i64
{
	type Output = Polynomial;
	fn mul(self, other: Polynomial) -> Polynomial { other * (self as f64) }
}
impl Mul<&Polynomial> for i64
{
	type Output = Polynomial;
	fn mul(self, other: &Polynomial) -> Polynomial { other * (self as f64) }
}
impl<T: ToPrimitive + Debug> MulAssign<T> for Polynomial
{
	fn mul_assign(&mut self, other: T)
	{
		let x: f64 = match other.to_f64() {
			Some(y) => y,
			None => panic!("Error when converting {:?} to f64", other),
		};
		*self *= Polynomial::from(vec![x]);
	}
}

// Fast tail recursive exponentiation

impl<T: PrimInt + ToPrimitive + Debug> Pow<T> for &Polynomial
{
	type Output = Result<Polynomial, String>;

	fn pow(self, other: T) -> Result<Polynomial, String>
	{
		let n: usize = match other.to_usize() {
			Some(x) => x,
			None => return Err(format!("Impossible to compute a negative power for a polynomial, trying to compute ({})^({:?})", self, other)),
		};
		fn aux(p: &Polynomial, n: usize, r: Polynomial) -> Result<Polynomial, String>
		{
			if n == 0 {
				Ok(r)
			} else if n % 2 == 0 {
				aux(&(p * p), n / 2, r)
			} else {
				aux(&(p * p), (n - 1) / 2, p * r)
			}
		}
		match self {
			Zero => Ok(Zero),
			X => {
				let mut coefs = vec![0f64; n + 1];
				coefs[n] = 1.;
				Ok(NonZero(coefs))
			}
			_ => aux(self, n, NonZero(vec![1.])),
		}
	}
}
impl<T: PrimInt + Debug> Pow<T> for Polynomial
{
	type Output = Result<Polynomial, String>;

	fn pow(self, other: T) -> Result<Polynomial, String> { (&self).pow(other) }
}
