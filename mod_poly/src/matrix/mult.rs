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

/// Mul operation for Polynomials, which don't have the Copy trait, and thus add by reference
/// In addition, this allows catching any error coming from the modular Arithmetic module
impl<'a, T> Mul for &'a Matrix<ModularArithmeticPolynomial<T>> 
where T: Number + From<complex::Complex<f64>>, complex::Complex<f64>: From<T> {
	type Output = MatrixResult<ModularArithmeticPolynomial<T>>;

	fn mul(self, other: &'a Matrix<ModularArithmeticPolynomial<T>>) -> MatrixResult<ModularArithmeticPolynomial<T>> {
		if self.cols != other.rows {
			return Err(MatrixError::UncompatibleMatrixShapes(
				format!("Uncompatible matrix shapes for multiplication, {:?} and {:?}", self.shape(), other.shape())
			));
		}

		let other_transposed = other.clone_transposed();

		mult(&MatrixView::new(&self), &MatrixView::new(&other_transposed))
	}
}

#[derive(Copy, Clone)]
struct MatrixView<'a, T> {
	m: &'a Matrix<T>,
	cols: usize,
	rows: usize,
	x: usize,
	y: usize
}
impl<'a, T: Number> Copy for MatrixView<'a, ModularArithmeticPolynomial<T>> {}

impl<'a, T: MatrixInput> MatrixView<'a, T> {
	fn new(m: &'a Matrix<T>) -> Self {
		Self{m: m, cols: m.cols, rows: m.rows, x: 0, y: 0}
	}

	fn view(&self, block_coord: (usize, usize), block_size: (usize, usize)) -> Self {
		Self{
			m: self.m,
			rows: block_size.0,
			cols: block_size.1,
			x: self.x + block_coord.0,
			y: self.y + block_coord.1
		}
	}

	fn len(&self) -> usize {
		self.cols * self.rows
	}

	fn first(&self) -> &T {
		&self.m[(0,0)]
	}

	fn row(&self, n: usize) -> Result<std::iter::Take<Skip<std::slice::Iter<T>>>, MatrixError> {
		Ok(self.m.row(self.x + n)?.skip(self.y).take(self.cols))
	}
}

/// Add operation for Polynomials, which don't have the Copy trait, and thus add by reference
/// In addition, this allows catching any error coming from the modular Arithmetic module
impl<'b, T: Number> Add for MatrixView<'b, ModularArithmeticPolynomial<T>> {
	type Output = MatrixResult<ModularArithmeticPolynomial<T>>;

	fn add(self, other: MatrixView<'b, ModularArithmeticPolynomial<T>>) -> MatrixResult<ModularArithmeticPolynomial<T>> {
		let mut vec = Vec::<ModularArithmeticPolynomial<T>>::with_capacity(self.len());

		for x in 0..self.rows {
			for (a,b) in self.row(x)?.zip(other.row(x)?) {
				vec.push((a + b)?);
			}
		}
		Matrix::<ModularArithmeticPolynomial<T>>::new(vec, self.rows, self.cols)
	}
}
/// Sub operation for Polynomials, which don't have the Copy trait, and thus add by reference
/// In addition, this allows catching any error coming from the modular Arithmetic module
impl<'b, T: Number> Sub for MatrixView<'b, ModularArithmeticPolynomial<T>> {
	type Output = MatrixResult<ModularArithmeticPolynomial<T>>;

	fn sub(self, other: MatrixView<'b, ModularArithmeticPolynomial<T>>) -> MatrixResult<ModularArithmeticPolynomial<T>> {
		let mut vec = Vec::<ModularArithmeticPolynomial<T>>::with_capacity(self.len());

		for x in 0..self.rows {
			for (a,b) in self.row(x)?.zip(other.row(x)?) {
				vec.push((a - b)?);
			}
		}
		Matrix::<ModularArithmeticPolynomial<T>>::new(vec, self.rows, self.cols)
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

fn naive_mult<'a, T>(a: &MatrixView<'a, ModularArithmeticPolynomial<T>>, b: &MatrixView<'a, ModularArithmeticPolynomial<T>>)
->MatrixResult<ModularArithmeticPolynomial<T>>
where T: Number + From<complex::Complex<f64>>, complex::Complex<f64>: From<T> {
	let modulus = a.first().modulus();
	let mut coefs = Vec::<ModularArithmeticPolynomial<T>>::with_capacity(a.rows * b.rows);

	for x in 0..a.rows {
		for y in 0..b.rows {
			let mut coef = ModularArithmeticPolynomial::<T>::new_zero(modulus);
			for (a,b) in a.row(x)?.zip(b.row(y)?) {
				coef += &(a * b)?;
			}
			coefs.push(coef);
		}
	}
	
	return Matrix::<ModularArithmeticPolynomial<T>>::new(coefs, a.rows, b.rows);
}

fn stressen_mult<'a, T>(a: &MatrixView<'a, ModularArithmeticPolynomial<T>>, b: &MatrixView<'a, ModularArithmeticPolynomial<T>>)
->MatrixResult<ModularArithmeticPolynomial<T>>
where T: Number + From<complex::Complex<f64>>, complex::Complex<f64>: From<T> {
	assert_eq!(a.cols, a.rows);
	assert_eq!(b.cols, b.rows);
	assert_eq!(a.cols, b.rows);

	let size = a.rows;

	// Size must be a power of 2 every step of the way
	if (size & (size - 1)) != 0 {
		panic!("Stressen: size is not a power of 2");
	}

	if size < 3 {
		return naive_mult(&a, &b);
	}

	let cut = size >> 1;

	let a_00 = a.view((0,0), (cut, cut));
	let a_01 = a.view((0,cut), (cut, cut));
	let a_10 = a.view((cut,0), (cut, cut));
	let a_11 = a.view((cut, cut), (cut, cut));

	let b_00 = b.view((0,0), (cut, cut));
	let b_01 = b.view((0, cut), (cut, cut));
	let b_10 = b.view((cut, 0), (cut, cut));
	let b_11 = b.view((cut, cut), (cut, cut));

	let m1 = stressen_mult(&MatrixView::new(&(a_00 + a_11)?), &MatrixView::new(&(b_00 + b_11)?))?;
	let m2 = stressen_mult(&MatrixView::new(&(a_10 + a_11)?), &b_00)?;
	let m3 = stressen_mult(&a_00, &MatrixView::new(&(b_10 - b_11)?))?;

	let mut c_00 = stressen_mult(&MatrixView::new(&(a_01 - a_11)?), &MatrixView::new(&(b_01 + b_11)?))?;
	let mut c_10 = stressen_mult(&a_11, &MatrixView::new(&(b_01 - b_00)?))?;
	let mut c_01 = stressen_mult(&MatrixView::new(&(a_00 + a_01)?), &b_11)?;
	let mut c_11 = stressen_mult(&MatrixView::new(&(a_10 - a_00)?), &MatrixView::new(&(b_00 + b_10)?))?;

	c_00 += &m1;
	c_00 += &c_10;
	c_00 -= &c_01;

	c_01 += &m3;
	c_10 += &m2;

	c_11 += &m1;
	c_11 -= &m2;
	c_11 += &m3;

	let modulus = a.first().modulus();
	let mut coefs = vec![ModularArithmeticPolynomial::<T>::new_zero(modulus) ; size * size];
	for x in 0..cut {
		let mut idx = x*size;
		for val in c_00.row_mut(x)? {
			std::mem::swap(&mut coefs[idx], val);
			idx += 1;

		}
		for val in c_01.row_mut(x)? {
			std::mem::swap(&mut coefs[idx], val);
			idx += 1;
		}
	}
	for x in cut..size {
		let mut idx = x*size;
		for val in c_10.row_mut(x-cut)? {
			std::mem::swap(&mut coefs[idx], val);
			idx += 1;

		}
		for val in c_11.row_mut(x-cut)? {
			std::mem::swap(&mut coefs[idx], val);
			idx += 1;
		}
	}

	Matrix::<ModularArithmeticPolynomial<T>>::new(coefs, size, size)
}

fn mult<'a, T>(a: &MatrixView<'a, ModularArithmeticPolynomial<T>>, b: &MatrixView<'a, ModularArithmeticPolynomial<T>>) 
->MatrixResult<ModularArithmeticPolynomial<T>>
where T: Number + From<complex::Complex<f64>>, complex::Complex<f64>: From<T> {
	let rows = a.rows;
	let cols = b.rows;
	let inner = a.cols;

	if rows == 1 || cols == 1 || inner == 1 {
		return naive_mult(a, b);
	}

	let rows_cut = next_power_of_2(rows) >> 1;
	let cols_cut = next_power_of_2(cols) >> 1;
	let inner_cut = next_power_of_2(inner) >> 1;
	let &cut = [rows_cut, cols_cut, inner_cut].iter().min().unwrap();

	let a_00 = a.view((0,0), (cut, cut));
	let a_01 = a.view((0,cut), (cut, inner - cut));
	let a_10 = a.view((cut,0), (rows-cut, cut));
	let a_11 = a.view((cut, cut), (rows-cut, inner-cut));

	let b_00 = b.view((0,0), (cut, cut));
	let b_01 = b.view((0, cut), (cut, inner - cut));
	let b_10 = b.view((cut, 0), (cols-cut, cut));
	let b_11 = b.view((cut, cut), (cols-cut, inner-cut));

	let mut c_00 = (&stressen_mult(&a_00, &b_00)? + &mult(&a_01, &b_01)?)?;
	let mut c_01 = (&mult(&a_00, &b_10)? + &mult(&a_01, &b_11)?)?;
	let mut c_10 = (&mult(&a_10, &b_00)? + &mult(&a_11, &b_01)?)?;
	let mut c_11 = (&mult(&a_10, &b_10)? + &mult(&a_11, &b_11)?)?;

	let modulus = a.first().modulus();
	let mut coefs = vec![ModularArithmeticPolynomial::<T>::new_zero(modulus) ; rows * cols];
	for x in 0..cut {
		let mut idx = x*cols;
		for val in c_00.row_mut(x)? {
			std::mem::swap(&mut coefs[idx], val);
			idx += 1;

		}
		for val in c_01.row_mut(x)? {
			std::mem::swap(&mut coefs[idx], val);
			idx += 1;
		}
	}
	for x in cut..rows {
		let mut idx = x*cols;
		for val in c_10.row_mut(x-cut)? {
			std::mem::swap(&mut coefs[idx], val);
			idx += 1;

		}
		for val in c_11.row_mut(x-cut)? {
			std::mem::swap(&mut coefs[idx], val);
			idx += 1;
		}
	}

	Matrix::<ModularArithmeticPolynomial<T>>::new(coefs, rows, cols)
}

#[test]
fn matrix_view() {
	let m = Matrix::<f32>::new(vec![0.0, 1.0, 2.0, 0.0, 1.0, 2.0], 2, 3).unwrap();

	let view_1 = MatrixView::new(&m);
	assert_eq!(view_1.row(0).unwrap().copied().collect::<Vec<f32>>(), vec![0.0, 1.0, 2.0]);
	assert_eq!(view_1.row(1).unwrap().copied().collect::<Vec<f32>>(), vec![0.0, 1.0, 2.0]);
	assert_eq!(view_1.row(0).unwrap().copied().collect::<Vec<f32>>(), vec![0.0, 1.0, 2.0]);
	assert_eq!(view_1.row(1).unwrap().copied().collect::<Vec<f32>>(), vec![0.0, 1.0, 2.0]);

	let view_2 = view_1.view((0,0), (2,2));
	assert_eq!(view_2.row(0).unwrap().copied().collect::<Vec<f32>>(), vec![0.0, 1.0]);
	assert_eq!(view_2.row(1).unwrap().copied().collect::<Vec<f32>>(), vec![0.0, 1.0]);

	let view_3 = view_1.view((1,1), (2,2));
	assert_eq!(view_3.row(0).unwrap().copied().collect::<Vec<f32>>(), vec![1.0, 2.0]);
	match view_3.row(1) {
		Err(MatrixError::OutOfBoundsIndex(_)) => (),
		_ => panic!("Wrong error type")
	}
}
