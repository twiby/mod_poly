use std::ops::{AddAssign, Mul};

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
