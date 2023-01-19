//! This module implements matrix simple operations.

#[cfg(test)]
mod test;
mod mult;
mod matrix_view;

use crate::complex::Number;
use crate::polynomial::{ModularArithmeticPolynomial, ModularArithmeticError};

use std::ops::{Add, Sub, Index, IndexMut};
use std::iter::{Skip, StepBy};

/// We define the trait representing the minimum operations necessary to build a matrix out if it
pub trait MatrixInput: Clone + std::fmt::Display {}
impl MatrixInput for f32 {}

/// We define all our error types here
#[derive(Debug)]
pub enum MatrixError {
	ZeroDimension(String),
	WrongInputArraySize(String),
	UncompatibleMatrixShapes(String),
	OutOfBoundsIndex(String),
	ModularArithmeticError(ModularArithmeticError)
}
impl From<ModularArithmeticError> for MatrixError {
	fn from(e: ModularArithmeticError) -> Self {
		return MatrixError::ModularArithmeticError(e);
	}
}
type MatrixResult<T> = Result<Matrix<T>, MatrixError>;

#[derive(Debug, Clone)]
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
	pub fn new(arr: Vec<T>, x: usize, y: usize) -> MatrixResult<T> {
		Self::check_zero_dimension(x, y)?;
		if arr.len() != x*y {
			return Err(MatrixError::WrongInputArraySize(format!("Wrong input size: {} instead of {}", arr.len(), x*y)));
		}

		return Ok(Self{cols: y, rows: x, arr: arr});
	}

	#[inline]
	pub fn len(&self) -> usize {
		self.rows * self.cols
	}

	#[inline]
	pub fn shape(&self) -> (usize, usize) {
		(self.rows, self.cols)
	}

	/// Public helper to help detect fraudulant indexing (allows error reporting)
	pub fn check_idx(&self, x: usize, y: usize) -> Result<(), MatrixError> {
		if x >= self.rows {
			return Err(MatrixError::OutOfBoundsIndex(format!("x index too high: {} for size {}", x, self.rows)));
		} else if y >= self.cols {
			return Err(MatrixError::OutOfBoundsIndex(format!("y index too high: {} for size {}", y, self.cols)))
		} else {
			Ok(())
		}
	}

	#[inline]
	fn idx(&self, x: usize, y: usize) -> usize {
		x * self.cols + y
	}

	#[allow(dead_code)]
	fn row(&self, x: usize) -> Result<std::slice::Iter<T>, MatrixError> {
		if x >= self.rows {
			return Err(MatrixError::OutOfBoundsIndex(format!("x index too high: {} for size {}", x, self.rows)));
		}

		Ok(self.arr[x*self.cols..(x+1)*self.cols].iter())
	}

	#[allow(dead_code)]
	fn col(&self, y: usize) -> Result<StepBy<Skip<std::slice::Iter<T>>>, MatrixError> {
		if y >= self.cols {
			return Err(MatrixError::OutOfBoundsIndex(format!("y index too high: {} for size {}", y, self.cols)));
		}

		Ok(self.arr.iter().skip(y).step_by(self.cols))
	}

	fn clone_transposed(&self) -> Matrix<T> {
		let mut arr = Vec::<T>::with_capacity(self.len());
		for y in 0..self.cols {
			for x in 0..self.rows {
				arr.push(self[(x,y)].clone());
			}
		}
		Self::new(arr, self.cols, self.rows).unwrap()
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

/// Sub operation for any input that is a Number (in particular: has Copy and Add by value)
impl<'a, T: MatrixInput + Number> Sub for &'a Matrix<T> {
	type Output = MatrixResult<T>;

	fn sub(self, other: &'a Matrix<T>) -> MatrixResult<T> {
		if self.shape() != other.shape() {
			return Err(MatrixError::UncompatibleMatrixShapes(
				format!("Uncompatible matrix shapes for addition, {:?} and {:?}", self.shape(), other.shape())
			));
		}

		let mut vec = Vec::<T>::with_capacity(self.len());
		for i in 0..self.len() {
			vec.push(self.arr[i] - other.arr[i]);
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

/// Add operation for Polynomials, which don't have the Copy trait, and thus add by reference
/// In addition, this allows catching any error coming from the modular Arithmetic module
impl<'a, T: Number> Sub for &'a Matrix<ModularArithmeticPolynomial<T>> {
	type Output = MatrixResult<ModularArithmeticPolynomial<T>>;

	fn sub(self, other: &'a Matrix<ModularArithmeticPolynomial<T>>) -> MatrixResult<ModularArithmeticPolynomial<T>> {
		if self.shape() != other.shape() {
			return Err(MatrixError::UncompatibleMatrixShapes(
				format!("Uncompatible matrix shapes for addition, {:?} and {:?}", self.shape(), other.shape())
			));
		}

		let mut vec = Vec::<ModularArithmeticPolynomial<T>>::with_capacity(self.len());
		for i in 0..self.len() {
			vec.push((&self.arr[i] - &other.arr[i])?);
		}
		Ok(Matrix::<ModularArithmeticPolynomial<T>>{arr: vec, cols: self.cols, rows: self.rows})
	}
}
