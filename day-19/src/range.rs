use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

impl Display for Range {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {})", self.start, self.end)
    }
}
