use crate::parser::Ops;
use std::fmt;

#[derive(Debug, Clone)]
pub(crate) enum PolynomialError
{
	NoBinaryOperator,
	BinaryOperatorZeroOperand(Ops),
	BinaryOperatorOneOperand(Ops, String),
	ImpossiblePower(String, String),
	ImpossiblePower2Polynomials(String, String),
	ImpossibleDivision(String, String),
	ImpossibleOpen,
	ImpossibleClose,
	UnaryMinusFailed(Ops),
	UnsupportedCharacter(char),
	EmptyStringInput,
}

impl fmt::Display for PolynomialError
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		use PolynomialError::*;
		match self {
			NoBinaryOperator => write!(f, "NoBinaryOperator"),
			BinaryOperatorZeroOperand(op) => write!(f, "BinaryOperatorZeroOperand({})", op),
			BinaryOperatorOneOperand(op, p) => write!(f, "BinaryOperatorOneOperand({}, {})", op, p),
			ImpossiblePower(p, n) => write!(f, "ImpossiblePower({}, {:?})", p, n),
			ImpossiblePower2Polynomials(p1, p2) =>
				write!(f, "ImpossiblePower2Polynomials({}, {})", p1, p2),
			ImpossibleDivision(p1, p2) => write!(f, "ImpossibleDivision({}, {})", p1, p2),
			ImpossibleOpen => write!(f, "ImpossibleOpen"),
			ImpossibleClose => write!(f, "ImpossibleClose"),
			UnaryMinusFailed(op) => write!(f, "UnaryMinusFailed({})", op),
			UnsupportedCharacter(c) => write!(f, "UnsupportedCharacter({})", c),
			EmptyStringInput => write!(f, "EmptyStringInput"),
		}
	}
}

impl std::error::Error for PolynomialError {}
