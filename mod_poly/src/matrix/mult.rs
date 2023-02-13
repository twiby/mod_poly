use crate::matrix::matrix_view::{matrix_mult, matrix_mult_poly};
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
		Ok(matrix_mult(self, &other_transposed))
	}
}

/// Mul operation for Polynomials, which don't have the Copy trait, and thus add by reference
/// In addition, this allows catching any error coming from the modular Arithmetic module
impl<'a, T> Mul for &'a Matrix<ModularArithmeticPolynomial<T>> 
where T: MatrixInput + Number + From<complex::Complex<f64>>, complex::Complex<f64>: From<T> {
	type Output = MatrixResult<ModularArithmeticPolynomial<T>>;

	fn mul(self, other: &'a Matrix<ModularArithmeticPolynomial<T>>) -> MatrixResult<ModularArithmeticPolynomial<T>> {
		if self.cols != other.rows {
			return Err(MatrixError::UncompatibleMatrixShapes(
				format!("Uncompatible matrix shapes for multiplication, {:?} and {:?}", self.shape(), other.shape())
			));
		}

		let other_transposed = other.clone_transposed();
		Ok(matrix_mult_poly(self, &other_transposed))
	}
}
