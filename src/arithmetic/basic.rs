use crate::Polynomial;
use crate::Polynomial::{NonZero, Zero, X};
use num_traits::{PrimInt, ToPrimitive};
use std::fmt::Debug;

impl Polynomial
{
	// Some basic methods needed elsewhere

	pub fn eval<T: ToPrimitive + Debug>(&self, _x: T) -> f64
	{
		// Computes self(x)
		let x: f64 = match _x.to_f64() {
			Some(y) => y,
			None => panic!("Error when converting {:?} to f64", _x),
		};
		match self {
			Zero => 0.,
			X => x,
			NonZero(coefs) => {
				let mut result: f64 = 0.;
				for index in 0..=self.degree() {
					result = x * result + coefs[self.degree() - index];
				}
				result
			}
		}
	}

	pub fn degree(&self) -> usize
	{
		// Gives the degree of self
		match self {
			Zero => 0usize,
			X => 1usize,
			NonZero(coefs) => (coefs.len() - 1) as usize,
		}
	}

	pub fn coef<T: PrimInt + Debug>(&self, _n: T) -> f64
	{
		// Computes the coefficient of nth degree of self
		let n: usize = match _n.to_usize() {
			Some(m) => m,
			None => panic!("Error when converting {:?} to usize", _n),
		};
		match self {
			Zero => 0.,
			X => {
				if n == 1 {
					1.
				} else {
					0.
				}
			}
			NonZero(coefs) => {
				if n >= coefs.len() {
					0.
				} else {
					coefs[n]
				}
			}
		}
	}
}
