pub mod parse;

use std::iter;
use itertools::Itertools;
use aoc::is_sorted;

pub type Range = aoc::range::Range<u64>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeedMap {
    name: String,
    entries: Vec<SeedMapEntry>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SeedMapEntry {
    src_range: Range,
    dest: u64,
}

impl SeedMapEntry {
    pub(crate) fn new(src: u64, dest: u64, len: u64) -> Self {
        Self {
            src_range: Range::from_start_len(src, len),
            dest
        }
    }

    fn translate(&self, src_ix: u64) -> u64 {
        self.dest
            .wrapping_sub(self.src_range.start())
            .wrapping_add(src_ix)
    }

    fn translate_range(&self, range: Range) -> Range {
        Range::from_start_len(self.translate(range.start()), range.len())
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
            let len = entry.src_range.start() - prev_end;
            if len > 0 {
                new_entries.push(SeedMapEntry::new(prev_end, prev_end, len));
            }
            new_entries.push(entry);
            prev_end = entry.src_range.end();
        }

        let len = u64::MAX - prev_end;
        if len > 0 {
            new_entries.push(SeedMapEntry::new(prev_end, prev_end, len));
        }

        debug_assert!(is_sorted(&new_entries));
        new_entries
    }

    fn intersect_join<A, B, FA, FB>(&self, mut a_iter: A, mut a_key: FA, mut b_iter: B, mut b_key: FB) ->
        impl Iterator<Item=(A::Item, B::Item, Range)>
        where
            A: Iterator, A::Item: Copy,
            B: Iterator, B::Item: Copy,
            FA: FnMut(A::Item) -> Range,
            FB: FnMut(B::Item) -> Range,
    {
        let mut a_opt: Option<A::Item> = a_iter.next();
        let mut b_opt: Option<B::Item> = b_iter.next();

        iter::from_fn(move || {
            while let (Some(a), Some(b)) = (a_opt, b_opt) {
                let a_range = a_key(a);
                let b_range = b_key(b);

                let result_opt = a_range.intersection(&b_range)
                    .map(|intersect| (a, b, intersect));

                if b_range.end() <= a_range.end() {
                    b_opt = b_iter.next();
                } else {
                    a_opt = a_iter.next();
                }

                if result_opt.is_some() {
                    return result_opt;
                }
            }

            None
        })
    }

    pub fn get_many_ordered(&self, src_ranges: &[Range]) -> Vec<Range> {
        assert!(is_sorted(src_ranges), "src_ranges param unsorted");

        let a_iter = self.entries.iter().copied();
        let b_iter = src_ranges.iter().copied();
        let mut intersect_ranges = self
            .intersect_join(a_iter, |a| a.src_range, b_iter, |b| b)
            .map(|(a, _b, inter)| a.translate_range(inter))
            .collect_vec();

        if cfg!(debug_assertions) {
            let mapping_count: u64 = intersect_ranges.iter().map(|r| r.len()).sum();
            let src_count: u64 = src_ranges.iter().map(|r| r.len()).sum();
            debug_assert_eq!(mapping_count, src_count, "Intersections not equal to source range");
        }

        intersect_ranges.sort();
        intersect_ranges
    }
}
