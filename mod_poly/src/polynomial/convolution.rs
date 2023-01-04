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
