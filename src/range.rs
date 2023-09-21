use std::ops;

pub struct Range<'a, T: ?Sized> {
    inner: &'a T,
    offset: isize,
    len: usize,
}

impl<T: ?Sized> Copy for Range<'_, T> {}

impl<T: ?Sized> Clone for Range<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T: ?Sized> Range<'a, T> {
    pub fn new(inner: &'a T, offset: isize, len: usize) -> Self {
        Self { inner, offset, len }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn inner(&self) -> &'a T {
        self.inner
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

pub trait RangeBounds: Sized + Clone {
    fn try_index(self, len: usize) -> Option<(usize, usize)>;
    fn index(self, len: usize) -> (usize, usize) {
        match self.clone().try_index(len) {
            Some(index) => index,
            None => panic!("range out of bounds"),
        }
    }
}

impl RangeBounds for ops::Range<usize> {
    fn try_index(self, len: usize) -> Option<(usize, usize)> {
        if self.start > self.end || self.end > len {
            return None
        }

        Some((self.start, self.end))
    }
}

impl RangeBounds for ops::RangeFrom<usize> {
    fn try_index(self, len: usize) -> Option<(usize, usize)> {
        if self.start > len {
            return None
        }

        Some((self.start, len - self.start))
    }
}

impl RangeBounds for ops::RangeTo<usize> {
    fn try_index(self, len: usize) -> Option<(usize, usize)> {
        if self.end > len {
            return None
        }

        Some((0, self.end))
    }
}

impl RangeBounds for ops::RangeFull {
    fn try_index(self, len: usize) -> Option<(usize, usize)> {
        Some((0, len))
    }
}

pub trait Slice: ops::Index<ops::Range<usize>> {
    fn len(&self) -> usize;
    fn empty<'a>() -> &'a Self;
    fn as_slice(&self, range: ops::Range<usize>) -> &Self;
    fn common_prefix_len(&self, other: &Self) -> usize;
    fn common_suffix_len(&self, other: &Self) -> usize;
    fn starts_with(&self, prefix: &Self) -> bool;
    fn ends_with(&self, suffix: &Self) -> bool;
}

impl Slice for str {
    fn len(&self) -> usize {
        self.len()
    }

    fn empty<'a>() -> &'a Self {
        ""
    }

    fn as_slice(&self, range: ops::Range<usize>) -> &Self {
        &self[range]
    }

    fn common_prefix_len(&self, other: &Self) -> usize {
        self.as_bytes()
            .iter()
            .zip(other.as_bytes().iter())
            .take_while(|(a, b)| a == b)
            .count()
    }

    fn common_suffix_len(&self, other: &Self) -> usize {
        self.as_bytes()
            .iter()
            .rev()
            .zip(other.as_bytes().iter().rev())
            .take_while(|(a, b)| a == b)
            .count()
    }

    fn starts_with(&self, prefix: &Self) -> bool {
        self.starts_with(prefix)
    }

    fn ends_with(&self, suffix: &Self) -> bool {
        self.ends_with(suffix)
    }
}

impl<T: PartialEq> Slice for [T] {
    fn len(&self) -> usize {
        self.len()
    }

    fn empty<'a>() -> &'a Self {
        &[]
    }

    fn as_slice(&self, range: ops::Range<usize>) -> &Self {
        &self[range]
    }

    fn common_prefix_len(&self, other: &Self) -> usize {
        self.iter()
            .zip(other.iter())
            .take_while(|(a, b)| a == b)
            .count()
    }

    fn common_suffix_len(&self, other: &Self) -> usize {
        self.iter()
            .rev()
            .zip(other.iter().rev())
            .take_while(|(a, b)| a == b)
            .count()
    }

    fn starts_with(&self, prefix: &Self) -> bool {
        self.starts_with(prefix)
    }

    fn ends_with(&self, suffix: &Self) -> bool {
        self.ends_with(suffix)
    }
}