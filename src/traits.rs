//! Central codec and (de)serialization utilities
use std::ops::Range;

use crate::Region;

pub trait SliceRange<'a, R> {
    type Output<'data>;

    /// Get a slice using the start and end indices of a span.
    fn slice<'data>(&self, range: R) -> Self::Output<'data>;
    fn slice_to<'data>(&self, index: usize) -> Self::Output<'data>;
    fn slice_from<'data>(&self, start: usize) -> Self::Output<'data>;
    fn slice_len(&self) -> usize;
}

impl<'a> SliceRange<'a, Range<usize>> for &'a str {
    type Output<'data> = &'data str;

    fn slice<'data>(&self, range: Range<usize>) -> Self::Output<'data> {
        &self[range]
    }

    fn slice_to<'data>(&self, index: usize) -> Self::Output<'data> {
        &self[..index]
    }

    fn slice_from<'data>(&self, start: usize) -> Self::Output<'data> {
        &self[start..]
    }

    fn slice_len(&self) -> usize {
        self.len()
    }
}

impl<'range, T> SliceRange<'range, Range<*const T>> for Region<'range, T> {
    type Output<'data> = Region<'data, T>;

    fn slice<'data>(&self, range: Range<*const T>) -> Self::Output<'data> {
        Region::from_ptr_range(range)
    }

    fn slice_to<'data>(&self, index: usize) -> Self::Output<'data> {
        &self.as_slice()[..index]
    }

    fn slice_from<'data>(&self, from: usize) -> Self::Output<'data> {
        &self.as_slice()[from..]
    }

    fn slice_len(&self) -> usize {
        let len = self.as_slice().len();
        let size = self.size;
        assert_eq!(len, size);
        len
    }
}
