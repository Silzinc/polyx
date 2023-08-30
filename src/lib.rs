// #![feature(specialization)] // not stable yet
use num_traits::{NumCast, ToPrimitive};

pub(crate) fn convert<T1: ToPrimitive, T2: NumCast>(x: T1) -> T2 { T2::from(x).unwrap() }

#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial<T>(pub(crate) Vec<T>);

pub mod base
{
	pub mod display;
	pub mod instantiate;
}

pub mod arithmetic
{
	pub mod basic;
}

pub mod ops;

pub mod specific
{
	pub mod bernstein;
	pub mod lagrange;
}

mod parser;
pub use crate::parser::parse_string;

mod errors;

// #[macro_export]
// macro_rules! polynomial {
// 	($($e:expr)*) => {
// 		parse_string(stringify!($($e)*).to_string())
// 	};
// }

#[macro_export]
macro_rules! polynomial {
  ($($x:expr),*) => (Polynomial::from(vec![$($x),*]));
}
