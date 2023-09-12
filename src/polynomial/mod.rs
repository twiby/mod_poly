//! This module implements modular polynomial arithmetic
//! Its has basic setup to create and handle a polynomial, 
//! but operations are implemented for modular polynoms only

#[cfg(test)]
mod test;

pub mod convolution;
// use convolution::convolution_for_polynomial_mult_in_modular_arithmetic as convolution;
use convolution::convolution as convolution;

use crate::complex;
use crate::complex::Number;

use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, Neg};

/// Type defining a general polynomial: 
/// We store all coefficients in a Vec, its index in the Vec representing its degree.
/// This sadly means it leaves on the heap, and cannot have the Copy trait. 
/// TODO: If the maximum degree can be known at compile time, this could be on the stack, 
/// and be made much more efficiently.
#[derive(Clone, Default)]
pub struct Polynomial<T: Number> {
	coefs: Vec<T>	
}

/// Implement the Display trait
impl<T: Number> std::fmt::Display for Polynomial<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut ret = "[".to_string();
		for i in 0..self.coefs.len()-1 {
			ret.push_str(&self.coefs[i].to_string());
			ret.push_str(", ");
		}
		if let Some(c) = self.coefs.last() {
			ret.push_str(&c.to_string());
		}
		ret.push(']');
		f.write_str(&ret)
	}
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

	/// Public getter for a coef
	pub fn coef(&self, n: usize) -> T {
		self.coefs[n]
	}
	/// Public setter for a coef
	pub fn coef_mut(&mut self, n: usize) -> &mut T {
		&mut self.coefs[n]
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

	/// Internal unsymmetrical sub_assign: we assume other.coefs has a smaller size
	fn sub_to_self(&mut self,  other: &Polynomial<T>) {
		for i in 0..other.coefs.len() {
			self.coefs[i] -= other.coefs[i];
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

/// The SubAssign operation for polynomials references
impl<'a, T: Number> SubAssign<&'a Polynomial<T>> for Polynomial<T> {
	fn sub_assign(&mut self, other: &'a Polynomial<T>) {
		if other.coefs.len() > self.coefs.len() {
			self.coefs.resize(other.coefs.len(), T::from(0.0));
		}
		self.sub_to_self(&other);
	}
}

/// The Sub operation for polynomials references.
impl<'a, T: Number> Sub for &'a Polynomial<T> {
	type Output = Polynomial<T>;

	fn sub(self, other: &'a Polynomial<T>) -> Polynomial<T> {
		let mut ret = self.clone();
		ret.coefs.resize(other.coefs.len(), T::from(0.0));
		ret -= other;
		ret
	}
}

/// The Neg operattion for polynomials references
impl<'a, T: Number> Neg for &'a Polynomial<T> {
	type Output = Polynomial<T>;

	fn neg(self) -> Polynomial<T> {
		let mut coefs = Vec::<T>::with_capacity(self.coefs.len());
		for &val in self.coefs.iter() {
			coefs.push(-val);
		}
		Polynomial{coefs: coefs}
	}
}

/// Modular arithmetic error types
#[derive(Debug)]
pub enum ModularArithmeticError {
	ModulusMismatched(String),
	DegreeAboveModulus(String)
}
type ModularArithmeticResult<T> = Result<ModularArithmeticPolynomial<T>, ModularArithmeticError>;

/// Type representing a polynomial mod(x^modulus - 1).
/// The coefs Vec inside polynomial must have length modulus.
#[derive(Clone, Default)]
pub struct ModularArithmeticPolynomial<T: Number> {
	polynomial: Polynomial<T>
}

/// Implement the Display trait
impl<T: Number> std::fmt::Display for ModularArithmeticPolynomial<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&self.polynomial.to_string())
	}
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

	/// Check coefficient access
	fn check_coef(&self, n: usize) -> Result<(), ModularArithmeticError> {
		if n >= self.modulus() {
			return Err(ModularArithmeticError::DegreeAboveModulus(
				format!("Degree {} higher than or equal to modulus {}", n, self.modulus())
			));
		}
		Ok(())
	}
	/// Public getter for a coef
	pub fn coef(&self, n: usize) -> Result<T, ModularArithmeticError> {
		self.check_coef(n)?;
		Ok(self.polynomial.coef(n))
	}
	/// Public setter for a coef
	pub fn coef_mut(&mut self, n: usize) -> Result<&mut T, ModularArithmeticError> {
		self.check_coef(n)?;
		Ok(self.polynomial.coef_mut(n))
	}

	pub fn modulus(&self) -> usize {
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
			return Err(ModularArithmeticError::ModulusMismatched(format!("Modulus mismatch: {}, {}", self.modulus(), other.modulus())));
		}
		Ok(())
	}
}

/// The Add operation for polynomials references in a modular arithmetic.
///
/// This operation runs on references to avoid borrowing values (since Polynomial 
/// doesn't implement the Copy trait). This returns a Result because there potentially 
/// could be a mismatch of moduli between the two polynomials.
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
///
/// This operation can potentially panic, if the two polynomials don't have
/// the same modulus
impl<'a, T: Number> AddAssign<&'a ModularArithmeticPolynomial<T>> for ModularArithmeticPolynomial<T> {
	fn add_assign(&mut self, other: &'a ModularArithmeticPolynomial<T>) {
		self.check_modulus(&other).expect("AddAssign in modular arithmetic: modulus mismatched");
		self.polynomial.add_to_self(&other.polynomial);
	}
}

/// The Sub operation for polynomials references in a modular arithmetic.
///
/// This operation runs on references to avoid borrowing values (since Polynomial 
/// doesn't implement the Copy trait). This returns a Result because there potentially 
/// could be a mismatch of moduli between the two polynomials.
impl<'a, T: Number> Sub for &'a ModularArithmeticPolynomial<T> {
	type Output = ModularArithmeticResult<T>;

	fn sub(self, other: &'a ModularArithmeticPolynomial<T>) -> ModularArithmeticResult<T> {
		self.check_modulus(&other)?;
		let mut ret = self.polynomial.clone();
		ret -= &other.polynomial;
		Ok(ModularArithmeticPolynomial::<T>{
			polynomial: ret
		})
	}
}

/// The SubAssign operation for polynomials reference in a modular arithmetic.
///
/// This operation can potentially panic, if the two polynomials don't have
/// the same modulus
impl<'a, T: Number> SubAssign<&'a ModularArithmeticPolynomial<T>> for ModularArithmeticPolynomial<T> {
	fn sub_assign(&mut self, other: &'a ModularArithmeticPolynomial<T>) {
		self.check_modulus(&other).expect("SubAssign in modular arithmetic: modulus mismatched");
		self.polynomial.sub_to_self(&other.polynomial);
	}
}

/// The Neg operation for polynomials references in a modular arithmetic.
///
/// This operation runs on references to avoid borrowing values (since Polynomial 
/// doesn't implement the Copy trait).
impl<'a, T: Number> Neg for &'a ModularArithmeticPolynomial<T> {
	type Output = ModularArithmeticPolynomial<T>;

	fn neg(self) -> ModularArithmeticPolynomial<T> {
		ModularArithmeticPolynomial{
			polynomial: -&self.polynomial
		}
	}
}

/// The Mul operation for polynomials references in a modular arithmetic.
///
/// This is equivalent to a convolution of the coefficient reresentation of the polynoms. For low degrees, this
/// uses a naive convolution implementation, with complexity O(n²). For higher degrees it applies FFT to the 
/// coefficient representation of the polynoms to turn the convolution into a dot product, and the complexity
/// is then O(nlog(n))
///
/// This operation runs on references to avoid borrowing values (since Polynomial 
/// doesn't implement the Copy trait). This returns a Result because there potentially 
/// could be a mismatch of moduli between the two polynomials.
impl<'a, T> Mul for &'a ModularArithmeticPolynomial<T> 
where T: Number + From<complex::Complex<f64>>, complex::Complex<f64>: From<T> {
	type Output = ModularArithmeticResult<T>;

	fn mul(self, other: &'a ModularArithmeticPolynomial<T>) -> ModularArithmeticResult<T> {
		self.check_modulus(&other)?;

		let convolution = convolution(&self.polynomial.coefs, &other.polynomial.coefs);

		Ok(ModularArithmeticPolynomial::<T>::new(
			&Polynomial::<T>::new(&convolution),
			self.modulus()
		))
	}
}

impl<T: Number> crate::matrix::MatrixInput for ModularArithmeticPolynomial<T> {}
