//! This module contains all the bindings for types and methods accessible in Python. Treatment of errors is in another, private module.
//!
//! This module makes 4 types accessible to Python, with basic arithmetic, printing, and get/set for all elements.

use crate::py_bindings::complex;
use crate::py_bindings::polynomial;
use crate::py_bindings::matrix;

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

fn to_internal_vector<Output, Input>(vec: &[Input]) -> Vec<Output> 
where Output: From<Input>, Input: Clone {
		vec.iter().map(|t| Output::from(t.clone())).collect::<Vec<Output>>()
}

/// Type representing complex numbers, made out of 2 float 64
#[pyclass]
#[derive(Clone, Copy)]
pub struct Complex {
	val: complex::Complex<f64>
}
#[pymethods]
impl Complex {
	/// Constructor from 2 float 64
	#[new]
	pub fn new(r: f64, i: f64) -> PyResult<Self> {
		Ok(Self{val: complex::Complex::<f64>::new(r, i)})
	}

	/// Binding of addition
	pub fn __add__(&self, other: &Self) -> PyResult<Self> {
		Ok(Self{val: self.val + other.val})
	}
	/// Binding of multiplication
	pub fn __mul__(&self, other: &Self) -> PyResult<Self> {
		Ok(Self{val: self.val * other.val})
	}

	/// Getter, via index, for real part or imaginary part
	pub fn __getitem__(&self, n: usize) -> PyResult<f64> {
		match n {
			0 => Ok(self.val.real()),
			1 => Ok(self.val.imag()),
			_ => Err(PyErr::new::<PyValueError, _>(format!("Index too high for a complex number: {}", n)))
		}
	}
	/// Setter, via index, for real part or imaginary part
	pub fn __setitem__(&mut self, n: usize, val: f64) -> PyResult<()> {
		match n {
			0 => Ok(*self.val.real_mut() = val),
			1 => Ok(*self.val.imag_mut() = val),
			_ => Err(PyErr::new::<PyValueError, _>(format!("Index too high for a complex number: {}", n)))
		}
	}

	/// Bind string conversion for Python
	pub fn __str__(&self) -> PyResult<String> {
		return Ok(self.val.to_string());
	}
}
impl From<complex::Complex<f64>> for Complex {
	fn from(c: complex::Complex<f64>) -> Self {
		Self{val: c}
	}
}
impl From<Complex> for complex::Complex<f64> {
	fn from(c: Complex) -> Self {
		c.val
	}
}

/// Type representing a polynomial in a modular ring. 
///
/// If too many coefficients are provided, the polynomial will be recomputes according to ring rules.
#[pyclass]
#[derive(Clone)]
pub struct Polynomial {
	val: polynomial::ModularArithmeticPolynomial<complex::Complex<f64>>
}
#[pymethods]
impl Polynomial {
	/// Constructor from a vector of Complex and a ring modulus
	#[new]
	pub fn new(coefs: Vec<Complex>, modulus: usize) -> PyResult<Self> {
		let coefs_complex = to_internal_vector(&coefs);
		Ok(Self{val: polynomial::ModularArithmeticPolynomial::<complex::Complex<f64>>::new(
			&polynomial::Polynomial::<complex::Complex<f64>>::new(&coefs_complex),
			modulus
		)})
	}

	/// Binding of addition
	pub fn __add__(&self, other: &Self) -> PyResult<Self> {
		Ok(Self{val: (&self.val + &other.val)?})
	}
	/// Binding of multiplication
	pub fn __mul__(&self, other: &Self) -> PyResult<Self> {
		Ok(Self{val: (&self.val * &other.val)?})
	}

	/// Getter, via index, for any coefficient
	pub fn __getitem__(&self, c: usize) -> PyResult<Complex> {
		Ok(Complex::from(self.val.coef(c)?))
	}
	/// Setter, via index, for any coefficient (sub indexing will not work)
	pub fn __setitem__(&mut self, c:usize, val: Complex) -> PyResult<()> {
		Ok(*self.val.coef_mut(c)? = complex::Complex::from(val))
	}

	/// Bind string conversion for Python
	pub fn __str__(&self) -> PyResult<String> {
		return Ok(self.val.to_string());
	}
}
impl From<polynomial::ModularArithmeticPolynomial<complex::Complex<f64>>> for Polynomial {
	fn from(c: polynomial::ModularArithmeticPolynomial<complex::Complex<f64>>) -> Self {
		Self{val: c}
	}
}
impl From<Polynomial> for polynomial::ModularArithmeticPolynomial<complex::Complex<f64>> {
	fn from(c: Polynomial) -> Self {
		c.val
	}
}

/// Type representing a Matrix of complex numbers
#[pyclass]
pub struct Matrix {
	val: matrix::Matrix<complex::Complex<f64>>
}
#[pymethods]
impl Matrix {
	#[new]
	/// Constructor from a vector of Complex and a shape
	pub fn new(values: Vec<Complex>, rows: usize, cols: usize) -> PyResult<Self> {
		let values_complex = to_internal_vector(&values);
		Ok(Self{
			val: matrix::Matrix::new(values_complex, rows, cols)?
		})
	}

	/// Binding of addition
	pub fn __add__(&self, other: &Self) -> PyResult<Self> {
		Ok(Self{val: (&self.val + &other.val)?})
	}
	/// Binding of multiplication
	pub fn __mul__(&self, other: &Self) -> PyResult<Self> {
		Ok(Self{val: (&self.val * &other.val)?})
	}

	/// Getter, via index, for any coefficient
	pub fn __getitem__(&self, t: (usize, usize)) -> PyResult<Complex> {
		self.val.check_idx(t.0, t.1)?;
		Ok(Complex::from(self.val[t]))
	}
	/// Setter, via index, for any coefficient
	pub fn __setitem__(&mut self, t: (usize, usize), c: Complex) -> PyResult<()> {
		self.val.check_idx(t.0, t.1)?;
		Ok(self.val[t] = complex::Complex::<f64>::from(c))
	}

	/// Bind string conversion for Python
	pub fn __str__(&self) -> PyResult<String> {
		return Ok(self.val.to_string());
	}
}

/// Type representing a matrix of polynomials
#[pyclass]
pub struct PolynomialMatrix {
	val: matrix::Matrix<polynomial::ModularArithmeticPolynomial<complex::Complex<f64>>>
}
#[pymethods]
impl PolynomialMatrix {
	#[new]
	/// Constructor from a vector of Polynomial and a shape
	pub fn new(values: Vec<Polynomial>, rows: usize, cols: usize) -> PyResult<Self> {
		let values_complex = to_internal_vector(&values);
		Ok(Self{
			val: matrix::Matrix::new(values_complex, rows, cols)?
		})
	}

	/// Binding of addition
	pub fn __add__(&self, other: &Self) -> PyResult<Self> {
		Ok(Self{val: (&self.val + &other.val)?})
	}
	/// Binding of multiplication
	pub fn __mul__(&self, other: &Self) -> PyResult<Self> {
		Ok(Self{val: (&self.val * &other.val)?})
	}

	/// Getter, via index, for any coefficient
	pub fn __getitem__(&self, t: (usize, usize)) -> PyResult<Polynomial> {
		self.val.check_idx(t.0, t.1)?;
		Ok(Polynomial::from(self.val[t].clone()))
	}
	/// Setter, via index, for any coefficient
	pub fn __setitem__(&mut self, t: (usize, usize), p: Polynomial) -> PyResult<()> {
		self.val.check_idx(t.0, t.1)?;
		Ok(self.val[t] = polynomial::ModularArithmeticPolynomial::<complex::Complex<f64>>::from(p))
	}

	/// Bind string conversion for Python
	pub fn __str__(&self) -> PyResult<String> {
		return Ok(self.val.to_string());
	}
}
