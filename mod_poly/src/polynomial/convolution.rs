//! This module implements different versions of the convolution operation

use std::ops::{AddAssign, Mul};

/// The convolution actually used for polynomial multiplication
///
/// It performs a classic convolution for low degrees, and an fft-based convolution for higher degrees. 
/// The threshold that controls the decision is based on a crude analysis done via timing the different versions on my
/// personal computer.
pub fn convolution<T>(a: &Vec<T>, b: &Vec<T>) -> Vec<T>
where T: Clone + Copy + From<f32> + AddAssign + Mul<Output = T> + From<complex::Complex<f64>>, FftComplex: From<T> {
	if a.len() > 350 {
		return convolution_via_fft(a, b);
	} else {
		return convolution_for_polynomial_mult_in_modular_arithmetic(a, b);
	}
}

/// This is a convolution implementation specifically designed for a modular arithmetic.
///
/// In particular, the size of the output is the same size as the input: any higher
/// order term is "spilling over" in lower order terms. Each resulting term
/// has an equal number of addition.
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
pub fn convolution_via_fft<T>(a: &Vec<T>, b: &Vec<T>) -> Vec<T>
where T: Clone + Copy + From<f32> + AddAssign + Mul<Output = T> + From<complex::Complex<f64>>, FftComplex: From<T> {

	let size = a.len();
	let target_size = next_power_of_2(2*a.len());

	// Copy coefs in complex form, and pad with zeros to get target size: the first power of 2 above 2*size
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

	return _convolution_via_fft(&a_coefs, &b_coefs).iter().take(2*a.len()-1).map(|x| T::from(*x)).collect::<Vec<T>>();
}

fn _convolution_via_fft(a: &Vec<FftComplex>, b: &Vec<FftComplex>) -> Vec<FftComplex> {

	if a.len() == 0 {
		return vec![];
	} else if a.len() == 1 {
		return vec![a[0] * b[0]];
	}

	// This weird resorting allows separating even and odd indexed values of "a" via slice manipulation
	let indices = oddeven_sort(a.len());
	let sorted_a: Vec<FftComplex> = indices.iter().map(|n| a[*n]).collect();
	let sorted_b: Vec<FftComplex> = indices.iter().map(|n| b[*n]).collect();

	let mut roots = get_roots_of_unity(a.len());

	let a_ft = _fft_forward(&sorted_a, &roots);
	let b_ft = _fft_forward(&sorted_b, &roots);

	// The term by term product in Fourier space is equivalent to the convolution in the vector space
	let prod: Vec<FftComplex> = a_ft.iter().zip(b_ft.iter()).map(|(x, y)| *x * *y).collect();

	// Again resorting to easily separate odd and even-indexed values
	let sorted_prod: Vec<FftComplex> = indices.iter().map(|n| prod[*n]).collect();

	return _fft_backward(&sorted_prod, &mut roots);
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

/// Returns roots of unity contained in the upper trig circle
fn get_roots_of_unity(size: usize) -> Vec<FftComplex> {
	if size == 1 {
		return vec![FftComplex::from(1.0)];
	}

	let half_size = size >> 1;

	let theta = 2.0 * std::f64::consts::PI / (size as f64);

	// Store all nth root of unity
	let mut roots = Vec::<FftComplex>::with_capacity(half_size);
	if half_size == 1 {
		roots = vec![FftComplex::from(1.0)];
	} else if half_size == 2 {
		roots = vec![FftComplex::from(1.0), FftComplex::new(0.0, 1.0)];
	} else {
		let quarter_size = half_size >> 1;

		// Compute cos and sin on a quarter circle
		let mut current_angle:f64 = 0.0;
		for _ in 0..quarter_size {
			roots.push(FftComplex::new(current_angle.cos(), current_angle.sin()));
			current_angle += theta;
		}

		// Deduce cos and sin on the second quarter circle
		for i in quarter_size..half_size {
			roots.push(FftComplex::new(-roots[i-quarter_size].imag(), roots[i-quarter_size].real()));
		}
	}

	return roots;
}

fn _fft_forward(a: &[FftComplex], roots: &[FftComplex]) -> Vec<FftComplex> {
	_fft(a, &roots).unwrap()
}
fn _fft_backward(a: &[FftComplex], roots: &mut [FftComplex]) -> Vec<FftComplex> {
	// We must change the sign of the imaginary part of roots to define them of the bottom circle	
	for val in roots.into_iter() {
		*val = FftComplex::new(val.real(), -val.imag());
	}

	// Compute backward fft
	let mut y = _fft(a, &roots).unwrap();

	// Apply scaling
	let size_f = a.len() as f64;
	let size_inverse = FftComplex::from(1.0 / size_f);
	for coef in &mut y {
		*coef *= size_inverse;
	}

	return y;
}

fn _fft(a: &[FftComplex], roots: &[FftComplex]) -> Option<Vec<FftComplex>> {
	let size = a.len();
	let half_size = size >> 1;

	// Recursion end
	if size == 1 {
		return Some(vec![FftComplex::from(a[0])]);
	}

	// Size must be a power of 2 every step of the way
	if (size & (size - 1)) != 0 {
		return None;
	}

	// Get our stride to access roots of unity
	let roots_stride: usize = roots.len() / half_size;

	// Recursive call
	let y_even = _fft(&a[..half_size], roots).unwrap();
	let y_odd = _fft(&a[half_size..], roots).unwrap();

	// Extract actual resulting array
	let mut y = vec![FftComplex::from(0.0); size];

	for i in 0..half_size {
		let odd_term = roots[i * roots_stride] * y_odd[i];
		let even_term = y_even[i];

		y[i] = even_term + odd_term;
		y[i + half_size] = even_term - odd_term;
	}

	return Some(y);
}

fn oddeven_sort(size: usize) -> Vec<usize> {
	// Size must be a power of 2 every step of the way
	if (size & (size - 1)) != 0 || size == 0 {
		panic!();
	}

	if size == 1 {
		return vec![0];
	} else if size == 2 {
		return vec![0, 1];
	} else {
		let half_size = size >> 1;
		let previous = oddeven_sort(half_size);
		let mut ret = Vec::<usize>::with_capacity(size);

		for i in 0..half_size {
			ret.push(previous[i]);
			ret.push(previous[i] + half_size);
		}

		return ret;
	}
}
