//! `BitBox` iterator implementations.

use super::BitBox;

use crate::{
	cursor::Cursor,
	pointer::BitPtr,
	store::BitStore,
	slice::BitSlice,
};

use core::iter::FusedIterator;

impl<C, T> IntoIterator for BitBox<C, T>
where C: Cursor, T: BitStore {
	type Item = bool;
	type IntoIter = IntoIter<C, T>;

	fn into_iter(self) -> Self::IntoIter {
		IntoIter {
			region: self.bitptr,
			bitbox: self,
		}
	}
}

impl<'a, C, T> IntoIterator for &'a BitBox<C, T>
where C: Cursor, T: 'a + BitStore {
	type Item = bool;
	type IntoIter = <&'a BitSlice<C, T> as IntoIterator>::IntoIter;

	fn into_iter(self) -> Self::IntoIter {
		self.as_bits().into_iter()
	}
}

#[repr(C)]
pub struct IntoIter<C, T>
where C: Cursor, T: BitStore {
	/// Owning pointer to the full slab
	pub(crate) bitbox: BitBox<C, T>,
	/// Slice descriptor for the region undergoing iteration.
	pub(crate) region: BitPtr<T>,
}

impl<C, T> IntoIter<C, T>
where C: Cursor, T: BitStore {
	fn iterator(&self) -> <&BitSlice<C, T> as IntoIterator>::IntoIter {
		self.region.into_bitslice().into_iter()
	}
}

impl<C, T> DoubleEndedIterator for IntoIter<C, T>
where C: Cursor, T: BitStore {
	fn next_back(&mut self) -> Option<Self::Item> {
		let mut slice_iter = self.iterator();
		let out = slice_iter.next_back();
		self.region = slice_iter.bitptr();
		out
	}
}

impl<C, T> ExactSizeIterator for IntoIter<C, T>
where C: Cursor, T: BitStore {}

impl<C, T> FusedIterator for IntoIter<C, T>
where C: Cursor, T: BitStore {}

impl<C, T> Iterator for IntoIter<C, T>
where C: Cursor, T: BitStore {
	type Item = bool;

	fn next(&mut self) -> Option<Self::Item> {
		let mut slice_iter = self.iterator();
		let out = slice_iter.next();
		self.region = slice_iter.bitptr();
		out
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		self.iterator().size_hint()
	}

	fn count(self) -> usize {
		self.len()
	}

	fn nth(&mut self, n: usize) -> Option<Self::Item> {
		let mut slice_iter = self.iterator();
		let out = slice_iter.nth(n);
		self.region = slice_iter.bitptr();
		out
	}

	fn last(mut self) -> Option<Self::Item> {
		self.next_back()
	}
}