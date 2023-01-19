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
fn complex_from_f32() {
	let a = Complex::<f32>::from(3.5);
	let b = Complex::<f64>::from(3.5);

	assert_eq!(a.r, 3.5);
	assert_eq!(a.i, 0.0);
	assert_eq!(b.r, 3.5);
	assert_eq!(b.i, 0.0);
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
fn complex_sub() {
	let a = Complex::new(0.0, 0.0);
	let mut b = a;

	b.r = 1.0;
	b.i = 1.0;

	assert_eq!(b - a, b);
}

#[test]
fn complex_dot() {
	let a = Complex::new(1.0, 1.0);
	let b = Complex::new(2.0, 3.0);

	assert_eq!(Complex::dot(a, b), Complex::new(2.0, 3.0));
}

#[test]
fn complex_mul() {
	let a = Complex::new(1.0, 1.0);
	let b = Complex::new(2.0, 3.0);

	assert_eq!(a * b, Complex::new(-1.0, 5.0));
}

#[test]
fn complex_addassign() {
	let mut a = Complex::new(1.0, 1.0);
	let b = Complex::new(2.0, 3.0);

	a += b;

	assert_eq!(a, Complex::new(3.0, 4.0));
}

#[test]
fn complex_subassign() {
	let mut a = Complex::new(1.0, 1.0);
	let b = Complex::new(2.0, 3.0);

	a -= b;

	assert_eq!(a, Complex::new(-1.0, -2.0));
}

#[test]
fn complex_mulassign() {
	let mut a = Complex::new(1.0, 1.0);
	let b = Complex::new(2.0, 3.0);

	a *= b;

	assert_eq!(a, Complex::new(-1.0, 5.0));
}
