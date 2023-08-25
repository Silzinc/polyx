use num_traits::{Float, NumCast, ToPrimitive};

pub(crate) fn convert<T1: ToPrimitive, T2: NumCast>(x: T1) -> T2 { T2::from(x).unwrap() }

#[derive(Debug, PartialEq, Clone)]
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

mod parser;
pub use crate::parser::parse_string;

mod errors;

#[macro_export]
macro_rules! polynomial {
	($($e:expr)*) => {
		parse_string(stringify!($($e)*).to_string())
	};
}
