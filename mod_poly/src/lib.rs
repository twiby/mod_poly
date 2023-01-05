//! This is a custom made module for modular polynomial arithmetic.
//! It is made to be bound with Python or to be used as a crate
//!
//! In the py_bindings module are all utilities necessary for Python binding, as we don't
//! want them to spill over to the rest of the code

pub mod complex;
pub mod polynomial;
pub mod matrix;
pub mod py_bindings;

use pyo3::prelude::*;

#[pymodule]
fn poly_arithmetic(_py: Python, m: &PyModule) -> PyResult<()> {
	m.add_class::<py_bindings::types::Complex>()?;
	m.add_class::<py_bindings::types::Polynomial>()?;
	m.add_class::<py_bindings::types::Matrix>()?;
	m.add_class::<py_bindings::types::PolynomialMatrix>()?;
	return Ok(());
}
