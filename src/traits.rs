use num::complex::Complex;
use num_traits::{FromPrimitive, Inv, One, PrimInt, Signed, ToPrimitive, Zero};
use std::{
	fmt::Debug,
	ops::{Div, Mul, Sub},
};

pub trait HasNorm
{
	fn norm(&self) -> f64;
}

pub trait Num:
	Debug
	+ num_traits::Num
	+ PartialEq
	+ Clone
	+ One
	+ Zero
	+ Mul<Output = Self>
	+ Sub<Output = Self>
	+ Div<Output = Self>
	+ HasNorm
	+ PartialEq
{
}

pub trait Primitive: Num + ToPrimitive + FromPrimitive {}

duplicate::duplicate! {
	[primitive_type; [f64]; [f32]; [i8]; [i16]; [i32]; [i64]; [isize]; [i128]; [u8]; [u16]; [u32]; [u64]; [usize]; [u128]]
	impl HasNorm for primitive_type {
		#[inline]
		fn norm(&self) -> f64 { (*self as f64).abs() }
	}
	impl HasNorm for Complex<primitive_type> {
		#[inline]
		fn norm(&self) -> f64 { (Complex {
			re: self.re.to_f64().unwrap(),
			im: self.im.to_f64().unwrap(),
		}).norm() }
	}
	impl Num for primitive_type {}
	impl Num for Complex<primitive_type> {}
	impl Primitive for primitive_type {}
}

pub trait FloatLike: Num + Inv<Output = Self> {}
impl FloatLike for f32 {}
impl FloatLike for f64 {}
impl FloatLike for Complex<f32> {}
impl FloatLike for Complex<f64> {}

pub trait SignedIntLike: Clone + Debug + PrimInt + Signed + HasNorm {}
impl<T> SignedIntLike for T where T: Clone + Debug + PrimInt + Signed + HasNorm {}
