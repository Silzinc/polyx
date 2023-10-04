use crate::*;
use num::Complex;

#[test]
fn test_instantiate()
{
	let p1 = polynomial![0, 2, 1]; // X^2 + 2X
	let p2: Polynomial<i32> = polynomial_expr!(X(X + 2));
	let p3: Polynomial<i32> = polynomial_expr!(X ^ 2 + 2X);
	assert_eq!(p1, p2);
	assert_eq!(p1, p3);

	let p4 = Polynomial::<Complex<f64>>::parse_string_complex("i (X + i)^2".to_string()).unwrap();
	println!("{}", p4);
}

#[test]
fn test_op()
{
	let p1: Polynomial<i32> = polynomial_expr!(X ^ 3);
	let p2: Polynomial<i32> = polynomial_expr!(X ^ 2(1 + X));
	assert_eq!(&p1 - &p2, polynomial![0, 0, -1]);
	assert_eq!(&p1 + &p2, polynomial![0, 0, 1, 2]);
	assert_eq!(&p1 * &p2, polynomial![0, 0, 0, 0, 0, 1, 1]);
}

#[test]
fn test_euclid()
{
	let a = polynomial![1, 0, 2];
	let b = polynomial![1, 1];
	let (q, r) = Polynomial::euclidean_division_immutable(&a, &b);
	assert_eq!(b * q + r, a);

	let mut a = polynomial![1, 0, 2];
	let mut b = polynomial![1, 1];
	let (q, r) = Polynomial::euclidean_division(&mut a, &mut b);
	assert_eq!(b * q + r, a);
}

#[test]
fn test_inv_sp()
{
	let p = polynomial![1, -4, 0, -2, 5, 1, 1, 1];
	let inv10 = Polynomial::inverse(&p, 10);
	assert_eq!(Polynomial::short_product(&p, &inv10, 10), polynomial![1]);
}

#[test]
fn test_shift()
{
	let p = polynomial![1, 0, 2];
	assert_eq!(&p << 2, polynomial![0, 0, 1, 0, 2]);
	assert_eq!(&p >> 2, polynomial![2]);
}

#[test]
fn test_latex()
{
	let p = polynomial![1, 2, -5, -3];
	println!("{}", p.to_latex_string());
}
