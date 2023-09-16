use super::inner_macros::*;
use crate::Polynomial;
use num_traits::{One, Zero};
use std::fmt::Debug;
use std::ops::{Mul, MulAssign, Sub};

impl<T> Mul<&Polynomial<T>> for &Polynomial<T>
	where T: Mul<T, Output = T> + Sub<T, Output = T> + Clone + Zero + Debug
{
	type Output = Polynomial<T>;

	#[inline]
	fn mul(self, other: &Polynomial<T>) -> Polynomial<T>
	{
		Polynomial::karatsuba(self, other, self.degree() + other.degree() + 1)
	}
}

impl_op_all!(Mul, MulAssign, mul, mul_assign, Sub);

impl<T> Polynomial<T>
	where T: Mul<T, Output = T> + Sub<T, Output = T> + Clone + Zero + One + PartialEq + Debug
{
	// Implements fast integer exponentiation
	/* Example:
	let p = polynomial![1, 0, 2];
	assert_eq!(p.powi(3), polynomial![1, 0, 6, 0, 4]);
	*/
	#[inline]
	fn powi_aux(p: &Polynomial<T>, n: usize, r: Polynomial<T>) -> Polynomial<T>
	{
		if n == 0 {
			r
		} else if n & 1 == 0 {
			// n & 1 is the same as n % 2
			Self::powi_aux(&(p * p), n / 2, r)
		} else {
			Self::powi_aux(&(p * p), (n - 1) / 2, p * r)
		}
	}

	#[inline]
	pub fn powi<U: Into<usize>>(&self, exp: U) -> Polynomial<T>
	{
		let n: usize = exp.into();
		if self.is_zero() {
			return Self::zero();
		}
		if self.is_x() {
			let mut coefs = vec![T::zero(); n + 1];
			coefs[n] = T::one();
			return Self::from(coefs);
		}
		Self::powi_aux(self, n, crate::polynomial![T::one()])
	}
}

impl<T> Polynomial<T> where T: Mul<T, Output = T> + Sub<T, Output = T> + Clone + Zero + Debug
{
	#[inline]
	// Returns p1 * p2 modulo modulus
	pub fn short_product(p1: &Self, p2: &Self, modulus: usize) -> Self
	{
		Self::karatsuba(p1, p2, modulus)
	}
}
