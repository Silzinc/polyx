use std::ops::{Shl, ShlAssign, Shr, ShrAssign};

use num_traits::{PrimInt, Zero};

use crate::Polynomial;

/// Shifts the degree of a polynomial to the left by a given integer amount.
///
/// # Examples
///
/// ```
/// use polyx::Polynomial;
///
/// let p = Polynomial::from(vec![1, 2, 3]);
/// let q = &p << 2;
/// assert_eq!(q, Polynomial::from(vec![0, 0, 1, 2, 3]));
/// ```
#[allow(clippy::suspicious_arithmetic_impl)]
impl<T, I> Shl<I> for &Polynomial<T>
where
	T: Clone + Zero,
	I: PrimInt,
{
	type Output = Polynomial<T>;

	#[inline]
	fn shl(self, rhs_: I) -> Self::Output {
		let rhs = rhs_
			.to_usize()
			.unwrap_or_else(|| panic!("Tried to shift a polynomial by a negative integer"));
		let mut new_coefficients = Vec::with_capacity(self.0.len() + rhs);
		new_coefficients.resize(rhs, T::zero());
		new_coefficients.extend(self.0.iter().cloned());
		Polynomial(new_coefficients)
	}
}

impl<T, I> Shl<I> for Polynomial<T>
where
	T: Clone + Zero,
	I: PrimInt,
{
	type Output = Polynomial<T>;

	#[inline]
	fn shl(self, rhs_: I) -> Self::Output {
		&self << rhs_
	}
}

impl<T, I> ShlAssign<I> for Polynomial<T>
where
	T: Clone + Zero,
	I: PrimInt,
{
	#[inline]
	fn shl_assign(&mut self, rhs_: I) {
		let rhs = rhs_
			.to_usize()
			.unwrap_or_else(|| panic!("Tried to shift a polynomial by a negative integer"));
		let d = self.degree();
		self.0.reserve(rhs);
		self.0.resize(d + 1 + rhs, T::zero());
		for i in (0..=d).rev() {
			self.0[i + rhs] = self.0[i].clone();
		}
		for i in 0..rhs {
			self.0[i] = T::zero();
		}
	}
}

/// Right shift the polynomial by a given integer degree.
///
/// # Examples
///
/// ```
/// use polyx::Polynomial;
///
/// let p = Polynomial::from(vec![1, 2, 3]);
/// assert_eq!(p >> 1, Polynomial::from(vec![2, 3]));
/// ```
impl<T, I> Shr<I> for &Polynomial<T>
where
	T: Clone,
	I: PrimInt,
{
	type Output = Polynomial<T>;

	#[inline]
	fn shr(self, rhs_: I) -> Self::Output {
		let rhs = rhs_
			.to_usize()
			.unwrap_or_else(|| panic!("Tried to shift a polynomial by a negative integer"));
		if rhs >= self.0.len() {
			Polynomial(Vec::new())
		} else {
			Polynomial(self.0[rhs..].to_vec())
		}
	}
}

impl<T, I> Shr<I> for Polynomial<T>
where
	T: Clone,
	I: PrimInt,
{
	type Output = Polynomial<T>;

	#[inline]
	fn shr(self, rhs_: I) -> Self::Output {
		&self >> rhs_
	}
}

impl<T, I> ShrAssign<I> for Polynomial<T>
where
	T: Clone,
	I: PrimInt,
{
	#[inline]
	fn shr_assign(&mut self, rhs_: I) {
		let rhs = rhs_
			.to_usize()
			.unwrap_or_else(|| panic!("Tried to shift a polynomial by a negative integer"));
		if rhs >= self.0.len() {
			self.0.clear();
		} else {
			self.0.drain(0..rhs);
		}
	}
}
