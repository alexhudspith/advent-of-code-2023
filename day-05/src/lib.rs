#![feature(is_sorted)]

pub mod parse;

use std::cmp::{max, min};

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeedMap {
    name: String,
    entries: Vec<SeedMapEntry>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SeedMapEntry {
    src: u64,
    dest: u64,
    len: u64,
}

impl SeedMapEntry {
    pub(crate) fn new(src: u64, dest: u64, len: u64) -> Self {
        Self { src, dest, len }
    }

    #[allow(dead_code)]
    fn src_range(&self) -> Range {
        Range::from_start_len(self.src, self.len)
    }

    #[allow(dead_code)]
    fn dest_range(&self) -> Range {
        Range::from_start_len(self.dest, self.len)
    }

    fn translate(&self, src_ix: u64) -> u64 {
        self.dest
            .wrapping_sub(self.src_range().start)
            .wrapping_add(src_ix)
    }

    fn translate_range(&self, src_range: Range) -> Range {
        Range::from_start_len(self.translate(src_range.start), src_range.len())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Range {
    start: u64,
    end: u64,
}

impl Range {
    pub fn new(start: u64, end: u64) -> Self {
        assert!(start <= end);
        Self { start, end }
    }

    pub fn from_start_len(start: u64, len: u64) -> Self {
        Self::new(start, start + len)
    }

    pub fn start(&self) -> u64 {
        self.start
    }

    pub fn end(&self) -> u64 {
        self.end
    }

    pub fn len(&self) -> u64 {
        self.end - self.start
    }

    pub fn intersection(&self, other: &Range) -> Option<Range> {
        let start = max(self.start, other.start);
        let end = min(self.end, other.end);
        (start < end).then(|| Range::new(start, end))
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl SeedMap {
    fn new(name: String, entries: Vec<SeedMapEntry>) -> Self {
        Self { name, entries: Self::fill_gaps(entries) }
    }

    fn fill_gaps(mut entries: Vec<SeedMapEntry>) -> Vec<SeedMapEntry> {
        entries.sort();

        let mut new_entries = Vec::with_capacity(entries.len() * 2 + 1);
        let mut prev_end = 0;
        for entry in entries {
            let len = entry.src.saturating_sub(prev_end);
            if len > 0 {
                new_entries.push(SeedMapEntry { src: prev_end, dest: prev_end, len });
            }
            new_entries.push(entry);
            prev_end = entry.src_range().end;
        }

        let len = u64::MAX.saturating_sub(prev_end);
        if len > 0 {
            new_entries.push(SeedMapEntry { src: prev_end, dest: prev_end, len });
        }

        assert!(new_entries.is_sorted());
        new_entries
    }

    fn intersection(&self, src_range: &Range) -> Vec<Range> {
        let result = self.entries.iter()
            .map(|entry| (entry, entry.src_range().intersection(src_range)))
            .filter_map(|(entry, range_opt)|
                range_opt.map(|r| entry.translate_range(r))
            )
            .collect_vec();

        let mapping_count = result.iter().map(|r| r.len()).sum::<u64>();
        assert_eq!(mapping_count, src_range.len(), "Intersections not equal to source range");
        result
    }

    pub fn get_many_ordered(&self, ranges: &[Range]) -> Vec<Range> {
        // assert!(ranges.is_sorted(), "ranges param unsorted");
        ranges.iter()
            .flat_map(|range| self.intersection(range))
            .sorted()
            .collect_vec()
    }
}
