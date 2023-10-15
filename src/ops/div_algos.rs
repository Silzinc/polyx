use crate::{
	traits::{FloatLike, HasNorm},
	Polynomial,
};
use num_traits::{Signed, Zero};
use std::fmt::Debug;

impl<T> Polynomial<T> where T: Clone + Debug + Signed + HasNorm
{
	/// Implements an algorithm to invert a polynomial modulo an integer.
	/// Based on <https://thibautverron.github.io/enseignement/2018-CompAlg2-notes.pdf>, page 26.
	/// Time complexity: O(M(n)), where M(n) is the time complexity of the
	/// multiplication algorithm and n the degree of u. Space complexity: O(n)
	/// Example:
	/// ```rust
	/// use polyx::*;
	/// let p = polynomial![1, -4, 0, -2, 5, 1, 1, 1];
	/// let inv10 = Polynomial::inverse(&p, 10);
	/// assert_eq!(Polynomial::short_product(&p, &inv10, 10), Polynomial::from(1));
	/// ```
	/// Be careful with this function, overflows can happen pretty easily (i32
	/// might not be big enough).

	pub fn inverse(u: &Self, modulus: usize) -> Self
	{
		if !u[0].is_one() && !(-u[0].clone()).is_one() {
			panic!(
			       "The constant coefficient of the inverted polynomial must be 1 or -1\nIf you are using a polynomial with floating point coefficients, \
			        use Polynomial::inverse_float instead"
			);
		}
		let two = Self::from(T::one() + T::one());
		let mut v = Self::from(u[0].clone());
		let mut result_modulus = 1;
		while result_modulus < modulus {
			v = Self::short_product(&v, &(&two - Self::short_product(&u, &v, modulus)), modulus);
			result_modulus <<= 1;
		}
		Polynomial::from(v)
	}
}

impl<T> Polynomial<T> where T: FloatLike + HasNorm
{
	/// Same function as `inverse`, but for floating point coefficients
	/// This allows to invert a polynomial with any non-zero constant coefficient
	pub fn inverse_float(u: &Self, modulus: usize) -> Self
	{
		let two = Self::from(T::one() + T::one());
		let mut v = Self::from(u[0].clone().inv());
		let mut result_modulus = 1;
		while result_modulus < modulus {
			v = Self::short_product(&v, &(&two - Self::short_product(&u, &v, modulus)), modulus);
			result_modulus <<= 1;
		}
		Polynomial::from(v)
	}
}

impl<T> Polynomial<T> where T: Clone + Debug + Signed + HasNorm
{
	/// Performs Euclidean division of two polynomials. Based on <https://thibautverron.github.io/enseignement/2018-CompAlg2-notes.pdf>, page 26.
	///
	/// # Arguments
	///
	/// * `p1` - The dividend polynomial.
	/// * `p2` - The divisor polynomial.
	///
	/// # Panics
	///
	/// This function will panic if the divisor polynomial is zero or if its
	/// leading coefficient is not 1 or -1.
	///
	/// # Returns
	///
	/// A tuple containing the quotient and remainder polynomials.
	///
	/// # Example
	/// ```rust
	/// use polyx::*;
	/// let mut a = polynomial![1, 0, 2];
	/// let mut b = polynomial![1, 1];
	/// let (q, r) = Polynomial::euclidean_division(&mut a, &mut b);
	///
	/// assert_eq!(b * q + r, a);
	/// ```
	pub fn euclidean_division(p1: &mut Self, p2: &mut Self) -> (Self, Self)
	{
		if p2.is_zero() {
			panic!("Polynomial division by zero");
		}
		if !p2[p2.degree()].clone().is_one() && !(-p2[p2.degree()].clone()).is_one() {
			panic!("The leading coefficient of the dividor polynomial must be 1 or -1");
		}
		if p1.degree() < p2.degree() {
			return (Self::zero(), p1.clone());
		}
		let m = p1.degree();
		let n = p2.degree();
		p1.rev_inplace();
		p2.rev_inplace();
		let h = Self::inverse(&p2, m - n + 1);
		let mut q = Self::short_product(&p1, &h, m - n + 1);
		p1.rev_inplace();
		p2.rev_inplace();
		q.rev_inplace();
		// Some zero coefficients are lost in the short-product and reverse. We have to
		// recover them to make deg(q) = m - n
		q <<= m - n - q.degree();

		let r = &*p1 - &*p2 * &q;
		(q, r)
	}

	/// This version forces to create a copy of p1 and p2 and is therefore less
	/// efficient, but allows for immutable arguments.
	/// Example:
	/// ```rust
	/// use polyx::*;
	///
	/// let a = polynomial![1, 0, 2];
	/// let b = polynomial![1, 1];
	/// let (q, r) = Polynomial::euclidean_division_immutable(&a, &b);
	///
	/// assert_eq!(b * q + r, a);
	/// ```
	pub fn euclidean_division_immutable(p1: &Self, p2: &Self) -> (Self, Self)
	{
		if p2.is_zero() {
			panic!("Polynomial division by zero");
		}
		if p1.degree() < p2.degree() {
			return (Self::zero(), p1.clone());
		}
		let m = p1.degree();
		let n = p2.degree();
		let p1_rev = p1.rev();
		let p2_rev = p2.rev();
		let h = Self::inverse(&p2_rev, m - n + 1);
		let mut q = Self::short_product(&p1_rev, &h, m - n + 1);
		q.rev_inplace();
		// Some zero coefficients are lost in the short-product and reverse. We have to
		// recover them to make deg(q) = m - n
		q <<= m - n - q.degree();

		let r = p1 - p2 * &q;
		(q, r)
	}
}

impl<T> Polynomial<T> where T: FloatLike
{
	pub fn euclidean_division_float(p1: &mut Self, p2: &mut Self) -> (Self, Self)
	{
		if p2.is_zero() {
			panic!("Polynomial division by zero");
		}
		if p1.degree() < p2.degree() {
			return (Self::zero(), p1.clone());
		}
		let m = p1.degree();
		let n = p2.degree();
		p1.rev_inplace();
		p2.rev_inplace();
		let h = Self::inverse_float(&p2, m - n + 1);
		let mut q = Self::short_product(&p1, &h, m - n + 1);
		p1.rev_inplace();
		p2.rev_inplace();
		q.rev_inplace();
		// Some zero coefficients are lost in the short-product and reverse. We have to
		// recover them to make deg(q) = m - n
		q <<= m - n - q.degree();

		let mut r = &*p1 - &*p2 * &q;
		r.clean_zeros();
		(q, r)
	}

	pub fn euclidean_division_immutable_float(p1: &Self, p2: &Self) -> (Self, Self)
	{
		if p2.is_zero() {
			panic!("Polynomial division by zero");
		}
		if p1.degree() < p2.degree() {
			return (Self::zero(), p1.clone());
		}
		let m = p1.degree();
		let n = p2.degree();
		let p1_rev = p1.rev();
		let p2_rev = p2.rev();
		let h = Self::inverse_float(&p2_rev, m - n + 1);
		let mut q = Self::short_product(&p1_rev, &h, m - n + 1);
		q.rev_inplace();
		// Some zero coefficients are lost in the short-product and reverse. We have to
		// recover them to make deg(q) = m - n
		q <<= m - n - q.degree();

		let r = p1 - p2 * &q;
		(q, r)
	}
}
