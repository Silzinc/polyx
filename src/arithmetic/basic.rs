use crate::Polynomial;
use num_traits::{One, Zero};
use std::ops::{Add, Index, IndexMut, Mul};

// Some basic implementations needed elsewhere

impl<T: Zero + Clone> Index<usize> for Polynomial<T>
{
	// Gives the coefficient of the monomial of degree index
	/* Example:
	let p = polynomial![1, 0, 2];
	assert_eq!(p[0], 1);
	*/
	type Output = T;
	#[inline]
	fn index(&self, index: usize) -> &Self::Output
	{
		if index > self.degree() || self.is_zero() {
			&T::zero()
		} else {
			&(self.0)[index]
		}
	}
}

impl<T: Zero + Clone> IndexMut<usize> for Polynomial<T>
{
	#[inline]
	fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self[index] }
}

macro_rules! impl_iter {
	// Implementations to iterate over the coefficients of a polynomial
	($t:ty, $item_spec:ty, $iter_type:ty, $method:ident) => {
		impl<'a, T> IntoIterator for $t
		{
			type Item = $item_spec;
			type IntoIter = $iter_type;
			#[inline]
			fn into_iter(self) -> Self::IntoIter { self.0.$method() }
		}
	};
}

impl_iter!(Polynomial<T>, T, std::vec::IntoIter<T>, into_iter);
impl_iter!(&'a Polynomial<T>, &'a T, std::slice::Iter<'a, T>, iter);
impl_iter!(&'a mut Polynomial<T>, &'a mut T, std::slice::IterMut<'a, T>, iter_mut);

impl<T> FromIterator<T> for Polynomial<T>
{
	#[inline]
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self { Polynomial::from(iter.into_iter().collect()) }
}

impl<T> Polynomial<T>
{
	// Gives the degree of self
	// Note that the degree of the zero polynomial is 0 to avoid usize underflow
	// The is_zero() method is preferred to distinguish the zero polynomial from the
	// other constant polynomials
	/* Example:
	let p = polynomial![1, 0, 2];
	assert_eq!(p.degree(), 2);
	*/
	#[inline]
	pub fn degree(&self) -> usize
	{
		if self.0.len() == 0 {
			0
		} else {
			self.0.len() - 1
		}
	}
}

impl<T> Polynomial<T>
where T: One + Zero + PartialEq + Clone
{
	#[inline]
	pub fn is_x(&self) -> bool { self.degree() == 1 && self[1] == T::one() && self[0] == T::zero() }
}

impl<T> Polynomial<T>
where T: Zero + Mul<T, Output = T> + Add<T, Output = T>
{
	// Evaluates self(x)
	/* Example:
	let p = polynomial![1, 0, 2];
	assert_eq!(p.eval(2), 9);
	*/
	#[inline]
	pub fn eval<U: Into<T>>(&self, _x: U) -> T
	{
		// Computes self(x)
		let x: T = _x.into();
		let mut result = T::zero();
		for coef in self.into_iter().rev() {
			result = x * result + *coef;
		}
		result
	}
}
