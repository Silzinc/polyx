// use rustfft::{num_complex::Complex, FftNum, FftPlanner};
use std::{
	cmp::{max, min, Ordering},
	fmt::Debug,
	ops::{Mul, Sub},
};

use num_traits::Zero;

use crate::{traits::HasNorm, Polynomial};

// impl<T> Polynomial<T> where T: FftNum + HasNorm
// {
// 	// Implements the Schönhage-Strassen algorithm for polynomial multiplication
// 	// This implementation is not optimal and therefore not used by default
// 	// Also, it only works with FftNum types including floating points
// 	// Time complexity: O(n log n log log n)
// 	// Space complexity: O(n)
// 	pub fn schonhage_strassen(p1: &Self, p2: &Self) -> Self
// 	{
// 		let zero = T::from_i32(0).unwrap();
// 		if p1.degree() == 0 {
// 			let c = p1[0];
// 			Self::from(Vec::from_iter(p2.into_iter().map(|&x| c * x)))
// 		} else if p2.degree() == 0 {
// 			let c = p2[0];
// 			Self::from(Vec::from_iter(p1.into_iter().map(|&x| c * x)))
// 		} else {
// 			let n = p1.degree() + p2.degree() + 1;
// 			let mut planner = FftPlanner::new();
// 			let mut buffer = vec![Complex { re: zero, im: zero }; n];

// 			{
// 				let mut p2_buffer = vec![Complex { re: zero, im: zero }; n];

// 				for k in 0..=p1.degree() {
// 					buffer[k].re = p1[k];
// 				}
// 				for k in 0..=p2.degree() {
// 					p2_buffer[k].re = p2[k];
// 				}

// 				let fft = planner.plan_fft_forward(n);
// 				fft.process(&mut buffer);
// 				fft.process(&mut p2_buffer);

// 				for k in 0..n {
// 					buffer[k] = buffer[k] * p2_buffer[k];
// 				}
// 			}

// 			let fft_inv = planner.plan_fft_inverse(n);
// 			fft_inv.process(&mut buffer);

// 			let new_p1: Vec<T> = buffer.iter()
// 			                           .map(|&x| T::from(x.re /
// T::from_usize(n).unwrap())) 			                           .collect();
// 			Self::from(new_p1)
// 		}
// 	}
// }

impl<T> Polynomial<T>
where
	T: Mul<T, Output = T> + Clone + Zero + Debug + HasNorm, /* Zero implicitly requires Add */
{
	/// Implements the naive convolution algorithm for polynomial (short)
	/// multiplication
	///
	/// # Arguments
	///
	/// * `p1` - The first polynomial to multiply
	/// * `p2` - The second polynomial to multiply
	/// * `truncate` - The maximum degree of the resulting polynomial
	///
	/// # Example
	///
	/// ```
	/// use polyx::*;
	/// let p1 = polynomial![1, 2, 3];
	/// let p2 = polynomial![4, 5, 6];
	/// let p3 = Polynomial::convolve(&p1, &p2, 5);
	/// assert_eq!(p3, polynomial![4, 13, 28, 27, 18]);
	/// ```
	///
	/// # Time complexity
	///
	/// O(n^2)
	///
	/// # Space complexity
	///
	/// O(n)
	#[inline]
	pub fn convolve(p1: &Self, p2: &Self, truncate: usize) -> Self {
		if p1.is_empty() || p2.is_empty() {
			return Self::zero();
		}
		if p1.degree() < p2.degree() {
			return Self::convolve(p2, p1, truncate);
		}
		let mut result =
			vec![T::zero(); std::cmp::min(p1.degree() + p2.degree() + 1, truncate + 1)];
		for k in 0..=p2.degree() {
			for j in 0..=std::cmp::min(p1.degree(), truncate - k) {
				result[j + k] = result[j + k].clone() + p1[j].clone() * p2[k].clone();
			}
		}
		Self::from(result)
	}
}

impl<T> Polynomial<T>
where
	T: Mul<T, Output = T> + Sub<T, Output = T> + Clone + Zero + Debug + HasNorm,
{
	/// Implements the Karatsuba algorithm for polynomial (short) multiplication
	/// Based on this paper: <https://members.loria.fr/EThome/files/kara.pdf>
	///
	/// # Time complexity
	///
	/// O(n^1.585) (1.585 ~ log2(3)) where n = max(p1.degree(), p2.degree())
	///
	/// # Space complexity
	///
	/// At most ~2n additional allocations done once at the beginning of the
	/// algorithm
	///
	/// # Example
	///
	/// ```
	/// use polyx::*;
	/// let p1 = polynomial![1, 2, 3, 4, 5, 6, 7, 8];
	/// let p2 = polynomial![4, 5, 6, 7, 8, 9, 10, 11];
	/// let p3 = Polynomial::karatsuba(&p1, &p2, 12);
	/// assert_eq!(
	///   p3,
	///   polynomial![4, 13, 28, 50, 80, 119, 168, 228, 252, 263, 260, 242]
	/// );
	/// ```
	#[inline]
	pub fn karatsuba(p1: &Self, p2: &Self, truncate: usize) -> Self {
		if truncate == 0 || p1.is_zero() || p2.is_zero() {
			return Self::zero();
		}
		// println!("p1: {p1:?}, p2: {p2:?}");
		let fact_p2 = p2.into_iter().position(|x| !x.is_zero()).unwrap();
		let fact_p1 = p1.into_iter().position(|x| !x.is_zero()).unwrap();
		let eff_p2 = &p2.0[fact_p2..min(truncate, p2.degree() + 1)];
		let eff_p1 = &p1.0[fact_p1..min(truncate, p1.degree() + 1)];

		let mut binding = vec![T::zero(); max(eff_p1.len(), eff_p2.len())];

		let (p1_slice, p2_slice) = match eff_p1.len().cmp(&eff_p2.len()) {
			Ordering::Less => {
				binding[0..eff_p1.len()].clone_from_slice(eff_p1);
				(binding.as_slice(), eff_p2)
			}
			Ordering::Greater => {
				binding[0..eff_p2.len()].clone_from_slice(eff_p2);
				(eff_p1, binding.as_slice())
			}
			Ordering::Equal => (eff_p1, eff_p2),
		};

		let mut binding_result =
			vec![T::zero(); fact_p1 + fact_p2 + p1_slice.len() + p2_slice.len() - 1];
		let result = binding_result.as_mut_slice();

		let mut binding_buffer = vec![T::zero(); p1_slice.len() - 1 + (p1_slice.len() & 1)];
		let buffer = binding_buffer.as_mut_slice();

		// println!("Initial call");
		Self::karatsuba_inplace(
			p1_slice,
			p2_slice,
			&mut result[(fact_p1 + fact_p2)..],
			buffer,
		);
		binding_result.truncate(truncate);
		Self::from(binding_result)
	}

	/// Takes two polynomials `p1` and `p2 and puts their product mod X^`truncate`
	/// in `result` (which is assumed to be large enough to hold the result)
	#[inline]
	pub(crate) fn karatsuba_inplace(p1: &[T], p2: &[T], result: &mut [T], buffer: &mut [T]) {
		// println!("p1: {p1:?}, p2: {p2:?}");
		assert_eq!(p1.len(), p2.len());
		result.fill_with(|| T::zero());

		if p2.len() < 6 {
			for k in 0..p2.len() {
				for j in 0..p1.len() {
					result[j + k] = result[j + k].clone() + p1[j].clone() * p2[k].clone();
				}
			}
			return;
		}

		let n = p1.len();
		let p = n >> 1;
		let q = n - p;
		let odd = p != q;

		/* Step 1 */
		for k in 0..p {
			result[k] = p1[k].clone() - p1[k + p].clone();
		}
		if odd {
			result[p] = T::zero() - p1[n - 1].clone();
		}

		/* Step 2 */
		for k in 0..p {
			result[k + q] = p2[k + p].clone() - p2[k].clone();
		}
		if odd {
			result[n] = p2[n - 1].clone();
		}

		/* Step 3 */
		// This is necessary to obey the borrow checker rules
		let (lower_result, temp) = result.split_at_mut(q);
		let (middle_result, upper_result) = temp.split_at_mut(q);
		// println!("Step 3 call");
		Self::karatsuba_inplace(
			lower_result,
			middle_result,
			&mut buffer[0..((q << 1) - 1)],
			upper_result,
		);

		/* Step 4 */
		let (_, upper_result) = temp.split_at_mut((p << 1) - q);
		// println!("Step 4 call");
		Self::karatsuba_inplace(&p1[p..n], &p2[p..n], upper_result, lower_result);

		/* Step 5 */
		for k in 0..((q << 1) - 1) {
			buffer[k] = buffer[k].clone() + result[k + (p << 1)].clone();
		}

		/* Step 6 */
		result[p..(p << 1)].clone_from_slice(&buffer[0..p]);

		/* Step 7 */
		for k in p..((q << 1) - 1) {
			result[k + p] = result[k + p].clone() + buffer[k].clone();
		}

		/* Step 8 */
		// println!("Step 8 call");
		Self::karatsuba_inplace(
			&p1[0..p],
			&p2[0..p],
			&mut buffer[0..((p << 1) - 1)],
			&mut result[0..p],
		);

		/* Step 9 */
		result[0..p].clone_from_slice(&buffer[0..p]);

		/* Step 10 */
		for k in p..((p << 1) - 1) {
			result[k] = result[k].clone() + buffer[k].clone();
		}

		/* Step 11 */
		for k in 0..((p << 1) - 1) {
			result[k + p] = result[k + p].clone() + buffer[k].clone();
		}
	}
}
