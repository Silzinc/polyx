//! A library for working with polynomials. Apart from basic arithmetic
//! operations, it also supports polynomial expression parsing and evaluation.
//! Most of its functionalities are however non-optimal and are not recommended
//! for use in production.

#[derive(Debug, Clone, PartialEq)]
/// A polynomial with coefficients of type `T`.
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
