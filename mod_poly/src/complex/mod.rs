#[cfg(test)]
mod test;

use std::ops::{Add, Mul, Sub, AddAssign};

pub const I_32: Complex::<f32> = Complex{r: 0.0, i: 1.0};
pub const I_64: Complex::<f64> = Complex{r: 0.0, i: 1.0};

// Custom trait to enable only certain types
pub trait Number: Copy + PartialEq + From<f32> + AddAssign {}
pub trait RealNumber: Copy + PartialEq + From<f32> + AddAssign {}
impl Number for f32 {}
impl Number for f64 {}
impl RealNumber for f32 {}
impl RealNumber for f64 {}

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

impl<T: RealNumber> From<(T, T)> for Complex<T> {
	fn from(t: (T, T)) -> Self {
		Self::new(t.0, t.1)
	}
}

impl<T: RealNumber> From<&[T]> for Complex<T> {
	fn from(t: &[T]) -> Self {
		assert!(t.len() > 1);
		Self::new(t[0], t[1])
	}
}

impl<T: RealNumber> From<f32> for Complex<T> {
	fn from(t: f32) -> Self {
		Self::new(T::from(t), T::from(0.0))
	}
}

impl<T> Add for Complex<T> 
where T: RealNumber + Add<Output = T> {
	type Output = Complex<T>;

	fn add(self, other: Complex<T>) -> Self {
		Self{r: self.r + other.r, i: self.i + other.i}
	}
}

impl<T> Mul for Complex<T> 
where T: RealNumber + Add<Output = T> + Sub<Output = T> + Mul<Output = T> {
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

impl<T: RealNumber> Number for Complex<T> {}
