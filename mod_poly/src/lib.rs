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

fn to_internal_vector<Output, Input>(vec: &[Input]) -> Vec<Output> 
where Output: From<Input>, Input: Clone {
		vec.iter().map(|t| Output::from(t.clone())).collect::<Vec<Output>>()
}

#[pyclass]
#[derive(Clone, Copy)]
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

	fn __getitem__(&self, n: usize) -> PyResult<f64> {
		match n {
			0 => Ok(self.val.real()),
			1 => Ok(self.val.imag()),
			_ => Err(PyErr::new::<PyValueError, _>(format!("Index too high for a complex number: {}", n)))
		}
	}
	fn __setitem__(&mut self, n: usize, val: f64) -> PyResult<()> {
		match n {
			0 => Ok(*self.val.real_mut() = val),
			1 => Ok(*self.val.imag_mut() = val),
			_ => Err(PyErr::new::<PyValueError, _>(format!("Index too high for a complex number: {}", n)))
		}
	}

	fn __str__(&self) -> PyResult<String> {
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

#[pyclass]
#[derive(Clone)]
struct Polynomial {
	val: polynomial::ModularArithmeticPolynomial<complex::Complex<f64>>
}
#[pymethods]
impl Polynomial {
	#[new]
	fn new(coefs: Vec<Complex>, modulus: usize) -> PyResult<Self> {
		let coefs_complex = to_internal_vector(&coefs);
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

	fn __getitem__(&self, c: usize) -> PyResult<Complex> {
		Ok(Complex::from(self.val.coef(c)?))
	}
	fn __setitem__(&mut self, c:usize, val: Complex) -> PyResult<()> {
		Ok(*self.val.coef_mut(c)? = complex::Complex::from(val))
	}

	fn __str__(&self) -> PyResult<String> {
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

#[pyclass]
struct Matrix {
	val: matrix::Matrix<complex::Complex<f64>>
}
#[pymethods]
impl Matrix {
	#[new]
	fn new(values: Vec<Complex>, rows: usize, cols: usize) -> PyResult<Self> {
		let values_complex = to_internal_vector(&values);
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

	fn __getitem__(&self, t: (usize, usize)) -> PyResult<Complex> {
		self.val.check_idx(t.0, t.1)?;
		Ok(Complex::from(self.val[t]))
	}
	fn __setitem__(&mut self, t: (usize, usize), c: Complex) -> PyResult<()> {
		self.val.check_idx(t.0, t.1)?;
		Ok(self.val[t] = complex::Complex::<f64>::from(c))
	}

	fn __str__(&self) -> PyResult<String> {
		return Ok(self.val.to_string());
	}
}

#[pyclass]
struct PolynomialMatrix {
	val: matrix::Matrix<polynomial::ModularArithmeticPolynomial<complex::Complex<f64>>>
}
#[pymethods]
impl PolynomialMatrix {
	#[new]
	fn new(values: Vec<Polynomial>, rows: usize, cols: usize) -> PyResult<Self> {
		let values_complex = to_internal_vector(&values);
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

	fn __getitem__(&self, t: (usize, usize)) -> PyResult<Polynomial> {
		self.val.check_idx(t.0, t.1)?;
		Ok(Polynomial::from(self.val[t].clone()))
	}
	fn __setitem__(&mut self, t: (usize, usize), p: Polynomial) -> PyResult<()> {
		self.val.check_idx(t.0, t.1)?;
		Ok(self.val[t] = polynomial::ModularArithmeticPolynomial::<complex::Complex<f64>>::from(p))
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
	m.add_class::<PolynomialMatrix>()?;
	return Ok(());
}
