use std::cmp::{max, min};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Range {
    start: u64,
    end: u64,
}

impl Range {
    pub const fn new(start: u64, end: u64) -> Self {
        assert!(start <= end);
        Self { start, end }
    }

    pub const fn from_start_len(start: u64, len: u64) -> Self {
        Self::new(start, start + len)
    }

    pub const fn start(&self) -> u64 {
        self.start
    }

    pub const fn end(&self) -> u64 {
        self.end
    }

    pub const fn len(&self) -> u64 {
        self.end - self.start
    }

    pub fn intersection(&self, other: &Range) -> Option<Range> {
        let start = max(self.start, other.start);
        let end = min(self.end, other.end);
        (start < end).then(|| Range::new(start, end))
    }

    pub fn intersects(&self, other: &Range) -> bool {
        self.intersection(other).is_some()
    }

    pub fn split_at(&self, i: u64) -> (Range, Range) {
        if self.start <= i && i <= self.end {
            (Range::new(self.start, i), Range::new(i, self.end))
        } else {
            panic!("split_at({}) out of range for {:?}", i, self);
        }
    }

    pub const fn contains(&self, i: u64) -> bool {
        self.start <= i && i < self.end
    }

    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl IntoIterator for Range {
    type Item = u64;
    type IntoIter = std::ops::Range<u64>;

    fn into_iter(self) -> Self::IntoIter {
        self.start..self.end
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {})", self.start, self.end)
    }
}
