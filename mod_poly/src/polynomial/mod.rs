//! This module implements modular polynomial arithmetic
//! Its has basic setup to create and handle a polynomial, 
//! but operations are implemented for modular polynoms only

#[cfg(test)]
mod test;

use crate::complex::Number;

use std::ops::{Add, AddAssign, Mul};

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
/// We store all coefficients in a Vec, its index in the Vec representing its degree.
/// This sadly means it leaves on the heap, and cannot have the Copy trait. 
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

	/// Internal unsymetrical add operation: p1 has at least as many coefs as p2
	fn add_internal(p1: &Polynomial<T>, p2: &Polynomial<T>) -> Polynomial<T> {
		let mut ret = p1.clone();
		ret.add_to_self(&p2);
		ret
	}

	/// Internal unsymmetrical add_assign: we assume other.coefs has a smaller size
	fn add_to_self(&mut self,  other: &Polynomial<T>) {
		for i in 0..other.coefs.len() {
			self.coefs[i] += other.coefs[i];
		}
	}
}

/// The AddAssign operation for polynomials references
impl<'a, T: Number> AddAssign<&'a Polynomial<T>> for Polynomial<T> {
	fn add_assign(&mut self, other: &'a Polynomial<T>) {
		if other.coefs.len() > self.coefs.len() {
			self.coefs.resize(other.coefs.len(), T::from(0.0));
		}
		self.add_to_self(&other);
	}
}

/// The Add operation for polynomials references.
impl<'a, T: Number> Add for &'a Polynomial<T> {
	type Output = Polynomial<T>;

	fn add(self, other: &'a Polynomial<T>) -> Polynomial<T> {
		if self.coefs.len() >  other.coefs.len() {
			return Polynomial::<T>::add_internal(&self, &other);
		} else {
			return Polynomial::<T>::add_internal(&other, &self);
		}
	}
}

/// Modular arithmetic error types
#[derive(Debug)]
pub enum ModularArithmeticError {
	ModulusMismatched
}
type ModularArithmeticResult<T> = Result<ModularArithmeticPolynomial<T>, ModularArithmeticError>;

/// Type representing a polynomial mod(x^modulus - 1).
/// The coefs Vec inside polynomial must have length modulus.
#[derive(Clone)]
pub struct ModularArithmeticPolynomial<T: Number> {
	polynomial: Polynomial<T>
}

impl<T: Number> ModularArithmeticPolynomial<T> {
	/// Polynomial doesn't need to already respect the modular arithmetic
	pub fn new(poly: &Polynomial<T>, modulus: usize) -> Self {
		Self{polynomial: Self::sanitize(poly, modulus)}
	}

	/// Constructor for a zero polynomial
	pub fn new_zero(modulus: usize) -> Self {
		Self{polynomial: Self::sanitize(&Polynomial::<T>::new(&[]), modulus)}
	}

	/// Calls the underlying polynomial call function
	pub fn apply(&self, x: T) -> T {
		self.polynomial.apply(x)
	}

	fn modulus(&self) -> usize {
		self.polynomial.coefs.len()
	}

	/// Computes a lowest degree polynomial congruent to the input one in the modular arithmetic
	fn sanitize(poly: &Polynomial<T>, modulus: usize) -> Polynomial<T> {
		let mut ret = poly.clone();
		ret.coefs.resize(modulus, T::from(0.0));

		let size = poly.coefs.len();
		let mut reduced_i = 0;
		for i in modulus..size {
			ret.coefs[reduced_i] += poly.coefs[i]; 
			reduced_i += 1;
			reduced_i -= ((reduced_i == modulus) as usize) * modulus;
		}

		ret
	}

	/// Check the modulus of another polynomial against this one
	fn check_modulus(&self, other: &ModularArithmeticPolynomial<T>) -> Result<(), ModularArithmeticError> {
		if self.modulus() != other.modulus() {
			return Err(ModularArithmeticError::ModulusMismatched);
		}
		Ok(())
	}
}

/// The Add operation for polynomials references in a modular arithmetic.
impl<'a, T: Number> Add for &'a ModularArithmeticPolynomial<T> {
	type Output = ModularArithmeticResult<T>;

	fn add(self, other: &'a ModularArithmeticPolynomial<T>) -> ModularArithmeticResult<T> {
		self.check_modulus(&other)?;
		Ok(ModularArithmeticPolynomial::<T>{
			polynomial: Polynomial::<T>::add_internal(&self.polynomial, &other.polynomial)
		})
	}
}

/// The AddAssign operation for polynomials reference in a modular arithmetic.
impl<'a, T: Number> AddAssign<&'a ModularArithmeticPolynomial<T>> for ModularArithmeticPolynomial<T> {
	fn add_assign(&mut self, other: &'a ModularArithmeticPolynomial<T>) {
		self.check_modulus(&other).expect("AddAssign in modular arithmetic: modulus mismatched");
		self.polynomial.add_to_self(&other.polynomial);
	}
}

/// The Mul operation for polynomials references in a modular arithmetic.
impl<'a, T: Number> Mul for &'a ModularArithmeticPolynomial<T> {
	type Output = ModularArithmeticResult<T>;

	fn mul(self, other: &'a ModularArithmeticPolynomial<T>) -> ModularArithmeticResult<T> {
		self.check_modulus(&other)?;

		let modulus = self.modulus();
		let mut ret = ModularArithmeticPolynomial::<T>::new_zero(self.modulus());

		for deg in 0..modulus {
			let mut b_idx = deg;
			for a_idx in 0..modulus {
				ret.polynomial.coefs[deg] += self.polynomial.coefs[a_idx] * other.polynomial.coefs[b_idx];
				b_idx += ((b_idx == 0) as usize) * modulus;
				b_idx -= 1;
			}
		}

		Ok(ret)
	}
}
