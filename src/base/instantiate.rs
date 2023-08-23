use crate::Polynomial;
use crate::Polynomial::{NonZero, Zero, X};
use num_traits::ToPrimitive;
use std::{clone::Clone, convert::From, fmt::Debug, marker::Copy};

impl<F: ToPrimitive + Copy + Debug> From<&Vec<F>> for Polynomial
{
	fn from(values: &Vec<F>) -> Polynomial
	{
		// This version does not take ownership
		if values.len() == 0 {
			Zero
		} else {
			let mut coefs = vec![0f64; values.len()];
			for k in 0..values.len() {
				coefs[k] = match values[k].to_f64() {
					Some(x) => x,
					None => panic!("Error when converting {:?} to f64", values[k]),
				};
			}
			NonZero(coefs)
		}
	}
}

impl From<Vec<f64>> for Polynomial
{
	fn from(value: Vec<f64>) -> Polynomial
	{
		// This version does take ownership
		let mut last_index = value.len();
		while last_index > 0 && value[last_index - 1] == 0f64 {
			last_index -= 1;
		}
		if last_index == 0 {
			Zero
		} else {
			NonZero(value[0..last_index].to_vec())
		}
	}
}

impl Clone for Polynomial
{
	fn clone(&self) -> Self
	{
		match self {
			Zero => Zero,
			X => X,
			NonZero(coefs) => NonZero(coefs.clone()),
		}
	}

	fn clone_from(&mut self, source: &Self)
	{
		match source {
			Zero => {
				*self = Zero;
			}
			X => {
				*self = X;
			}
			NonZero(coefs) => {
				*self = NonZero(coefs.clone());
			}
		}
	}
}
