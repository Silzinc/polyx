// #![feature(specialization)] // not stable yet
#![feature(associated_type_bounds)]
#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial<T>(pub(crate) Vec<T>);

mod ops;
mod specific;
mod util;

mod complex_parser;
pub mod consts;
mod errors;
mod parser;
pub mod traits;

#[cfg(test)]
mod tests;
