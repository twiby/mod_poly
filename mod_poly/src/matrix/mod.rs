//! This module implements matrix simple operations.

#[cfg(test)]
mod test;

use std::ops::{Index, IndexMut};

use crate::complex::Number;

#[derive(Debug)]
pub enum MatrixError {
	WrongInputArraySize(String),
}

#[derive(Debug)]
pub struct Matrix<T: Number> {
	arr: Vec<T>,
	cols: usize,
	rows: usize
}

impl<T: Number> Matrix<T> {
	/// Creates a new Matrix filled with zeros.
	pub fn new_empty(rows: usize, cols: usize) -> Self {
		Self{arr: vec![T::from(0.0); rows*cols], cols: cols, rows: rows}
	}

	/// Creates a new matrix with the provided data (which should spans columns before rows)
	pub fn new(arr: &[T], x: usize, y: usize) -> Result<Self, MatrixError> {
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
	fn idx(&self, x: usize, y: usize) -> usize {
		x * self.cols + y
	}
}

impl<T: Number> Index<(usize, usize)> for Matrix<T> {
	type Output = T;

	fn index(&self, index: (usize, usize)) -> &Self::Output {
		&self.arr[self.idx(index.0, index.1)]
	}
}
impl<T: Number> IndexMut<(usize, usize)> for Matrix<T> {
	fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
		let idx = self.idx(index.0, index.1);
		&mut self.arr[idx]
	}
}
