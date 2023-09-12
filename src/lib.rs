//! This is a custom made module for modular polynomial arithmetic.
//! It is made to be bound with Python or to be used as a crate
//!
//! In the py_bindings module are all utilities necessary for Python binding, as we don't
//! want them to spill over to the rest of the code

pub mod complex;
pub mod matrix;
pub mod polynomial;

#[cfg(feature = "pyo3")]
pub mod py_bindings;
