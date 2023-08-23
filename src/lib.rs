#[derive(Debug, PartialEq)]
pub enum Polynomial
{
	Zero,
	NonZero(Vec<f64>),
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
