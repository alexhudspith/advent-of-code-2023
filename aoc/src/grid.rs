use std::fmt::{Display, Formatter};
use std::io::BufRead;
use std::iter;
use std::iter::{once, repeat};
use std::ops::Index;
use std::rc::Rc;

use itertools::Itertools;

use enumset::{EnumSet, EnumSetType};

use crate as aoc;

#[derive(Debug, EnumSetType)]
pub enum Axis {
    Row = 0, Column = 1
}

impl Axis {
    pub fn all() -> EnumSet<Axis> {
        EnumSet::all()
    }

    pub fn other(&self) -> Self {
        match self {
            Axis::Row => Axis::Column,
            Axis::Column => Axis::Row,
        }
    }
}

#[derive(Clone)]
pub struct Grid<T> {
    shape: (usize, usize),
    cells: Vec<T>,
    display_transform: Rc<dyn Fn(&T) -> char>
}

impl<T> Grid<T> {
    pub fn shape(&self) -> (usize, usize) {
        self.shape
    }

    pub fn len(&self, axis: Axis) -> usize {
        match axis {
            Axis::Row => self.shape.0,
            Axis::Column => self.shape.1,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.shape.0 == 0 || self.shape.1 == 0
    }

    pub fn get(&self, axis: Axis, i: usize) -> impl Iterator<Item=&T> + '_ {
        (0..self.len(axis.other())).map(move |j| {
            match axis {
                Axis::Row => &self[i][j],
                Axis::Column => &self[j][i],
            }
        })
    }

    pub fn iter(&self, axis: Axis) -> impl Iterator<Item=impl Iterator<Item=&T>> + '_ {
        let mut i = 0;
        iter::from_fn(move || {
            if i == self.len(axis) {
                return None;
            }

            let iter = self.get(axis, i);
            i += 1;
            Some(iter)
        })
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

    pub fn transpose(&mut self) {
        for k in 0..self.cells.len() {
            let (i, j) = self.to_2d(k);
            let k2 = self.to_1d((j, i));
            if k != k2 {
                self.cells.swap(k, k2);
            }
        }

        self.shape = (self.shape.1, self.shape.0);
    }

    pub fn position<F>(&self, mut predicate: F) -> Option<(usize, usize)>
        where F: FnMut(&T) -> bool
    {
        let (i, _) = self.cells.iter().enumerate().find(|&(_, cell)| predicate(cell))?;
        Some(self.to_2d(i))
    }
}

impl<T: PartialEq> PartialEq for Grid<T> {
    fn eq(&self, other: &Self) -> bool {
        self.shape == other.shape && self.cells == other.cells
    }
}

impl<T: Eq> Eq for Grid<T> {}

impl<T> Display for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let display = &self.display_transform;
        for row in self.iter_rows() {
            for cell in row {
                write!(f, "{}", display(cell))?;
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

pub fn read_grid<R, T>(reader: &mut R, padding: Option<T>) -> Result<Grid<T>, aoc::Error>
    where
        R: BufRead,
        T: Clone + From<u8> + Into<u8>
{
    read_grid_with_transform(reader, padding, T::from, |t| t.clone().into() as char)
}

// Optionally pad the grid edges with `padding` rows and columns for easier processing
pub fn read_grid_with_transform<R, T, F, D>(
    reader: &mut R,
    padding_value: Option<T>,
    mut transform: F,
    display_transform: D
) -> Result<Grid<T>, aoc::Error>
    where
        R: BufRead,
        T: Clone,
        F: FnMut(u8) -> T,
        D: Fn(&T) -> char + 'static,
{
    let mut cells: Vec<T> = vec![];
    let mut expected_col_count = 0;
    for (r, line) in reader.lines().enumerate() {
        let line = line?;
        if line.is_empty() {
            if cells.is_empty() {
                // Skip leading blank lines
                continue;
            } else {
                // Stop at first trailing blank line
                break;
            }
        }

        let col_count = line.len() + padding_value.as_ref().map_or(0, |_| 2);
        if cells.is_empty() {
            expected_col_count = col_count;
            if let Some(padding) = padding_value.clone() {
                // First row all padding
                cells.extend(repeat(padding).take(col_count));
            }
        }

        if col_count != expected_col_count {
            return Err(format!("Ragged line at line {}", r + 1).into());
        }

        if let Some(padding) = padding_value.clone() {
            // First/last column padding
            cells.push(padding.clone());
            cells.extend(line.bytes().map(&mut transform));
            cells.push(padding.clone());
        } else {
            cells.extend(line.bytes().map(&mut transform));
        }
    }

    if expected_col_count != 0 && padding_value.is_some() {
        // Clone first row of padding
        let padding = cells.iter().take(expected_col_count).cloned().collect_vec();
        cells.extend(padding);
    }

    let shape = if expected_col_count != 0 {
        (cells.len() / expected_col_count, expected_col_count)
    } else {
        (0, 0)
    };
    Ok(Grid { cells, shape, display_transform: Rc::new(display_transform) })
}
