use num::complex::Complex;
use num_traits::{Inv, One, PrimInt, Signed, ToPrimitive, Zero};
use std::{
	fmt::Debug,
	ops::{Mul, Sub},
};

pub trait FloatLike:
	Debug + Clone + One + Zero + Inv<Output = Self> + Mul<Output = Self> + Sub<Output = Self> + HasNorm
{
}
impl FloatLike for f32 {}
impl FloatLike for f64 {}
impl FloatLike for Complex<f32> {}
impl FloatLike for Complex<f64> {}

pub trait SignedIntLike: Clone + Debug + PrimInt + Signed + HasNorm {}
impl<T> SignedIntLike for T where T: Clone + Debug + PrimInt + Signed + HasNorm {}

pub trait HasNorm
{
	fn norm(&self) -> f64;
}
duplicate::duplicate! {
	[primitive_type; [f64]; [f32]; [i8]; [i16]; [i32]; [i64]; [isize]; [i128]; [u8]; [u16]; [u32]; [u64]; [usize]; [u128]]
	impl HasNorm for primitive_type {
		#[inline]
		fn norm(&self) -> f64 { (*self as f64).abs() }
	}
}

impl<T: ToPrimitive> HasNorm for Complex<T>
{
	fn norm(&self) -> f64
	{
		(Complex { re: self.re.to_f64().unwrap(),
		           im: self.im.to_f64().unwrap(), }).norm()
	}
}
