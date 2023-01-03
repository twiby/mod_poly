#[cfg(test)]
mod test;

use std::ops::{Add, Mul};

use crate::complex::Number;

#[derive(Clone)]
struct Polynomial<T: Number> {
	coefs: Vec<T>	
}

// Efficient integer power, mercilessly stolen from num crate
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
	pub fn new(arr: &[T]) -> Self {
		assert!(arr.len() > 0);
		Self{coefs: arr.to_vec()}
	}

	pub fn new_monomial(coef: T, deg: usize) -> Self {
		let mut ret = Polynomial::new(&vec![T::from(0.0); deg]);
		ret.coefs.push(coef);
		ret
	}

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
