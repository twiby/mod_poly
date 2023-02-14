use crate::matrix::matrix_view::*;
use crate::matrix::{Number, ModularArithmeticPolynomial};
use core::ops::{AddAssign, SubAssign, Add, Sub, Mul, Neg};


pub trait InnerAddAssign {
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

pub trait InnerSubAssign {
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

pub trait InnerNeg {
	fn inner_neg(a: &Self) -> Self;
}
impl<T: Number> InnerNeg for T {
	fn inner_neg(a: &T) -> T {
		-*a
	}
}
impl<T: Number> InnerNeg for ModularArithmeticPolynomial<T> {
	fn inner_neg(a: &ModularArithmeticPolynomial<T>) -> ModularArithmeticPolynomial<T> {
		-a
	}
}

pub trait InnerMul {
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

impl<'a, 'b:'a, T: MatrixInput + InnerNeg> Neg for &'b MatrixView<'a, T> {
	type Output = MatrixView<'a, T>;

	fn neg(self) -> MatrixView<'a, T> {
		if self.m.is_none() {
			return self.clone();
		}
		let mut coefs = Vec::<T>::with_capacity(self.actual_rows * self.actual_cols);

		for x in 0..self.actual_rows {
			for val in self.row(x) {
				coefs.push(<T as InnerNeg>::inner_neg(&val));
			}
		}

		let mut ret = MatrixView::<T>::new(coefs, self.actual_rows, self.actual_cols).unwrap();
		ret.rows = self.rows;
		ret.cols = self.cols;
		return ret;
	}
}

impl<'a, 'b:'a, T: MatrixInput + InnerAddAssign> Add for &'b MatrixView<'a, T> {
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

impl<'a, 'b:'a, T: MatrixInput + InnerSubAssign + InnerNeg> Sub for &'b MatrixView<'a, T> {
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

impl<'a, 'b:'a, T: MatrixInput + InnerMul + InnerAddAssign + Default> Mul for &'b MatrixView<'a, T> {
	type Output = MatrixView<'a, T>;

	fn mul(self: &'b MatrixView<'a, T>, other_transposed: &'b MatrixView<'a, T>) -> MatrixView<'a, T> {
		assert_eq!(self.cols, other_transposed.cols);

		if self.m.is_none() || other_transposed.m.is_none() {
			return MatrixView::<T>::none((self.rows, other_transposed.rows));
		}

		let mut ret = MatrixView::<T>::new(
			vec![T::default(); self.actual_rows * other_transposed.actual_rows], 
			self.actual_rows, 
			other_transposed.actual_rows
		).unwrap();

		let target_size = next_power_of_2(
			*[self.rows, self.cols, other_transposed.rows, other_transposed.cols].iter().max().unwrap()
		);
		matrix_mult(
			self.view((0,0), (target_size, target_size)), 
			other_transposed.view((0,0), (target_size, target_size)),
			ret.writer((0,0), (target_size, target_size)));

		ret.rows = self.rows;
		ret.cols = other_transposed.rows;
		ret
	}
}

fn next_power_of_2(mut num: usize) -> usize {
	let mut val:u32 = 0;

	num=num-1;

	while val <= 4 {
		num = num | (num >> 2i32.pow(val));
		val = val +1;
	}

	num=num+1;
	return num;
}

fn matrix_mult<'a, T>(
	a: MatrixView<'a, T>, 
	b_transposed: MatrixView<'a, T>,
	mut ret: MatrixView<'a, T>)
where T: MatrixInput + InnerMul + InnerAddAssign {

 	// Ensure inputs are square matrices whose sizes are a power of 2
 	assert_eq!(a.rows, b_transposed.rows);
 	assert_eq!(a.cols, b_transposed.cols);
 	assert_eq!(a.cols, b_transposed.rows);
 	let size = a.rows;
 	assert!(!((size & (size - 1)) != 0 || size == 0));

 	if a.m.is_none() || b_transposed.m.is_none() {
		return;
	} else if size < 2 {
		naive_mult(a, b_transposed, &mut ret);
		return;
	}

 	let cut = size >> 1;

 	for &x in [0, cut].iter() {
 		for &y in [0, cut].iter() {
	 		let mut w = ret.writer((x,y), (cut,cut));
	 		if w.m.is_none() { continue; }

		 	naive_mult(
		 		a.view((x,0),(cut,cut)),
		 		b_transposed.view((y,0),(cut,cut)),
		 		&mut w);
		 	add_naive_mult(
		 		a.view((x,cut),(cut,cut)),
		 		b_transposed.view((y,cut),(cut,cut)),
		 		&mut w);
		}
 	}
 }

fn naive_mult<'a, T>(
	a: MatrixView<'a, T>, 
	b_transposed: MatrixView<'a, T>,
	ret: &mut MatrixView<'a, T>)
where T: MatrixInput + InnerMul + InnerAddAssign {
	for x in 0..a.actual_rows {
		for y in 0..b_transposed.actual_rows {
			let mut it_self = a.row(x);
			let mut it_other = b_transposed.row(y);
			let mut coef = <T as InnerMul>::inner_mul(
				it_self.next().unwrap(), 
				it_other.next().unwrap());
			for (a,b) in it_self.zip(it_other) {
				<T as InnerAddAssign>::inner_add_assign(
					&mut coef,
					&<T as InnerMul>::inner_mul(a,b));
			}
			ret[(x,y)] = coef;
		}
	}
}

fn add_naive_mult<'a, T>(
	a: MatrixView<'a, T>, 
	b_transposed: MatrixView<'a, T>,
	ret: &mut MatrixView<'a, T>)
where T: MatrixInput + InnerMul + InnerAddAssign {
	for x in 0..a.actual_rows {
		for y in 0..b_transposed.actual_rows {
			let mut it_self = a.row(x);
			let mut it_other = b_transposed.row(y);
			let mut coef = <T as InnerMul>::inner_mul(
				it_self.next().unwrap(), 
				it_other.next().unwrap());
			for (a,b) in it_self.zip(it_other) {
				<T as InnerAddAssign>::inner_add_assign(
					&mut coef,
					&<T as InnerMul>::inner_mul(a,b));
			}
			<T as InnerAddAssign>::inner_add_assign(&mut ret[(x,y)], &coef);
		}
	}
}
