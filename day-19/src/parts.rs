use std::ops::{Index, IndexMut};
use crate::range::Range;

pub type Part = Ratings<u64>;
pub type RangedPart = Ratings<Range>;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Category { X, M, A, S }

const CATS: usize = 4;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Ratings<T> {
    xmas: [T; CATS],
}

impl<T> Index<Category> for Ratings<T> {
    type Output = T;

    fn index(&self, index: Category) -> &Self::Output {
        &self.xmas[index as usize]
    }
}

impl<T> Index<&Category> for Ratings<T> {
    type Output = T;

    fn index(&self, index: &Category) -> &Self::Output {
        &self[*index]
    }
}

impl<T> IndexMut<Category> for Ratings<T> {
    fn index_mut(&mut self, index: Category) -> &mut Self::Output {
        &mut self.xmas[index as usize]
    }
}

impl<T> IndexMut<&Category> for Ratings<T> {
    fn index_mut(&mut self, index: &Category) -> &mut Self::Output {
        &mut self[*index]
    }
}

impl Part {
    pub fn sum(&self) -> u64 {
        self.xmas.iter().sum()
    }
}

impl RangedPart {
    pub const fn all(range: Range) -> Self {
        Self { xmas: [range; CATS] }
    }

    pub const fn empty() -> Self {
        Self { xmas: [Range::new(0, 0); CATS] }
    }

    pub fn is_empty(&self) -> bool {
        self.xmas.iter().any(|x| x.is_empty())
    }

    pub fn product(&self) -> u64 {
        self.xmas.iter().map(|x| x.len()).product()
    }
}
