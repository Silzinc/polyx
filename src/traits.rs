use num::complex::Complex;
use num_traits::{FromPrimitive, Inv, One, PrimInt, Signed, ToPrimitive, Zero};
use std::{
	fmt::{Debug, Display},
	ops::{Div, Mul, Sub},
};

pub trait HasNorm
{
	fn norm(&self) -> f64;
}

pub trait PolyxNum:
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
	+ Display
{
}

pub trait Primitive: PolyxNum + ToPrimitive + FromPrimitive + PartialOrd {}

duplicate::duplicate! {
	[primitive_type; [f64]; [f32]; [i8]; [i16]; [i32]; [i64]; [isize]; [i128]; [u8]; [u16]; [u32]; [u64]; [usize]; [u128]]
	impl HasNorm for primitive_type {
		#[inline]
		fn norm(&self) -> f64 { (*self as f64).abs() }
	}
	impl Primitive for primitive_type {}
}

impl<T: Primitive> PolyxNum for T {}
impl<T: Primitive> PolyxNum for Complex<T> {}
impl<T: Primitive> HasNorm for Complex<T>
{
	#[inline]
	fn norm(&self) -> f64
	{
		(Complex { re: self.re.to_f64().unwrap(),
		           im: self.im.to_f64().unwrap(), }).norm()
	}
}

pub trait FloatLike: PolyxNum + Inv<Output = Self> {}
impl FloatLike for f32 {}
impl FloatLike for f64 {}
impl FloatLike for Complex<f32> {}
impl FloatLike for Complex<f64> {}

pub trait SignedIntLike: Clone + Debug + PrimInt + Signed + HasNorm {}
impl<T> SignedIntLike for T where T: Clone + Debug + PrimInt + Signed + HasNorm {}
