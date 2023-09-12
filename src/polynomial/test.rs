use crate::complex::{Complex, I_F32};
use crate::polynomial::{Polynomial, ModularArithmeticPolynomial, ModularArithmeticError};

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
fn polynomial_neg() {
	// P(x) = 1 + 2*x + x²
	let a = 1.0;
	let b = 2.0;
	let c = 1.0;

	let poly_1 = Polynomial::new(&[a,b,c]);
	let poly_2 = -&poly_1;

	assert_eq!(poly_2.apply(2.0), -poly_1.apply(2.0));
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
fn sub_polynomial() {
	// P(x) = 1 + 2i*x + (1 + i)*x²
	let a = Complex::<f32>::from(1.0);
	let b = I_F32 * Complex::<f32>::from(2.0);
	let c = Complex::new(1.0, 1.0);

	let zero = Complex::<f32>::from(0.0);

	let poly_1 = Polynomial::new(&[a,b,zero]);
	let poly_2 = Polynomial::new(&[zero,zero,c]);

	let sum_1 = &poly_1 - &poly_2;
	assert_eq!(sum_1.apply(Complex::<f32>::from(1.0)), Complex::new(0.0, 1.0));
}

#[test]
fn subassign_polynomial() {
	// P(x) = 1 + 2i*x + (1 + i)*x²
	let a = Complex::<f32>::from(1.0);
	let b = I_F32 * Complex::<f32>::from(2.0);
	let c = Complex::new(1.0, 1.0);

	let zero = Complex::<f32>::from(0.0);

	let mut poly_1 = Polynomial::new(&[a,b,zero]);
	let poly_2 = Polynomial::new(&[zero,zero,c]);

	poly_1 -= &poly_2;
	assert_eq!(poly_1.apply(Complex::<f32>::from(1.0)), Complex::new(0.0, 1.0));
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
fn sub_mod_polynomial() {
	let mono_5 = Polynomial::new_monomial(1.0, 5);
	let mono_6 = Polynomial::new_monomial(1.0, 6);

	let mod_poly_5 = ModularArithmeticPolynomial::new(&mono_5, 3);
	let mod_poly_6 = ModularArithmeticPolynomial::new(&mono_6, 3);

	let sum_mod_poly = (&mod_poly_5 - &mod_poly_6).expect("");
	assert_eq!(sum_mod_poly.apply(2.0), 3.0);
}

#[test]
fn neg_mod_polynomial() {
	let mono_5 = Polynomial::new_monomial(1.0, 5);

	let mod_poly_5 = ModularArithmeticPolynomial::new(&mono_5, 3);
	let mod_poly_6 = -&mod_poly_5;

	assert_eq!(mod_poly_6.apply(2.0), -mod_poly_5.apply(2.0));
}

#[test]
fn subassign_mod_polynomial() {
	let mono_5 = Polynomial::new_monomial(1.0, 5);
	let mono_6 = Polynomial::new_monomial(1.0, 6);

	let mut mod_poly_5 = ModularArithmeticPolynomial::new(&mono_5, 3);
	let mod_poly_6 = ModularArithmeticPolynomial::new(&mono_6, 3);

	mod_poly_5 -= &mod_poly_6;
	assert_eq!(mod_poly_5.apply(2.0), 3.0);
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

fn nearly_equal_f32(a: f32, b: f32) -> bool {
	let abs_a = a.abs();
	let abs_b = b.abs();
	let diff = (a - b).abs();

	if a == b { // Handle infinities.
		true
	} else if a == 0.0 || b == 0.0 || diff < f32::MIN_POSITIVE {
		// One of a or b is zero (or both are extremely close to it,) use absolute error.
		diff < f32::EPSILON
	} else { // Use relative error.
		(diff / f32::min(abs_a + abs_b, f32::MAX)) < f32::EPSILON
	}
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
	assert!(nearly_equal_f32(prod.polynomial.coefs[0].real(), -2.0));
	assert!(nearly_equal_f32(prod.polynomial.coefs[1].real(), -3.0));
	assert!(nearly_equal_f32(prod.polynomial.coefs[2].real(), 0.0));
	assert!(nearly_equal_f32(prod.polynomial.coefs[0].imag(), 2.0));
	assert!(nearly_equal_f32(prod.polynomial.coefs[1].imag(), 4.0));
	assert!(nearly_equal_f32(prod.polynomial.coefs[2].imag(), 6.0));
}

#[test]
fn mult_mod_polynomial_f64() {
	// P1(x) = 1 + 2x + x²
	// P2(x) = 1 + x + 2x²

	let mod_poly_1 = ModularArithmeticPolynomial::<f64>::new(&Polynomial::new(&[1.0,2.0,1.0]), 3);
	let mod_poly_2 = ModularArithmeticPolynomial::<f64>::new(&Polynomial::new(&[1.0,1.0,2.0]), 3);

	assert_eq!(mod_poly_1.apply(1.0), 4.0);
	assert_eq!(mod_poly_2.apply(1.0), 4.0);

	let prod = (&mod_poly_1 * &mod_poly_2).expect("");
	assert_eq!(prod.polynomial.coefs.len(), 3);
	assert_eq!(prod.polynomial.coefs[0], 6.0);
	assert_eq!(prod.polynomial.coefs[1], 5.0);
	assert_eq!(prod.polynomial.coefs[2], 5.0);
}

#[test]
fn mult_mod_polynomial_f32() {
	// P1(x) = 1 + 2x + x²
	// P2(x) = 1 + x + 2x²

	let mod_poly_1 = ModularArithmeticPolynomial::<f32>::new(&Polynomial::new(&[1.0,2.0,1.0]), 3);
	let mod_poly_2 = ModularArithmeticPolynomial::<f32>::new(&Polynomial::new(&[1.0,1.0,2.0]), 3);

	assert_eq!(mod_poly_1.apply(1.0), 4.0);
	assert_eq!(mod_poly_2.apply(1.0), 4.0);

	let prod = (&mod_poly_1 * &mod_poly_2).expect("");
	assert_eq!(prod.polynomial.coefs.len(), 3);
	assert_eq!(prod.polynomial.coefs[0], 6.0);
	assert_eq!(prod.polynomial.coefs[1], 5.0);
	assert_eq!(prod.polynomial.coefs[2], 5.0);
}

use crate::polynomial::convolution::{convolution_via_fft, naive_convolution};

#[test]
fn convolution_via_fft_test() {
	const N: usize = 100;

	let mut a = Vec::<f32>::with_capacity(N);
	let mut b = Vec::<f32>::with_capacity(N);

	for i in 0..N {
		a.push(i as f32);
		b.push((N-i) as f32);
	}

	let conv_classic = naive_convolution(&a, &b);
	let conv_fft = convolution_via_fft(&a, &b);

	assert_eq!(conv_classic.len(), conv_fft.len());

	for i in 0..conv_classic.len() {
		assert!(nearly_equal_f32(conv_fft[i], conv_classic[i]));
	}
}
