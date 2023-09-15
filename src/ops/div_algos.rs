use super::inner_macros::*;
use crate::Polynomial;
use num_traits::Zero;
use std::fmt::Debug;
use std::ops::*;

impl<T> Polynomial<T>
{
	pub fn div(p1: &Self, p2: &Self) -> (Self, Self)
	{
		if p2.is_zero() {
			panic!("Division by zero polynomial");
		}
		if p1.degree() < p2.degree() {
			return (Polynomial::zero(), p1.clone());
		}
		let n = p2.degree() + 1;
		let m = p1.degree() - p2.degree();
		todo!()
	}
}
