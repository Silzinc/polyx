use crate::convert;
use crate::Polynomial::{self, NonZero, Zero, X};
use num_traits::{Float, PrimInt, ToPrimitive};

impl<F: Float> Polynomial<F>
{
	// Some basic methods needed elsewhere
	pub fn eval<T: ToPrimitive>(&self, _x: T) -> F
	{
		// Computes self(x)
		let x = convert(_x);
		match self {
			Zero => convert(0),
			X => x,
			NonZero(coefs) => {
				let mut result = convert(0);
				let deg: usize = self.degree();
				for index in 0..=deg {
					result = x * result + coefs[deg - index];
				}
				result
			},
		}
	}
	pub fn degree<T: PrimInt>(&self) -> T
	{
		// Gives the degree of self
		match self {
			Zero => T::from(0).unwrap(),
			X => T::from(1).unwrap(),
			NonZero(coefs) => T::from(coefs.len() - 1).unwrap(),
		}
	}
	pub fn coef<T: PrimInt>(&self, _n: T) -> F
	{
		// Computes the coefficient of nth degree of self
		let n = _n.to_usize().unwrap();
		match self {
			Zero => F::zero(),
			X =>
				if n == 1 {
					convert(1)
				} else {
					convert(0)
				},
			NonZero(coefs) =>
				if n >= coefs.len() {
					convert(0)
				} else {
					coefs[n]
				},
		}
	}
}
