use crate::{consts::TOL, traits::HasNorm, Polynomial};
use num_traits::{One, Zero};
use std::{convert::From, default::Default, fmt::Debug, ops::Sub};

#[macro_export]
macro_rules! polynomial {
  ($($x:expr),*) => (Polynomial::from(vec![$($x),*]));
  ($($x:expr);*) => (Polynomial::from(vec![$($x);*]));
}

impl<T> Zero for Polynomial<T> where T: Clone + Zero + Debug + HasNorm
{
	#[inline]
	fn zero() -> Self { Polynomial(Vec::new()) }

	#[inline]
	fn is_zero(&self) -> bool { self.0.is_empty() }
}

impl<T> One for Polynomial<T> where T: Clone + Zero + Debug + One + Sub<Output = T> + HasNorm
{
	#[inline]
	fn one() -> Self { Polynomial(vec![T::one()]) }
}

impl<T> Default for Polynomial<T>
{
	#[inline]
	fn default() -> Self { Polynomial(Vec::new()) }
}

impl<T> Polynomial<T>
{
	#[inline]
	pub fn new() -> Self { Polynomial(Vec::new()) }

	#[inline]
	pub fn is_empty(&self) -> bool { self.0.is_empty() }
}

impl<T> From<Vec<T>> for Polynomial<T> where T: Zero + Clone + HasNorm
{
	#[inline]
	fn from(mut values: Vec<T>) -> Self
	{
		let effective_len = match values.iter().rposition(|x| x.norm() > TOL) {
			Some(index) => index + 1,
			None => 0,
		};
		values.truncate(effective_len);
		Polynomial(values)
	}
}

impl<T: HasNorm> Polynomial<T>
{
	#[inline]
	pub fn clean_zeros(&mut self)
	{
		let effective_len = match self.into_iter().rposition(|x| x.norm() > TOL) {
			Some(index) => index + 1,
			None => 0,
		};
		self.0.truncate(effective_len);
	}
}

impl<T> From<&[T]> for Polynomial<T> where T: Zero + Clone + HasNorm
{
	#[inline]
	fn from(values: &[T]) -> Self { Polynomial::from(values.to_vec()) }
}

impl<T> From<&Vec<T>> for Polynomial<T> where T: Zero + Clone + HasNorm
{
	#[inline]
	fn from(values: &Vec<T>) -> Self { Polynomial::from(values.clone()) }
}

impl<T> From<T> for Polynomial<T> where T: Zero + Clone + HasNorm
{
	#[inline]
	fn from(value: T) -> Self { Polynomial::from(vec![value]) }
}

impl<T> FromIterator<T> for Polynomial<T> where T: Zero + Clone + HasNorm
{
	#[inline]
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self
	{
		Polynomial::from(Vec::from_iter(iter.into_iter()))
	}
}
