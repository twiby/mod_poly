//! This is a custom made module for modular polynomial arithmetic.
//! It is made to be bound with Python

pub mod complex;
pub mod polynomial;
pub mod matrix;

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

trait ErrorTypeToString { fn str() -> String; }
fn py_value_error<ErrorType: ErrorTypeToString>(msg: &str) -> pyo3::PyErr {
	let mut string = ErrorType::str();
	string.push_str(": ");
	string.push_str(msg);
	PyErr::new::<PyValueError, _>(string)
}

impl ErrorTypeToString for polynomial::ModularArithmeticError { fn str() -> String {"ModularArithmeticError: ".to_string()} }
impl From<polynomial::ModularArithmeticError> for pyo3::PyErr {
	fn from(e: polynomial::ModularArithmeticError) -> Self {
		match e {
			polynomial::ModularArithmeticError::ModulusMismatched(s) => py_value_error::<polynomial::ModularArithmeticError>(&s)
		}
	}
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
		let coefs_complex = coefs.iter().map(|t| complex::Complex::<f64>::new(t.0, t.1)).collect::<Vec<complex::Complex<f64>>>();
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

#[pymodule]
fn poly_arithmetic(_py: Python, m: &PyModule) -> PyResult<()> {
	m.add_class::<Complex>()?;
	m.add_class::<Polynomial>()?;
	return Ok(());
}
