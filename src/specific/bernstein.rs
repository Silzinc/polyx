use crate::Polynomial;
use crate::Polynomial::{NonZero, Zero};
use num_traits::{Pow, ToPrimitive};
use std::fmt::Debug;

fn binom<U: ToPrimitive + Debug>(_n: U, _k: U) -> u32
{
	let n = match _n.to_u32() {
		Some(y) => y,
		None => panic!("{:?} cannot be converted to unsigned integer", _n),
	};
	let k = match _k.to_u32() {
		Some(y) => y,
		None => panic!("{:?} cannot be converted to unsigned integer", _k),
	};
	if k > n {
		0u32
	} else {
		let mut b: u32 = 1;
		for j in 0..k {
			b *= n - j;
		}
		for j in 1..=k {
			b /= j;
		}
		b
	}
}

impl Polynomial
{
	pub fn bernstein<U: ToPrimitive + Debug>(_m: U, _i: U) -> Polynomial
	{
		let m = match _m.to_usize() {
			Some(y) => y,
			None => panic!("{:?} cannot be converted to unsigned integer", _m),
		};
		let i = match _i.to_usize() {
			Some(y) => y,
			None => panic!("{:?} cannot be converted to unsigned integer", _i),
		};
		if i > m {
			Zero
		} else {
			let b = binom(m, i) as f64;
			let v = match NonZero(vec![1., -1.]).pow(m - i) {
				Ok(NonZero(coefs)) => coefs,
				_ => unreachable!(), // Won't happen since m >= i
			};

			let mut coefs = vec![0f64; m + 1];
			for k in i..=m {
				coefs[k] = (b * v[k - i]) as f64;
			}

			NonZero(coefs)
		}
	}
}
