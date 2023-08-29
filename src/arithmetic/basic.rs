use crate::Polynomial;
use num_traits::{One, Zero};
use std::ops::{Add, Mul, Index, IndexMut};

// Some basic implementations needed elsewhere

impl<T> Index<usize> for Polynomial<T>
{
  // Gives the coefficient of the monomial of degree index
  type Output = T;
  #[inline]
  fn index(&self, index: usize) -> &Self::Output
  {
    (self.0)[index]
  }
}

impl<T> IndexMut<usize> for Polynomial<T>
{
  #[inline]
  fn index_mut(&mut self, index: usize) -> &mut Self::Output
  {
    &mut (self.0)[index]
  }
}

macro_rules! impl_iter
{
  // Implementations to iterate over the coefficients of a polynomial
  ($t:ty, $iter_type:ty, $method:ident) => {
    impl<T> IntoIterator for $t
    {
      type Item = T;
      type IntoIter = $iter_type;
      #[inline]
      fn into_iter(self) -> Self::IntoIter
      {
        self.0.$method()
      }
    }
  }
}

impl_iter!(Polynomial<T>, std::vec::IntoIter<T>, into_iter);
impl_iter!(&Polynomial<T>, std::slice::Iter<T>, iter);
impl_iter!(&mut Polynomial<T>, std::slice::IterMut<T>, iter_mut);

impl<T> Polynomial<T>
{
  // Gives the degree of self
  // Note that the degree of the zero polynomial is 0 to avoid usize underflow
  // The is_zero() method is preferred to distinguish the zero polynomial from the other constant polynomials
  #[inline]
	pub fn degree(&self) -> usize { if self.is_zero() 0 else self.0.len() - 1 }
}

impl<T> Polynomial<T>
where T: Zero + Mul<T, Output = T> + Add<T, Output = T>
{
  #[inline]
	pub fn eval<U: Into<T>>(&self, _x: U) -> T
	{
		// Computes self(x)
		let x: T = _x.into();
		let mut result = T::zero();
		for coef in self.into_iter().rev() {
			result = x * result + coef;
		}
	}
}
