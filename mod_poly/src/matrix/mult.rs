use crate::complex;
use crate::complex::Number;
use crate::matrix::*;

use std::ops::Mul;


/// Mul operation for any input that is a Number (in particular: has Copy and Mul by value)
impl<'a, T: MatrixInput + Number> Mul for &'a Matrix<T> {
	type Output = MatrixResult<T>;

	fn mul(self, other: &'a Matrix<T>) -> MatrixResult<T> {
		if self.cols != other.rows {
			return Err(MatrixError::UncompatibleMatrixShapes(
				format!("Uncompatible matrix shapes for multiplication, {:?} and {:?}", self.shape(), other.shape())
			));
		}

		let other_transposed = other.clone_transposed();
		let mut coefs = Vec::<T>::with_capacity(self.rows * other.cols);

		for x in 0..self.rows {
			for y in 0..other.cols {
				let mut coef = T::from(0.0);
				for (a,b) in self.row(x)?.zip(other_transposed.row(y)?) {
					coef += *a * *b;
				}
				coefs.push(coef);
			}
		}

		Matrix::<T>::new(coefs, self.rows, other.cols)
	}
}

/// Mul operation for Polynomials, which don't have the Copy trait, and thus add by reference
/// In addition, this allows catching any error coming from the modular Arithmetic module
impl<'a, T> Mul for &'a Matrix<ModularArithmeticPolynomial<T>> 
where T: Number + From<complex::Complex<f64>>, complex::Complex<f64>: From<T> {
	type Output = MatrixResult<ModularArithmeticPolynomial<T>>;

	fn mul(self, other: &'a Matrix<ModularArithmeticPolynomial<T>>) -> MatrixResult<ModularArithmeticPolynomial<T>> {
		if self.cols != other.rows {
			return Err(MatrixError::UncompatibleMatrixShapes(
				format!("Uncompatible matrix shapes for multiplication, {:?} and {:?}", self.shape(), other.shape())
			));
		}

		let other_transposed = other.clone_transposed();

		&MatrixView::new(&self) * &MatrixView::new(&other_transposed)
	}
}

struct MatrixView<'a, T: MatrixInput> {
	m: &'a Matrix<T>,
	cols: usize,
	rows: usize,
	x: usize,
	y: usize
}

impl<'a, T: MatrixInput> MatrixView<'a, T> {
	fn new(m: &'a Matrix<T>) -> Self {
		Self{m: m, cols: m.cols, rows: m.rows, x: 0, y: 0}
	}

	fn view(&self, block_coord: (usize, usize), block_size: (usize, usize)) -> Self {
		Self{
			m: self.m,
			rows: block_size.0,
			cols: block_size.1,
			x: self.x + block_coord.0,
			y: self.y + block_coord.1
		}
	}

	fn shape(&self) -> (usize, usize) {
		(self.rows, self.cols)
	}

	fn first(&self) -> &T {
		&self.m[(0,0)]
	}

	fn row(&self, n: usize) -> Result<std::iter::Take<Skip<std::slice::Iter<T>>>, MatrixError> {
		Ok(self.m.row(self.x + n)?.skip(self.y).take(self.cols))
	}
}

/// Add operation for Polynomials, which don't have the Copy trait, and thus add by reference
/// In addition, this allows catching any error coming from the modular Arithmetic module
impl<'a, 'b, T: Number> Add for &'a MatrixView<'b, ModularArithmeticPolynomial<T>> {
	type Output = MatrixResult<ModularArithmeticPolynomial<T>>;

	fn add(self, other: &'a MatrixView<'b, ModularArithmeticPolynomial<T>>) -> MatrixResult<ModularArithmeticPolynomial<T>> {
		if self.shape() != other.shape() {
			return Err(MatrixError::UncompatibleMatrixShapes(
				format!("Uncompatible matrix shapes for addition, {:?} and {:?}", self.shape(), other.shape())
			));
		}

		let mut vec = Vec::<ModularArithmeticPolynomial<T>>::with_capacity(self.m.len());
		for i in 0..self.m.len() {
			vec.push((&self.m.arr[i] + &other.m.arr[i])?);
		}
		Ok(Matrix::<ModularArithmeticPolynomial<T>>{arr: vec, cols: self.cols, rows: self.rows})
	}
}

/// Mul operation for Polynomials, which don't have the Copy trait, and thus add by reference
/// In addition, this allows catching any error coming from the modular Arithmetic module
impl<'a, 'b, T> Mul for &'a MatrixView<'b, ModularArithmeticPolynomial<T>> 
where T: Number + From<complex::Complex<f64>>, complex::Complex<f64>: From<T> {
	type Output = MatrixResult<ModularArithmeticPolynomial<T>>;

	fn mul(self, other_transposed: &'a MatrixView<'b, ModularArithmeticPolynomial<T>>) -> MatrixResult<ModularArithmeticPolynomial<T>> {
		if self.cols != other_transposed.cols {
			return Err(MatrixError::UncompatibleMatrixShapes(
				format!("Uncompatible matrix shapes for multiplication, {:?} and {:?}", self.shape(), other_transposed.shape())
			));
		}


		let modulus = self.first().modulus();
		let mut coefs = Vec::<ModularArithmeticPolynomial<T>>::with_capacity(self.rows * other_transposed.rows);

		for x in 0..self.rows {
			for y in 0..other_transposed.rows {
				let mut coef = ModularArithmeticPolynomial::<T>::new_zero(modulus);
				for (a,b) in self.row(x)?.zip(other_transposed.row(y)?) {
					coef += &(a * b)?;
				}
				coefs.push(coef);
			}
		}
		
		Matrix::<ModularArithmeticPolynomial<T>>::new(coefs, self.rows, other_transposed.rows)
	}
}

#[test]
fn matrix_view() {
	let m = Matrix::<f32>::new(vec![0.0, 1.0, 2.0, 0.0, 1.0, 2.0], 2, 3).unwrap();

	let view_1 = MatrixView::new(&m);
	assert_eq!(view_1.row(0).unwrap().copied().collect::<Vec<f32>>(), vec![0.0, 1.0, 2.0]);
	assert_eq!(view_1.row(1).unwrap().copied().collect::<Vec<f32>>(), vec![0.0, 1.0, 2.0]);
	assert_eq!(view_1.row(0).unwrap().copied().collect::<Vec<f32>>(), vec![0.0, 1.0, 2.0]);
	assert_eq!(view_1.row(1).unwrap().copied().collect::<Vec<f32>>(), vec![0.0, 1.0, 2.0]);

	let view_2 = view_1.view((0,0), (2,2));
	assert_eq!(view_2.row(0).unwrap().copied().collect::<Vec<f32>>(), vec![0.0, 1.0]);
	assert_eq!(view_2.row(1).unwrap().copied().collect::<Vec<f32>>(), vec![0.0, 1.0]);

	let view_3 = view_1.view((1,1), (2,2));
	assert_eq!(view_3.row(0).unwrap().copied().collect::<Vec<f32>>(), vec![1.0, 2.0]);
	match view_3.row(1) {
		Err(MatrixError::OutOfBoundsIndex(_)) => (),
		_ => panic!("Wrong error type")
	}
}
