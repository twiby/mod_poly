use crate::matrix::matrix_view::*;
use crate::matrix::{Number, ModularArithmeticPolynomial};
use core::ops::{AddAssign, SubAssign, Add, Sub, Mul, Neg};

pub trait InnerOps : Default {
	fn inner_add_assign(a: &mut Self, b: &Self);
	fn inner_sub_assign(a: &mut Self, b: &Self);
	fn inner_neg(a: &Self) -> Self;
	fn inner_mul(a: &Self, b: &Self) -> Self;
}
impl <T: Number> InnerOps for T {
	fn inner_add_assign(a: &mut T, b: &T) { *a += *b }
	fn inner_sub_assign(a: &mut T, b: &T) { *a -= *b }
	fn inner_neg(a: &T) -> T { -*a }
	fn inner_mul(a: &T, b: &T) -> T { *a * *b }
}
impl <T> InnerOps for ModularArithmeticPolynomial<T>
where T: Number + From<crate::complex::Complex<f64>>, crate::complex::Complex<f64>: From<T> {
	fn inner_add_assign(a: &mut Self, b: &Self) { *a += b }
	fn inner_sub_assign(a: &mut Self, b: &Self) { *a -= b }
	fn inner_neg(a: &Self) -> Self { -a }
	fn inner_mul(a: &Self, b: &Self) -> Self {
		match a * b {
			Ok(ret) => ret,
			Err(e) => panic!("Error in matrix multiplication of polynomials: {:?}", e)
		}
	}
}

impl<'a, 'b:'a, T: MatrixInput + InnerOps> AddAssign<&'b MatrixView<'a, T>> for MatrixView<'a, T> {
	fn add_assign(&mut self, other: &'b MatrixView<'a, T>) {
		assert_eq!(self.shape(), other.shape());
		assert!(self.actual_rows >= other.actual_rows);
		assert!(self.actual_cols >= other.actual_cols);

		if other.m.is_none() {
			return;
		}

		for x in 0..other.actual_rows {
			for (val, add) in self.row_mut(x).zip(other.row(x)) {
				<T as InnerOps>::inner_add_assign(val, add);
			}
		}
	}
}

impl<'a, 'b:'a, T: MatrixInput + InnerOps> SubAssign<&'b MatrixView<'a, T> > for MatrixView<'a, T> {
	fn sub_assign(&mut self, other: &'b MatrixView<'a, T>) {
		assert_eq!(self.shape(), other.shape());
		assert!(self.actual_rows >= other.actual_rows);
		assert!(self.actual_cols >= other.actual_cols);

		if other.m.is_none() {
			return;
		}

		for x in 0..other.actual_rows {
			for (val, sub) in self.row_mut(x).zip(other.row(x)) {
				<T as InnerOps>::inner_sub_assign(val, sub);
			}
		}
	}
}

impl<'a, 'b:'a, T: MatrixInput + InnerOps> Neg for &'b MatrixView<'a, T> {
	type Output = MatrixView<'a, T>;

	fn neg(self) -> MatrixView<'a, T> {
		if self.m.is_none() {
			return self.clone();
		}
		let mut coefs = Vec::<T>::with_capacity(self.actual_rows * self.actual_cols);

		for x in 0..self.actual_rows {
			for val in self.row(x) {
				coefs.push(<T as InnerOps>::inner_neg(&val));
			}
		}

		let mut ret = MatrixView::<T>::new(coefs, self.actual_rows, self.actual_cols).unwrap();
		ret.rows = self.rows;
		ret.cols = self.cols;
		return ret;
	}
}

impl<'a, 'b:'a, T: MatrixInput + InnerOps> Add for &'b MatrixView<'a, T> {
	type Output = MatrixView<'a, T>;

	fn add(self, other: &'b MatrixView<'a, T>) -> MatrixView<'a, T> {
		if self.m.is_none() {
			return other.clone();
		}
		let mut ret = (*self).clone();
		ret += other;
		return ret;
	}
}

impl<'a, 'b:'a, T: MatrixInput + InnerOps> Sub for &'b MatrixView<'a, T> {
	type Output = MatrixView<'a, T>;

	fn sub(self, other: &'b MatrixView<'a, T>) -> MatrixView<'a, T> {
		if self.m.is_none() {
			return -other;
		}
		let mut ret = (*self).clone();
		ret -= other;
		return ret;
	}
}

impl<'a, 'b:'a, T: MatrixInput + InnerOps> Mul for &'b MatrixView<'a, T> {
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
				let mut coef = <T as InnerOps>::inner_mul(
					it_self.next().unwrap(), 
					it_other.next().unwrap());
				for (a,b) in it_self.zip(it_other) {
					<T as InnerOps>::inner_add_assign(
						&mut coef,
						&<T as InnerOps>::inner_mul(a,b));
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