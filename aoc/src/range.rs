use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use num::Num;

pub trait Number: Num + PartialOrd + Copy + Debug {}
impl<T: Num + PartialOrd + Copy + Debug> Number for T {}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Range<T> {
    start: T,
    end: T,
}

impl<T: Number> Range<T> {
    pub fn new(start: T, end: T) -> Self {
        assert!(start <= end);
        Self { start, end }
    }

    pub fn from_start_len(start: T, len: T) -> Self {
        Self::new(start, start + len)
    }

    pub const fn start(&self) -> T {
        self.start
    }

    pub const fn end(&self) -> T {
        self.end
    }

    pub fn len(&self) -> T {
        self.end - self.start
    }

    fn max(v1: T, v2: T) -> T {
        match v1.partial_cmp(&v2).unwrap() {
            Ordering::Less | Ordering::Equal => v2,
            Ordering::Greater => v1,
        }
    }

    fn min(v1: T, v2: T) -> T {
        match v1.partial_cmp(&v2).unwrap() {
            Ordering::Less | Ordering::Equal => v1,
            Ordering::Greater => v2,
        }
    }

    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let start = Self::max(self.start, other.start);
        let end = Self::min(self.end, other.end);
        (start < end).then(|| Range::new(start, end))
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.intersection(other).is_some()
    }

    pub fn split_at(&self, i: T) -> (Self, Self) {
        if self.start <= i && i <= self.end {
            (Self::new(self.start, i), Self::new(i, self.end))
        } else {
            panic!("split_at({:?}) out of range for {:?}", i, self);
        }
    }

    pub fn contains(&self, i: T) -> bool {
        self.start <= i && i < self.end
    }

    pub fn is_empty(&self) -> bool {
        self.len() == T::zero()
    }
}

impl<T: std::iter::Step> IntoIterator for Range<T> {
    type Item = T;
    type IntoIter = std::ops::Range<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.start..self.end
    }
}

impl<T: Display> Display for Range<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {})", self.start, self.end)
    }
}
