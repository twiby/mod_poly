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

use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Mul operation for Polynomials, which don't have the Copy trait, and thus add by reference
/// In addition, this allows catching any error coming from the modular Arithmetic module
impl<'a, T> Mul for &'a Matrix<ModularArithmeticPolynomial<T>> 
where T: Number + From<complex::Complex<f64>> + std::marker::Sync + std::marker::Send, complex::Complex<f64>: From<T> {
	type Output = MatrixResult<ModularArithmeticPolynomial<T>>;

	fn mul(self, other: &'a Matrix<ModularArithmeticPolynomial<T>>) -> MatrixResult<ModularArithmeticPolynomial<T>> {
		if self.cols != other.rows {
			return Err(MatrixError::UncompatibleMatrixShapes(
				format!("Uncompatible matrix shapes for multiplication, {:?} and {:?}", self.shape(), other.shape())
			));
		}

		let rows = self.rows;
		let cols = other.cols;

		let other_transposed = other.clone_transposed();

		let modulus = self[(0,0)].modulus();
		let coefs = Arc::new(Mutex::new(vec![ModularArithmeticPolynomial::<T>::new_zero(modulus); rows*cols]));

		(0..rows).into_par_iter().for_each(|x| {
			let mut idx = x*cols;
			for y in 0..cols {
				let mut coef = ModularArithmeticPolynomial::<T>::new_zero(modulus);
				for (a,b) in self.row(x).unwrap().zip(other_transposed.row(y).unwrap()) {
					coef += &(a * b).unwrap();
				}
				coefs.lock().unwrap()[idx] = coef;
				idx += 1;
			}
		});
		
		Matrix::<ModularArithmeticPolynomial<T>>::new(Arc::<Mutex<Vec<ModularArithmeticPolynomial<T>>>>::try_unwrap(coefs).unwrap().into_inner().unwrap(), rows, cols)
	}
}
