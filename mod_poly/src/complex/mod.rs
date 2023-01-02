#[cfg(test)]
mod test;

// Custom trait to enable only certain types
trait Number: Copy + PartialEq {}
impl Number for f32 {}
impl Number for f64 {}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Complex<T: Number> {
	r: T,
	i: T
}

impl<T: Number> Complex<T> {
	pub fn new(real: T, imag: T) -> Self {
		Self{r: real, i: imag}
	}
}

impl<T: Number> From<(T, T)> for Complex<T> {
	fn from(t: (T, T)) -> Self {
		Self::new(t.0, t.1)
	}
}

impl<T: Number> From<&[T]> for Complex<T> {
	fn from(t: &[T]) -> Self {
		Self::new(t[0], t[1])
	}
}

impl<T> std::ops::Add<Complex<T>> for Complex<T> 
where T: Number + std::ops::Add<T, Output = T> {
	type Output = Complex<T>;

	fn add(self, other: Complex<T>) -> Self {
		Self{r: self.r + other.r, i: self.i + other.i}
	}
}

impl<T> std::ops::Mul<Complex<T>> for Complex<T> 
where T: Number + std::ops::Mul<T, Output = T> {
	type Output = Complex<T>;

	fn mul(self, other: Complex<T>) -> Self {
		Self{r: self.r * other.r, i: self.i * other.i}
	}
}
