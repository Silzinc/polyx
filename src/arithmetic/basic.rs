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
				for index in 0..=self.degree() {
					result = x * result + coefs[self.degree() - index];
				}
				result
			},
		}
	}
	pub fn degree(&self) -> usize
	{
		// Gives the degree of self
		match self {
			Zero => 0,
			X => 1,
			NonZero(coefs) => coefs.len() - 1,
		}
	}
	pub fn coef<T: PrimInt>(&self, _n: T) -> F
	{
		// Computes the coefficient of nth degree of self
		let n = _n.to_usize().unwrap();
		match self {
			Zero => convert(0),
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
