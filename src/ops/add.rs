use crate::convert;
use crate::Polynomial::{self, NonZero, Zero, X};
use duplicate::duplicate;
use num_traits::{Float, ToPrimitive};
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

impl<T: Float + AddAssign> Add<&Polynomial<T>> for &Polynomial<T>
{
	// Implements addition without taking ownership
	type Output = Polynomial<T>;

	fn add(self, other: &Polynomial<T>) -> Polynomial<T>
	{
		if other.degree() > self.degree() {
			other.add(self)
		} else {
			match (self, other) {
				(Zero, _) => other.clone(),
				(_, Zero) => self.clone(),
				(X, X) => NonZero(vec![convert(1), convert(2)]),
				(X, NonZero(other_coefs)) =>
					if other_coefs.len() == 0 {
						X
					} else if other_coefs.len() == 1 {
						NonZero(vec![other_coefs[0], convert(1)])
					} else {
						let mut new_coefs = other_coefs.clone();
						new_coefs[1] += convert(1);
						NonZero(new_coefs)
					},
				(NonZero(coefs), X) =>
					if coefs.len() == 0 {
						X
					} else if coefs.len() == 1 {
						NonZero(vec![coefs[0], convert(1)])
					} else {
						let mut new_coefs = coefs.clone();
						new_coefs[1] += convert(1);
						NonZero(new_coefs)
					},
				(NonZero(coefs), NonZero(other_coefs)) => {
					let mut new_coefs = coefs.clone();
					for k in 0..other_coefs.len() {
						new_coefs[k] += other_coefs[k];
					}
					NonZero(new_coefs)
				},
			}
		}
	}
}

impl<T: Float + AddAssign> Add<&Polynomial<T>> for Polynomial<T>
{
	type Output = Polynomial<T>;
	fn add(self, other: &Polynomial<T>) -> Polynomial<T> { &self + other }
}
impl<T: Float + AddAssign> Add<Polynomial<T>> for &Polynomial<T>
{
	type Output = Polynomial<T>;
	fn add(self, other: Polynomial<T>) -> Polynomial<T> { self + &other }
}
impl<T: Float + AddAssign> Add<Polynomial<T>> for Polynomial<T>
{
	type Output = Polynomial<T>;
	fn add(self, other: Polynomial<T>) -> Polynomial<T> { &self + &other }
}

// Number adding versions
impl<F, T> Add<T> for Polynomial<F>
where
	F: Float + AddAssign,
	T: ToPrimitive,
{
	type Output = Polynomial<F>;
	fn add(self, other: T) -> Polynomial<F>
	{
		let x = convert(other);
		&self + Polynomial::<F>::from(vec![x])
	}
}

impl<F, T> Add<T> for &Polynomial<F>
where
	F: Float + AddAssign,
	T: ToPrimitive,
{
	type Output = Polynomial<F>;
	fn add(self, other: T) -> Polynomial<F>
	{
		let x = convert(other);
		self + Polynomial::<F>::from(vec![x])
	}
}

duplicate! {
	[primitive_type; [f64]; [f32]; [i8]; [i16]; [i32]; [i64]; [isize]; [i128]; [u8]; [u16]; [u32]; [u64]; [usize]; [u128]]

	impl<T: Float + AddAssign> Add<Polynomial<T>> for primitive_type
	{
		type Output = Polynomial<T>;
		fn add(self, other: Polynomial<T>) -> Polynomial<T> { other + self }
	}
	impl<T: Float + AddAssign> Add<&Polynomial<T>> for primitive_type
	{
		type Output = Polynomial<T>;
		fn add(self, other: &Polynomial<T>) -> Polynomial<T> { other + self }
	}
}

// AddAssign implementations
// ====================================================================================

impl<T: Float + AddAssign> AddAssign<&Polynomial<T>> for Polynomial<T>
{
	fn add_assign(&mut self, other: &Polynomial<T>) { *self = *self + other }
}

// Ownership taking version
impl<T: Float + AddAssign> AddAssign<Polynomial<T>> for Polynomial<T>
{
	fn add_assign(&mut self, other: Polynomial<T>) { *self = *self + other }
}

// Number adding versions
impl<F, T> AddAssign<T> for Polynomial<F>
where
	F: Float + AddAssign,
	T: ToPrimitive,
{
	fn add_assign(&mut self, other: T) { *self = *self + other }
}

// Negating a Polynomial
// ========================================================================================

impl<T: Float> Neg for &Polynomial<T>
{
	// Implements negation (a copy of the Polynomial<T> is created when this is
	// called)
	type Output = Polynomial<T>;

	fn neg(self) -> Polynomial<T>
	{
		match self {
			Zero => Zero,
			X => NonZero(vec![convert(0), convert(-1)]),
			NonZero(coefs) => {
				let mut new_coefs = vec![convert(0); coefs.len()];
				for index in 0..new_coefs.len() {
					new_coefs[index] = -coefs[index];
				}
				NonZero(new_coefs)
			},
		}
	}
}
impl<T: Float> Neg for Polynomial<T>
{
	// Version that takes ownership
	type Output = Polynomial<T>;
	fn neg(self) -> Polynomial<T> { -&self }
}

// Subtraction
// =================================================================================================

impl<T: Float + AddAssign> Sub<&Polynomial<T>> for &Polynomial<T>
{
	// Implements subtraction
	type Output = Polynomial<T>;
	fn sub(self, other: &Polynomial<T>) -> Polynomial<T> { self + (-other) }
}

// Ownership taking versions

impl<T: Float + AddAssign> Sub<Polynomial<T>> for &Polynomial<T>
{
	type Output = Polynomial<T>;
	fn sub(self, other: Polynomial<T>) -> Polynomial<T> { self + (-&other) }
}
impl<T: Float + AddAssign> Sub<Polynomial<T>> for Polynomial<T>
{
	type Output = Polynomial<T>;
	fn sub(self, other: Polynomial<T>) -> Polynomial<T> { &self + (-&other) }
}
impl<T: Float + AddAssign> Sub<&Polynomial<T>> for Polynomial<T>
{
	type Output = Polynomial<T>;
	fn sub(self, other: &Polynomial<T>) -> Polynomial<T> { &self + (-other) }
}

// Number subtracting versions

impl<F, T> Sub<T> for Polynomial<F>
where
	F: Float + AddAssign,
	T: ToPrimitive,
{
	type Output = Polynomial<F>;
	fn sub(self, other: T) -> Polynomial<F>
	{
		let x: F = convert(other);
		&self + Polynomial::<F>::from(vec![-x])
	}
}
impl<F, T> Sub<T> for &Polynomial<F>
where
	F: Float + AddAssign,
	T: ToPrimitive,
{
	type Output = Polynomial<F>;
	fn sub(self, other: T) -> Polynomial<F>
	{
		let x: F = convert(other);
		self + Polynomial::<F>::from(vec![-x])
	}
}

duplicate! {
	[primitive_type; [f64]; [f32]; [i8]; [i16]; [i32]; [i64]; [isize]; [i128]; [u8]; [u16]; [u32]; [u64]; [usize]; [u128]]

	impl<T: Float + AddAssign> Sub<Polynomial<T>> for primitive_type
	{
		type Output = Polynomial<T>;
		fn sub(self, other: Polynomial<T>) -> Polynomial<T> { (-other) + self }
	}
	impl<T: Float + AddAssign> Sub<&Polynomial<T>> for primitive_type
	{
		type Output = Polynomial<T>;
		fn sub(self, other: &Polynomial<T>) -> Polynomial<T> { (-other) + self }
	}
}

// SubAssign implementations
// ===================================================================================

impl<T: Float + AddAssign> SubAssign<&Polynomial<T>> for Polynomial<T>
{
	fn sub_assign(&mut self, other: &Polynomial<T>) { *self += -other; }
}

// Ownership taking version
impl<T: Float + AddAssign> SubAssign<Polynomial<T>> for Polynomial<T>
{
	fn sub_assign(&mut self, other: Polynomial<T>) { *self += -&other; }
}
// Number subtracting versions
impl<F, T> SubAssign<T> for Polynomial<F>
where
	F: Float + AddAssign,
	T: ToPrimitive + Neg,
	<T as Neg>::Output: ToPrimitive,
{
	fn sub_assign(&mut self, other: T) { *self += -other; }
}
