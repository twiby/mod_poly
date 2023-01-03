//! This module implements modular polynomial arithmetic
//! Its has basic setup to create and handle a polynomial, 
//! but operations are implemented for modular polynoms only

#[cfg(test)]
mod test;

use crate::complex::Number;

/// Efficient integer power function, mercilessly stolen from num crate (unused for now)
#[inline]
pub fn pow<T: Number>(mut base: T, mut exp: usize) -> T {
	if exp == 0 { return T::from(1.0); }

	while exp & 1 == 0 {
		base = base.clone() * base;
		exp >>= 1;
	}
	if exp == 1 { return base }

	let mut acc = base.clone();
	while exp > 1 {
		exp >>= 1;
		base = base.clone() * base;
		if exp & 1 == 1 {
			acc = acc * base.clone();
		}
	}
	acc
}

/// Type defining a general polynomial: 
/// We store all coefficients in a Vec, its index in the Vec reprenting its degree.
/// TODO: If the maximum degree can be known at compile time, this could be on the stack, 
/// and be made much more efficiently.
#[derive(Clone)]
pub struct Polynomial<T: Number> {
	coefs: Vec<T>	
}

impl<T: Number> Polynomial<T> {
	/// Creates a polynomial: This array must contains coefficients stored in the order of
	/// their degree.
	pub fn new(arr: &[T]) -> Self {
		Self{coefs: arr.to_vec()}
	}

	/// Create a new monomial from one coefficient and its degree
	pub fn new_monomial(coef: T, deg: usize) -> Self {
		let mut ret = Polynomial::new(&vec![T::from(0.0); deg]);
		ret.coefs.push(coef);
		ret
	}

	/// Applies the polynomial, as a function, on an input
	pub fn apply(&self, x: T) -> T {
		let mut ret = T::from(0.0);
		let mut x_powers = T::from(1.0);
		for deg in 0..self.coefs.len() {
			ret += self.coefs[deg] * x_powers;
			x_powers *= x;
		}
		ret
	}
}

/// Type representing a polynomial mod(x^modulus - 1)
#[derive(Clone)]
pub struct ModularArithmeticPolynomial<T: Number> {
	modulus: usize,
	polynomial: Polynomial<T>
}

impl<T: Number> ModularArithmeticPolynomial<T> {
	/// Polynomial doesn't need to already respect the modular arithmetic
	pub fn new(poly: &Polynomial<T>, modulus: usize) -> Self {
		Self{polynomial: Self::sanitize(poly, modulus), modulus: modulus}
	}

	/// Calls the underlying polynomial call function
	pub fn apply(&self, x: T) -> T {
		self.polynomial.apply(x)
	}

	/// Computes a lowest degree polynomial congruent to the input one in the modular arithmetic
	fn sanitize(poly: &Polynomial<T>, modulus: usize) -> Polynomial<T> {
		let size = poly.coefs.len();
		if size < modulus { return poly.clone(); }

		let mut ret = Polynomial::<T>::new(&poly.coefs[0..modulus]);

		let mut reduced_i = 0;
		for i in modulus..size {
			ret.coefs[reduced_i] += poly.coefs[i]; 
			reduced_i += 1;
			reduced_i -= ((reduced_i == modulus) as usize) * modulus;
		}

		ret
	}
}

