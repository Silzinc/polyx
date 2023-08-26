use crate::convert;
use crate::Polynomial::{self, NonZero, Zero};
use num_traits::{Float, Pow, PrimInt, ToPrimitive};

fn binom<T: PrimInt, U1: PrimInt, U2: PrimInt>(_n: U1, _k: U2) -> T
{
	let n = _n.to_u32().unwrap();
	let k = _k.to_u32().unwrap();
	if k > n {
		convert(0)
	} else {
		let mut b: u32 = convert(1);
		for j in 0..k {
			b *= n - j;
		}
		for j in 1..=k {
			b /= j;
		}
		convert(b)
	}
}

impl<T: Float> Polynomial<T>
{
	pub fn bernstein<U: ToPrimitive>(_m: U, _i: U) -> Self
	{
		let m = _m.to_usize().unwrap();
		let i = _i.to_usize().unwrap();
		if i > m {
			Zero
		} else {
			let b: T = convert(binom::<u32, _, _>(m, i));
			let v: Vec<T> = match NonZero(vec![convert(1), convert(-1)]).pow(m - i) {
				Ok(NonZero(coefs)) => coefs,
				_ => unreachable!(), // Won't happen since m >= i
			};
			let mut coefs = vec![convert(0); m + 1];
			for k in i..=m {
				coefs[k] = b * v[k - i];
			}
			NonZero(coefs)
		}
	}
}
