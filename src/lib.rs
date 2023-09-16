// #![feature(specialization)] // not stable yet
#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial<T>(pub(crate) Vec<T>);

mod ops;
mod specific;
mod util;

mod errors;
mod parser;
