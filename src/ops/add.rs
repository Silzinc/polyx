use std::{
	fmt::Debug,
	ops::{Add, AddAssign, Neg, Sub, SubAssign},
};

use num_traits::Zero;

use super::inner_macros::*;
use crate::{traits::HasNorm, Polynomial};

//=================================================================================================

impl<T> Add<&Polynomial<T>> for &Polynomial<T>
where
	T: Add<T, Output = T> + Clone + Zero + Debug + HasNorm,
{
	// Implements addition without taking ownership
	/* Example:
	let p1 = polynomial![1, 0, 2];
	let p2 = polynomial![-1, 1, 5];
	assert_eq!(p1 + p2, polynomial![0, 1, 7]);
	*/
	type Output = Polynomial<T>;

	#[inline]
	fn add(self, other: &Polynomial<T>) -> Polynomial<T>
	where
		T: Zero,
	{
		if other.0.is_empty() {
			return self.clone();
		}
		if self.0.is_empty() {
			return other.clone();
		}
		if other.degree() > self.degree() {
			return other.add(self);
		}
		let mut sum = self.0.clone();
		for k in 0..=other.degree() {
			sum[k] = sum[k].clone() + other[k].clone();
		}
		Polynomial::from(sum)
	}
}

// False error by rust-analyzer
impl_op_all!(Add, AddAssign, add, add_assign);

// Negating a Polynomial
// ========================================================================================

impl<T> Neg for &Polynomial<T>
where
	T: Neg<Output = T> + Clone + Zero + HasNorm,
{
	// Implements negation without taking ownership
	/* Example:
	let p = polynomial![1, 0, 2];
	assert_eq!(-p, polynomial![-1, 0, -2]);
	*/
	type Output = Polynomial<T>;

	fn neg(self) -> Polynomial<T> {
		Polynomial::from(self.0.iter().map(|x| -x.clone()).collect::<Vec<T>>())
	}
}
impl<T> Neg for Polynomial<T>
where
	T: Neg<Output = T> + Clone + Zero + HasNorm,
{
	// Version that takes ownership
	type Output = Polynomial<T>;

	#[inline]
	fn neg(self) -> Polynomial<T> {
		-&self
	}
}

// Subtraction
// =================================================================================================

impl<T> Sub<&Polynomial<T>> for &Polynomial<T>
where
	T: Sub<Output = T> + Clone + Zero + HasNorm,
{
	// Implements subtraction without taking ownership
	/* Example:
	let p1 = polynomial![1, 0, 2];
	let p2 = polynomial![-1, 1, 5];
	assert_eq!(p1 - p2, polynomial![2, -1, -3]);
	*/
	type Output = Polynomial<T>;

	#[inline]
	fn sub(self, other: &Polynomial<T>) -> Polynomial<T> {
		if other.0.is_empty() {
			return self.clone();
		}
		if other.degree() > self.degree() || self.0.is_empty() {
			let mut minus_result = other.sub(self);
			for k in 0..minus_result.0.len() {
				minus_result[k] = T::zero() - minus_result[k].clone();
			}
			minus_result.clean_zeros();
			return minus_result;
		}
		let mut diff = self.0.clone();
		for k in 0..=other.degree() {
			diff[k] = diff[k].clone() - other[k].clone();
		}
		Polynomial::from(diff)
	}
}

impl_op_all!(Sub, SubAssign, sub, sub_assign);
