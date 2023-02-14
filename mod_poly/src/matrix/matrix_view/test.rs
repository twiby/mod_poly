use crate::matrix;
use crate::matrix::matrix_view::*;

#[test]
fn view_index() {
	let v = Viewer::Owner(vec![1,2,3]);
	assert_eq!(v[0], 1);
	assert_eq!(v[1], 2);
	assert_eq!(v[2], 3);
}

#[test]
fn view_inner() {
	let mut v = Viewer::Owner(vec![1,2,3]);
	let v2 = Viewer::from(&v);
	let v3 = Viewer::<Vec<usize>>::None;
	let vec = vec![1, 2, 3];
	let v4 = Viewer::from(vec);

	assert_eq!(v.inner(), Some(&vec![1,2,3]));
	assert_eq!(v2.inner(), Some(&vec![1,2,3]));
	assert_eq!(v4.inner(), Some(&vec![1,2,3]));
	assert_eq!(v3.inner(), None);

	v.inner_mut().unwrap()[0] = 10;
	assert_eq!(v.inner(), Some(&vec![10,2,3]));
}

#[test]
fn view_writer() {
	let mut v = Viewer::Owner(vec![1,2,3]);
	let mut w = v.writer();

	w[0] = 10;
	assert_eq!(v.inner(), Some(&vec![10,2,3]));
}

#[test]
fn view() {
	let m = matrix::Matrix::<f32>::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3).unwrap();
	let v = m.view((0,0), (2,3));

	assert_eq!(v.x, 0);
	assert_eq!(v.y, 0);
	assert_eq!(v.cols, 3);
	assert_eq!(v.rows, 2);

	match v.m {
		Viewer::Reader(mmm) => assert_eq!(mmm[(1,0)], 4.0),
		_ => panic!("Wrong viewer type")
	}
}

#[test]
fn as_view() {
	let m = matrix::Matrix::<f32>::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3).unwrap();
	let v = m.as_view();

	assert_eq!(v.x, 0);
	assert_eq!(v.y, 0);
	assert_eq!(v.cols, 3);
	assert_eq!(v.rows, 2);

	match v.m {
		Viewer::Reader(mmm) => assert_eq!(mmm[(1,0)], 4.0),
		_ => panic!("Wrong viewer type")
	}
}

#[test]
fn clone() {
	let m = matrix::Matrix::<f32>::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3).unwrap();
	let v = m.as_view();
	let v2 = v.clone();

	assert_eq!(v2.x, 0);
	assert_eq!(v2.y, 0);
	assert_eq!(v2.cols, 3);
	assert_eq!(v2.rows, 2);
	assert_eq!(v2.actual_cols, 3);
	assert_eq!(v2.actual_rows, 2);

	match v.m {
		Viewer::Reader(mmm) => assert_eq!(mmm[(1,0)], 4.0),
		_ => panic!("Wrong viewer type")
	}

	match v2.m {
		Viewer::Owner(mmm) => assert_eq!(mmm[(1, 0)], v.m[(1,0)]),
		_ => panic!("Wrong viewer type")
	}
}

#[test]
fn sub_view() {
	let m = matrix::Matrix::<f32>::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3).unwrap();
	let v = m.view((0,1), (2,2));

	assert_eq!(v.x, 0);
	assert_eq!(v.y, 1);
	assert_eq!(v.cols, 2);
	assert_eq!(v.rows, 2);

	assert_eq!(v[(0,0)], 2.0);
	assert_eq!(v[(0,1)], 3.0);
	assert_eq!(v[(1,0)], 5.0);
	assert_eq!(v[(1,1)], 6.0);
}

#[test]
fn neg() {
	let m = matrix::Matrix::<f32>::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3).unwrap();
	let v = m.view((0,1), (2,2));
	let v2 = -&v;

	assert_eq!(v2.x, 0);
	assert_eq!(v2.y, 0);
	assert_eq!(v2.cols, 2);
	assert_eq!(v2.rows, 2);

	assert_eq!(v2[(0,0)], -2.0);
	assert_eq!(v2[(0,1)], -3.0);
	assert_eq!(v2[(1,0)], -5.0);
	assert_eq!(v2[(1,1)], -6.0);
}

#[test]
fn sub_view_2() {
	let m = MatrixView::<f32>::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3).unwrap();
	let v = m.view((0,1), (2,2));

	assert_eq!(v.x, 0);
	assert_eq!(v.y, 1);
	assert_eq!(v.cols, 2);
	assert_eq!(v.rows, 2);

	assert_eq!(v[(0,0)], 2.0);
	assert_eq!(v[(0,1)], 3.0);
	assert_eq!(v[(1,0)], 5.0);
	assert_eq!(v[(1,1)], 6.0);
}

#[test]
fn sub_view_3() {
	let m = matrix::Matrix::<f32>::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3).unwrap();
	let v1 = m.view((3,0), (2,2));
	let v2 = m.view((0,3), (2,2));

	match v1.m {
		Viewer::None => (),
		_ => panic!("Wrong viewer type")
	}
	match v2.m {
		Viewer::None => (),
		_ => panic!("Wrong viewer type")
	}
}

#[test]
fn sub_view_4() {
	let m = MatrixView::<f32>::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3).unwrap();
	let v1 = m.view((3,0), (2,2));
	let v2 = m.view((0,3), (2,2));

	match v1.m {
		Viewer::None => (),
		_ => panic!("Wrong viewer type")
	}
	match v2.m {
		Viewer::None => (),
		_ => panic!("Wrong viewer type")
	}
}

#[test]
fn none() {
	let v = MatrixView::<f32>::none((2,2));
	assert_eq!(v.x, 0);
	assert_eq!(v.y, 0);
	assert_eq!(v.cols, 2);
	assert_eq!(v.rows, 2);
	assert_eq!(v.actual_rows, 0);
	assert_eq!(v.actual_cols, 0);
}

#[test]
#[should_panic]
fn none_idx() {
	let v = MatrixView::<f32>::none((2,2));
	assert_eq!(v[(0,0)], 6.0);
}

#[test]
#[should_panic]
fn mat_view_idx() {
	let m = matrix::Matrix::<f32>::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3).unwrap();
	let mut v = m.view((0,0), (2,3));

	assert_eq!(v.x, 0);
	assert_eq!(v.y, 0);
	assert_eq!(v.cols, 3);
	assert_eq!(v.rows, 2);

	v[(1,0)] = 10.0;
}

#[test]
fn mat_own_idx() {
	let mut v = MatrixView::<f32>::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3).unwrap();

	assert_eq!(v.x, 0);
	assert_eq!(v.y, 0);
	assert_eq!(v.cols, 3);
	assert_eq!(v.rows, 2);

	match v.m {
		Viewer::Owner(ref mut mmm) => mmm[(1,0)] = 5.0,
		_ => panic!("Wrong viewer type")
	}
	v[(1,0)] = 10.0;
	assert_eq!(v[(1,0)], 10.0);
}

#[test]
fn mat_own_writer() {
	let mut v = MatrixView::<f32>::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3).unwrap();

	assert_eq!(v.x, 0);
	assert_eq!(v.y, 0);
	assert_eq!(v.cols, 3);
	assert_eq!(v.rows, 2);
	assert_eq!(v[(1,0)], 4.0);


	match v.m {
		Viewer::Owner(ref mut mmm) => mmm[(1,0)] = 5.0,
		_ => panic!("Wrong viewer type")
	}

	let mut w = v.writer((0,1), (2,2));

	w[(1,1)] = -5.0;
	assert_eq!(v[(0,0)], 1.0);
	assert_eq!(v[(0,1)], 2.0);
	assert_eq!(v[(0,2)], 3.0);
	assert_eq!(v[(1,0)], 5.0);
	assert_eq!(v[(1,1)], 5.0);
	assert_eq!(v[(1,2)], -5.0);
}

#[test]
fn own() {
	let mut v = MatrixView::<f32>::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3).unwrap();

	assert_eq!(v.x, 0);
	assert_eq!(v.y, 0);
	assert_eq!(v.cols, 3);
	assert_eq!(v.rows, 2);

	match v.m {
		Viewer::Owner(ref mut mmm) => mmm[(1,0)] = 5.0,
		_ => panic!("Wrong viewer type")
	}
	match v.m {
		Viewer::Owner(ref mmm) => assert_eq!(mmm[(1,0)], 5.0),
		_ => panic!("Wrong viewer type")
	}
}

#[test]
fn row() {
	let m = MatrixView::<f32>::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3).unwrap();
	let v = m.view((0,1), (3,1));

	assert_eq!(v.actual_rows, 2);
	assert_eq!(v.actual_cols, 1);
	assert_eq!(v.row(0).copied().collect::<Vec<f32>>(), vec![2.0]);
	assert_eq!(v.row(1).copied().collect::<Vec<f32>>(), vec![5.0]);
}

#[test]
#[should_panic]
fn row_fail() {
	let m = MatrixView::<f32>::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3).unwrap();
	let v = m.view((0,1), (3,2));

	assert_eq!(v.row(2).copied().collect::<Vec<f32>>(), vec![]);
}

#[test]
fn actual_shape() {
	let m = matrix::Matrix::<f32>::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3).unwrap();
	let v = m.view((1,1), (2,3));

	assert_eq!(v.rows, 2);
	assert_eq!(v.cols, 3);
	assert_eq!(v.actual_rows, 1);
	assert_eq!(v.actual_cols, 2);
}

#[test]
fn add_assign() {
	let mut m1 = MatrixView::<f32>::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3).unwrap();

	let m = Matrix::<f32>::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3).unwrap();
	let m2 = m.view((1,1), (2, 3));

	m1 += &m2;

	assert_eq!(m1[(0,0)], 6.0);
	assert_eq!(m1[(0,1)], 8.0);
	assert_eq!(m1[(0,2)], 3.0);
	assert_eq!(m1[(1,0)], 4.0);
	assert_eq!(m1[(1,1)], 5.0);
	assert_eq!(m1[(1,2)], 6.0);
}

#[test]
fn sub_assign() {
	let mut m1 = MatrixView::<f32>::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3).unwrap();

	let m = Matrix::<f32>::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3).unwrap();
	let m2 = m.view((1,1), (2, 3));

	m1 -= &m2;

	assert_eq!(m1[(0,0)], -4.0);
	assert_eq!(m1[(0,1)], -4.0);
	assert_eq!(m1[(0,2)], 3.0);
	assert_eq!(m1[(1,0)], 4.0);
	assert_eq!(m1[(1,1)], 5.0);
	assert_eq!(m1[(1,2)], 6.0);
}

#[test]
fn sub() {
	let m1 = MatrixView::<f32>::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3).unwrap();

	let m = Matrix::<f32>::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3).unwrap();
	let m2 = m.view((1,1), (2, 3));

	let m3 = &m1 - &m2;

	assert_eq!(m3[(0,0)], -4.0);
	assert_eq!(m3[(0,1)], -4.0);
	assert_eq!(m3[(0,2)], 3.0);
	assert_eq!(m3[(1,0)], 4.0);
	assert_eq!(m3[(1,1)], 5.0);
	assert_eq!(m3[(1,2)], 6.0);
}

#[test]
fn sub_none() {
	let m1 = MatrixView::<f32>::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3).unwrap();

	let m = Matrix::<f32>::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3).unwrap();
	let m2 = m.view((1,1), (2, 3));
	let none = MatrixView::<f32>::none((2,3));

	let mut m3 = &m1 - &none;

	assert_eq!(m3[(0,0)], 1.0);
	assert_eq!(m3[(0,1)], 2.0);
	assert_eq!(m3[(0,2)], 3.0);
	assert_eq!(m3[(1,0)], 4.0);
	assert_eq!(m3[(1,1)], 5.0);
	assert_eq!(m3[(1,2)], 6.0);

	m3 = &none - &m2;
	assert_eq!(m3[(0,0)], -5.0);
	assert_eq!(m3[(0,1)], -6.0);
}

#[test]
fn add() {
	let m1 = MatrixView::<f32>::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3).unwrap();

	let m = Matrix::<f32>::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3).unwrap();
	let m2 = m.view((1,1), (2, 3));

	let m3 = &m1 + &m2;

	assert_eq!(m3[(0,0)], 6.0);
	assert_eq!(m3[(0,1)], 8.0);
	assert_eq!(m3[(0,2)], 3.0);
	assert_eq!(m3[(1,0)], 4.0);
	assert_eq!(m3[(1,1)], 5.0);
	assert_eq!(m3[(1,2)], 6.0);
}

#[test]
fn add_none() {
	let m1 = MatrixView::<f32>::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3).unwrap();

	let m = Matrix::<f32>::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3).unwrap();
	let m2 = m.view((1,1), (2, 3));
	let none = MatrixView::<f32>::none((2,3));

	let mut m3 = &m1 + &none;

	assert_eq!(m3[(0,0)], 1.0);
	assert_eq!(m3[(0,1)], 2.0);
	assert_eq!(m3[(0,2)], 3.0);
	assert_eq!(m3[(1,0)], 4.0);
	assert_eq!(m3[(1,1)], 5.0);
	assert_eq!(m3[(1,2)], 6.0);

	m3 = &none + &m2;
	assert_eq!(m3[(0,0)], 5.0);
	assert_eq!(m3[(0,1)], 6.0);
}

#[test]
fn mul() {
	let m1 = MatrixView::<f32>::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 3, 2).unwrap();

	let m = Matrix::<f32>::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 3, 2).unwrap();
	let m_t = m.clone_transposed();
	let m2 = m_t.view((1,1), (3, 2));

	let m3 = &m1 * &m2;

	assert_eq!(m3.rows, 3);
	assert_eq!(m3.cols, 3);
	assert_eq!(m3.actual_rows, 3);
	assert_eq!(m3.actual_cols, 1);
	assert_eq!(m3[(0,0)], 16.0);
	assert_eq!(m3[(1,0)], 36.0);
	assert_eq!(m3[(2,0)], 56.0);
}

use crate::complex;
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

	let poly_1 = matrix::ModularArithmeticPolynomial::new(&Polynomial::new(&[a,b,zero]), 3);
	let poly_2 = matrix::ModularArithmeticPolynomial::new(&Polynomial::new(&[zero,zero,c]), 3);

	let m1 = matrix::Matrix::new(vec![poly_1.clone(), poly_2], 2, 1).unwrap();
	let m2 = matrix::Matrix::new(vec![poly_1.clone(), poly_1], 2, 1).unwrap();

	let v1 = m1.as_view();
	let v2 = m2.as_view();

	let m3 = &v1 + &v2;

	assert_eq!(m3[(0,0)].coef(0).unwrap(), a_p_a);
	assert_eq!(m3[(0,0)].coef(1).unwrap(), b_p_b);
	assert_eq!(m3[(0,0)].coef(2).unwrap(), zero);
	assert_eq!(m3[(0,1)].coef(0).unwrap(), a);
	assert_eq!(m3[(0,1)].coef(1).unwrap(), b);
	assert_eq!(m3[(0,1)].coef(2).unwrap(), c);
}

#[test]
fn mul_matrix_of_polynomial() {
	// P(x) = 1 + 2i*x + (1 + i)*x²
	let a = complex::Complex::<f32>::from(1.0);
	let b = complex::I_F32 * complex::Complex::<f32>::from(2.0);
	let c = complex::Complex::new(1.0, 1.0);

	let zero = complex::Complex::<f32>::from(0.0);

	let poly_1 = matrix::ModularArithmeticPolynomial::new(&Polynomial::new(&[a,b,zero]), 3);
	let poly_2 = matrix::ModularArithmeticPolynomial::new(&Polynomial::new(&[zero,zero,c]), 3);

	let m1 = matrix::Matrix::new(vec![poly_1.clone(), poly_2.clone()], 1, 2).unwrap();
	let m2 = matrix::Matrix::new(vec![poly_1.clone(), poly_1.clone()], 1, 2).unwrap();

	let v1 = m1.as_view();
	let v2 = m2.as_view();

	let m3 = &v1 * &v2;
	assert_eq!(m3.cols, 1);
	assert_eq!(m3.rows, 1);

	let result = (&(&poly_1 * &poly_1).unwrap() + &(&poly_1 * &poly_2).unwrap()).unwrap();
	assert_eq!(result.coef(0).unwrap(), m3[(0,0)].coef(0).unwrap());
	assert_eq!(result.coef(1).unwrap(), m3[(0,0)].coef(1).unwrap());
	assert_eq!(result.coef(2).unwrap(), m3[(0,0)].coef(2).unwrap());
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

	let poly_1 = matrix::ModularArithmeticPolynomial::new(&Polynomial::new(&[a,b,zero]), 3);
	let poly_2 = matrix::ModularArithmeticPolynomial::new(&Polynomial::new(&[zero,zero,c]), 3);

	let m1 = matrix::Matrix::new(vec![poly_1.clone(), poly_2], 2, 1).unwrap();
	let m2 = matrix::Matrix::new(vec![poly_1.clone(), poly_1], 2, 1).unwrap();

	let v1 = m1.as_view();
	let v2 = m2.as_view();

	let m3 = &v1 - &v2;

	assert_eq!(m3[(0,0)].coef(0).unwrap(), zero);
	assert_eq!(m3[(0,0)].coef(1).unwrap(), zero);
	assert_eq!(m3[(0,0)].coef(2).unwrap(), zero);
	assert_eq!(m3[(0,1)].coef(0).unwrap(), m_a);
	assert_eq!(m3[(0,1)].coef(1).unwrap(), m_b);
	assert_eq!(m3[(0,1)].coef(2).unwrap(), c);
}