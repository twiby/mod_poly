//! This is a custom made module for modular polynomial arithmetic.
//! It is made to be bound with Python
//!
//! In the lib.rs file are all utilities necessary for Python binding, as we don't
//! want them to spill over to the rest of the code

pub mod complex;
pub mod polynomial;
pub mod matrix;

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

/// This trait allows to translate the name of a rust error
trait ErrorTypeToString { fn str() -> String; }

/// Converts a Rust error to python ValueError
fn py_value_error<ErrorType: ErrorTypeToString>(msg: &str) -> pyo3::PyErr {
	let mut string = ErrorType::str();
	string.push_str(": ");
	string.push_str(msg);
	PyErr::new::<PyValueError, _>(string)
}

/// Translate ModularArithmeticError name
impl ErrorTypeToString for polynomial::ModularArithmeticError { fn str() -> String {"ModularArithmeticError: ".to_string()} }
/// Immplementing the From trait for errors Rust -> Python
impl From<polynomial::ModularArithmeticError> for pyo3::PyErr {
	fn from(e: polynomial::ModularArithmeticError) -> Self {
		match e {
			polynomial::ModularArithmeticError::ModulusMismatched(s) => py_value_error::<polynomial::ModularArithmeticError>(&s)
		}
	}
}

/// Translate ModularArithmeticError name
impl ErrorTypeToString for matrix::MatrixError { fn str() -> String {"MatrixError: ".to_string()} }
/// Immplementing the From trait for errors Rust -> Python
impl From<matrix::MatrixError> for pyo3::PyErr {
	fn from(e: matrix::MatrixError) -> Self {
		match e {
			matrix::MatrixError::ZeroDimension(s) => py_value_error::<matrix::MatrixError>(&s),
			matrix::MatrixError::WrongInputArraySize(s) => py_value_error::<matrix::MatrixError>(&s),
			matrix::MatrixError::UncompatibleMatrixShapes(s) => py_value_error::<matrix::MatrixError>(&s),
			matrix::MatrixError::ModularArithmeticError(s) => pyo3::PyErr::from(s)
		}
	}
}

fn to_complex_vector<T: complex::RealNumber>(vec: &[(T, T)]) -> Vec<complex::Complex<T>> {
		vec.iter().map(|t| complex::Complex::<T>::new(t.0, t.1)).collect::<Vec<complex::Complex<T>>>()
}

#[pyclass]
struct Complex {
	val: complex::Complex<f64>
}
#[pymethods]
impl Complex {
	#[new]
	fn new(r: f64, i: f64) -> PyResult<Self> {
		Ok(Self{val: complex::Complex::<f64>::new(r, i)})
	}

	fn __add__(&self, other: &Self) -> PyResult<Self> {
		Ok(Self{val: self.val + other.val})
	}
	fn __mul__(&self, other: &Self) -> PyResult<Self> {
		Ok(Self{val: self.val * other.val})
	}

	fn __str__(&self) -> PyResult<String> {
		return Ok(self.val.to_string());
	}
}

#[pyclass]
struct Polynomial {
	val: polynomial::ModularArithmeticPolynomial<complex::Complex<f64>>
}
#[pymethods]
impl Polynomial {
	#[new]
	fn new(coefs: Vec<(f64, f64)>, modulus: usize) -> PyResult<Self> {
		let coefs_complex = to_complex_vector(&coefs);
		Ok(Self{val: polynomial::ModularArithmeticPolynomial::<complex::Complex<f64>>::new(
			&polynomial::Polynomial::<complex::Complex<f64>>::new(&coefs_complex),
			modulus
		)})
	}

	fn __add__(&self, other: &Self) -> PyResult<Self> {
		Ok(Self{val: (&self.val + &other.val)?})
	}
	fn __mul__(&self, other: &Self) -> PyResult<Self> {
		Ok(Self{val: (&self.val * &other.val)?})
	}

	fn __str__(&self) -> PyResult<String> {
		return Ok(self.val.to_string());
	}
}

#[pyclass]
struct Matrix {
	val: matrix::Matrix<complex::Complex<f64>>
}
#[pymethods]
impl Matrix {
	#[new]
	fn new(values: Vec<(f64, f64)>, rows: usize, cols: usize) -> PyResult<Self> {
		let values_complex = to_complex_vector(&values);
		Ok(Self{
			val: matrix::Matrix::new(&values_complex, rows, cols)?
		})
	}

	fn __add__(&self, other: &Self) -> PyResult<Self> {
		Ok(Self{val: (&self.val + &other.val)?})
	}
	fn __mul__(&self, other: &Self) -> PyResult<Self> {
		Ok(Self{val: (&self.val * &other.val)?})
	}

	fn __str__(&self) -> PyResult<String> {
		return Ok(self.val.to_string());
	}
}

#[pymodule]
fn poly_arithmetic(_py: Python, m: &PyModule) -> PyResult<()> {
	m.add_class::<Complex>()?;
	m.add_class::<Polynomial>()?;
	m.add_class::<Matrix>()?;
	return Ok(());
}
