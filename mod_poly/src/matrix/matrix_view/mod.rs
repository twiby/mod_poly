use crate::matrix::{Matrix, MatrixInput, MatrixError};

use core::ops::{Index, IndexMut};

#[cfg(test)]
mod test;
mod viewer;
mod ops;

use viewer::Viewer;

struct MatrixView<'a, T> {
	m: Viewer<'a, Matrix<T>>,
	cols: usize,
	rows: usize,
	x: usize,
	y: usize,
	actual_rows: usize,
	actual_cols: usize
}

impl<T: MatrixInput> Matrix<T> {
	fn view<'m: 'a, 'a>(&'m self, block_coord: (usize, usize), block_size: (usize, usize)) -> MatrixView<'a, T> {
		if block_coord.0 >= self.rows || block_coord.1 >= self.cols {
			return MatrixView::<T>::none(block_size);
		}

		MatrixView{
			m: Viewer::Reader(self),
			cols: block_size.1,
			rows: block_size.0,
			x: block_coord.0,
			y: block_coord.1,
			actual_rows: *[self.rows - block_coord.0, block_size.0].iter().min().unwrap(),
			actual_cols: *[self.cols - block_coord.1].iter().min().unwrap()
		}
	}

	fn as_view<'m: 'a, 'a>(&'m self) -> MatrixView<'a, T> {
		self.view((0,0), (self.rows, self.cols))
	}
}

impl<'a, T: MatrixInput> MatrixView<'a, T> {
	fn new(arr: Vec<T>, x: usize, y: usize) -> Result<Self, MatrixError> {
		Ok(Self{
			m: Viewer::Owner(Matrix::<T>::new(arr, x, y)?),
			cols: y,
			rows: x,
			x: 0,
			y: 0,
			actual_rows: x,
			actual_cols: y
		})
	}

	#[inline]
	fn none(block_size: (usize, usize)) -> Self {
		Self{
			m: Viewer::None,
			cols: block_size.1,
			rows: block_size.0,
			x: 0,
			y: 0,
			actual_cols: 0,
			actual_rows: 0
		}
	}

	fn view<'m>(&'m self, block_coord: (usize, usize), block_size: (usize, usize)) -> MatrixView<'m, T> {
		if block_coord.0 >= self.actual_rows || block_coord.1 >= self.actual_cols {
			return MatrixView::<T>::none(block_size);
		}

		MatrixView{
			m: self.m.view(),
			cols: block_size.1,
			rows: block_size.0,
			x: self.x + block_coord.0,
			y: self.y + block_coord.1,
			actual_rows: *[self.actual_rows - block_coord.0, block_size.0].iter().min().unwrap(),
			actual_cols: *[self.actual_cols - block_coord.1, block_size.1].iter().min().unwrap()
		}
	}

	#[inline]
	fn row(&self, n: usize) -> std::slice::Iter<T> {
		assert!(n < self.actual_rows);
		if self.m.is_none() {
			panic!("cannot take a row of a Viewer::None");
		}
		let idx = self.m.inner().unwrap().idx(self.x+n, self.y);
		self.m.inner().unwrap().arr[idx..(idx + self.actual_cols)].iter()
	}

	#[inline]
	fn row_mut(&mut self, n: usize) -> std::slice::IterMut<T> {
		assert!(n < self.actual_rows);
		if !self.m.is_owner() {
			panic!("cannot take a mutable row of a Viewer::None or Viewer::Reader");
		}
		let idx = self.m.inner().unwrap().idx(self.x+n, self.y);
		self.m.inner_mut().unwrap().arr[idx..(idx + self.actual_cols)].iter_mut()
	}

	#[inline]
	fn shape(&self) -> (usize, usize) {
		(self.rows, self.cols)
	}	
	#[allow(dead_code)]
	fn actual_shape(&self) -> (usize, usize) {
		(self.actual_rows, self.actual_cols)
	}
}

impl<'a, T: MatrixInput> Clone for MatrixView<'a, T> {
	fn clone(self: &MatrixView<'a, T>) -> MatrixView<'a, T> {
		if self.m.is_none() {
			return MatrixView::<T>::none((self.rows, self.cols));
		}

		let mut coefs = Vec::<T>::with_capacity(self.actual_rows * self.actual_cols);

		for x in 0..self.actual_rows {
			for val in self.row(x) {
				coefs.push(val.clone());
			}
		}

		let mut ret = MatrixView::<T>::new(coefs, self.actual_rows, self.actual_cols).unwrap();
		ret.rows = self.rows;
		ret.cols = self.cols;
		return ret;
	}
}

impl<'a, T: MatrixInput> From<MatrixView<'a, T>> for Matrix<T> {
	fn from(other: MatrixView<'a, T>) -> Matrix<T> {
		Option::<Matrix<T>>::from(other.m).unwrap()
	}
}

impl<'a, T: MatrixInput> Index<(usize, usize)> for MatrixView<'a, T> {
	type Output = T;

	fn index(&self, index: (usize, usize)) -> &T {
		&self.m[(self.x + index.0, self.y + index.1)]
	}
}
impl<'a, T: MatrixInput> IndexMut<(usize, usize)> for MatrixView<'a, T> {
	fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
		&mut self.m[(self.x + index.0, self.y + index.1)]
	}
}

pub fn matrix_mult<T>(a: &Matrix<T>, b: &Matrix<T>) -> Matrix<T>
where T: MatrixInput + ops::InnerMul + ops::InnerAddAssign + ops::InnerNeg + std::default::Default {
	let a_view = a.as_view();
	let b_view = b.as_view();
	let ret: MatrixView<T> = &a_view * &b_view;
	Matrix::<T>::from(ret)
}

