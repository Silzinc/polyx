// #![feature(specialization)] // not stable yet
#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial<T>(pub(crate) Vec<T>);

mod ops;
mod specific;
mod util;

pub mod consts;
mod errors;
mod parser;
pub mod traits;
