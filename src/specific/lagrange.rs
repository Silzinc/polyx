use crate::Polynomial;
use crate::Polynomial::{Zero, X};
use num_traits::ToPrimitive;
use std::fmt::Debug;

impl Polynomial
{
	pub fn lagrange<F1: ToPrimitive + Debug, F2: ToPrimitive + Debug>(_x_array: &Vec<F1>, values: &Vec<F2>) -> Polynomial
	{
		// Computes the interpolating polynomial of minimal degree between the two
		// arrays with the Lagrange barycentric method
		let n = _x_array.len(); // Output will be of degree n - 1
		if n != values.len() {
			panic!("Cannot make polynomial interpolation if the number of roots do not match the number of values")
		} else if n == 0 {
			Zero
		} else {
			let mut weights = vec![1f64; n];
			let mut factors = vec![Polynomial::from(vec![1.]); n];
			let x_array = Vec::from_iter(_x_array.iter().map(|x| match x.to_f64() {
				Some(y) => y,
				None => panic!("{:?} cannot be converted to f64", x),
			}));

			for j in 0..(n - 1) {
				for k in 0..j {
					factors[k] = X * &factors[k] - x_array[j] * &factors[k];
					weights[k] *= x_array[k] - x_array[j];
				}
				let f: Polynomial = X * &factors[j + 1] - x_array[j] * &factors[j + 1];
				for k in (j + 1)..n {
					factors[k] = f.clone();
					weights[k] *= x_array[k] - x_array[j];
				}
			}
			for k in 0..(n - 1) {
				factors[k] = X * &factors[k] - x_array[n - 1] * &factors[k];
				weights[k] *= x_array[k] - x_array[n - 1];
			}

			let mut result: Polynomial = Zero;
			for k in 0..n {
				result += ((match values[k].to_f64() {
					Some(y) => y,
					None => panic!("{:?} cannot be converted to f64", values[k]),
				}) / weights[k])
					* &factors[k];
			}
			result
		}
	}
}
