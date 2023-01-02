use crate::complex::Complex;

#[test]
fn complex_type() {
	let a = Complex{r: 0.0, i: 0.0};

	assert_eq!(a.r, 0.0);
	assert_eq!(a.i, 0.0);
}

#[test]
fn complex_new() {
	let a = Complex::new(0.0, 0.0);

	assert_eq!(a.r, 0.0);
	assert_eq!(a.i, 0.0);
}

#[test]
fn complex_from_tuple() {
	let a = Complex::from((0.0, 0.0));

	assert_eq!(a.r, 0.0);
	assert_eq!(a.i, 0.0);
}

#[test]
fn complex_from_slice() {
	let array = [0.0, 0.0];
	let a = Complex::from(&array[..]);

	assert_eq!(a.r, 0.0);
	assert_eq!(a.i, 0.0);
}

#[test]
#[should_panic]
fn complex_from_slice_buffer_overflow() {
	let array = [0.0];
	let a = Complex::from(&array[..]);

	assert_eq!(a.r, 0.0);
	assert_eq!(a.i, 0.0);
}

#[test]
fn complex_copy() {
	let a = Complex::new(0.0, 0.0);
	let mut b = a;

	b.r = 1.0;

	assert_eq!(a.r, 0.0);
	assert_eq!(b.r, 1.0);
}

#[test]
fn complex_add() {
	let a = Complex::new(0.0, 0.0);
	let mut b = a;

	b.r = 1.0;
	b.i = 1.0;

	assert_eq!(a + b, b);
}


#[test]
fn complex_mul() {
	let a = Complex::new(0.0, 0.0);
	let mut b = a;

	b.r = 1.0;
	b.i = 1.0;

	assert_eq!(a * b, a);
}
