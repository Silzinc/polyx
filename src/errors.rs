use crate::parser::Ops;
use thiserror::Error;

#[derive(Debug, Clone)]
pub(crate) struct PolynomialString(pub(crate) String);

impl std::fmt::Display for PolynomialString
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.0) }
}

impl crate::Polynomial
{
	pub(crate) fn as_string(&self) -> PolynomialString { PolynomialString(self.to_string()) }
}

#[derive(Debug, Clone, Error)]
#[allow(dead_code)]
pub(crate) enum PolynomialError
{
	#[error("tried to execute a binary operator but no binary operator was found")]
	NoBinaryOperator,
	#[error("tried binary operator {0} on zero operand")]
	BinaryOperatorZeroOperand(Ops),
	#[error("tried binary operator {0} on one operand {1}")]
	BinaryOperatorOneOperand(Ops, PolynomialString),
	#[error("impossible to compute ({0}) ^ {1:10.3e}")]
	ImpossiblePower(PolynomialString, f64),
	#[error("impossible to compute power ({0}) ^ ({1})")]
	ImpossiblePower2Polynomials(PolynomialString, PolynomialString),
	#[error("impossible to compute division ({0}) / ({1})")]
	ImpossibleDivision(PolynomialString, PolynomialString),
	#[error("impossible operator '(' between polynomials")]
	ImpossibleOpen,
	#[error("closed parenthesis without opening one previously")]
	ImpossibleClose,
	#[error("cannot put a unary minus sign before operator {0}")]
	UnaryMinusFailed(Ops),
	#[error("unsupported character '{0}'")]
	UnsupportedCharacter(char),
	#[error("empty string input in the parser")]
	EmptyStringInput,
}
