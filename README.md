## The `polyx` crate

#### *A Rust crate to handle polynomials*

### Why another crate to handle polynomials ?

Many other superb crates already exist: [`polynomial`](https://docs.rs/polynomial/latest/polynomial/), [`polynomen`](https://docs.rs/polynomen/latest/polynomen/index.html#), [`rustnomial`](https://github.com/philippeitis/rustnomial) just to name a few. Why create another one ? Because I felt like it 😛.

### Features provided

* A vector based `Polynomial` struct and a `polynomial` macro to instantiate one.
* Multiplication with a memory efficient Karatsuba algorithm, addition, subtraction. These operations can be used both on a bare `Polynomial` and on a reference `&Polynomial` to preserve its ownership.
* Euclidean division and modulo, but these are not implemented through the `/` and `%` operators as it requires different functions if the polynomial contains integers or floats as coefficients. The crate rather provides `euclidean_division` and `euclidean_division_float` functions. These take mutable inputs for better performance, but these functions `euclidean_division_immutable(_float)` are also provided.
* Parsing polynomials from strings with the `polynomial_expr` macro.
* Inverting a polynomial modulo $X^n$ with the `inverse(_float)` functions.
* Polynomial short product with the `short_product` function.
* Degree left and right shift with `<<` and `>>` operators. `p >> n` returns the quotient of $p\div X^n$ and `p << n` returns $X^n p$.
* Parsing into $\LaTeX$ strings with `to_latex_string` and `to_latex_string_complex` functions, to be used accordingly to the coefficients being complex or not.

The crate considers flaoting point coefficients as zero if their value goes below a `TOL` constant, which is fixed at $2^{-31}$. There also is a `gcd` function, but it uses the basic Euclid algorithm rather than the much faster half-gcd algorithm.

### Examples

```rust
use polyx::{polynomial, polynomial_expr, Polynomial};
```

#### Creating a polynomial

```rust
let p1 = polynomial![0, 2, 1]; // X^2 + 2X
let p2 = Polynomial::<i32>::parse_string("X(X + 2)".to_string()).unwrap();
let p3 = Polynomial::<i32>::parse_string("X ^ 2 + 2X".to_string()).unwrap();
assert_eq!(p1, p2);
assert_eq!(p1, p3);

let p4 = Polynomial::<Complex<f64>>::parse_string("i (X + i)^2".to_string()).unwrap();
assert_eq!(p4.eval(Complex::new(0.0, -1.0)), Complex::from(0.));
```

#### Using some operations on polynomials

```rust
let p1 = Polynomial::<i32>::parse_string("X ^ 3".to_string()).unwrap();
let p2 = Polynomial::<i32>::parse_string("X ^ 2(1 + X)".to_string()).unwrap();
assert_eq!(&p1 - &p2, polynomial![0, 0, -1]);
assert_eq!(&p1 + &p2, polynomial![0, 0, 1, 2]);
assert_eq!(&p1 * &p2, polynomial![0, 0, 0, 0, 0, 1, 1]);
```

#### Euclidean division

```rust
let a = polynomial![1, 0, 2];
let b = polynomial![1, 1];
let (q, r) = Polynomial::euclidean_division_immutable(&a, &b);
assert_eq!(b * q + r, a);

let mut a = polynomial![1, 0, 2];
let mut b = polynomial![1, 1];
let (q, r) = Polynomial::euclidean_division(&mut a, &mut b);
assert_eq!(b * q + r, a);
```

#### Inversion and short-product

```rust
let p = polynomial![1, -4, 0, -2, 5, 1, 1, 1];
let inv10 = Polynomial::inverse(&p, 10);
assert_eq!(Polynomial::short_product(&p, &inv10, 10), polynomial![1]);
```

#### Degree shifting

```rust
let p = polynomial![1, 0, 2];
assert_eq!(&p << 2, polynomial![0, 0, 1, 0, 2]);
assert_eq!(&p >> 2, polynomial![2]);
```

#### $\LaTeX$ parsing

```rust
let p = polynomial![1, 2, -5, -3];
println!("{}", p.to_latex());

let p = Polynomial::<Complex<f64>>::parse_string("(1 + i)X² +(-7i -1)X + 4i - 2X".to_string()).unwrap();
println!("{}", p.to_latex());
```

Outputs: 
$$-3.00\, X^{3}-5.00\, X^{2}+2.00\, X+1.00$$
$$(1.00+1.00i)\, X^{2}-(3.00+7.00i)\, X+4.00i$$