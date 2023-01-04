use crate::matrix;

#[test]
fn new_empty() {
	let _ = matrix::Matrix::<f32>::new_empty(5, 10);
}

#[test]
fn new() {
	let m = matrix::Matrix::<f32>::new(&vec![0.0,1.0,2.0,3.0,4.0,5.0], 2,3).unwrap();
	assert_eq!(m.len(), 6);

	assert_eq!(m[(0,0)], 0.0);
	assert_eq!(m[(0,1)], 1.0);
	assert_eq!(m[(0,2)], 2.0);
	assert_eq!(m[(1,0)], 3.0);
	assert_eq!(m[(1,1)], 4.0);
	assert_eq!(m[(1,2)], 5.0);
}

#[test]
fn new_error() {
	let e = matrix::Matrix::<f32>::new(&vec![0.0,1.0,2.0,3.0,4.0], 2,3);
	match e {
		Err(matrix::MatrixError::WrongInputArraySize(_)) => (),
		_ => panic!("Wrong error type")
	}
}

#[test]
fn index_mut() {
	let mut m = matrix::Matrix::<f32>::new(&vec![0.0,1.0,2.0,3.0,4.0,5.0], 2,3).unwrap();
	assert_eq!(m.len(), 6);
	m[(1,2)] = 10.0;
	assert_eq!(m[(1,2)], 10.0);
}
