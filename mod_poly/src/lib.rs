//! This is a custom made module for modular polynomial arithmetic.
//! It is made to be bound with Python

pub mod complex;
pub mod polynomial;

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

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

	fn __str__(&self) -> pyo3::PyResult<String> {
		return Ok(self.val.to_string());
	}
}

#[pymodule]
fn poly_arithmetic(_py: Python, m: &PyModule) -> PyResult<()> {
	m.add_class::<Complex>()?;
	// m.add_class::<PyPolynomial>()?;
	return Ok(());
}
