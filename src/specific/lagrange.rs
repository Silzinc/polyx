use crate::Polynomial;
use num_traits::{One, Zero};
use std::ops::{Div, Mul, Sub};

impl<T> Polynomial<T>
where T: Mul<T, Output = T> + Sub<T, Output = T> + Div<T, Output = T> + Clone + Zero + One
{
	pub fn lagrange<F1, F2>(_x_array: &Vec<F1>, _values: &Vec<F2>) -> Self
	where
		F1: Into<T>,
		F2: Into<T>,
	{
		// Computes the interpolating polynomial of minimal degree between the two
		// arrays with the Lagrange barycentric method
		#[allow(non_snake_case)]
		let X = crate::polynomial![T::zero(), T::one()];
		let n = _x_array.len(); // Output will be of degree n - 1
		if n != _values.len() {
			panic!("Cannot make polynomial interpolation from {} points and {} values", n, _values.len());
		}
		let mut weights = vec![T::one(); n];
		let mut factors: Vec<Polynomial<T>> = vec![crate::polynomial![T::one()]; n];
		let x_array: Vec<T> = _x_array.iter().map(|&x| x.into()).collect();
		let values: Vec<T> = _values.iter().map(|&x| x.into()).collect();
		for j in 0..(n - 1) {
			for k in 0..j {
				factors[k] = X * factors[k] - crate::polynomial![x_array[j]] * factors[k];
				weights[k] = weights[k] * (x_array[k] - x_array[j]);
			}
			let f = X * factors[j + 1] - crate::polynomial![x_array[j]] * factors[j + 1];
			for k in (j + 1)..n {
				factors[k] = f.clone();
				weights[k] = weights[k] * (x_array[k] - x_array[j]);
			}
		}
		for k in 0..(n - 1) {
			factors[k] = X * factors[k] - crate::polynomial![x_array[n - 1]] * factors[k];
			weights[k] = weights[k] * (x_array[k] - x_array[n - 1]);
		}
		let mut result = Polynomial::zero();
		for k in 0..n {
			result += crate::polynomial![values[k] / weights[k]] * factors[k];
		}
		result
	}
}
