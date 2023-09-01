// #![feature(specialization)] // not stable yet
#![feature(const_trait_impl)]
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
