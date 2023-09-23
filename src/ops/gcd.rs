use crate::traits::{FloatLike, SignedIntLike};
use crate::Polynomial;
use num_traits::Zero;

impl<T> Polynomial<T> where T: SignedIntLike
{
	// Input : Two polynomials p1 and p2
	// Output : gcd(p1, p2)

	// This implementation is clearly not optimal as it uses Euclid's algorithm.
	// It would be interesting to look at the more efficient Half-GCD algorithm.
	// See https://thibautverron.github.io/enseignement/2018-CompAlg2-notes.pdf page 41
	// However, I have not looked at it as of now.

	// This algorithm is not reliable most of the time, because it happens often
	// that the leading coefficient of p2 is not a divisor of every p1 and p2
	// coefficients. That can happen even though p1 and p2 are not coprime in Z[X].
	// Using gcd_float and rounding the resulting coefficients to integers through
	// gcd_rounded is probably a better idea.
	pub fn gcd(p1: &mut Self, p2: &mut Self) -> Self
	{
		let lc = p2[p2.degree()].clone();
		for i in 0..=p2.degree() {
			let previous = p2[i].clone();
			p2[i] = p2[i].clone() / lc.clone();
			if p2[i].clone() * lc.clone() != previous {
				p2[i] = previous.clone();
				for j in 0..i {
					p2[j] = p2[j].clone() * lc.clone();
				}
				panic!(
				       "GCD ERROR : Computing the gcd of\np1 = {p1:?}\nand\np2={p2:?}\nis impossible \
				        because their coefficients have integer types and p2's leading coefficient {lc:?} \
				        is not a divisor of p2's degree-{i} coefficient {:?}.",
				       previous
				)
			}
		}
		for i in 0..=p1.degree() {
			let previous = p1[i].clone();
			p1[i] = p1[i].clone() / lc.clone();
			if p1[i].clone() * lc.clone() != previous {
				p1[i] = previous.clone();
				for j in 0..i {
					p1[j] = p1[j].clone() * lc.clone();
				}
				for j in 0..=p2.degree() {
					p2[j] = p2[j].clone() * lc.clone();
				}
				panic!(
				       "GCD ERROR : Computing the gcd of\np1 = {p1:?}\nand\np2={p2:?}\nis impossible \
				        because their coefficients have integer types and p2's leading coefficient {lc:?} \
				        is not a divisor of p1's degree-{i} coefficient {:?}.",
				       previous
				)
			}
		}

		let res = if p2.is_zero() {
			p1.clone()
		} else {
			let (_, mut r) = Self::euclidean_division(p1, p2);
			Self::gcd(p2, &mut r)
		};
		for i in 0..p2.degree() {
			p2[i] = p2[i].clone() * lc.clone();
		}
		for i in 0..p1.degree() {
			p1[i] = p1[i].clone() * lc.clone();
		}
		res
	}

	pub fn gcd_immutable(p1: &Self, p2: &Self) -> Self { Self::gcd(&mut p1.clone(), &mut p2.clone()) }
}

impl<T> Polynomial<T> where T: SignedIntLike
{
	pub fn gcd_rounded(p1: &Self, p2: &Self) -> Self
	{
		let mut p1_float = p1.into_iter()
		                     .map(|x| x.clone().to_f64().unwrap())
		                     .collect::<Polynomial<f64>>();
		let mut p2_float = p2.into_iter()
		                     .map(|x| x.clone().to_f64().unwrap())
		                     .collect::<Polynomial<f64>>();
		let res_float = Polynomial::<f64>::gcd_float(&mut p1_float, &mut p2_float);
		let lc_inv = res_float[res_float.degree()].recip();
		res_float.into_iter()
		         .map(|x| T::from((x * lc_inv).round()).unwrap())
		         .collect::<Self>()
	}
}

impl<T> Polynomial<T> where T: FloatLike
{
	pub fn gcd_float(p1: &mut Self, p2: &mut Self) -> Self
	{
		let mut res = Self::gcd_float_aux(p1, p2);
		let lc_inv = res[res.degree()].clone().inv();
		for i in 0..=res.degree() {
			res[i] = res[i].clone() * lc_inv.clone();
		}
		res
	}

	pub fn gcd_float_aux(p1: &mut Self, p2: &mut Self) -> Self
	{
		if p2.is_zero() {
			p1.clone()
		} else {
			let (_, mut r) = Self::euclidean_division_float(p1, p2);
			Self::gcd_float_aux(p2, &mut r)
		}
	}

	pub fn gcd_float_immutable(p1: &Self, p2: &Self) -> Self
	{
		Self::gcd_float(&mut p1.clone(), &mut p2.clone())
	}
}

impl<T> Polynomial<T> where T: SignedIntLike
{
	// Input : Two polynomials p1 and p2
	// Output : (q1, q2) coprime such that p1 / p2 = q1 / q2
	pub fn cofactor_rounded(p1: &mut Self, p2: &mut Self) -> (Self, Self)
	{
		let mut gcd = Self::gcd_rounded(p1, p2);
		let (q1, _) = Self::euclidean_division(p1, &mut gcd);
		let (q2, _) = Self::euclidean_division(p2, &mut gcd);
		(q1, q2)
	}

	pub fn cofactor_rounded_immutable(p1: &Self, p2: &Self) -> (Self, Self)
	{
		Self::cofactor_rounded(&mut p1.clone(), &mut p2.clone())
	}
}

impl<T> Polynomial<T> where T: FloatLike
{
	// Input : Two polynomials p1 and p2
	// Output : (q1, q2) coprime such that p1 / p2 = q1 / q2
	pub fn cofactor_float(p1: &mut Self, p2: &mut Self) -> (Self, Self)
	{
		let mut gcd = Self::gcd_float(p1, p2);
		let (q1, _) = Self::euclidean_division_float(p1, &mut gcd);
		let (q2, _) = Self::euclidean_division_float(p2, &mut gcd);
		(q1, q2)
	}

	pub fn cofactor_float_immutable(p1: &Self, p2: &Self) -> (Self, Self)
	{
		Self::cofactor_float(&mut p1.clone(), &mut p2.clone())
	}
}
