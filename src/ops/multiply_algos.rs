use crate::Polynomial;
use num_traits::Zero;
use rustfft::{num_complex::Complex, FftNum, FftPlanner};
use std::ops::{Mul, Sub};

impl<T> Polynomial<T>
where T: FftNum
{
	// Implements the Schönhage-Strassen algorithm for polynomial multiplication
	// This implementation is not optimal and therefore not used by default
	// Also, it only works with FftNum types including floating points
	// Time complexity: O(n log n log log n)
	// Space complexity: O(n)
	pub fn schonhage_strassen(p1: &Self, p2: &Self) -> Self
	{
		let zero = T::from_i32(0).unwrap();
		if p1.degree() == 0 {
			let c = p1[0];
			Self::from(Vec::from_iter(p2.into_iter().map(|&x| c * x)))
		} else if p2.degree() == 0 {
			let c = p2[0];
			Self::from(Vec::from_iter(p1.into_iter().map(|&x| c * x)))
		} else {
			let n = p1.degree() + p2.degree() + 1;
			let mut planner = FftPlanner::new();
			let mut buffer = vec![Complex { re: zero, im: zero }; n];

			{
				let mut p2_buffer = vec![Complex { re: zero, im: zero }; n];

				for k in 0..=p1.degree() {
					buffer[k].re = p1[k];
				}
				for k in 0..=p2.degree() {
					p2_buffer[k].re = p2[k];
				}

				let fft = planner.plan_fft_forward(n);
				fft.process(&mut buffer);
				fft.process(&mut p2_buffer);

				for k in 0..n {
					buffer[k] = buffer[k] * p2_buffer[k];
				}
			}

			let fft_inv = planner.plan_fft_inverse(n);
			fft_inv.process(&mut buffer);

			let new_p1: Vec<T> = buffer.iter().map(|&x| T::from(x.re / T::from_usize(n).unwrap())).collect();
			Self::from(new_p1)
		}
	}
}

impl<T> Polynomial<T>
where T: Mul<T, Output = T> + Clone + Zero /* Zero implicitly requires Add */
{
	// Implements the naive convolution algorithm for polynomial multiplication
	// Time complexity: O(n^2)
	// Space complexity: O(n)
	/* Example:
	let p1 = polynomial![1, 2, 3];
	let p2 = polynomial![4, 5, 6];
	let p3 = Polynomial::convolve(&p1, &p2);
	assert_eq!(p3, polynomial![4, 13, 28, 27, 18]);
	*/
	#[inline]
	pub fn convolve(p1: &Self, p2: &Self) -> Self
	{
		if p1.is_empty() || p2.is_empty() {
			return Self::zero();
		}
		if p1.degree() < p2.degree() {
			return Self::convolve(p2, p1);
		}
		let mut result = vec![T::zero(); p1.degree() + p2.degree() + 1];
		for k in 0..=p2.degree() {
			for j in 0..=p1.degree() {
				result[j + k] = result[j + k].clone() + p1[j].clone() * p2[k].clone();
			}
		}
		Self::from(result)
	}
}

impl<T> Polynomial<T>
where T: Mul<T, Output = T> + Sub<T, Output = T> + Clone + Zero
{
	// Implements the Karatsuba algorithm for polynomial multiplication
	// Time complexity: O(n^1.585) (1.585 ~ log2(3))
	// Space complexity: O(n)
	/* Example:
	let p1 = polynomial![1, 2, 3];
	let p2 = polynomial![4, 5, 6];
	let p3 = Polynomial::karatsuba(&p1, &p2);
	assert_eq!(p3, polynomial![4, 13, 28, 27, 18]);
	*/
	#[inline]
	pub fn karatsuba(p1: &Self, p2: &Self) -> Self
	{
		if p1.degree() < p2.degree() {
			return Self::karatsuba(p2, p1);
		}
		if p2.degree() < 2 {
			return Self::convolve(p1, p2);
		}
		let p1_slice = p1.0.clone().as_mut_slice();
		let p2_slice = p2.0.clone().as_mut_slice();
		let buffer = vec![T::zero(); p1.degree() + p2.degree() + 1].as_mut_slice();
		Self::karatsuba_inplace(p1_slice, p2_slice, buffer);
		Self::from(buffer.to_vec())
	}

	// The following function takes two polynomials p1 and p2 and puts their product
	// in buffer (which is assumed to be large enough to hold the result)
	#[inline]
	fn karatsuba_inplace(p1: &mut [T], p2: &mut [T], buffer: &mut [T])
	// Got to make sure that buffer.len() >= p1.len() + p2.len() - 1
	{
		if p1.len() < p2.len() {
			return Self::karatsuba_inplace(p2, p1, buffer);
		}
		if p2.len() < 2 {
			for k in 0..p2.len() {
				for j in 0..p1.len() {
					buffer[j + k] = buffer[j + k].clone() + p1[j].clone() * p2[k].clone();
				}
			}
			return;
		}
		let n = p1.len() - 1;
		let m = if n & 1 != 0 { n >> 1 + 1 } else { n >> 1 };
		// m = ceil(n/2) to make sure the lower polynomials have the highest degree
		let (p1_lo, p1_hi) = p1.split_at_mut(m);
		let (p2_lo, p2_hi) = p2.split_at_mut(m);

		let upper_buffer = &mut buffer[(m << 1)..];
		let middle_buffer = &mut buffer[m..(3 * m - 1)];
		let lower_buffer = &mut buffer[0..(m << 1 - 1)];

		Self::karatsuba_inplace(p1_lo, p2_lo, lower_buffer);
		// lower_buffer.len() is 2*m - 1 = p1_lo.len() + p2_lo.len() - 1

		Self::karatsuba_inplace(p1_hi, p2_hi, upper_buffer);
		// upper_buffer.len() = buffer.len() - 2*m
		// >= p1.len() + p2.len() - 1 - 2*m
		// = p1_hi.len() + p1_lo.len() + p2_hi.len() + p2.len() - 1 - 2*m
		// = p1.len() + p2.len() - 1
		// The upper_buffer choice is correct. Not only that, but this part of the
		// buffer has not been touched yet.

		for k in 0..upper_buffer.len() {
			middle_buffer[k] = middle_buffer[k].clone() - upper_buffer[k].clone();
		}
		for k in 0..lower_buffer.len() {
			middle_buffer[k] = middle_buffer[k].clone() - lower_buffer[k].clone();
		}

		for k in 0..m {
			// Having p1_hi.len() <= p2_lo.len() = m allows
			// this operation to be done in-place
			p1_lo[k] = p1_lo[k].clone() + p2_hi[k].clone();
			p2_lo[k] = p2_lo[k].clone() + p1_hi[k].clone();
		}

		Self::karatsuba_inplace(p1_lo, p2_lo, middle_buffer);
		// middle_buffer.len() = 2*m - 1 = p1_lo.len() + p2_lo.len() - 1
		// Technically, we could use buffer[m..] but this is more explicit

		for k in 0..m {
			// Revert the previous operation
			p1_lo[k] = p1_lo[k].clone() - p2_hi[k].clone();
			p2_lo[k] = p2_lo[k].clone() - p1_hi[k].clone();
		}
	}
}

// impl<T> Polynomial<T>
// where T: Mul<T, Output = T> + Sub<T, Output = T> + Div<T, Output = T> + Clone
// + Zero {
// 	// Implements the Toom-Cook 3 algorithm following Bodrado's method for
// 	// polynomial multiplication
// 	#[inline]
// 	pub fn toom3(p1: &Self, p2: &Self) -> Self
// 	{
// 		if p1.degree() < p2.degree() {
// 			return Self::toom3(p2, p1);
// 		}
// 		if p2.degree() < 2 {
// 			return Self::convolve(p1, p2);
// 		}
// 		todo!()
// 	}

// 	#[inline]
// 	fn toom3_inplace(p1: &mut [T], p2: &mut [T], buffer: &mut [T], minus: bool,
// eval_buffer: &mut [T]) 	{
// 		if p1.len() < p2.len() {
// 			return Self::toom3_inplace(p2, p1, buffer, minus, eval_buffer);
// 		}
// 		if p2.len() < 2 {
// 			if minus {
// 				for k in 0..p2.len() {
// 					for j in 0..p1.len() {
// 						buffer[j + k] = buffer[j + k].clone() - p1[j].clone() * p2[k].clone();
// 					}
// 				}
// 				return;
// 			}
// 			for k in 0..p2.len() {
// 				for j in 0..p1.len() {
// 					buffer[j + k] = buffer[j + k].clone() + p1[j].clone() * p2[k].clone();
// 				}
// 			}
// 			return;
// 		}
// 		let n = p1.len() - 1;
// 		let m = if n % 3 != 0 { n / 3 + 1 } else { n / 3 };

// 		let (p1_lo, p1_temp) = p1.split_at_mut(m);
// 		let (p1_mid, p1_hi) = p1_temp.split_at_mut(m);

// 		let (p2_lo, p2_temp) = p2.split_at_mut(m);
// 		let (p2_mid, p2_hi) = p2_temp.split_at_mut(m);

// 		let buffer0 = &mut buffer[0..(m << 1 - 1)];
// 		let buffer1 = &mut buffer[m..(3 * m - 1)];
// 		let buffer2 = &mut buffer[(m << 1)..(m << 2 - 1)];
// 		let buffer3 = &mut buffer[(3 * m)..(5 * m - 1)];
// 		let buffer4 = &mut buffer[(m << 2)..];

// 		// Self::toom3_inplace(p1_lo, p2_lo, buffer0, minus);
// 		// Self::toom3_inplace(p1_hi, p2_hi, buffer4, minus);

// 		todo!()
// 	}
// }
