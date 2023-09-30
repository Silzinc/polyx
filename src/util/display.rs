use crate::{consts::TOL, Polynomial};
use num::complex::Complex;
use num_traits::{Num, ToPrimitive, Zero};
use std::fmt::{self, Debug};

fn pretty_float(x: f64, n: u8) -> String
{
	// Gives string representing x with n + 1 significant figures
	if x.abs() < TOL {
		return "0".to_string();
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
				"0.".to_string() + &result
			} else {
				result.insert(point_index, '.');
				result
			}
		}
	} else {
		format!(
		        "{0:10.1$}*10^{{ {2} }}",
		        rounded as f64,
		        n as usize,
		        power_ten.log10().round()
		)
	}
}

fn pretty_float_complex(x: Complex<f64>, n: u8) -> String
{
	// Gives string reprenseting x with n + 1 significant figures
	// Takes into account the potential parenthesis necessary
	if x.norm() < TOL {
		return "0".to_string();
	}
	if x.im.abs() < TOL {
		return pretty_float(x.re, n);
	}
	if x.re.abs() < TOL {
		if x.im < 0. {
			return format!("({}i)", pretty_float(x.im, n));
		}
		return format!("{}i", pretty_float(x.im, n));
	}
	if x.im < 0. {
		return format!("({}-{}i)", pretty_float(x.re, n), pretty_float(x.im, n));
	}
	return format!("({}+{}i)", pretty_float(x.re, n), pretty_float(x.im, n));
}

fn to_complexf64<T>(c: Complex<T>) -> Option<Complex<f64>>
	where T: Clone + ToPrimitive
{
	Some(Complex::new(c.re.to_f64()?, c.im.to_f64()?))
}

impl<T> Polynomial<T> where T: ToPrimitive + Clone + Zero
{
	// Gives a LaTeX code to print the polynomial as long as the coefficients can be
	// turned into f64 Only the first SIGNIF_FIGS + 1 significant figures are
	// printed Only the coefficients with absolute value greater than TOL are
	// printed
	/* Example:
	let p = polynomial![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
	println!("{}", p.to_latex_string());
	*/
	const SIGNIF_FIGS: u8 = 2;

	pub fn to_latex_string(&self) -> String
	{
		if self.is_empty() {
			return "0".to_string();
		}
		let mut degree = self.degree();
		while degree > 0 && self[degree].to_f64().unwrap().abs() < TOL {
			degree -= 1;
		}
		if degree == 0 {
			let c: f64 = self[0].to_f64().unwrap();
			if c.abs() > TOL {
				format!("{}", pretty_float(c, Self::SIGNIF_FIGS))
			} else {
				String::from("0")
			}
		} else if degree == 1 {
			let c0: f64 = self[0].to_f64().unwrap();
			let c1: f64 = self[1].to_f64().unwrap();
			if c1.abs() > TOL {
				if c0 > TOL {
					format!(
					        "{}\\, X + {}",
					        pretty_float(c1, Self::SIGNIF_FIGS),
					        pretty_float(c0, Self::SIGNIF_FIGS)
					)
				} else if c0 < -TOL {
					format!(
					        "{}\\, X - {}",
					        pretty_float(c1, Self::SIGNIF_FIGS),
					        pretty_float(-c0, Self::SIGNIF_FIGS)
					)
				} else {
					format!("{}\\, X", pretty_float(c1, Self::SIGNIF_FIGS))
				}
			// No need to check when c1 < -TOL in this particular case
			} else {
				format!("{}", pretty_float(c0, Self::SIGNIF_FIGS))
			}
		} else {
			let mut result_str_vec = Vec::with_capacity(degree + 2);
			let c: f64 = self[degree].to_f64().unwrap();
			if c >= 0. {
				result_str_vec.push(format!("{}\\, X^{{{degree}}}", pretty_float(c, Self::SIGNIF_FIGS)));
			} else {
				result_str_vec.push(format!("-{}\\, X^{{{degree}}}", pretty_float(-c, Self::SIGNIF_FIGS)));
			}
			for index in (2..degree).rev() {
				let c: f64 = self[index].to_f64().unwrap();
				// if index % 3 == 0 {
				// 	result_str_vec.push("\\\\".to_string());
				// }
				if c > TOL {
					result_str_vec.push(format!("+{}\\, X^{{{index}}}", pretty_float(c, Self::SIGNIF_FIGS)));
				} else if c < -TOL {
					result_str_vec.push(format!("-{}\\, X^{{{index}}}", pretty_float(-c, Self::SIGNIF_FIGS)));
				}
			}
			let c: f64 = self[1].to_f64().unwrap();
			// if degree % 3 == 1 {
			// 	result_str_vec.push("\\\\".to_string());
			// }
			if c > TOL {
				result_str_vec.push(format!("+{}\\, X", pretty_float(c, Self::SIGNIF_FIGS)));
			} else if c < -TOL {
				result_str_vec.push(format!("-{}\\, X", pretty_float(-c, Self::SIGNIF_FIGS)));
			}
			let c: f64 = self[0].to_f64().unwrap();
			// if degree % 3 == 0 {
			// 	result_str_vec.push("\\\\".to_string());
			// }
			if c > TOL {
				result_str_vec.push(format!("+{}", pretty_float(c, Self::SIGNIF_FIGS)));
			} else if c < -TOL {
				result_str_vec.push(format!("-{}", pretty_float(-c, Self::SIGNIF_FIGS)));
			}
			result_str_vec.join("")
			              .chars()
			              .map(|c| {
				              if c == '*' {
					              "\\cdot".to_string()
				              } else {
					              c.to_string()
				              }
			              })
			              .collect()
		}
	}
}

impl<T> Polynomial<Complex<T>> where T: ToPrimitive + Num + Clone + Debug
{
	pub fn to_latex_string_complex(&self) -> String
	{
		if self.is_empty() {
			return "0".to_string();
		}
		let mut degree = self.degree();
		while degree > 0 && to_complexf64(self[degree].clone()).unwrap().norm() < TOL {
			degree -= 1;
		}
		if degree == 0 {
			let c = to_complexf64(self[0].clone()).unwrap();
			if c.norm() > TOL {
				format!("{}", pretty_float_complex(c, Self::SIGNIF_FIGS))
			} else {
				String::from("0")
			}
		} else if degree == 1 {
			let c0 = to_complexf64(self[0].clone()).unwrap();
			let c1 = to_complexf64(self[1].clone()).unwrap();
			if c1.norm() > TOL {
				if c0.norm() > TOL {
					format!(
					        "{}\\, X + {}",
					        pretty_float_complex(c1, Self::SIGNIF_FIGS),
					        pretty_float_complex(c0, Self::SIGNIF_FIGS)
					)
				} else {
					format!("{}\\, X", pretty_float_complex(c1, Self::SIGNIF_FIGS))
				}
			// No need to check when c1 < -TOL in this particular case
			} else {
				format!("{}", pretty_float_complex(c0, Self::SIGNIF_FIGS))
			}
		} else {
			let mut result_str_vec = Vec::with_capacity(degree + 2);
			let c = to_complexf64(self[degree].clone()).unwrap();
			result_str_vec.push(format!(
				"{}\\, X^{{{degree}}}",
				pretty_float_complex(c, Self::SIGNIF_FIGS)
			));

			for index in (2..degree).rev() {
				let c = to_complexf64(self[index].clone()).unwrap();
				// if index % 3 == 0 {
				// 	result_str_vec.push("\\\\".to_string());
				// }
				if c.norm() > TOL {
					result_str_vec.push(format!(
						"+{}\\, X^{{{index}}}",
						pretty_float_complex(c, Self::SIGNIF_FIGS)
					));
				}
			}
			let c = to_complexf64(self[1].clone()).unwrap();
			// if degree % 3 == 1 {
			// 	result_str_vec.push("\\\\".to_string());
			// }
			if c.norm() > TOL {
				result_str_vec.push(format!("+{}\\, X", pretty_float_complex(c, Self::SIGNIF_FIGS)));
			}
			let c = to_complexf64(self[0].clone()).unwrap();
			// if degree % 3 == 0 {
			// 	result_str_vec.push("\\\\".to_string());
			// }
			if c.norm() > TOL {
				result_str_vec.push(format!("+{}", pretty_float_complex(c, Self::SIGNIF_FIGS)));
			}
			result_str_vec.join("")
			              .chars()
			              // This map allows for better scientifc notation display
			              .map(|c| {
				              if c == '*' {
					              "\\cdot".to_string()
				              } else {
					              c.to_string()
				              }
			              })
			              .collect()
		}
	}
}

impl<T> fmt::Display for Polynomial<T> where T: ToPrimitive + Clone + Zero
{
	// Allows to display the polynomial in a fancy way
	/* Example:
	let p = polynomial![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
	println!("{}", p);
	*/
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		if self.is_empty() {
			return write!(f, "Polynomial(-Inf)");
		}
		let degree = self.degree();
		if degree == 0 {
			return write!(
			              f,
			              "{}",
			              format!("Polynomial(0)\n {:10.3e}", self[0].to_f64().unwrap())
			);
		}

		let mut result_str_vec = Vec::with_capacity(degree + 2);

		result_str_vec.push(format!("Polynomial({degree})"));

		let c: f64 = self[degree].to_f64().unwrap();
		if c > TOL {
			result_str_vec.push(format!(" {:10.3e} X^{degree}", c));
		} else if c < -TOL {
			result_str_vec.push(format!("-{:10.3e} X^{degree}", -c));
		}

		for index in (1..degree).rev() {
			let c: f64 = self[index].to_f64().unwrap();
			if c > TOL {
				result_str_vec.push(format!("+{:10.3e} X^{index}", c));
			} else if c < -TOL {
				result_str_vec.push(format!("-{:10.3e} X^{index}", -c));
			}
		}

		let c: f64 = self[0].to_f64().unwrap();
		if c > TOL {
			result_str_vec.push(format!("+{:10.3e}", c));
		} else if c < -TOL {
			result_str_vec.push(format!("-{:10.3e}", -c));
		}

		write!(f, "{}", result_str_vec.join("\n"))
	}
}
