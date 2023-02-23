use crate::matrix::matrix_view::assigner::MatrixBinaryOperand;
use crate::matrix::matrix_view::assigner::MatrixBinaryOperation;
use crate::matrix::matrix_view::assigner::MatrixBinaryOperator;
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

pub struct MatrixMultiplicator {}
impl<T1, T2> MatrixBinaryOperator<T1, T2> for MatrixMultiplicator
where T1: MatrixBinaryOperand, T2: MatrixBinaryOperand<Data = T1::Data>, T1::Data: MatrixInput + InnerOps {
	type Data = T1::Data;
	fn shape(a: &T1, b: &T2) -> (usize, usize) {
		(a.shape().0, b.shape().0)
	}
	fn actual_shape(a: &T1, b: &T2) -> (usize, usize) {
		if a.actual_shape() == (0,0) || b.actual_shape() == (0,0) {
			return (0,0);
		}
		(a.actual_shape().0, b.actual_shape().0)
	}
	fn coef<'b>(a: &T1, b: &T2, x: usize, y: usize) -> Self::Data {
		let mut coef = <Self::Data as InnerOps>::inner_mul(
			&a.coef(x,0), 
			&b.coef(y,0));
		for i in 1..*[a.actual_shape().1, b.actual_shape().1].iter().min().unwrap() {
			<Self::Data as InnerOps>::inner_add_assign(
				&mut coef,
				&<Self::Data as InnerOps>::inner_mul(&a.coef(x,i),&b.coef(y,i)));
		}
		coef
	}
}

pub struct MatrixAdditionner {}
impl<T1, T2> MatrixBinaryOperator<T1, T2> for MatrixAdditionner
where T1: MatrixBinaryOperand, T2: MatrixBinaryOperand<Data = T1::Data>, T1::Data: MatrixInput + InnerOps {
	type Data = T1::Data;
	fn shape(a: &T1, _: &T2) -> (usize, usize) {
		a.shape()
	}
	fn actual_shape(a: &T1, _: &T2) -> (usize, usize) {
		a.actual_shape()
	}
	fn coef<'b>(a: &T1, b: &T2, x: usize, y: usize) -> Self::Data {
		let mut ret = a.coef(x,y).clone();
		Self::Data::inner_add_assign(&mut ret, &b.coef(x,y));
		ret
	}
}

impl<'a, T1, T> Mul<T1> for MatrixView<'a, T> 
where T: MatrixInput + InnerOps, T1: MatrixBinaryOperand<Data = T> {
	type Output = MatrixBinaryOperation<MatrixView<'a, T>, T1, MatrixMultiplicator>;

	fn mul(self: MatrixView<'a, T>, other_transposed: T1) -> Self::Output {
		assert_eq!(self.cols, other_transposed.shape().1);
		return Self::Output::new(self, other_transposed);
	}
}
impl<T, T1, T2, Op> Mul<T> for MatrixBinaryOperation<T1, T2, Op> 
where 
	T: MatrixBinaryOperand<Data = Op::Data>, 
	T1: MatrixBinaryOperand, T2: MatrixBinaryOperand, 
	Op: MatrixBinaryOperator<T1, T2>, 
	Op::Data: MatrixInput + InnerOps {
	type Output = MatrixBinaryOperation<MatrixBinaryOperation<T1, T2, Op>, T, MatrixMultiplicator>;

	fn mul(self: MatrixBinaryOperation<T1, T2, Op>, other_transposed: T) -> Self::Output {
		assert_eq!(self.shape().1, other_transposed.shape().1);
		return Self::Output::new(self, other_transposed);
	}
}