use crate::complex;
use crate::matrix;

#[test]
fn new_empty() {
    let _ = matrix::Matrix::<f32>::new_empty(5, 10, 0.0);
}

#[test]
fn new() {
    let m = matrix::Matrix::<f32>::new(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0], 2, 3).unwrap();
    assert_eq!(m.len(), 6);

    assert_eq!(m[(0, 0)], 0.0);
    assert_eq!(m[(0, 1)], 1.0);
    assert_eq!(m[(0, 2)], 2.0);
    assert_eq!(m[(1, 0)], 3.0);
    assert_eq!(m[(1, 1)], 4.0);
    assert_eq!(m[(1, 2)], 5.0);
}

#[test]
fn new_error() {
    let e = matrix::Matrix::<f32>::new(vec![0.0, 1.0, 2.0, 3.0, 4.0], 2, 3);
    match e {
        Err(matrix::MatrixError::WrongInputArraySize(_)) => (),
        _ => panic!("Wrong error type"),
    }
}

#[test]
fn new_error_empty() {
    let e = matrix::Matrix::<f32>::new(vec![], 2, 0);
    match e {
        Err(matrix::MatrixError::ZeroDimension(_)) => (),
        _ => panic!("Wrong error type"),
    }
}

#[test]
fn index_mut() {
    let mut m = matrix::Matrix::<f32>::new(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0], 2, 3).unwrap();
    assert_eq!(m.len(), 6);
    m[(1, 2)] = 10.0;
    assert_eq!(m[(1, 2)], 10.0);
}

#[test]
fn rows() {
    let m = matrix::Matrix::<f32>::new(
        vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0,
        ],
        4,
        3,
    )
    .unwrap();
    assert_eq!(
        m.row(0).unwrap().copied().collect::<Vec<f32>>(),
        vec![1.0, 2.0, 3.0]
    );
    assert_eq!(
        m.row(1).unwrap().copied().collect::<Vec<f32>>(),
        vec![4.0, 5.0, 6.0]
    );
    assert_eq!(
        m.row(2).unwrap().copied().collect::<Vec<f32>>(),
        vec![7.0, 8.0, 9.0]
    );
    assert_eq!(
        m.row(3).unwrap().copied().collect::<Vec<f32>>(),
        vec![10.0, 11.0, 12.0]
    );
    match m.row(4) {
        Err(matrix::MatrixError::OutOfBoundsIndex(_)) => (),
        _ => panic!("Wrong error type"),
    }
}

#[test]
fn cols() {
    let m = matrix::Matrix::<f32>::new(
        vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0,
        ],
        4,
        3,
    )
    .unwrap();
    assert_eq!(
        m.col(0).unwrap().copied().collect::<Vec<f32>>(),
        vec![1.0, 4.0, 7.0, 10.0]
    );
    assert_eq!(
        m.col(1).unwrap().copied().collect::<Vec<f32>>(),
        vec![2.0, 5.0, 8.0, 11.0]
    );
    assert_eq!(
        m.col(2).unwrap().copied().collect::<Vec<f32>>(),
        vec![3.0, 6.0, 9.0, 12.0]
    );
    match m.col(3) {
        Err(matrix::MatrixError::OutOfBoundsIndex(_)) => (),
        _ => panic!("Wrong error type"),
    }
}

#[test]
fn sum() {
    let a = complex::Complex::<f32>::new(1.0, 1.0);
    let b = complex::Complex::<f32>::new(0.0, 1.0);
    let c = complex::Complex::<f32>::new(2.0, 0.0);
    let d = complex::Complex::<f32>::new(2.0, -2.0);

    let m1 = matrix::Matrix::<complex::Complex<f32>>::new(vec![a, b, c, d], 2, 2).unwrap();
    let m2 = matrix::Matrix::<complex::Complex<f32>>::new(vec![d, d, d, d], 2, 2).unwrap();
    let m3 = (&m1 + &m2).unwrap();

    assert_eq!(m3[(0, 0)], complex::Complex::<f32>::new(3.0, -1.0));
    assert_eq!(m3[(0, 1)], complex::Complex::<f32>::new(2.0, -1.0));
    assert_eq!(m3[(1, 0)], complex::Complex::<f32>::new(4.0, -2.0));
    assert_eq!(m3[(1, 1)], complex::Complex::<f32>::new(4.0, -4.0));
}

#[test]
fn sub() {
    let a = complex::Complex::<f32>::new(1.0, 1.0);
    let b = complex::Complex::<f32>::new(0.0, 1.0);
    let c = complex::Complex::<f32>::new(2.0, 0.0);
    let d = complex::Complex::<f32>::new(2.0, -2.0);

    let m1 = matrix::Matrix::<complex::Complex<f32>>::new(vec![a, b, c, d], 2, 2).unwrap();
    let m2 = matrix::Matrix::<complex::Complex<f32>>::new(vec![d, d, d, d], 2, 2).unwrap();
    let m3 = (&m1 - &m2).unwrap();

    assert_eq!(m3[(0, 0)], complex::Complex::<f32>::new(-1.0, 3.0));
    assert_eq!(m3[(0, 1)], complex::Complex::<f32>::new(-2.0, 3.0));
    assert_eq!(m3[(1, 0)], complex::Complex::<f32>::new(0.0, 2.0));
    assert_eq!(m3[(1, 1)], complex::Complex::<f32>::new(0.0, 0.0));
}

use crate::polynomial::ModularArithmeticPolynomial;
use crate::polynomial::Polynomial;

#[test]
fn sum_matrix_of_polynomial() {
    // P(x) = 1 + 2i*x + (1 + i)*x²
    let a = complex::Complex::<f32>::from(1.0);
    let a_p_a = complex::Complex::<f32>::from(2.0);

    let b = complex::I_F32 * complex::Complex::<f32>::from(2.0);
    let b_p_b = complex::I_F32 * complex::Complex::<f32>::from(4.0);

    let c = complex::Complex::new(1.0, 1.0);

    let zero = complex::Complex::<f32>::from(0.0);

    let poly_1 = ModularArithmeticPolynomial::new(&Polynomial::new(&[a, b, zero]), 3);
    let poly_2 = ModularArithmeticPolynomial::new(&Polynomial::new(&[zero, zero, c]), 3);

    let m1 = matrix::Matrix::new(vec![poly_1.clone(), poly_2], 2, 1).unwrap();
    let m2 = matrix::Matrix::new(vec![poly_1.clone(), poly_1], 2, 1).unwrap();
    let m3 = (&m1 + &m2).unwrap();

    assert_eq!(m3[(0, 0)].coef(0).unwrap(), a_p_a);
    assert_eq!(m3[(0, 0)].coef(1).unwrap(), b_p_b);
    assert_eq!(m3[(0, 0)].coef(2).unwrap(), zero);
    assert_eq!(m3[(0, 1)].coef(0).unwrap(), a);
    assert_eq!(m3[(0, 1)].coef(1).unwrap(), b);
    assert_eq!(m3[(0, 1)].coef(2).unwrap(), c);
}

#[test]
fn sub_matrix_of_polynomial() {
    // P(x) = 1 + 2i*x + (1 + i)*x²
    let a = complex::Complex::<f32>::from(1.0);
    let m_a = complex::Complex::<f32>::from(-1.0);

    let b = complex::I_F32 * complex::Complex::<f32>::from(2.0);
    let m_b = complex::I_F32 * complex::Complex::<f32>::from(-2.0);

    let c = complex::Complex::new(1.0, 1.0);

    let zero = complex::Complex::<f32>::from(0.0);

    let poly_1 = ModularArithmeticPolynomial::new(&Polynomial::new(&[a, b, zero]), 3);
    let poly_2 = ModularArithmeticPolynomial::new(&Polynomial::new(&[zero, zero, c]), 3);

    let m1 = matrix::Matrix::new(vec![poly_1.clone(), poly_2], 2, 1).unwrap();
    let m2 = matrix::Matrix::new(vec![poly_1.clone(), poly_1], 2, 1).unwrap();
    let m3 = (&m1 - &m2).unwrap();

    assert_eq!(m3[(0, 0)].coef(0).unwrap(), zero);
    assert_eq!(m3[(0, 0)].coef(1).unwrap(), zero);
    assert_eq!(m3[(0, 0)].coef(2).unwrap(), zero);
    assert_eq!(m3[(0, 1)].coef(0).unwrap(), m_a);
    assert_eq!(m3[(0, 1)].coef(1).unwrap(), m_b);
    assert_eq!(m3[(0, 1)].coef(2).unwrap(), c);
}

#[test]
fn matrix_product() {
    let a = complex::Complex::<f32>::new(1.0, 1.0);
    let b = complex::Complex::<f32>::new(0.0, 1.0);
    let c = complex::Complex::<f32>::new(2.0, 0.0);
    let d = complex::Complex::<f32>::new(2.0, -2.0);

    let m1 = matrix::Matrix::<complex::Complex<f32>>::new(vec![a, b, c, d], 2, 2).unwrap();
    let m2 = matrix::Matrix::<complex::Complex<f32>>::new(vec![a, d], 2, 1).unwrap();
    let m3 = (&m1 * &m2).unwrap();

    assert_eq!(m3.shape(), (2, 1));
    assert_eq!(m3[(0, 0)], complex::Complex::<f32>::new(2.0, 4.0));
    assert_eq!(m3[(1, 0)], complex::Complex::<f32>::new(2.0, -6.0));
}

fn nearly_equal_f32(a: f32, b: f32) -> bool {
    let abs_a = a.abs();
    let abs_b = b.abs();
    let diff = (a - b).abs();

    if a == b {
        // Handle infinities.
        true
    } else if a == 0.0 || b == 0.0 || diff < f32::MIN_POSITIVE {
        // One of a or b is zero (or both are extremely close to it,) use absolute error.
        diff < f32::EPSILON
    } else {
        // Use relative error.
        (diff / f32::min(abs_a + abs_b, f32::MAX)) < f32::EPSILON
    }
}

#[test]
fn matrix_product_polynomials() {
    // P1(x) = 1 + 2i*x + (1 + i)*x²
    // P2(x) = 1 + i + x + 2i*x²
    let a = complex::Complex::<f32>::from(1.0);
    let b = complex::I_F32 * complex::Complex::<f32>::from(2.0);
    let c = complex::Complex::new(1.0, 1.0);

    let mod_poly_1 = ModularArithmeticPolynomial::new(&Polynomial::new(&[a, b, c]), 3);
    let mod_poly_2 = ModularArithmeticPolynomial::new(&Polynomial::new(&[c, a, b]), 3);

    let m1 = matrix::Matrix::new(vec![mod_poly_1.clone(), mod_poly_2.clone()], 1, 2).unwrap();
    let m2 = matrix::Matrix::new(vec![mod_poly_2, mod_poly_1], 2, 1).unwrap();

    assert_eq!((&m2 * &m1).unwrap().shape(), (2, 2));

    let m3 = (&m1 * &m2).unwrap();

    assert_eq!(m3.shape(), (1, 1));
    assert_eq!(m3[(0, 0)].modulus(), 3);
    assert!(nearly_equal_f32(m3[(0, 0)].coef(0).unwrap().real(), -4.0));
    assert!(nearly_equal_f32(m3[(0, 0)].coef(1).unwrap().real(), -6.0));
    assert!(nearly_equal_f32(m3[(0, 0)].coef(2).unwrap().real(), 0.0));
    assert!(nearly_equal_f32(m3[(0, 0)].coef(0).unwrap().imag(), 4.0));
    assert!(nearly_equal_f32(m3[(0, 0)].coef(1).unwrap().imag(), 8.0));
    assert!(nearly_equal_f32(m3[(0, 0)].coef(2).unwrap().imag(), 12.0));
}

#[test]
fn matrix_product_polynomials_2() {
    let a = complex::Complex::<f32>::from(1.0);
    let b = complex::I_F32 * complex::Complex::<f32>::from(2.0);
    let c = complex::Complex::new(1.0, 1.0);

    let mod_poly_1 = ModularArithmeticPolynomial::new(&Polynomial::new(&[a, b, c]), 3);
    let mod_poly_2 = ModularArithmeticPolynomial::new(&Polynomial::new(&[c, a, b]), 3);

    let m1 = matrix::Matrix::new(
        vec![
            mod_poly_1.clone(),
            mod_poly_2.clone(),
            mod_poly_1.clone(),
            mod_poly_2.clone(),
        ],
        2,
        2,
    )
    .unwrap();
    let m2 = matrix::Matrix::new(
        vec![
            mod_poly_2.clone(),
            mod_poly_1.clone(),
            mod_poly_2.clone(),
            mod_poly_1.clone(),
        ],
        2,
        2,
    )
    .unwrap();

    let _ = (&m1 * &m2).unwrap();
}

#[test]
fn matrix_product_polynomials_error() {
    // P1(x) = 1 + 2i*x + (1 + i)*x²
    // P2(x) = 1 + i + x + 2i*x²
    let a = complex::Complex::<f32>::from(1.0);
    let b = complex::I_F32 * complex::Complex::<f32>::from(2.0);
    let c = complex::Complex::new(1.0, 1.0);

    let mod_poly_1 = ModularArithmeticPolynomial::new(&Polynomial::new(&[a, b, c]), 3);
    let mod_poly_2 = ModularArithmeticPolynomial::new(&Polynomial::new(&[c, a, b]), 3);

    let m1 = matrix::Matrix::new(vec![mod_poly_1.clone(), mod_poly_2.clone()], 2, 1).unwrap();
    let m2 = matrix::Matrix::new(vec![mod_poly_2, mod_poly_1], 2, 1).unwrap();

    match &m1 * &m2 {
        Err(matrix::MatrixError::UncompatibleMatrixShapes(_)) => (),
        _ => panic!("Wrong error type"),
    }
}
