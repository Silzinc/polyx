use crate::Polynomial;
use num_traits::{Float, Signed};
use std::fmt::Debug;

impl<T> Polynomial<T> where T: Clone + Debug + Signed
{
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
