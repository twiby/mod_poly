//! This module implements matrix simple operations.

#[cfg(test)]
mod test;

use crate::complex::Number;
use crate::polynomial::{ModularArithmeticPolynomial, ModularArithmeticError};

use std::ops::{Add, Index, IndexMut};

/// We define the trait representing the minimum operations necessary to build a matrix out if it
pub trait MatrixInput: Clone {}
impl MatrixInput for f32 {}

/// We define all our error types here
#[derive(Debug)]
pub enum MatrixError {
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

impl<T: MatrixInput> Matrix<T> {
	/// Creates a new Matrix filled with zeros.
	pub fn new_empty(rows: usize, cols: usize, default: T) -> Self {
		Self{arr: vec![default.clone(); rows*cols], cols: cols, rows: rows}
	}

	/// Creates a new matrix with the provided data (which should spans columns before rows)
	pub fn new(arr: &[T], x: usize, y: usize) -> MatrixResult<T> {
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

/// Add operation for any input that implements Copy (and thus has addition by value)
impl<'a, T: MatrixInput + Copy + Add<Output = T>> Add for &'a Matrix<T> {
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
