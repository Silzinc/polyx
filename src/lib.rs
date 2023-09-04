// #![feature(specialization)] // not stable yet
#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial<T>(pub(crate) Vec<T>);

pub mod ops;
pub mod specific;
pub mod util;

mod errors;
mod parser;
