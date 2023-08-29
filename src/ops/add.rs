use crate::Polynomial;
use duplicate::duplicate;
use num_traits::Zero;
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

// The following macros help implementing binary operators for polynomials
// Inspired by polynomial crate
macro_rules! impl_op_polynomial {
	($op:ident, $method:ident) => {
		impl<T> $op<Polynomial<T>> for &Polynomial<T>
		where T: $op<T, Output = T> + Clone + Zero
		{
			type Output = Polynomial<T>;
			#[inline]
			fn $method(self, other: Polynomial<T>) -> Polynomial<T> { self.$method(other) }
		}

		impl<T> $op<&Polynomial<T>> for Polynomial<T>
		where T: $op<T, Output = T> + Clone + Zero
		{
			type Output = Polynomial<T>;
			#[inline]
			fn $method(self, other: &Polynomial<T>) -> Polynomial<T> { self.$method(other) }
		}

		impl<T> $op<Polynomial<T>> for Polynomial<T>
		where T: $op<T, Output = T> + Clone + Zero
		{
			type Output = Polynomial<T>;
			#[inline]
			fn $method(self, other: Polynomial<T>) -> Polynomial<T> { (&self).$method(other) }
		}
	};
}

// The following macros allow to add polynomials with numbers
// Rust's specialization feature is not stable yet so we have to duplicate the
// code for each primitive type
macro_rules! impl_op_some_primitive {
	($op:ident, $method:ident, $t:ty) => {
		impl<T> $op<Polynomial<T>> for $t
		where T: $op<T, Output = T> + Clone + From<$t> + Zero
		{
			type Output = Polynomial<T>;
			#[inline]
			fn $method(self, other: Polynomial<T>) -> Polynomial<T> { other.$method(Polynomial::from(vec![self.into()])) }
		}
		impl<T> $op<&Polynomial<T>> for $t
		where T: $op<T, Output = T> + Clone + From<$t> + Zero
		{
			type Output = Polynomial<T>;
			#[inline]
			fn $method(self, other: &Polynomial<T>) -> Polynomial<T> { other.$method(Polynomial::from(vec![self.into()])) }
		}
		impl<T> $op<$t> for Polynomial<T>
		where T: $op<T, Output = T> + Clone + From<$t> + Zero
		{
			type Output = Polynomial<T>;
			#[inline]
			fn $method(self, other: $t) -> Polynomial<T> { self.$method(Polynomial::from(vec![other.into()])) }
		}
		impl<T> $op<$t> for &Polynomial<T>
		where T: $op<T, Output = T> + Clone + From<$t> + Zero
		{
			type Output = Polynomial<T>;
			#[inline]
			fn $method(self, other: $t) -> Polynomial<T> { self.$method(Polynomial::from(vec![other.into()])) }
		}
	};
}

macro_rules! impl_op_all_primitive {
	($op:ident, $method:ident) => {
		duplicate! {
			[primitive_type; [f64]; [f32]; [i8]; [i16]; [i32]; [i64]; [isize]; [i128]; [u8]; [u16]; [u32]; [u64]; [usize]; [u128]]
		impl_op_some_primitive!($op, $method, primitive_type);
		}
	};
}

// The next macro implements the assign versions of the operators
macro_rules! impl_assign_op {
	($op:ident, $assign_op:ident, $method:ident, $assign_method: ident) => {
		impl<T> $assign_op<Polynomial<T>> for Polynomial<T>
		where T: $op<T, Output = T> + Clone + Zero
		{
			#[inline]
			fn $assign_method(&mut self, other: Polynomial<T>) { *self = std::mem::take(self).$method(&other) }
		}
		impl<T> $assign_op<&Polynomial<T>> for Polynomial<T>
		where T: $op<T, Output = T> + Clone + Zero
		{
			#[inline]
			fn $assign_method(&mut self, other: &Polynomial<T>) { *self = std::mem::take(self).$method(other) }
		}
		duplicate! {
			[primitive_type; [f64]; [f32]; [i8]; [i16]; [i32]; [i64]; [isize]; [i128]; [u8]; [u16]; [u32]; [u64]; [usize]; [u128]]
			impl<T> $assign_op<primitive_type> for Polynomial<T>
			where T: $op<T, Output = T> + Clone + From<primitive_type> + Zero
			{
				#[inline]
				fn $assign_method(&mut self, other: primitive_type) { *self = std::mem::take(self).$method(Polynomial::from(vec![other.into()])) }
			}
		}
	};
}

macro_rules! impl_op_all {
	($op:ident, $assign_op:ident, $method:ident, $assign_method:ident) => {
		impl_op_polynomial!($op, $method);
		impl_op_all_primitive!($op, $method);
		impl_assign_op!($op, $assign_op, $method, $assign_method);
	};
	($op:ident, $method:ident) => {
		impl_op_polynomial!($op, $method);
		impl_op_all_primitive!($op, $method);
	};
}

//=================================================================================================

impl<T> Add<&Polynomial<T>> for &Polynomial<T>
where T: Add<T, Output = T> + Clone + Zero
{
	// Implements addition without taking ownership
	/* Example:
	let p1 = polynomial![1, 0, 2];
	let p2 = polynomial![-1, 1, 5];
	assert_eq!(p1 + p2, polynomial![0, 1, 7]);
	*/
	type Output = Polynomial<T>;
	#[inline]
	fn add(self, other: &Polynomial<T>) -> Polynomial<T>
	where T: Zero
	{
		if other.0.len() == 0 {
			return self.clone();
		}
		if self.0.len() == 0 {
			return other.clone();
		}
		if other.degree() > self.degree() {
			return other.add(self);
		}
		let mut sum = self.0.clone();
		for k in 0..=other.degree() {
			sum[k] = sum[k].clone() + other[k].clone();
		}
		Polynomial::from(sum)
	}
}

impl_op_all!(Add, AddAssign, add, add_assign);

// Negating a Polynomial
// ========================================================================================

impl<T> Neg for &Polynomial<T>
where T: Neg<Output = T> + Clone + Zero
{
	// Implements negation without taking ownership
	/* Example:
	let p = polynomial![1, 0, 2];
	assert_eq!(-p, polynomial![-1, 0, -2]);
	*/
	type Output = Polynomial<T>;
	fn neg(self) -> Polynomial<T> { Polynomial::from(self.0.iter().map(|x| -x.clone()).collect::<Vec<T>>()) }
}
impl<T> Neg for Polynomial<T>
where T: Neg<Output = T> + Clone + Zero
{
	// Version that takes ownership
	type Output = Polynomial<T>;
	#[inline]
	fn neg(self) -> Polynomial<T> { -&self }
}

// Subtraction
// =================================================================================================

impl<T> Sub<&Polynomial<T>> for &Polynomial<T>
where T: Sub<T, Output = T> + Clone + Neg<Output = T> + Zero
{
	// Implements subtraction without taking ownership
	/* Example:
	let p1 = polynomial![1, 0, 2];
	let p2 = polynomial![-1, 1, 5];
	assert_eq!(p1 - p2, polynomial![2, -1, -3]);
	*/
	type Output = Polynomial<T>;
	#[inline]
	fn sub(self, other: &Polynomial<T>) -> Polynomial<T>
	{
		if other.0.len() == 0 {
			return self.clone();
		}
		if self.0.len() == 0 {
			return -other;
		}
		if other.degree() > self.degree() {
			return -(other.sub(self));
		}
		let mut diff = self.0.clone();
		for k in 0..=other.degree() {
			diff[k] = diff[k].clone() - other[k].clone();
		}
		Polynomial::from(diff)
	}
}

impl_op_all!(Sub, SubAssign, sub, sub_assign);
