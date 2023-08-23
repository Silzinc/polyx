use crate::Polynomial;
use crate::Polynomial::{NonZero, Zero, X};
use std::fmt;

fn pretty_float(x: f64, n: u8) -> String
{
	// Gives string represeting x with n + 1 significant figures
	if x == 0. {
		return String::from("0");
	}
	if x < 0. {
		return format!("-{}", pretty_float(-x, n));
	}
	let r: i32 = x.log10().floor() as i32;
	let d = r - n as i32;
	let power_ten = 10f64.powi(d);
	let rounded: i64 = (x / power_ten).round() as i64;
	if d == 0 {
		format!("{}", rounded)
	} else if r.abs() < n as i32 {
		if d > 0 {
			let mut result: String = rounded.to_string();
			for _ in 0..d {
				result.push('0');
			}
			result
		} else {
			let mut result: String = rounded.to_string();
			let point_index = result.len() - (-d) as usize;
			if point_index == 0 {
				String::from("0.") + &result
			} else {
				result.insert(point_index, '.');
				result
			}
		}
	} else {
		format!("{0:10.1$e}", (rounded as f64) * power_ten, n as usize)
	}
}

impl fmt::Display for Polynomial
{
	// Allows to display the polynomial in a fancy way
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		let tol: f64 = 0.0000000001;
		match self {
			Zero => write!(f, "Polynomial(-Inf)"),
			X => write!(f, "Polynomial(1)\n    1.000e0 X^1"),
			NonZero(coefs) => {
				let degree = self.degree();
				if degree == 0 {
					write!(f, "{}", format!("Polynomial(0)\n {:10.3e}", coefs[0]))
				} else {
					let mut result_str: String = format!("Polynomial({})", degree);
					let c = coefs[degree];
					if c > tol {
						result_str += &format!("\n {:10.3e} X^{}", c, degree);
					} else if c < -tol {
						result_str += &format!("\n-{:10.3e} X^{}", -c, degree);
					}
					for index in 1..degree {
						let c = coefs[degree - index];
						if c > tol {
							result_str += &format!("\n+{:10.3e} X^{}", c, degree - index);
						} else if c < -tol {
							result_str += &format!("\n-{:10.3e} X^{}", -c, degree - index);
						}
					}
					let c = coefs[0];
					if c > tol {
						result_str += &format!("\n+{:10.3e}", c);
					} else if c < -tol {
						result_str += &format!("\n-{:10.3e}", -c);
					}
					write!(f, "{}", result_str)
				}
			}
		}
	}
}

impl Polynomial
{
	pub fn to_latex_string(&self) -> String
	{
		let tol: f64 = 2f64.powi(-40);
		match self {
			Zero => String::from("0"),
			X => String::from("X"),
			NonZero(coefs) => {
				let mut degree = self.degree();
				while degree > 0 && coefs[degree].abs() < tol {
					degree -= 1;
				}
				if degree == 0 {
					let c = coefs[0];
					if c.abs() > tol {
						format!("{}", pretty_float(c, 2u8))
					} else {
						String::from("0")
					}
				} else if degree == 1 {
					let c0 = coefs[0];
					let c1 = coefs[1];
					if c1.abs() > tol {
						if c0 > tol {
							format!("{}\\, X + {}", pretty_float(c1, 2u8), pretty_float(c0, 2u8))
						} else if c0 < -tol {
							format!("{}\\, X - {}", pretty_float(c1, 2u8), pretty_float(-c0, 2u8))
						} else {
							format!("{}\\, X", pretty_float(c1, 2u8))
						}
					// No need to check when c1 < -tol in this particular case
					} else {
						format!("{}", pretty_float(c0, 2u8))
					}
				} else {
					let mut result_str: String = String::from("");
					let c = coefs[degree];
					if c >= 0. {
						result_str += &format!("{}\\, &X^{{{}}}&", pretty_float(c, 2u8), degree);
					} else {
						result_str += &format!("-{}\\, &X^{{{}}}&", pretty_float(-c, 2u8), degree);
					}
					for index in 1..(degree - 1) {
						let c = coefs[degree - index];
						if index % 3 == 0 {
							result_str += "\\\\";
						}
						if c > tol {
							result_str += &format!("+{}\\, &X^{{{}}}&", pretty_float(c, 2u8), degree - index);
						} else if c < -tol {
							result_str += &format!("-{}\\, &X^{{{}}}&", pretty_float(-c, 2u8), degree - index);
						}
					}
					let c = coefs[1];
					if degree % 3 == 1 {
						result_str += "\\\\";
					}
					if c > tol {
						result_str += &format!("+{}\\, &X&", pretty_float(c, 2u8));
					} else if c < -tol {
						result_str += &format!("-{}\\, &X&", pretty_float(-c, 2u8));
					}
					let c = coefs[0];
					if degree % 3 == 0 {
						result_str += "\\\\";
					}
					if c > tol {
						result_str += &format!("+{} &", pretty_float(c, 2u8));
					} else if c < -tol {
						result_str += &format!("-{} &", pretty_float(-c, 2u8));
					}
					result_str
				}
			}
		}
	}
}
