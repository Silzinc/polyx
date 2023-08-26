use crate::convert;
use crate::Polynomial::{self, NonZero, Zero, X};
use num_traits::{Float, ToPrimitive};
use std::ops::{AddAssign, MulAssign};

impl<T: Float + AddAssign + MulAssign> Polynomial<T>
{
	pub fn lagrange<F1, F2>(_x_array: &Vec<F1>, values: &Vec<F2>) -> Self
	where
		F1: ToPrimitive + Copy,
		F2: ToPrimitive + Copy,
	{
		// Computes the interpolating polynomial of minimal degree between the two
		// arrays with the Lagrange barycentric method
		let n = _x_array.len(); // Output will be of degree n - 1
		if n != values.len() {
			panic!("Cannot make polynomial interpolation if the number of roots do not match the number of values")
		} else if n == 0 {
			Zero
		} else {
			let mut weights = vec![convert(1); n];
			let mut factors = vec![NonZero(vec![convert(1)]); n];
			let x_array: Vec<T> = _x_array.iter().map(|&x| convert(x)).collect();
			for j in 0..(n - 1) {
				for k in 0..j {
					factors[k] = &factors[k] * X - &factors[k] * x_array[j];
					weights[k] *= x_array[k] - x_array[j];
				}
				let f = &factors[j + 1] * X - &factors[j + 1] * x_array[j];
				for k in (j + 1)..n {
					factors[k] = f.clone();
					weights[k] *= x_array[k] - x_array[j];
				}
			}
			for k in 0..(n - 1) {
				factors[k] = &factors[k] * X - &factors[k] * x_array[n - 1];
				weights[k] *= x_array[k] - x_array[n - 1];
			}
			let mut result = Zero;
			for k in 0..n {
				result += &factors[k] * (convert::<_, T>(values[k]) / weights[k]);
			}
			result
		}
	}
}
