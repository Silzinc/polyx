use crate::convert;
use crate::Polynomial::{self, NonZero, Zero, X};
use duplicate::duplicate;
use num_traits::{Float, Pow, PrimInt, ToPrimitive};
use rustfft::{num_complex::Complex, FftPlanner};
use std::{
	fmt::Debug,
	ops::{Mul, MulAssign},
};

// Base multiplication with the Schönhage-Strassen algorithm
// There is an ugly conversion to f64 everytime to make the algorithm work
// TODO : rewrite fft to avoid this and have a more optimized version

impl<T: Float + ToPrimitive> Mul<&Polynomial<T>> for &Polynomial<T>
{
	type Output = Polynomial<T>;

	fn mul(self, other: &Polynomial<T>) -> Polynomial<T>
	{
		let zero = T::from(0).unwrap();
		match (self, other) {
			(Zero, _) => Zero,
			(_, Zero) => Zero,
			(X, X) => NonZero(vec![zero, zero, T::from(1).unwrap()]),
			(NonZero(coefs), X) => {
				let mut new_coefs = vec![zero; coefs.len() + 1];
				for k in 1..=coefs.len() {
					new_coefs[k] = coefs[k - 1];
				}
				NonZero(new_coefs)
			},
			(X, _) => other.mul(self),
			(NonZero(coefs), NonZero(other_coefs)) =>
				if coefs.len() == 1 {
					let c = coefs[0];
					Polynomial::<T>::from(Vec::from_iter(other_coefs.iter().map(|&x| c * x)))
				} else if other_coefs.len() == 1 {
					let c = other_coefs[0];
					Polynomial::<T>::from(Vec::from_iter(coefs.iter().map(|&x| c * x)))
				} else {
					let n = coefs.len() + other_coefs.len() - 1;
					let mut planner = FftPlanner::new();
					let mut buffer = vec![Complex { re: 0f64, im: 0f64 }; n];

					{
						let mut other_buffer = vec![Complex { re: 0f64, im: 0f64 }; n];

						for k in 0..coefs.len() {
							buffer[k].re = coefs[k].to_f64().unwrap();
						}
						for k in 0..other_coefs.len() {
							other_buffer[k].re = other_coefs[k].to_f64().unwrap();
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

					let new_coefs: Vec<T> = buffer.iter().map(|&x| T::from(x.re / n as f64).unwrap()).collect();
					Polynomial::<T>::from(new_coefs)
				},
		}
	}
}

// Ownership taking versions

impl<T: Float> Mul<&Polynomial<T>> for Polynomial<T>
{
	type Output = Polynomial<T>;

	fn mul(self, other: &Polynomial<T>) -> Polynomial<T> { &self * other }
}
impl<T: Float> Mul<Polynomial<T>> for &Polynomial<T>
{
	type Output = Polynomial<T>;

	fn mul(self, other: Polynomial<T>) -> Polynomial<T> { self * &other }
}
impl<T: Float> Mul<Polynomial<T>> for Polynomial<T>
{
	type Output = Polynomial<T>;

	fn mul(self, other: Polynomial<T>) -> Polynomial<T> { &self * &other }
}

// Multiplication by constant

impl<T: ToPrimitive, F: Float> Mul<T> for &Polynomial<F>
{
	type Output = Polynomial<F>;
	fn mul(self, other: T) -> Polynomial<F> { self * Polynomial::from(vec![convert(other)]) }
}
impl<T: ToPrimitive, F: Float> Mul<T> for Polynomial<F>
{
	type Output = Polynomial<F>;

	fn mul(self, other: T) -> Polynomial<F> { self * Polynomial::from(vec![convert(other)]) }
}

duplicate! {
	[primitive_type; [f64]; [f32]; [i8]; [i16]; [i32]; [i64]; [isize]; [i128]; [u8]; [u16]; [u32]; [u64]; [usize]; [u128]]

	impl<T: Float> Mul<Polynomial<T>> for primitive_type
	{
		type Output = Polynomial<T>;
		fn mul(self, other: Polynomial<T>) -> Polynomial<T> { other * self }
	}
	impl<T: Float> Mul<&Polynomial<T>> for primitive_type
	{
		type Output = Polynomial<T>;
		fn mul(self, other: &Polynomial<T>) -> Polynomial<T> { other * self }
	}
}

// MulAssign implementations
// ====================================================================================

impl<T: Float> MulAssign<&Polynomial<T>> for Polynomial<T>
{
	fn mul_assign(&mut self, other: &Polynomial<T>) { *self = *self * other }
}
impl<T: Float> MulAssign<Polynomial<T>> for Polynomial<T>
{
	fn mul_assign(&mut self, other: Polynomial<T>) { *self = *self * other }
}
impl<T: ToPrimitive, F: Float> MulAssign<T> for Polynomial<F>
{
	fn mul_assign(&mut self, other: T) { *self *= Polynomial::from(vec![convert(other)]) }
}

// Fast tail recursive exponentiation

fn pow_aux<F: Float>(p: &Polynomial<F>, n: usize, r: Polynomial<F>) -> Result<Polynomial<F>, String>
{
	if n == 0 {
		Ok(r)
	} else if n % 2 == 0 {
		pow_aux(&(p * p), n / 2, r)
	} else {
		pow_aux(&(p * p), (n - 1) / 2, p * r)
	}
}

impl<T, F> Pow<T> for &Polynomial<F>
where
	T: PrimInt + Debug,
	F: Float,
{
	type Output = Result<Polynomial<F>, String>;

	fn pow(self, other: T) -> Result<Polynomial<F>, String>
	{
		let n = other.to_usize().ok_or(format!("Impossible to raise a non constant polynomial to power {other:?}"))?;
		match self {
			Zero => Ok(Zero),
			X => {
				let mut coefs = vec![convert(0); n + 1];
				coefs[n] = F::from(1).unwrap();
				Ok(NonZero(coefs))
			},
			_ => pow_aux(self, n, NonZero(vec![convert(1)])),
		}
	}
}
impl<T: PrimInt + Debug, F: Float> Pow<T> for Polynomial<F>
{
	type Output = Result<Polynomial<F>, String>;

	fn pow(self, other: T) -> Result<Polynomial<F>, String> { (&self).pow(other) }
}
