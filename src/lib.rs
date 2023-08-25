use num_traits::{Float, ToPrimitive};

pub(crate) fn convert<T1: ToPrimitive, T2: Float>(x: T1) -> T2 { T2::from(x).unwrap() }

#[derive(Debug, PartialEq)]
pub enum Polynomial<T: Float>
{
	Zero,
	NonZero(Vec<T>),
	X,
}

pub mod base
{
	pub mod display;
	pub mod instantiate;
}

pub mod arithmetic
{
	pub mod basic;
}

pub mod ops
{
	pub mod add;
	pub mod multiply;
}

pub mod specific
{
	pub mod bernstein;
	pub mod lagrange;
}

pub mod parser;

mod errors;
