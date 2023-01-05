//! This module implements matrix simple operations.

#[cfg(test)]
mod test;

use crate::complex::Number;
use crate::polynomial::{ModularArithmeticPolynomial, ModularArithmeticError};

use std::ops::{Add, Mul, Index, IndexMut};

/// We define the trait representing the minimum operations necessary to build a matrix out if it
pub trait MatrixInput: Clone + std::fmt::Display {}
impl MatrixInput for f32 {}

/// We define all our error types here
#[derive(Debug)]
pub enum MatrixError {
	ZeroDimension(String),
	WrongInputArraySize(String),
	UncompatibleMatrixShapes(String),
	ModularArithmeticError(ModularArithmeticError)
}
impl From<ModularArithmeticError> for MatrixError {
	fn from(e: ModularArithmeticError) -> Self {
		return MatrixError::ModularArithmeticError(e);
	}
}
type MatrixResult<T> = Result<Matrix<T>, MatrixError>;

#[derive(Debug)]
pub struct Matrix<T> {
	arr: Vec<T>,
	cols: usize,
	rows: usize
}

/// Implement the Display trait
impl<T: MatrixInput> std::fmt::Display for Matrix<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut ret = "[".to_string();

		for x in 0..self.rows {
			ret.push('[');
			for y in 0..self.cols-1 {
				ret.push_str(&self[(x,y)].to_string());
				ret.push_str(", ");
			}
			ret.push_str(&self[(x, self.cols-1)].to_string());
			ret.push(']');
			if x != self.rows-1 {
				ret.push(',');
				ret.push('\n');
			}
		}

		ret.push(']');
		f.write_str(&ret)
	}
}

impl<T: MatrixInput> Matrix<T> {
	fn check_zero_dimension(rows: usize, cols: usize) -> Result<(), MatrixError> {
		if rows == 0 || cols == 0 {
			return Err(MatrixError::ZeroDimension("All dimensions must be non-zero".to_string()));
		}
		Ok(())
	}

	/// Creates a new Matrix filled with zeros.
	pub fn new_empty(rows: usize, cols: usize, default: T) -> MatrixResult<T> {
		Self::check_zero_dimension(rows, cols)?;
		Ok(Self{arr: vec![default.clone(); rows*cols], cols: cols, rows: rows})
	}

	/// Creates a new matrix with the provided data (which should spans columns before rows)
	pub fn new(arr: &[T], x: usize, y: usize) -> MatrixResult<T> {
		Self::check_zero_dimension(x, y)?;
		if arr.len() != x*y {
			return Err(MatrixError::WrongInputArraySize(format!("Wrong input size: {} instead of {}", arr.len(), x*y)));
		}

		return Ok(Self{cols: y, rows: x, arr: arr.to_vec()});
	}

	#[inline]
	pub fn len(&self) -> usize {
		self.rows * self.cols
	}

	#[inline]
	pub fn shape(&self) -> (usize, usize) {
		(self.rows, self.cols)
	}

	#[inline]
	fn idx(&self, x: usize, y: usize) -> usize {
		x * self.cols + y
	}
}

impl<T: MatrixInput> Index<(usize, usize)> for Matrix<T> {
	type Output = T;

	fn index(&self, index: (usize, usize)) -> &Self::Output {
		&self.arr[self.idx(index.0, index.1)]
	}
}
impl<T: MatrixInput> IndexMut<(usize, usize)> for Matrix<T> {
	fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
		let idx = self.idx(index.0, index.1);
		&mut self.arr[idx]
	}
}

/// Add operation for any input that is a Number (in particular: has Copy and Add by value)
impl<'a, T: MatrixInput + Number> Add for &'a Matrix<T> {
	type Output = MatrixResult<T>;

	fn add(self, other: &'a Matrix<T>) -> MatrixResult<T> {
		if self.shape() != other.shape() {
			return Err(MatrixError::UncompatibleMatrixShapes(
				format!("Uncompatible matrix shapes for addition, {:?} and {:?}", self.shape(), other.shape())
			));
		}

		let mut vec = Vec::<T>::with_capacity(self.len());
		for i in 0..self.len() {
			vec.push(self.arr[i] + other.arr[i]);
		}
		Ok(Matrix::<T>{arr: vec, cols: self.cols, rows: self.rows})
	}
}

/// Add operation for Polynomials, which don't have the Copy trait, and thus add by reference
/// In addition, this allows catching any error coming from the modular Arithmetic module
impl<'a, T: Number> Add for &'a Matrix<ModularArithmeticPolynomial<T>> {
	type Output = MatrixResult<ModularArithmeticPolynomial<T>>;

	fn add(self, other: &'a Matrix<ModularArithmeticPolynomial<T>>) -> MatrixResult<ModularArithmeticPolynomial<T>> {
		if self.shape() != other.shape() {
			return Err(MatrixError::UncompatibleMatrixShapes(
				format!("Uncompatible matrix shapes for addition, {:?} and {:?}", self.shape(), other.shape())
			));
		}

		let mut vec = Vec::<ModularArithmeticPolynomial<T>>::with_capacity(self.len());
		for i in 0..self.len() {
			vec.push((&self.arr[i] + &other.arr[i])?);
		}
		Ok(Matrix::<ModularArithmeticPolynomial<T>>{arr: vec, cols: self.cols, rows: self.rows})
	}
}

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
impl<'a, T: Number> Mul for &'a Matrix<ModularArithmeticPolynomial<T>> {
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
