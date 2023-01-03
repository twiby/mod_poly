//! This module implements modular polynomial arithmetic
//! Its has basic setup to create and handle a polynomial, 
//! but operations are implemented for modular polynoms only

#[cfg(test)]
mod test;

use crate::complex::Number;

/// Type defining a general polynomial: 
/// We store all coefficients in a Vec, its index in the Vec reprenting its degree.
///  If the maximum degree can be known at compile time, this could be on the stack, 
/// and be made much more efficiently.
#[derive(Clone)]
pub struct Polynomial<T: Number> {
	coefs: Vec<T>	
}

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
