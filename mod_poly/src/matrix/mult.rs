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

		let mut m = Matrix::<T>::new_empty(self.rows, other.cols, T::from(0.0)).unwrap();

		for x in 0..self.rows {
			for y in 0..other.cols {
				for i in 0..self.cols {
					m[(x,y)] += self[(x, i)] * other[(i, y)];
				}
			}
		}

		Ok(m)
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

		let modulus = self[(0,0)].modulus();
		let mut m = Matrix::<ModularArithmeticPolynomial<T>>::new_empty(self.rows, other.cols, ModularArithmeticPolynomial::<T>::new_zero(modulus)).unwrap();

		for x in 0..self.rows {
			for y in 0..other.cols {
				for i in 0..self.cols {
					m[(x,y)] += &(&self[(x, i)] * &other[(i, y)])?;
				}
			}
		}
		
		Ok(m)
	}
}
