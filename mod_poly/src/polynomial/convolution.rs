//! This module implements different versions of the convolution operation

use std::ops::{AddAssign, Mul};

/// This is a convolution implementation specifically designed for a modular arithmetic.
///
/// In particular, the size of the output is the same size as the input: any higher
/// order term is "spilling over" in lower order terms. Each resulting term
/// has an equal number of addition.
#[allow(dead_code)]
pub fn convolution_for_polynomial_mult_in_modular_arithmetic<T>(a: &Vec<T>, b: &Vec<T>) -> Vec<T> 
where T: Clone + Copy + From<f32> + AddAssign + Mul<Output = T> {
	assert!(a.len() == b.len());
	let size = a.len();
	let b_rev: Vec<T> = b.into_iter().rev().copied().collect::<Vec<T>>();
	let mut convolution = vec![T::from(0.0); size];

	_naive_convolution_with_reversed_signal_begin(&a, &b_rev, &mut convolution[0..size], size);
	_naive_convolution_with_reversed_signal_end(&a, &b_rev, &mut convolution[0..size], size);

	return convolution;
}

/// This a straight up naive school book convolution
#[allow(dead_code)]
pub fn naive_convolution<T>(a: &Vec<T>, b: &Vec<T>) -> Vec<T>
where T: Clone + Copy + From<f32> + AddAssign + Mul<Output = T> {
	assert!(a.len() == b.len());
	let size = a.len();
	let b_rev: Vec<T> = b.into_iter().rev().copied().collect::<Vec<T>>();
	let mut convolution = vec![T::from(0.0); 2*size-1];

	_naive_convolution_with_reversed_signal_begin(&a, &b_rev, &mut convolution[0..size], size);
	_naive_convolution_with_reversed_signal_end(&a, &b_rev, &mut convolution[size..2*size-1], size);

	return convolution;
}

fn _naive_convolution_with_reversed_signal_begin<T>(a: &[T], b: &[T], dst: &mut[T], size: usize) 
where T: Clone + Copy + From<f32> + AddAssign + Mul<Output = T> {
	for deg in 0..size {
		dst[deg] += _scalar_product(&a[..deg+1], &b[size-deg-1..]);
	}
}
fn _naive_convolution_with_reversed_signal_end<T>(a: &[T], b: &[T], dst: &mut[T], size: usize) 
where T: Clone + Copy + From<f32> + AddAssign + Mul<Output = T> {
	for deg in 0..size-1 {
		dst[deg] += _scalar_product(&a[deg+1..], &b[..size-deg-1]);
	}
}

fn _scalar_product<T>(a: &[T], b: &[T]) -> T
where T: Clone + Copy + From<f32> + AddAssign + Mul<Output = T> {
	let mut ret = T::from(0.0);
	for (&aa, &bb) in a.iter().zip(b.iter()) {
		ret += aa * bb;
	}
	return ret;
}

use crate::complex;
type FftComplex = complex::Complex<f64>;

/// This is the classic optimization of convolution using fft
///
/// It is based on a very nice theorem: the Fourier transform of a convolution of 2 signals is equal to the 
/// product of the Fourier transorms of each signal. This is a general result which allows computing any convolution
/// in O(nlog(n)) complexity (the bottleneck is the fft).
/// In the context of polynomials in particular, the forward Fourier transform is converting between the coefficient representation
/// of the polynomial to the point-value representation at the roots of unity. The backward Fourier transform is then the 
/// interpolation of the point-value representation, to get the coefficient representation.
pub fn convolution_via_fft<T>(a: &Vec<T>, b: &Vec<T>) -> Vec<FftComplex>
where T: Clone + Copy + From<f32> + AddAssign + Mul<Output = T>, FftComplex: From<T> {

	let size = a.len();
	let target_size = next_power_of_2(2*a.len());

	// Copy coefs in complex form
	let mut a_coefs = Vec::<FftComplex>::with_capacity(target_size);
	let mut b_coefs = Vec::<FftComplex>::with_capacity(target_size);
	for val in a {
		a_coefs.push(FftComplex::from(*val));
	}
	for val in b {
		b_coefs.push(FftComplex::from(*val));
	}
	for _ in size..target_size {
		a_coefs.push(FftComplex::from(T::from(0.0)));
		b_coefs.push(FftComplex::from(T::from(0.0)));
	}
	assert_eq!(target_size, a_coefs.len());
	assert_eq!(target_size, b_coefs.len());

	return _convolution_via_fft(&mut a_coefs, &mut b_coefs);
}

fn _convolution_via_fft(a: &Vec<FftComplex>, b: &Vec<FftComplex>) -> Vec<FftComplex> {
	let a_ft = _fft_forward(&a);
	let b_ft = _fft_forward(&b);

	let prod: Vec<FftComplex> = a_ft.iter().zip(b_ft.iter()).map(|(x, y)| *x * *y).collect();

	return _fft_backward(&prod).iter().copied().take(2*a.len() - 1).collect::<Vec<FftComplex>>();
}

fn next_power_of_2(mut num: usize) -> usize {
	let mut val:u32 = 0;

	num=num-1;

	while val <= 4 {
		num = num | (num >> 2i32.pow(val));
		val = val +1;
	}

	num=num+1;
	return num;
}

fn _fft_forward(a: &[FftComplex]) -> Vec<FftComplex> {
	_fft(a, true).unwrap()
}
fn _fft_backward(a: &[FftComplex]) -> Vec<FftComplex> {
	let size = a.len() as f64;
	let size_inverse = FftComplex::from(1.0 / size);

	let mut y = _fft(a, false).unwrap();
	for coef in &mut y {
		*coef *= size_inverse;
	}

	return y;
}

fn _fft(a: &[FftComplex], forward: bool) -> Option<Vec<FftComplex>> {
// where T: Clone + Copy/* + From<f32> + AddAssign + Mul<Output = T>*/, FftComplex: From<T> {
	let size = a.len();
	assert!(size > 0);

	// Size must be a power of 2 every step of the way
	if (size & (size - 1)) != 0 {
		return None;
	}

	// Recursion end
	if size == 1 {
		return Some(vec![FftComplex::from(a[0])]);
	}

	// Frist nth roots of unity
	let theta:f64;
	if forward {
		theta = 2.0 * std::f64::consts::PI / (size as f64);
	} else {
		theta = -2.0 * std::f64::consts::PI / (size as f64);
	}

	// Store all nth root of unity
	let mut current_angle:f64 = 0.0;
	let mut w = Vec::<FftComplex>::with_capacity(size);
	for _ in 0..size {
		w.push(FftComplex::new(current_angle.cos(), current_angle.sin()));
		current_angle += theta;
	}

	// Recursive call
	let a_even: Vec<FftComplex> = a.iter().copied().step_by(2).collect();
	let a_odd: Vec<FftComplex> = a.iter().skip(1).copied().step_by(2).collect();
	let y_even = _fft(&a_even, forward).unwrap();
	let y_odd = _fft(&a_odd, forward).unwrap();

	// Extract actual resulting array
	let half_size = size >> 1;
	let mut y = vec![FftComplex::from(0.0); size];

	for i in 0..half_size {
		let odd_term = w[i] * y_odd[i];
		let even_term = y_even[i];

		y[i] = even_term + odd_term;
		y[i + half_size] = even_term - odd_term;
	}

	return Some(y);
}
