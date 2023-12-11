use std::fmt::{Debug, Display, Formatter};
use std::io::{BufRead, BufReader, Read};
use std::iter::{once, repeat};
use std::ops::Index;
use itertools::Itertools;

use crate as aoc;

#[derive(Default, Clone, PartialEq, Eq)]
pub struct Grid<T> {
    cells: Vec<T>,
    shape: (usize, usize),
}

impl<T> Grid<T> {
    pub fn shape(&self) -> (usize, usize) {
        self.shape
    }

    pub fn iter_rows(&self) -> impl Iterator<Item=&[T]> + '_ {
        self.cells.chunks(self.shape.0)
    }

    fn to_1d(&self, (r, c): (usize, usize)) -> usize {
        r * self.shape.1 + c
    }

    fn to_2d(&self, i: usize) -> (usize, usize) {
        (i / self.shape.1, i % self.shape.1)
    }

    pub fn position<F>(&self, mut predicate: F) -> Option<(usize, usize)>
        where F: FnMut(&T) -> bool
    {
        let (i, _) = self.cells.iter().enumerate().find(|&(_, cell)| predicate(cell))?;
        Some(self.to_2d(i))
    }
}

impl<T: Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.iter_rows() {
            for cell in row.iter() {
                write!(f, "{:?}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.iter_rows() {
            for cell in row.iter() {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        let start = self.to_1d((index, 0));
        let end = self.to_1d((index + 1, 0));
        &self.cells[start..end]
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.cells[self.to_1d(index)]
    }
}

pub fn read_grid<R, T, F>(reader: R, padding: Option<T>, mut transform: F) -> Result<Grid<T>, aoc::Error>
    where
        R: Read,
        T: Clone,
        F: FnMut(u8) -> T
{
    // Pad the grid edges with '.'' rows and columns for easier processing
    let mut cells: Vec<T> = vec![];
    let mut expected_col_count = 0;
    for (r, line) in BufReader::new(reader).lines().enumerate() {
        let line = line?;
        if line.is_empty() {
            continue;
        }

        let col_count = line.len() + padding.as_ref().map_or(0, |_| 2);
        if cells.is_empty() {
            // First row all padding
            expected_col_count = col_count;
            if let Some(padding) = padding.clone() {
                let padding = repeat(padding).take(col_count);
                cells.extend(padding);
            }
        }

        if col_count != expected_col_count {
            return Err(format!("Ragged line at line {}", r + 1).into());
        }

        // First/last column padding
        if let Some(padding) = padding.clone() {
            cells.extend(
                once(padding.clone())
                .chain(line.bytes().map(&mut transform))
                .chain(once(padding))
            );
        }
    }

    let padding = cells.iter().take(expected_col_count).cloned().collect_vec();
    cells.extend(padding);
    let shape = (cells.len() / expected_col_count, expected_col_count);
    Ok(Grid { cells, shape })
}
