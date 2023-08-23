use crate::Polynomial;
use crate::Polynomial::{NonZero, Zero, X};
use num_traits::ToPrimitive;
use std::{
	fmt::Debug,
	ops::{Add, AddAssign, Neg, Sub, SubAssign},
};

impl Add<&Polynomial> for &Polynomial
{
	// Implements addition without taking ownership
	type Output = Polynomial;

	fn add(self, other: &Polynomial) -> Polynomial
	{
		if other.degree() > self.degree() {
			other.add(self)
		} else {
			match (self, other) {
				(Zero, _) => other.clone(),
				(_, Zero) => self.clone(),
				(X, X) => NonZero(vec![0., 2.]),
				(X, NonZero(other_coefs)) => {
					if other_coefs.len() == 0 {
						X
					} else if other_coefs.len() == 1 {
						NonZero(vec![other_coefs[0], 1.])
					} else {
						let mut new_coefs = other_coefs.clone();
						new_coefs[1] += 1.;
						NonZero(new_coefs)
					}
				}
				(NonZero(coefs), X) => {
					if coefs.len() == 0 {
						X
					} else if coefs.len() == 1 {
						NonZero(vec![coefs[0], 1.])
					} else {
						let mut new_coefs = coefs.clone();
						new_coefs[1] += 1.;
						NonZero(new_coefs)
					}
				}
				(NonZero(coefs), NonZero(other_coefs)) => {
					let mut new_coefs = coefs.clone();
					for k in 0..other_coefs.len() {
						new_coefs[k] += other_coefs[k];
					}
					NonZero(new_coefs)
				}
			}
		}
	}
}

// The following ones work but take ownership of instances of Polynomial. It is
// recommended to always add &Polynomial.

impl Add<&Polynomial> for Polynomial
{
	type Output = Polynomial;
	fn add(self, other: &Polynomial) -> Polynomial { &self + other }
}
impl Add<Polynomial> for Polynomial
{
	type Output = Polynomial;

	fn add(self, other: Polynomial) -> Polynomial { &self + &other }
}
impl Add<Polynomial> for &Polynomial
{
	type Output = Polynomial;
	fn add(self, other: Polynomial) -> Polynomial { self + &other }
}

// AddAssign implementations
// ====================================================================================

impl AddAssign<&Polynomial> for Polynomial
{
	fn add_assign(&mut self, other: &Polynomial)
	{
		match self {
			Zero => {
				self.clone_from(other);
			}
			X => {
				*self = &X + other;
			}
			NonZero(coefs) => {
				*self = &NonZero((&coefs).to_vec()) + other;
			}
		};
	}
}

// Ownership taking version
impl AddAssign<Polynomial> for Polynomial
{
	fn add_assign(&mut self, other: Polynomial) { *self += &other; }
}

// Number adding versions
impl<T: ToPrimitive + Debug> AddAssign<T> for Polynomial
{
	fn add_assign(&mut self, other: T)
	{
		let x: f64 = match other.to_f64() {
			Some(y) => y,
			None => panic!("Error when converting {:?} to f64", other),
		};
		match self {
			Zero => {
				*self = NonZero(vec![x]);
			}
			X => {
				*self = NonZero(vec![x, 1f64]);
			}
			NonZero(coefs) => {
				coefs[0] += x;
			}
		};
	}
}

// Adding floats 32 bits to polynomials
// ========================================================================

impl<T: ToPrimitive + Debug> Add<T> for Polynomial
{
	type Output = Polynomial;
	fn add(self, other: T) -> Polynomial
	{
		let x: f64 = match other.to_f64() {
			Some(y) => y,
			None => panic!("Error when converting {:?} to f64", other),
		};
		&self + Polynomial::from(vec![x])
	}
}
impl<T: ToPrimitive + Debug> Add<T> for &Polynomial
{
	type Output = Polynomial;
	fn add(self, other: T) -> Polynomial
	{
		let x: f64 = match other.to_f64() {
			Some(y) => y,
			None => panic!("Error when converting  {:?} to f64", other),
		};
		self + Polynomial::from(vec![x])
	}
}
impl Add<Polynomial> for f64
{
	type Output = Polynomial;
	fn add(self, other: Polynomial) -> Polynomial { other + self }
}
impl Add<&Polynomial> for f64
{
	type Output = Polynomial;
	fn add(self, other: &Polynomial) -> Polynomial { other + self }
}
impl Add<Polynomial> for f32
{
	type Output = Polynomial;
	fn add(self, other: Polynomial) -> Polynomial { other + (self as f64) }
}
impl Add<&Polynomial> for f32
{
	type Output = Polynomial;
	fn add(self, other: &Polynomial) -> Polynomial { other + (self as f64) }
}
impl Add<Polynomial> for i32
{
	type Output = Polynomial;
	fn add(self, other: Polynomial) -> Polynomial { other + (self as f64) }
}
impl Add<&Polynomial> for i32
{
	type Output = Polynomial;
	fn add(self, other: &Polynomial) -> Polynomial { other + (self as f64) }
}
impl Add<Polynomial> for i64
{
	type Output = Polynomial;
	fn add(self, other: Polynomial) -> Polynomial { other + (self as f64) }
}
impl Add<&Polynomial> for i64
{
	type Output = Polynomial;
	fn add(self, other: &Polynomial) -> Polynomial { other + (self as f64) }
}

// Negating a polynomial
// ========================================================================================

impl Neg for &Polynomial
{
	// Implements negation (a copy of the polynomial is created when this is called)
	type Output = Polynomial;

	fn neg(self) -> Polynomial
	{
		match self {
			Zero => Zero,
			X => NonZero(vec![0., -1.]),
			NonZero(coefs) => {
				let mut new_coefs = vec![0f64; coefs.len()];
				for index in 0..new_coefs.len() {
					new_coefs[index] = -coefs[index];
				}
				NonZero(new_coefs)
			}
		}
	}
}
impl Neg for Polynomial
{
	// Version that takes ownership
	type Output = Polynomial;
	fn neg(self) -> Polynomial { -&self }
}

// Subtraction
// =================================================================================================

impl Sub<&Polynomial> for &Polynomial
{
	// Implements subtraction
	type Output = Polynomial;
	fn sub(self, other: &Polynomial) -> Polynomial { self + (-other) }
}

// Ownership taking versions

impl Sub<Polynomial> for &Polynomial
{
	type Output = Polynomial;
	fn sub(self, other: Polynomial) -> Polynomial { self + (-&other) }
}
impl Sub<Polynomial> for Polynomial
{
	type Output = Polynomial;
	fn sub(self, other: Polynomial) -> Polynomial { &self + (-&other) }
}
impl Sub<&Polynomial> for Polynomial
{
	type Output = Polynomial;
	fn sub(self, other: &Polynomial) -> Polynomial { &self + (-other) }
}

// Subtracting floats 32 bits to polynomials
// ========================================================================

impl<T: ToPrimitive + Debug> Sub<T> for Polynomial
{
	type Output = Polynomial;
	fn sub(self, other: T) -> Polynomial
	{
		let x: f64 = match other.to_f64() {
			Some(y) => y,
			None => panic!("Error when converting {:?} to f64", other),
		};
		&self + Polynomial::from(vec![-x])
	}
}
impl<T: ToPrimitive + Debug> Sub<T> for &Polynomial
{
	type Output = Polynomial;
	fn sub(self, other: T) -> Polynomial
	{
		let x: f64 = match other.to_f64() {
			Some(y) => y,
			None => panic!("Error when converting {:?} to f64", other),
		};
		self + Polynomial::from(vec![-x])
	}
}
impl Sub<Polynomial> for f64
{
	type Output = Polynomial;
	fn sub(self, other: Polynomial) -> Polynomial { (-other) + self }
}
impl Sub<&Polynomial> for f64
{
	type Output = Polynomial;
	fn sub(self, other: &Polynomial) -> Polynomial { (-other) + self }
}
impl Sub<Polynomial> for f32
{
	type Output = Polynomial;
	fn sub(self, other: Polynomial) -> Polynomial { (-other) + (self as f64) }
}
impl Sub<&Polynomial> for f32
{
	type Output = Polynomial;
	fn sub(self, other: &Polynomial) -> Polynomial { (-other) + (self as f64) }
}
impl Sub<Polynomial> for i32
{
	type Output = Polynomial;
	fn sub(self, other: Polynomial) -> Polynomial { (-other) + (self as f64) }
}
impl Sub<&Polynomial> for i32
{
	type Output = Polynomial;
	fn sub(self, other: &Polynomial) -> Polynomial { (-other) + (self as f64) }
}
impl Sub<Polynomial> for i64
{
	type Output = Polynomial;
	fn sub(self, other: Polynomial) -> Polynomial { (-other) + (self as f64) }
}
impl Sub<&Polynomial> for i64
{
	type Output = Polynomial;
	fn sub(self, other: &Polynomial) -> Polynomial { (-other) + (self as f64) }
}

// SubAssign implementations
// ===================================================================================

impl SubAssign<&Polynomial> for Polynomial
{
	fn sub_assign(&mut self, other: &Polynomial) { *self += -other; }
}

// Ownership taking version
impl SubAssign<Polynomial> for Polynomial
{
	fn sub_assign(&mut self, other: Polynomial) { *self += -&other; }
}
// Number adding versions
impl<T: ToPrimitive + Debug> SubAssign<T> for Polynomial
{
	fn sub_assign(&mut self, other: T)
	{
		let x: f64 = match other.to_f64() {
			Some(y) => y,
			None => panic!("Error when converting {:?} to f64", other),
		};
		*self += -x;
	}
}
