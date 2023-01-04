use crate::complex::{Complex, I_F32};

use crate::polynomial;
use crate::polynomial::{Polynomial, ModularArithmeticPolynomial, ModularArithmeticError};

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
fn polynomial_empty() {
	let p = Polynomial::<f32>::new(&[]);

	assert_eq!(p.apply(1000.0), 0.0);
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
	let b = I_F32 * Complex::<f32>::from(2.0);
	let c = Complex::new(1.0, 1.0);

	let poly_1 = Polynomial::new(&[a,b,c]);

	assert_eq!(poly_1.apply(Complex::<f32>::from(1.0)), Complex::new(2.0, 3.0));
}

#[test]
fn monomial() {
	// P(x) = i*x²
	let poly_1 = Polynomial::new_monomial(I_F32, 2);

	assert_eq!(poly_1.apply(Complex::<f32>::from(2.0)), I_F32 * Complex::<f32>::from(4.0));
}

#[test]
fn mod_polynomial() {
	let mono_1 = Polynomial::new_monomial(1.0, 1);
	let mono_2 = Polynomial::new_monomial(1.0, 2);
	let mono_3 = Polynomial::new_monomial(1.0, 3);
	let mono_4 = Polynomial::new_monomial(1.0, 4);
	let mono_5 = Polynomial::new_monomial(1.0, 5);
	let mono_6 = Polynomial::new_monomial(1.0, 6);

	let mut mod_poly = ModularArithmeticPolynomial::new(&mono_1, 3);
	assert_eq!(mod_poly.polynomial.coefs.len(), mod_poly.modulus());
	assert_eq!(mod_poly.apply(2.0), 2.0);

	mod_poly = ModularArithmeticPolynomial::new(&mono_2, 3);
	assert_eq!(mod_poly.polynomial.coefs.len(), mod_poly.modulus());
	assert_eq!(mod_poly.apply(2.0), 4.0);

	mod_poly = ModularArithmeticPolynomial::new(&mono_3, 3);
	assert_eq!(mod_poly.polynomial.coefs.len(), mod_poly.modulus());
	assert_eq!(mod_poly.apply(2.0), 1.0);

	mod_poly = ModularArithmeticPolynomial::new(&mono_4, 3);
	assert_eq!(mod_poly.polynomial.coefs.len(), mod_poly.modulus());
	assert_eq!(mod_poly.apply(2.0), 2.0);

	mod_poly = ModularArithmeticPolynomial::new(&mono_5, 3);
	assert_eq!(mod_poly.polynomial.coefs.len(), mod_poly.modulus());
	assert_eq!(mod_poly.apply(2.0), 4.0);

	mod_poly = ModularArithmeticPolynomial::new(&mono_6, 3);
	assert_eq!(mod_poly.polynomial.coefs.len(), mod_poly.modulus());
	assert_eq!(mod_poly.apply(2.0), 1.0);
}

#[test]
fn add_polynomial() {
	// P(x) = 1 + 2i*x + (1 + i)*x²
	let a = Complex::<f32>::from(1.0);
	let b = I_F32 * Complex::<f32>::from(2.0);
	let c = Complex::new(1.0, 1.0);

	let zero = Complex::<f32>::from(0.0);

	let mut poly_1 = Polynomial::new(&[a,b,zero]);
	let mut poly_2 = Polynomial::new(&[zero,zero,c]);

	let sum_1 = &poly_1 + &poly_2;
	assert_eq!(sum_1.apply(Complex::<f32>::from(1.0)), Complex::new(2.0, 3.0));

	let sum_2 = &poly_2 + &poly_1;
	assert_eq!(sum_2.apply(Complex::<f32>::from(1.0)), Complex::new(2.0, 3.0));

	poly_1 = Polynomial::new(&[a,b,Complex::<f32>::from(1.0)]);
	poly_2 = Polynomial::new(&[zero,zero,I_F32]);
	let sum = &poly_1 + &poly_2;
	assert_eq!(sum.apply(Complex::<f32>::from(2.0)), Complex::new(5.0, 8.0));
}

#[test]
fn add_assign_polynomial() {
	// P(x) = 1 + 2i*x + (1 + i)*x²
	let a = Complex::<f32>::from(1.0);
	let b = I_F32 * Complex::<f32>::from(2.0);
	let c = Complex::new(1.0, 1.0);

	let zero = Complex::<f32>::from(0.0);

	let mut poly_1 = Polynomial::new(&[a,b,zero]);
	let mut poly_2 = Polynomial::new(&[zero,zero,c]);

	let mut sum_1 = poly_1.clone();
	sum_1 += &poly_2;
	assert_eq!(sum_1.apply(Complex::<f32>::from(1.0)), Complex::new(2.0, 3.0));

	let mut sum_2 = poly_2.clone(); 
	sum_2 += &poly_1;
	assert_eq!(sum_2.apply(Complex::<f32>::from(1.0)), Complex::new(2.0, 3.0));

	poly_1 = Polynomial::new(&[a,b,Complex::<f32>::from(1.0)]);
	poly_2 = Polynomial::new(&[zero,zero,I_F32]);
	let mut sum = poly_1.clone();
	sum += &poly_2;
	assert_eq!(sum.apply(Complex::<f32>::from(2.0)), Complex::new(5.0, 8.0));
}

#[test]
fn add_mod_polynomial() {
	let mono_5 = Polynomial::new_monomial(1.0, 5);
	let mono_6 = Polynomial::new_monomial(1.0, 6);

	let mod_poly_5 = ModularArithmeticPolynomial::new(&mono_5, 3);
	let mod_poly_6 = ModularArithmeticPolynomial::new(&mono_6, 3);

	let sum_mod_poly = (&mod_poly_5 + &mod_poly_6).expect("");
	assert_eq!(sum_mod_poly.apply(2.0), 5.0);
}

#[test]
fn add_mod_polynomial_error() {
	let mono_5 = Polynomial::new_monomial(1.0, 5);
	let mono_6 = Polynomial::new_monomial(1.0, 6);

	let mod_poly_5 = ModularArithmeticPolynomial::new(&mono_5, 3);
	let mod_poly_6 = ModularArithmeticPolynomial::new(&mono_6, 2);

	match &mod_poly_5 + &mod_poly_6 {
		Err(ModularArithmeticError::ModulusMismatched(_)) => (),
		_ => panic!("Wrong error type")
	};
}

#[test]
fn add_assign_mod_polynomial() {
	let mono_5 = Polynomial::new_monomial(1.0, 5);
	let mono_6 = Polynomial::new_monomial(1.0, 6);

	let mod_poly_5 = ModularArithmeticPolynomial::new(&mono_5, 3);
	let mod_poly_6 = ModularArithmeticPolynomial::new(&mono_6, 3);

	let mut sum_mod_poly = mod_poly_5.clone();
	sum_mod_poly += &mod_poly_6;
	assert_eq!(sum_mod_poly.apply(2.0), 5.0);
}

#[test]
#[should_panic]
fn add_assign_mod_polynomial_error() {
	let mono_5 = Polynomial::new_monomial(1.0, 5);
	let mono_6 = Polynomial::new_monomial(1.0, 6);

	let mod_poly_5 = ModularArithmeticPolynomial::new(&mono_5, 3);
	let mod_poly_6 = ModularArithmeticPolynomial::new(&mono_6, 2);

	
	let mut sum = mod_poly_5.clone();
	sum += &mod_poly_6;
}

#[test]
fn mult_mod_polynomial() {
	// P1(x) = 1 + 2i*x + (1 + i)*x²
	// P2(x) = 1 + i + x + 2i*x²
	let a = Complex::<f32>::from(1.0);
	let b = I_F32 * Complex::<f32>::from(2.0);
	let c = Complex::new(1.0, 1.0);

	let mod_poly_1 = ModularArithmeticPolynomial::new(&Polynomial::new(&[a,b,c]), 3);
	let mod_poly_2 = ModularArithmeticPolynomial::new(&Polynomial::new(&[c,a,b]), 3);

	assert_eq!(mod_poly_1.apply(Complex::<f32>::from(1.0)), Complex::new(2.0, 3.0));
	assert_eq!(mod_poly_2.apply(Complex::<f32>::from(1.0)), Complex::new(2.0, 3.0));

	let prod = (&mod_poly_1 * &mod_poly_2).expect("");
	assert_eq!(prod.polynomial.coefs.len(), 3);
	assert_eq!(prod.polynomial.coefs[0], Complex::<f32>::new(-2.0, 2.0));
	assert_eq!(prod.polynomial.coefs[1], Complex::<f32>::new(-3.0, 4.0));
	assert_eq!(prod.polynomial.coefs[2], Complex::<f32>::new(0.0, 6.0));
}
