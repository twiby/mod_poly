use core::ops::{Index, IndexMut};

pub enum Viewer<'a, T> {
	Writer(&'a mut T),
	Reader(&'a T),
	Owner(T),
	None
}
impl<'a, T> Viewer<'a, T> {
	pub fn inner(&self) -> Option<&T> {
		match self {
			Viewer::Writer(ref s) => Some(s),
			Viewer::Reader(ref s) => Some(s),
			Viewer::Owner(ref s) => Some(s),
			Viewer::None => None
		}
	}
	pub fn inner_mut(&mut self) -> Option<&mut T> {
		match self {
			Viewer::Reader(_) => None,
			Viewer::Writer(ref mut s) => Some(s),
			Viewer::Owner(ref mut s) => Some(s),
			Viewer::None => None
		}
	}

	pub fn view<'m: 'n, 'n>(&'m self) -> Viewer<'n, T> {
		match self.inner() {
			None => Viewer::None,
			Some(m) => Viewer::Reader(m)
		}
	}

	pub fn writer<'m: 'n, 'n>(&'m mut self) -> Viewer<'n, T> {
		match self.inner_mut() {
			None => Viewer::None,
			Some(s) => Viewer::Writer(s)
		}
	}

	pub fn is_none(&self) -> bool {
		matches!(self, Viewer::None)
	}

	pub fn is_owner(&self) -> bool {
		matches!(self, Viewer::Owner(_))
	}

	pub fn is_reader(&self) -> bool {
		matches!(self, Viewer::Reader(_))
	}
}

impl<'a, T: Clone> Clone for Viewer<'a, T> {
	fn clone(self: &Viewer<'a, T>) -> Self {
		match self.inner() {
			None => Viewer::None,
			Some(m) => Viewer::Owner(m.clone())
		}
	}
}

impl<'a, T> From<Option<&'a T>> for Viewer<'a, T> {
	fn from(other: Option<&'a T>) -> Viewer<'a, T> {
		match other {
			None => Viewer::None,
			Some(s) => Viewer::Reader(s)
		}
	}
}
impl<'a, T> From<T> for Viewer<'a, T> {
	fn from(other: T) -> Viewer<'a, T> {
		Viewer::Owner(other)
	}
}

impl<'a, 'b: 'a, T> From<&'b Viewer<'a, T>> for Viewer<'a, T> {
	fn from(other: &'b Viewer<'a, T>) -> Viewer<'a, T> {
		Viewer::from(other.inner())
	}
}

impl<'a, T: Clone> From<Viewer<'a, T>> for Option<T> {
	fn from(other: Viewer<'a, T>) -> Option<T> {
		match other {
			Viewer::None => None,
			Viewer::Owner(m) => Some(m),
			Viewer::Reader(m) => Some(m.clone()),
			Viewer::Writer(m) => Some(m.clone())
		}
	}
}

impl<'a, T, I> Index<I> for Viewer<'a, T> 
where T: Index<I> {
	type Output = <T as Index<I>>::Output;

	fn index(&self, index: I) -> &Self::Output {
		match self.inner() {
			Some(ref s) => &s[index],
			None => panic!("Viewer::None cannot implement Index")
		}
	}
}
impl<'a, T, I> IndexMut<I> for Viewer<'a, T> 
where T: IndexMut<I> {
	fn index_mut(&mut self, index: I) -> &mut Self::Output {
		match self.inner_mut() {
			Some(s) => &mut s[index],
			None => panic!("Viewer::None or Viewer::Reader cannot implement IndexMut")
		}
	}
}