use crate::Polynomial;
use num_traits::{Float, Signed};
use std::fmt::Debug;

impl<T> Polynomial<T> where T: Clone + Debug + Signed
{
	// Implements an algorithm to invert a polynomial modulo an integer
	// Based on https://thibautverron.github.io/enseignement/2018-CompAlg2-notes.pdf, page 26
	// Time complexity: O(M(n)), where M(n) is the time complexity of the
	// multiplication algorithm
	// Space complexity: O(n)
	/* Example:
	let p = polynomial![1, -4, 0, -2, 5, 1, 1, 1];
	let inv10 = Polynomial::inverse(&p, 10);
	assert_eq!(Polynomial::short_product(&p, &inv10, 10),
						 Polynomial::from(1));
	 */
	// Be careful with this function, overflows can happen pretty easily (i32 might
	// not be big enough)

	pub fn inverse(u: &Self, modulus: usize) -> Self
	{
		if !u[0].is_one() && !(T::zero() - u[0].clone()).is_one() {
			panic!(
			       "The constant coefficient of the inverted polynomial must be 1 or -1\nIf you are \
			        using a polynomial with floating point coefficients, use Polynomial::inverse_float \
			        instead"
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

impl<T> Polynomial<T> where T: Clone + Debug + Float
{
	// Same function as above, but for floating point coefficients
	// This allows to invert a polynomial with any non-zero constant coefficient
	pub fn inverse_float(u: &Self, modulus: usize) -> Self
	{
		let two = Self::from(T::one() + T::one());
		let mut v = Self::from(u[0].recip());
		let mut result_modulus = 1;
		while result_modulus < modulus {
			v = Self::short_product(&v, &(&two - Self::short_product(&u, &v, modulus)), modulus);
			result_modulus <<= 1;
		}
		Polynomial::from(v)
	}
}
