use crate::Polynomial;
use num_traits::{One, Zero};
use std::ops::{Mul, Sub};

fn binom<T: From<usize>, U1: Into<usize>, U2: Into<usize>>(_n: U1, _k: U2) -> T
{
	let n: usize = _n.into();
	let k: usize = _k.into();
	if k > n {
		T::from(0)
	} else {
		let mut b: usize = 1;
		for j in 0..k {
			b *= n - j;
		}
		for j in 1..=k {
			b /= j;
		}
		T::from(b)
	}
}

impl<T> Polynomial<T>
where T: Mul<T, Output = T> + Sub<T, Output = T> + Clone + Zero + One + PartialEq + From<usize>
{
	pub fn bernstein<U: Into<usize>>(_m: U, _i: U) -> Self
	{
		let m: usize = _m.into();
		let i: usize = _i.into();
		if i > m {
			Self::zero()
		} else {
			let b: T = binom::<usize, _, _>(m, i).into();
			crate::polynomial![T::one(), T::zero() - T::one()].powi(m - i).into_iter().map(|x| x * b).collect::<Polynomial<T>>() * crate::polynomial![T::zero(), T::one()].powi(i)
		}
	}
}
