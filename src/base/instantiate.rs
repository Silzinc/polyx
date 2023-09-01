use crate::Polynomial;
use num_traits::Zero;
use std::{convert::From, default::Default, fmt::Debug};

impl<T> Zero for Polynomial<T> where T: Clone + Zero + Debug
{
	#[inline]
	fn zero() -> Self { Polynomial(Vec::new()) }

	#[inline]
	fn is_zero(&self) -> bool { self.0.is_empty() }
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

impl<T> From<Vec<T>> for Polynomial<T> where T: Zero + Clone
{
	#[inline]
	fn from(mut values: Vec<T>) -> Self
	{
		let effective_len = match values.iter().rposition(|x| !x.is_zero()) {
			Some(index) => index + 1,
			None => 0,
		};
		values.truncate(effective_len);
		Polynomial(values)
	}
}
