//! This module implements different versions of the convolution operation

use std::ops::{AddAssign, Mul};

/// The convolution actually used for polynomial multiplication
///
/// It performs a classic convolution for low degrees, and an fft-based convolution for higher degrees. 
/// The threshold that controls the decision is based on a crude analysis done via timing the different versions on my
/// personal computer.
pub fn convolution<T>(a: &Vec<T>, b: &Vec<T>) -> Vec<T>
where T: Clone + Copy + From<f32> + AddAssign + Mul<Output = T> + From<complex::Complex<f64>>, FftComplex: From<T> {
	if a.len() > 130 {
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

	// Allocating buffer for fft signal
	let mut a_fft = vec![FftComplex::from(0.0); a.len()];
	let mut b_fft = vec![FftComplex::from(0.0); b.len()];

	// This weird resorting allows separating even and odd indexed values of "a" via slice manipulation
	let mut indices = vec![0; a.len()];
	oddeven_sort(a.len(), &mut indices);
	let sorted_a: Vec<FftComplex> = indices.iter().map(|n| a[*n]).collect();
	let sorted_b: Vec<FftComplex> = indices.iter().map(|n| b[*n]).collect();

	let mut roots = get_roots_of_unity(a.len());

	_fft_forward(&sorted_a, &roots, &mut a_fft);
	_fft_forward(&sorted_b, &roots, &mut b_fft);

	// The term by term product in Fourier space is equivalent to the convolution in the vector space
	let prod: Vec<FftComplex> = a_fft.iter().zip(b_fft.iter()).map(|(x, y)| *x * *y).collect();

	// Again resorting to easily separate odd and even-indexed values
	let sorted_prod: Vec<FftComplex> = indices.iter().map(|n| prod[*n]).collect();

	_fft_backward(&sorted_prod, &mut roots, &mut a_fft);
	return a_fft;
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
	let half_size = size >> 1;

	let theta = 2.0 * std::f64::consts::PI / (size as f64);
	let first_root = FftComplex::new(theta.cos(), theta.sin());

	let mut roots = vec![FftComplex::from(1.0); half_size];

	for i in 1..half_size {
		roots[i] = roots[i-1] * first_root;
	}

	return roots;
}

fn _fft_forward(a: &[FftComplex], roots: &[FftComplex], dst: &mut Vec<FftComplex>) {
	_fft(a, &roots, dst);
}
fn _fft_backward(a: &[FftComplex], roots: &mut [FftComplex], dst: &mut Vec<FftComplex>) {
	// We must change the sign of the imaginary part of roots to define them of the bottom circle	
	for val in roots.into_iter() {
		*val = FftComplex::new(val.real(), -val.imag());
	}

	// Compute backward fft
	_fft(a, &roots, dst);

	// Apply scaling
	let size_f = a.len() as f64;
	let size_inverse = FftComplex::from(1.0 / size_f);
	for coef in dst {
		*coef *= size_inverse;
	}
}

fn _fft(a: &[FftComplex], roots: &[FftComplex], dst: &mut [FftComplex]) {
	let size = a.len();

	// Quick exit
	if size == 1 {
		dst[0] = a[0];
		return;
	} else if size == 2 {
		dst[0] = a[0] + a[1];
		dst[1] = a[0] - a[1];
	} else if size == 4 {
		dst[0] = a[0] + a[1] + a[2] + a[3];
		dst[2] = a[0] + a[1] - a[2] - a[3];
		dst[1] = a[0] - a[1] + roots[1] * (a[2] - a[3]);
		dst[3] = a[0] - a[1] - roots[1] * (a[2] - a[3]);		
	}

	// Size must be a power of 2 every step of the way
	if (size & (size - 1)) != 0 {
		panic!("FFT: size is not a power of 2");
	}

	for i in (0..size).step_by(4) {
		dst[i] = a[i] + a[i+1] + a[i+2] + a[i+3];
		dst[i+2] = a[i] + a[i+1] - a[i+2] - a[i+3];
		dst[i+1] = a[i] - a[i+1] + roots[roots.len()>>1] * (a[i+2] - a[i+3]);
		dst[i+3] = a[i] - a[i+1] - roots[roots.len()>>1] * (a[i+2] - a[i+3]);
	}
	
	let mut step = 8;
	let mut half_step = step >> 1;
	let mut roots_stride = size >> 3;
	while step <= size {
		for sub_fft in dst.chunks_mut(step) {
			for i in 0..half_step {
				let odd_term = roots[i * roots_stride] * sub_fft[half_step + i];
				sub_fft[i + half_step] = sub_fft[i] - odd_term;		
				sub_fft[i] += odd_term;
			}
		}
		half_step <<= 1;
		step <<= 1;
		roots_stride >>= 1;
	}
}

fn oddeven_sort(size: usize, indices: &mut [usize]) {
	// Size must be a power of 2 every step of the way
	if (size & (size - 1)) != 0 || size == 0 {
		panic!();
	}
	let shift = usize::BITS - size.ilog2();
	indices.iter_mut().enumerate().for_each(|(i, n)| *n = i.reverse_bits() >> shift);
}
