use core::marker::PhantomData;
use crate::matrix::MatrixInput;
use crate::matrix::matrix_view::ops::InnerOps;
use crate::matrix::matrix_view::MatrixView;

pub trait MatrixBinaryOperator<LhsType, RhsType> {
	type Data;
	fn shape(a: &LhsType, b: &RhsType) -> (usize, usize);
	fn actual_shape(a: &LhsType, b: &RhsType) -> (usize, usize);
	fn coef(a: &LhsType, b: &RhsType, x: usize, y: usize) -> Self::Data;
}

pub trait MatrixBinaryOperand {
	type Data;
	fn shape(&self) -> (usize, usize);
	fn actual_shape(&self) -> (usize, usize);
	fn coef(&self, x: usize, y: usize) -> Self::Data;
}

pub struct MatrixBinaryOperation<T1, T2, Op> {
	a: T1,
	b: T2,
	op: PhantomData<Op>,
	shape: (usize, usize),
	actual_shape: (usize, usize)
}

impl<T1, T2, Op> MatrixBinaryOperation<T1, T2, Op> 
where Op: MatrixBinaryOperator<T1, T2>, Op::Data: MatrixInput + InnerOps {
	pub fn new(a: T1, b: T2) -> MatrixBinaryOperation<T1, T2, Op> {
		MatrixBinaryOperation{
			op: PhantomData::<Op>,
			shape: Op::shape(&a, &b),
			actual_shape: Op::actual_shape(&a, &b),
			a: a, 
			b: b, 
		}
	}

	pub fn make<'a>(&self) -> MatrixView<'a, Op::Data> {
		let shape = self.shape();
		let actual_shape = self.actual_shape();

		if actual_shape == (0,0) {
			return MatrixView::<Op::Data>::none(shape);
		}

		let mut ret = MatrixView::<Op::Data>::new(
			vec![Op::Data::default(); actual_shape.0 * actual_shape.1],
			actual_shape.0,
			actual_shape.1
		).unwrap();

		ret.rows = shape.0;
		ret.cols = shape.1;

		self.set(&mut ret);
		ret
	}

	fn set<'b>(&self, mat: &mut MatrixView<'b, Op::Data>) {
		for x in 0..mat.actual_rows {
			for y in 0..mat.actual_cols {
				mat[(x,y)] = self.coef(x, y);
			}
		}
	}
}

impl<'a, T: MatrixInput + InnerOps> MatrixView<'a, T> {
	pub fn set<T1, T2, Op: MatrixBinaryOperator<T1, T2, Data = T>>(&mut self, other: MatrixBinaryOperation<T1, T2, Op>) {
		other.set(self);
	}
}
impl<'a, T: MatrixInput + InnerOps> MatrixBinaryOperand for MatrixView<'a, T> {
	type Data = T;
	fn shape(&self) -> (usize, usize) { (self.rows, self.cols) }
	fn actual_shape(&self) -> (usize, usize) { (self.actual_rows, self.actual_cols) }
	fn coef(&self, x: usize, y:usize) -> T { self[(x,y)].clone() }
}

impl<T1, T2, Op> MatrixBinaryOperand for MatrixBinaryOperation<T1, T2, Op>
where Op: MatrixBinaryOperator<T1, T2>, Op::Data: MatrixInput + InnerOps {
	type Data = Op::Data;
	fn shape(&self) -> (usize, usize) { self.shape }
	fn actual_shape(&self) -> (usize, usize) { self.actual_shape }
	fn coef(&self, x:usize, y:usize) -> Self::Data { Op::coef(&self.a, &self.b, x, y) }
}