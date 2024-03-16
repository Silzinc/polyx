use std::{
  fmt::Debug,
  ops::{
    Div,
    Mul,
    Neg,
    Sub,
  },
};

use num_traits::{
  One,
  Zero,
};

use crate::{
  traits::HasNorm,
  Polynomial,
};

impl<T> Polynomial<T>
where
  T: Mul<T, Output = T>
    + Sub<T, Output = T>
    + Div<T, Output = T>
    + Clone
    + Zero
    + One
    + Neg<Output = T>
    + Debug
    + HasNorm,
{
  /// Computes the interpolating polynomial of minimal degree between the two
  /// arrays with the Lagrange barycentric method.
  /// NOTE : it is not recommended to use this method if `T` is an integer type
  pub fn lagrange<F1, F2>(_x_array: &[F1], _values: &[F2]) -> Self
  where
    F1: Into<T> + Clone,
    F2: Into<T> + Clone,
  {
    #[allow(non_snake_case)]
    let X = crate::polynomial![T::zero(), T::one()];
    let n = _x_array.len(); // Output will be of degree n - 1
    if n != _values.len() {
      panic!(
        "Cannot make polynomial interpolation from {} points and {} values",
        n,
        _values.len()
      );
    }
    let mut weights = vec![T::one(); n];
    let mut factors = vec![crate::polynomial![T::one()]; n];
    let x_array: Vec<T> = _x_array.iter().map(|x| x.clone().into()).collect();
    let values: Vec<T> = _values.iter().map(|x| x.clone().into()).collect();
    for j in 0..(n - 1) {
      for k in 0..j {
        factors[k] = &X * &factors[k] - crate::polynomial![x_array[j].clone()] * &factors[k];
        weights[k] = weights[k].clone() * (x_array[k].clone() - x_array[j].clone());
      }
      let f = &X * &factors[j + 1] - crate::polynomial![x_array[j].clone()] * &factors[j + 1];
      for k in (j + 1)..n {
        factors[k] = f.clone();
        weights[k] = weights[k].clone() * (x_array[k].clone() - x_array[j].clone());
      }
    }
    // Now when j = n - 1
    for k in 0..(n - 1) {
      factors[k] = &X * &factors[k] - crate::polynomial![x_array[n - 1].clone()] * &factors[k];
      weights[k] = weights[k].clone() * (x_array[k].clone() - x_array[n - 1].clone());
    }
    let mut result = Polynomial::zero();
    for k in 0..n {
      result += crate::polynomial![values[k].clone() / weights[k].clone()] * &factors[k];
    }
    result
  }
}
