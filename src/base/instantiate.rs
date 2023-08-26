use crate::convert;
use crate::Polynomial::{self, NonZero, Zero};
use num_traits::{self, Float};
use std::{convert::From, default::Default};

impl<T: Float> Default for Polynomial<T>
{
	fn default() -> Self { Zero }
}

impl<T: Float> From<&Vec<T>> for Polynomial<T>
{
	fn from(values: &Vec<T>) -> Self
	{
		// This version does not take ownership
		if values.len() == 0 {
			Zero
		} else {
			let mut coefs = vec![convert(0); values.len()];
			for k in 0..values.len() {
				coefs[k] = values[k];
			}
			NonZero(coefs)
		}
	}
}

impl<T> From<Vec<T>> for Polynomial<T>
where T: num_traits::Float + num_traits::Zero
{
	fn from(value: Vec<T>) -> Self
	{
		// This version does take ownership
		let mut last_index = value.len();
		while last_index > 0 && value[last_index - 1] == convert(0) {
			last_index -= 1;
		}
		if last_index == 0 {
			Zero
		} else {
			NonZero(value[0..last_index].to_vec())
		}
	}
}
