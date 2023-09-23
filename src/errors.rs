use crate::parser::Ops;
use num_traits::{ToPrimitive, Zero};
use std::fmt;

#[derive(Debug, Clone)]
pub(crate) struct PolynomialString(pub(crate) String);

impl std::fmt::Display for PolynomialString
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.0) }
}

impl<T> crate::Polynomial<T> where T: ToPrimitive + Clone + Zero
{
	pub(crate) fn as_string(&self) -> PolynomialString { PolynomialString(self.to_string()) }
}

#[derive(Debug, Clone)]
pub(crate) enum PolynomialError<T>
{
	NoBinaryOperator,
	BinaryOperatorZeroOperand(Ops),
	BinaryOperatorOneOperand(Ops, PolynomialString),
	ImpossiblePower(PolynomialString, T),
	ImpossiblePower2Polynomials(PolynomialString, PolynomialString),
	ImpossibleDivision(PolynomialString, PolynomialString),
	ImpossibleOpen,
	ImpossibleClose,
	UnaryMinusFailed(Ops),
	UnsupportedCharacter(char),
	EmptyStringInput,
}

impl<T: fmt::Debug> fmt::Display for PolynomialError<T>
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		use PolynomialError::*;
		match self {
			NoBinaryOperator => write!(f, "NoBinaryOperator"),
			BinaryOperatorZeroOperand(op) =>
				write!(f, "BinaryOperatorZeroOperand({})", op),
			BinaryOperatorOneOperand(op, p) =>
				write!(f, "BinaryOperatorOneOperand({}, {})", op, p),
			ImpossiblePower(p, n) => write!(f, "ImpossiblePower({}, {:?})", p, n),
			ImpossiblePower2Polynomials(p1, p2) =>
				write!(f, "ImpossiblePower2Polynomials({}, {})", p1, p2),
			ImpossibleDivision(p1, p2) =>
				write!(f, "ImpossibleDivision({}, {})", p1, p2),
			ImpossibleOpen => write!(f, "ImpossibleOpen"),
			ImpossibleClose => write!(f, "ImpossibleClose"),
			UnaryMinusFailed(op) => write!(f, "UnaryMinusFailed({})", op),
			UnsupportedCharacter(c) => write!(f, "UnsupportedCharacter({})", c),
			EmptyStringInput => write!(f, "EmptyStringInput"),
		}
	}
}

impl<T: fmt::Debug> std::error::Error for PolynomialError<T> {}
