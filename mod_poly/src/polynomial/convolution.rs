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
	let mut convolution = vec![T::from(0.0); size];

	for i in 0..size {
		let mut k = i;
		for j in 0..size {
			convolution[i] += a[j] * b[k];
			k += ((k == 0) as usize) * size;
			k -= 1;
		}
	}

	return convolution;
}

/// This a straight up naive school book convolution
#[allow(dead_code)]
pub fn naive_convolution<T>(a: &Vec<T>, b: &Vec<T>) -> Vec<T>
where T: Clone + Copy + From<f32> + AddAssign + Mul<Output = T> {
	assert!(a.len() == b.len());
	let size = a.len();
	let conv_size = 2*size-1;
	let b_rev: Vec<T> = b.into_iter().rev().copied().collect::<Vec<T>>();
	let mut convolution = vec![T::from(0.0); conv_size];


	for deg in 0..size {
		for (&aa, &bb) in a[..deg+1].iter().zip(b_rev[size-deg-1..].iter()) {
			convolution[deg] += aa * bb;
		}
	}

	for deg in size..conv_size {
		for (&aa, &bb) in a[deg-size+1..].iter().zip(b_rev[..size+size-deg-1].iter()) {
			convolution[deg] += aa * bb;
		}
	}

	return convolution;
}
