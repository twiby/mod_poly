use crate::matrix;
use crate::complex;

#[test]
fn new_empty() {
	let _ = matrix::Matrix::<f32>::new_empty(5, 10, 0.0);
}

#[test]
fn new() {
	let m = matrix::Matrix::<f32>::new(&vec![0.0,1.0,2.0,3.0,4.0,5.0], 2,3).unwrap();
	assert_eq!(m.len(), 6);

	assert_eq!(m[(0,0)], 0.0);
	assert_eq!(m[(0,1)], 1.0);
	assert_eq!(m[(0,2)], 2.0);
	assert_eq!(m[(1,0)], 3.0);
	assert_eq!(m[(1,1)], 4.0);
	assert_eq!(m[(1,2)], 5.0);
}

#[test]
fn new_error() {
	let e = matrix::Matrix::<f32>::new(&vec![0.0,1.0,2.0,3.0,4.0], 2,3);
	match e {
		Err(matrix::MatrixError::WrongInputArraySize(_)) => (),
		_ => panic!("Wrong error type")
	}
}

#[test]
fn new_error_empty() {
	let e = matrix::Matrix::<f32>::new(&vec![], 2,0);
	match e {
		Err(matrix::MatrixError::ZeroDimension(_)) => (),
		_ => panic!("Wrong error type")
	}
}

#[test]
fn index_mut() {
	let mut m = matrix::Matrix::<f32>::new(&vec![0.0,1.0,2.0,3.0,4.0,5.0], 2,3).unwrap();
	assert_eq!(m.len(), 6);
	m[(1,2)] = 10.0;
	assert_eq!(m[(1,2)], 10.0);
}

#[test]
fn sum() {
	let a = complex::Complex::<f32>::new(1.0, 1.0);
	let b = complex::Complex::<f32>::new(0.0, 1.0);
	let c = complex::Complex::<f32>::new(2.0, 0.0);
	let d = complex::Complex::<f32>::new(2.0, -2.0);

	let m1 = matrix::Matrix::<complex::Complex<f32>>::new(&vec![a,b,c,d], 2, 2).unwrap();
	let m2 = matrix::Matrix::<complex::Complex<f32>>::new(&vec![d,d,d,d], 2, 2).unwrap();
	let m3 = (&m1 + &m2).unwrap();

	assert_eq!(m3[(0,0)], complex::Complex::<f32>::new(3.0, -1.0));
	assert_eq!(m3[(0,1)], complex::Complex::<f32>::new(2.0, -1.0));
	assert_eq!(m3[(1,0)], complex::Complex::<f32>::new(4.0, -2.0));
	assert_eq!(m3[(1,1)], complex::Complex::<f32>::new(4.0, -4.0));
}

use crate::polynomial::Polynomial;
use crate::polynomial::ModularArithmeticPolynomial;

#[test]
fn sum_matrix_of_polynomial() {
	// P(x) = 1 + 2i*x + (1 + i)*x²
	let a = complex::Complex::<f32>::from(1.0);
	let a_p_a = complex::Complex::<f32>::from(2.0);

	let b = complex::I_F32 * complex::Complex::<f32>::from(2.0);
	let b_p_b = complex::I_F32 * complex::Complex::<f32>::from(4.0);

	let c = complex::Complex::new(1.0, 1.0);

	let zero = complex::Complex::<f32>::from(0.0);

	let poly_1 = ModularArithmeticPolynomial::new(&Polynomial::new(&[a,b,zero]), 3);
	let poly_2 = ModularArithmeticPolynomial::new(&Polynomial::new(&[zero,zero,c]), 3);

	let m1 = matrix::Matrix::new(&vec![poly_1.clone(), poly_2], 2, 1).unwrap();
	let m2 = matrix::Matrix::new(&vec![poly_1.clone(), poly_1], 2, 1).unwrap();
	let m3 = (&m1 + &m2).unwrap();

	assert_eq!(m3[(0,0)].coef(0), a_p_a);
	assert_eq!(m3[(0,0)].coef(1), b_p_b);
	assert_eq!(m3[(0,0)].coef(2), zero);
	assert_eq!(m3[(0,1)].coef(0), a);
	assert_eq!(m3[(0,1)].coef(1), b);
	assert_eq!(m3[(0,1)].coef(2), c);
}

#[test]
fn matrix_product() {
	let a = complex::Complex::<f32>::new(1.0, 1.0);
	let b = complex::Complex::<f32>::new(0.0, 1.0);
	let c = complex::Complex::<f32>::new(2.0, 0.0);
	let d = complex::Complex::<f32>::new(2.0, -2.0);

	let m1 = matrix::Matrix::<complex::Complex<f32>>::new(&vec![a,b,c,d], 2, 2).unwrap();
	let m2 = matrix::Matrix::<complex::Complex<f32>>::new(&vec![a,d], 2, 1).unwrap();
	let m3 = (&m1 * &m2).unwrap();

	assert_eq!(m3.shape(), (2, 1));
	assert_eq!(m3[(0,0)], complex::Complex::<f32>::new(2.0, 4.0));
	assert_eq!(m3[(1,0)], complex::Complex::<f32>::new(2.0, -6.0));
}

#[test]
fn matrix_product_polynomials() {
	// P1(x) = 1 + 2i*x + (1 + i)*x²
	// P2(x) = 1 + i + x + 2i*x²
	let a = complex::Complex::<f32>::from(1.0);
	let b = complex::I_F32 * complex::Complex::<f32>::from(2.0);
	let c = complex::Complex::new(1.0, 1.0);

	let mod_poly_1 = ModularArithmeticPolynomial::new(&Polynomial::new(&[a,b,c]), 3);
	let mod_poly_2 = ModularArithmeticPolynomial::new(&Polynomial::new(&[c,a,b]), 3);

	let m1 = matrix::Matrix::new(&vec![mod_poly_1.clone(), mod_poly_2.clone()], 1, 2).unwrap();
	let m2 = matrix::Matrix::new(&vec![mod_poly_2, mod_poly_1], 2, 1).unwrap();

	assert_eq!((&m2 * &m1).unwrap().shape(), (2, 2));

	let m3 = (&m1 * &m2).unwrap();

	assert_eq!(m3.shape(), (1, 1));
	assert_eq!(m3[(0,0)].modulus(), 3);
	assert_eq!(m3[(0,0)].coef(0), complex::Complex::<f32>::new(-4.0, 4.0));
	assert_eq!(m3[(0,0)].coef(1), complex::Complex::<f32>::new(-6.0, 8.0));
	assert_eq!(m3[(0,0)].coef(2), complex::Complex::<f32>::new(0.0, 12.0));
}

#[test]
fn matrix_product_polynomials_error() {
	// P1(x) = 1 + 2i*x + (1 + i)*x²
	// P2(x) = 1 + i + x + 2i*x²
	let a = complex::Complex::<f32>::from(1.0);
	let b = complex::I_F32 * complex::Complex::<f32>::from(2.0);
	let c = complex::Complex::new(1.0, 1.0);

	let mod_poly_1 = ModularArithmeticPolynomial::new(&Polynomial::new(&[a,b,c]), 3);
	let mod_poly_2 = ModularArithmeticPolynomial::new(&Polynomial::new(&[c,a,b]), 3);

	let m1 = matrix::Matrix::new(&vec![mod_poly_1.clone(), mod_poly_2.clone()], 2, 1).unwrap();
	let m2 = matrix::Matrix::new(&vec![mod_poly_2, mod_poly_1], 2, 1).unwrap();

	match &m1 * &m2 {
		Err(matrix::MatrixError::UncompatibleMatrixShapes(_)) => (),
		_ => panic!("Wrong error type")
	}
}