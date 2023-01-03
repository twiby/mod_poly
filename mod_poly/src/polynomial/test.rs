use crate::complex::{Complex, I_32};

use crate::polynomial;
use crate::polynomial::Polynomial;

#[test]
fn pow_basic() {
	let a = 2.0;
	let b = 10;
	assert_eq!(polynomial::pow(a, b), 1024.0);
}

#[test]
fn pow_real() {
	let a = Complex::<f32>::from(2.0);
	let exp = 10;
	assert_eq!(polynomial::pow(a, exp), Complex::from(1024.0));
}

#[test]
fn pow_imag() {
	let a = Complex::new(0.0, 2.0);
	let exp = 2;
	assert_eq!(polynomial::pow(a, exp), Complex::from(-4.0));
}

#[test]
fn pow_complex() {
	let a = Complex::new(1.0, 2.0);
	let exp = 3;
	assert_eq!(polynomial::pow(a, exp), Complex::new(-11.0, -2.0));
}

#[test]
#[should_panic]
fn polynomial_empty() {
	let _ = Polynomial::<f32>::new(&[]);
}

#[test]
fn polynomial_real() {
	// P(x) = 1 + 2*x + x²
	let a = 1.0;
	let b = 2.0;
	let c = 1.0;

	let poly_1 = Polynomial::new(&[a,b,c]);

	assert_eq!(poly_1.apply(2.0), 9.0);
}

#[test]
fn polynomial_complex() {
	// P(x) = 1 + 2i*x + (1 + i)*x²
	let a = Complex::<f32>::from(1.0);
	let b = I_32 * Complex::<f32>::from(2.0);
	let c = Complex::new(1.0, 1.0);

	let poly_1 = Polynomial::new(&[a,b,c]);

	assert_eq!(poly_1.apply(Complex::<f32>::from(1.0)), Complex::new(2.0, 3.0));
}

#[test]
fn monomial() {
	// P(x) = i*x²
	let poly_1 = Polynomial::new_monomial(I_32, 2);

	assert_eq!(poly_1.apply(Complex::<f32>::from(2.0)), I_32 * Complex::<f32>::from(4.0));
}
