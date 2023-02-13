use crate::matrix::matrix_view::*;
use crate::matrix::{Number, ModularArithmeticPolynomial};
use crate::matrix::{Matrix, MatrixInput};
use core::ops::{AddAssign, SubAssign, Add, Sub, Mul};


trait InnerAddAssign {
	fn inner_add_assign(a: &mut Self, b: &Self);
}
impl<T: Number> InnerAddAssign for T {
	fn inner_add_assign(a: &mut T, b: &T) {
		*a += *b;
	}
}
impl<T: Number> InnerAddAssign for ModularArithmeticPolynomial<T> {
	fn inner_add_assign(a: &mut ModularArithmeticPolynomial<T>, b: &ModularArithmeticPolynomial<T>) {
		*a += b;
	}
}

trait InnerSubAssign {
	fn inner_sub_assign(a: &mut Self, b: &Self);
}
impl<T: Number> InnerSubAssign for T {
	fn inner_sub_assign(a: &mut T, b: &T) {
		*a -= *b;
	}
}
impl<T: Number> InnerSubAssign for ModularArithmeticPolynomial<T> {
	fn inner_sub_assign(a: &mut ModularArithmeticPolynomial<T>, b: &ModularArithmeticPolynomial<T>) {
		*a -= b;
	}
}

trait InnerMul {
	fn inner_mul(a: &Self, b: &Self) -> Self;
}
impl<T: Number> InnerMul for T {
	fn inner_mul(a: &T, b: &T) -> T {
		*a * *b
	}
}
impl<T> InnerMul for ModularArithmeticPolynomial<T>
where T: Number + From<crate::complex::Complex<f64>>, crate::complex::Complex<f64>: From<T> {
	fn inner_mul(a: &ModularArithmeticPolynomial<T>, b: &ModularArithmeticPolynomial<T>) -> ModularArithmeticPolynomial<T> {
		match a * b {
			Ok(ret) => ret,
			Err(e) => panic!("Error in matrix multiplication of polynomials: {:?}", e)
		}
	}
}

pub fn matrix_mult<T: Number + MatrixInput>(a: &Matrix<T>, b: &Matrix<T>) -> Matrix<T> {
	let a_view = a.as_view();
	let b_view = b.as_view();
	let ret: MatrixView<T> = &a_view * &b_view;
	Matrix::<T>::from(ret)
}
pub fn matrix_mult_poly<T>(a: &Matrix<ModularArithmeticPolynomial<T>>, b: &Matrix<ModularArithmeticPolynomial<T>>) -> Matrix<ModularArithmeticPolynomial<T>> 
where T: Number + MatrixInput + From<crate::complex::Complex<f64>>, crate::complex::Complex<f64>: From<T> {
	let a_view = a.as_view();
	let b_view = b.as_view();
	let ret: MatrixView<ModularArithmeticPolynomial<T>> = &a_view * &b_view;
	Matrix::<ModularArithmeticPolynomial<T>>::from(ret)
}

impl<'a, 'b:'a, T: MatrixInput + InnerAddAssign> AddAssign<&'b MatrixView<'a, T> > for MatrixView<'a, T> {
	fn add_assign(&mut self, other: &'b MatrixView<'a, T>) {
		assert_eq!(self.shape(), other.shape());
		assert!(self.actual_rows >= other.actual_rows);
		assert!(self.actual_cols >= other.actual_cols);

		if other.m.is_none() {
			return;
		}

		for x in 0..other.actual_rows {
			for (val, add) in self.row_mut(x).zip(other.row(x)) {
				<T as InnerAddAssign>::inner_add_assign(val, add);
			}
		}
	}
}

impl<'a, 'b:'a, T: MatrixInput + InnerSubAssign> SubAssign<&'b MatrixView<'a, T> > for MatrixView<'a, T> {
	fn sub_assign(&mut self, other: &'b MatrixView<'a, T>) {
		assert_eq!(self.shape(), other.shape());
		assert!(self.actual_rows >= other.actual_rows);
		assert!(self.actual_cols >= other.actual_cols);

		if other.m.is_none() {
			return;
		}

		for x in 0..other.actual_rows {
			for (val, sub) in self.row_mut(x).zip(other.row(x)) {
				<T as InnerSubAssign>::inner_sub_assign(val, sub);
			}
		}
	}
}

impl<'a, 'b:'a, T: MatrixInput + InnerAddAssign> Add for &'b MatrixView<'a, T> {
	type Output = MatrixView<'a, T>;

	fn add(self, other: &'b MatrixView<'a, T>) -> MatrixView<'a, T> {
		let mut ret = (*self).clone();
		ret += other;
		return ret;
	}
}

impl<'a, 'b:'a, T: MatrixInput + InnerSubAssign> Sub for &'b MatrixView<'a, T> {
	type Output = MatrixView<'a, T>;

	fn sub(self, other: &'b MatrixView<'a, T>) -> MatrixView<'a, T> {
		let mut ret = (*self).clone();
		ret -= other;
		return ret;
	}
}

impl<'a, 'b:'a, T: MatrixInput + InnerMul + InnerAddAssign> Mul for &'b MatrixView<'a, T> {
	type Output = MatrixView<'a, T>;

	fn mul(self: &'b MatrixView<'a, T>, other_transposed: &'b MatrixView<'a, T>) -> MatrixView<'a, T> {
		assert_eq!(self.cols, other_transposed.cols);

		if self.m.is_none() || other_transposed.m.is_none() {
			return MatrixView::<T>::none((self.rows, self.cols));
		}

		let mut coefs = Vec::<T>::with_capacity(self.actual_rows * other_transposed.actual_rows);
		for x in 0..self.actual_rows {
			for y in 0..other_transposed.actual_rows {
				let mut it_self = self.row(x);
				let mut it_other = other_transposed.row(y);
				let mut coef = <T as InnerMul>::inner_mul(
					it_self.next().unwrap(), 
					it_other.next().unwrap());
				for (a,b) in it_self.zip(it_other) {
					<T as InnerAddAssign>::inner_add_assign(
						&mut coef,
						&<T as InnerMul>::inner_mul(a,b));
				}
				coefs.push(coef);
			}
		}

		let mut ret = MatrixView::<T>::new(coefs, self.actual_rows, other_transposed.actual_rows).unwrap();
		ret.rows = self.rows;
		ret.cols = other_transposed.rows;

		ret
	}
}