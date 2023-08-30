use crate::impl_op_all;
use crate::Polynomial;
use num_traits::Zero;
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

//=================================================================================================

impl<T> Add<&Polynomial<T>> for &Polynomial<T>
where T: Add<T, Output = T> + Clone + Zero
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
	where T: Zero
	{
		if other.0.len() == 0 {
			return self.clone();
		}
		if self.0.len() == 0 {
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

impl_op_all!(Add, AddAssign, add, add_assign);

// Negating a Polynomial
// ========================================================================================

impl<T> Neg for &Polynomial<T>
where T: Neg<Output = T> + Clone + Zero
{
	// Implements negation without taking ownership
	/* Example:
	let p = polynomial![1, 0, 2];
	assert_eq!(-p, polynomial![-1, 0, -2]);
	*/
	type Output = Polynomial<T>;
	fn neg(self) -> Polynomial<T> { Polynomial::from(self.0.iter().map(|x| -x.clone()).collect::<Vec<T>>()) }
}
impl<T> Neg for Polynomial<T>
where T: Neg<Output = T> + Clone + Zero
{
	// Version that takes ownership
	type Output = Polynomial<T>;
	#[inline]
	fn neg(self) -> Polynomial<T> { -&self }
}

// Subtraction
// =================================================================================================

impl<T> Sub<&Polynomial<T>> for &Polynomial<T>
where T: Sub<T, Output = T> + Clone + Neg<Output = T> + Zero
{
	// Implements subtraction without taking ownership
	/* Example:
	let p1 = polynomial![1, 0, 2];
	let p2 = polynomial![-1, 1, 5];
	assert_eq!(p1 - p2, polynomial![2, -1, -3]);
	*/
	type Output = Polynomial<T>;
	#[inline]
	fn sub(self, other: &Polynomial<T>) -> Polynomial<T>
	{
		if other.0.len() == 0 {
			return self.clone();
		}
		if self.0.len() == 0 {
			return -other;
		}
		if other.degree() > self.degree() {
			return -(other.sub(self));
		}
		let mut diff = self.0.clone();
		for k in 0..=other.degree() {
			diff[k] = diff[k].clone() - other[k].clone();
		}
		Polynomial::from(diff)
	}
}

impl_op_all!(Sub, SubAssign, sub, sub_assign);
