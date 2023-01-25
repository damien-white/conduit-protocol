//! Amalgam network protocol
use core::{marker::PhantomData, ops::Range};
use std::slice;

pub use traits::SliceRange;

mod traits;

pub struct Region<'data, T> {
    base: *const T,
    size: usize,
    lifetime: PhantomData<&'data T>,
}

impl<'data, T> Region<'data, T> {
    pub fn new(base: *const T, size: usize) -> Region<'data, T> {
        Region {
            base,
            size,
            lifetime: PhantomData,
        }
    }

    pub fn as_ptr(&self) -> *const T {
        self.base.cast()
    }

    pub fn as_mut_ptr(&self) -> *mut T {
        self.base.cast_mut()
    }

    pub fn as_ptr_range(&self) -> Range<*const T> {
        let start = self.base.cast::<T>();
        let end = unsafe { start.add(self.size) };
        Range { start, end }
    }

    /// Return the Region as a slice.
    pub fn as_slice(&self) -> &'data [T] {
        unsafe { slice::from_raw_parts(self.base, self.size) }
    }

    /// Construct an [Allocated] instance from a range of pointers.
    pub fn from_ptr_range(range: Range<*const T>) -> Region<'data, T> {
        let base = range.start;
        let size = range.end as usize - range.start as usize;
        Region {
            base,
            size,
            lifetime: PhantomData,
        }
    }

    /// Construct an [Allocated] instance from a range of pointers.
    pub fn from_ptr_range_mut(range: Range<*mut T>) -> Region<'data, T> {
        let base = range.start;
        let size = range.end as usize - range.start as usize;
        Region {
            base,
            size,
            lifetime: PhantomData,
        }
    }
}
