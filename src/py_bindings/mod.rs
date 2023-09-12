//! This module declares all data types and error types necessary for python bindings

use crate::complex;
use crate::polynomial;
use crate::matrix;

mod errors;
pub mod types;

use pyo3::prelude::*;

#[pymodule]
fn poly_arithmetic(_py: Python, m: &PyModule) -> PyResult<()> {
	m.add_class::<types::Complex>()?;
	m.add_class::<types::Polynomial>()?;
	m.add_class::<types::Matrix>()?;
	m.add_class::<types::PolynomialMatrix>()?;
	return Ok(());
}
