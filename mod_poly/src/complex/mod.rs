//! This module implements some of the basic arithmetic operations on complex numbers.
//! The architecture allows building complex numbers out of any numerical underlying type.
//! As of now, it is only accessible for f32 and f64 representations.

#[cfg(test)]
mod test;

use std::ops::{Add, Mul, Sub, AddAssign, MulAssign};

/// number i, on a f32 representation
pub const I_F32: Complex::<f32> = Complex{r: 0.0, i: 1.0};
/// number i, on a f64 representation
pub const I_F64: Complex::<f64> = Complex{r: 0.0, i: 1.0};

/// Custom trait for what can be a number (real or complex)
pub trait Number: 
	Copy + std::fmt::Debug + std::fmt::Display + PartialEq + From<f32> + 
	AddAssign + MulAssign + Add<Output = Self> + Mul<Output = Self> {}
/// Custom trait for what can be a real number
pub trait RealNumber: 
	Copy + std::fmt::Debug + std::fmt::Display + PartialEq + From<f32> +  
	AddAssign + Add<Output = Self> + Mul<Output = Self> + Sub<Output = Self> {}

impl Number for f32 {}
impl Number for f64 {}
impl RealNumber for f32 {}
impl RealNumber for f64 {}

/// Type representing complex numbers. 
/// It depends on a generic parameter which represents real part and imaginary part. 
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Complex<T: RealNumber> {
	r: T,
	i: T
}

impl<T: RealNumber> Complex<T> {
	pub fn new(real: T, imag: T) -> Self {
		Self{r: real, i: imag}
	}

	#[allow(dead_code)]
	pub fn dot(self, other: Complex<T>) -> Self 
	where T: std::ops::Mul<T, Output= T> {
		Self{r: self.r * other.r, i: self.i * other.i}
	}
}

/// Implement the Display trait
impl<T: RealNumber> std::fmt::Display for Complex<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut ret = self.r.to_string();
		ret.push_str(" + ");
		ret.push_str(&self.i.to_string());
		ret.push_str("i");
		f.write_str(&ret)
	}
}

/// Constructor from tuple
impl<T: RealNumber> From<(T, T)> for Complex<T> {
	fn from(t: (T, T)) -> Self {
		Self::new(t.0, t.1)
	}
}

/// Constructor from array (size at least 2)
impl<T: RealNumber> From<&[T]> for Complex<T> {
	fn from(t: &[T]) -> Self {
		assert!(t.len() > 1);
		Self::new(t[0], t[1])
	}
}

/// Constructor from f32
impl<T: RealNumber> From<f32> for Complex<T> {
	fn from(t: f32) -> Self {
		Self::new(T::from(t), T::from(0.0))
	}
}

impl<T: RealNumber> Add for Complex<T> {
	type Output = Complex<T>;

	fn add(self, other: Complex<T>) -> Self {
		Self{r: self.r + other.r, i: self.i + other.i}
	}
}

impl<T: RealNumber> Mul for Complex<T> {
	type Output = Complex<T>;

	fn mul(self, other: Complex<T>) -> Self {
		Self{
			r: self.r * other.r - self.i * other.i, 
			i: self.r * other.i + self.i * other.r}
	}
}

impl<T: RealNumber> AddAssign for Complex<T> {
	fn add_assign(&mut self, other: Complex<T>) {
		self.r += other.r;
		self.i += other.i;
	}
}

impl<T: RealNumber> MulAssign for Complex<T> {
	fn mul_assign(&mut self, other: Complex<T>) {
		let real = self.r;
		self.r = self.r * other.r - self.i * other.i;
		self.i = real * other.i + self.i * other.r;
	}
}

impl<T: RealNumber> Number for Complex<T> {}
impl<T: RealNumber> crate::matrix::MatrixInput for Complex<T> {}
