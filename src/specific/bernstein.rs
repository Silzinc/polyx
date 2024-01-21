use crate::{traits::HasNorm, Polynomial};
use num_traits::{PrimInt, Zero};
use std::fmt::Debug;

fn binom<T: PrimInt, U1: PrimInt, U2: PrimInt>(_n: U1, _k: U2) -> T
{
	let n: usize = _n.to_usize().unwrap();
	let k: usize = _k.to_usize().unwrap();
	if k > n {
		T::from(0).unwrap()
	} else {
		let mut b: usize = 1;
		for j in 0..k {
			b *= n - j;
		}
		for j in 1..=k {
			b /= j;
		}
		T::from(b).unwrap()
	}
}

impl<T> Polynomial<T> where T: Debug + PrimInt + HasNorm
{
	/// Computes the Bernstein polynomial of degree `m` and index `i` for a given
	/// (unsigned) integer type `U`.
	pub fn bernstein<U: PrimInt>(_m: U, _i: U) -> Self
	{
		let m: usize = _m.to_usize().unwrap();
		let i: usize = _i.to_usize().unwrap();
		if i > m {
			Self::zero()
		} else {
			let b = T::from(binom::<usize, _, _>(m, i)).unwrap();
			crate::polynomial![T::one(), T::zero() - T::one()].powi(m - i)
			                                                  .into_iter()
			                                                  .map(|x| x * b)
			                                                  .collect::<Polynomial<T>>()
			* crate::polynomial![T::zero(), T::one()].powi(i)
		}
	}
}
