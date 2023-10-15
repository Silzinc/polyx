use crate::{traits::HasNorm, Polynomial};
use num_traits::{One, Zero};
use std::ops::{Add, Index, IndexMut, Mul};

// Some basic implementations needed elsewhere

impl<T: Zero + Clone> Index<usize> for Polynomial<T>
{
	type Output = T;

	/// Gives a reference to the coefficient of the monomial of degree index
	/// Example:
	/// ```rust
	/// use polyx::*;
	/// let p = polynomial![1, 0, 2];
	/// assert_eq!(p[0], 1);
	/// ```
	#[inline]
	fn index(&self, index: usize) -> &Self::Output { &(self.0)[index] }
}

impl<T: Zero + Clone> IndexMut<usize> for Polynomial<T>
{
	#[inline]
	fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut (self.0)[index] }
}

macro_rules! impl_iter {
	// Implementations to iterate over the coefficients of a polynomial
	($t:ty, $item_spec:ty, $iter_type:ty, $method:ident) => {
		impl<'a, T> IntoIterator for $t
		{
			type IntoIter = $iter_type;
			type Item = $item_spec;

			#[inline]
			fn into_iter(self) -> Self::IntoIter { self.0.$method() }
		}
	};
}

impl_iter!(Polynomial<T>, T, std::vec::IntoIter<T>, into_iter);
impl_iter!(&'a Polynomial<T>, &'a T, std::slice::Iter<'a, T>, iter);
impl_iter!(&'a mut Polynomial<T>, &'a mut T, std::slice::IterMut<'a, T>, iter_mut);

impl<T> Polynomial<T>
{
	/// Returns the degree of the polynomial.
	/// Note that the degree of the zero polynomial is 0 to avoid usize underflow
	/// The `is_zero` method is preferred to distinguish the zero polynomial from
	/// the other constant polynomials
	/// Example:
	/// ```rust
	/// use polyx::*;
	/// let p = polynomial![1, 0, 2];
	/// assert_eq!(p.degree(), 2);
	/// ```
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

impl<T> Polynomial<T> where T: One + Zero + PartialEq + Clone
{
	#[inline]
	pub fn is_x(&self) -> bool { self.degree() == 1 && self[1] == T::one() && self[0] == T::zero() }
}

impl<T> Polynomial<T> where T: Zero + Mul<T, Output = T> + Add<T, Output = T> + Clone
{
	/// Evaluates the polynomial at the given value.
	///
	/// # Arguments
	///
	/// * `x`: The value at which to evaluate the polynomial.
	///
	/// # Returns
	///
	/// The result of evaluating the polynomial at `x`.
	///
	/// Example:
	/// ```
	/// use polyx::Polynomial;
	/// let p = Polynomial::from(vec![1, 0, 2]);
	/// assert_eq!(p.eval(2), 9);
	/// ```
	#[inline]
	pub fn eval<U: Into<T>>(&self, x: U) -> T
	{
		// Computes self(x)
		let x: T = x.into();
		let mut result = T::zero();
		for coef in self.into_iter().rev() {
			result = x.clone() * result + coef.clone();
		}
		result
	}
}

impl<T> Polynomial<T> where T: Clone + Zero + HasNorm
{
	/// Returns a new polynomial with the coefficients in reverse order.
	pub fn rev(&self) -> Polynomial<T> { self.0.iter().map(|x| x.clone()).rev().collect() }

	/// Reverses the coefficients of the polynomial in place.
	pub fn rev_inplace(&mut self) { self.0.reverse() }
}
