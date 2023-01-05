//! This module contains facilities to translate Rust error types into python error types. 
//! Right now all types are converted to a Pyuthon ValueError, but with a string explaining the error. 
//! In the future maybe, we can have specific error types accessible in Python.

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

use crate::py_bindings::polynomial;
use crate::py_bindings::matrix;

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
			polynomial::ModularArithmeticError::ModulusMismatched(s) => py_value_error::<polynomial::ModularArithmeticError>(&s),
			polynomial::ModularArithmeticError::DegreeAboveModulus(s) => py_value_error::<polynomial::ModularArithmeticError>(&s)
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
			matrix::MatrixError::OutOfBoundsIndex(s) => py_value_error::<matrix::MatrixError>(&s),
			matrix::MatrixError::ModularArithmeticError(s) => pyo3::PyErr::from(s)
		}
	}
}
