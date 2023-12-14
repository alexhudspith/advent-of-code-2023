#![allow(clippy::redundant_field_names)]

use std::borrow::Borrow;
use std::fmt::{Debug, Display, Formatter};
use std::io::BufRead;
use std::{fmt, iter};
use std::hash::{Hash, Hasher};
use std::iter::repeat;
use std::ops::{Index, IndexMut};
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

    pub const fn other(&self) -> Self {
        match self {
            Axis::Row => Axis::Column,
            Axis::Column => Axis::Row,
        }
    }
}

fn debug_to_str<T>(value: &[T], f: &mut Formatter<'_>) -> fmt::Result
    where [T]: Debug
{
    value.fmt(f)
}

fn ascii_to_str(value: &[u8], f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}", String::from_utf8_lossy(value))
}

fn chars_to_str(value: &[char], f: &mut Formatter<'_>) -> fmt::Result {
    let s: String = value.iter().copied().collect();
    write!(f, "{:?}", s)
}

pub type RowFormatter<T> = dyn Fn(&[T], &mut Formatter<'_>) -> fmt::Result;

#[derive(Clone)]
pub struct Grid<T=u8> {
    shape: (usize, usize),
    cells: Vec<T>,
    debug_fmt: Rc<RowFormatter<T>>,
}

impl<T: Hash> Hash for Grid<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.shape.hash(state);
        self.cells.hash(state);
    }
}

impl Grid<u8> {
    pub fn new_ascii(rows: usize, cols: usize) -> Self {
        let cells = vec![b'.'; rows * cols];
        Self {
            shape: (rows, cols),
            cells: cells,
            debug_fmt: Rc::from(ascii_to_str),
        }
    }
}

impl Grid<char> {
    pub fn new_char(rows: usize, cols: usize) -> Self {
        let cells = vec!['.'; rows * cols];

        Self {
            shape: (rows, cols),
            cells: cells,
            debug_fmt: Rc::from(chars_to_str),
        }
    }
}

impl<T> Grid<T> {
    pub fn new(rows: usize, cols: usize) -> Self where T: Default + 'static, [T]: Debug {
        let cells = iter::repeat_with(T::default).take(rows * cols).collect();
        Self {
            shape: (rows, cols),
            cells: cells,
            debug_fmt: Rc::from(debug_to_str),
        }
    }

    pub const fn shape(&self) -> (usize, usize) {
        self.shape
    }

    pub const fn len(&self, axis: Axis) -> usize {
        match axis {
            Axis::Row => self.shape.0,
            Axis::Column => self.shape.1,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.shape.0 == 0 || self.shape.1 == 0
    }

    pub fn get(&self, axis: Axis, i: usize) -> impl DoubleEndedIterator<Item=&T> + ExactSizeIterator + '_ {
        (0..self.len(axis.other())).map(move |j| {
            match axis {
                Axis::Row => &self[i][j],
                Axis::Column => &self[j][i],
            }
        })
    }

    pub fn iter(&self, major: Axis) -> impl DoubleEndedIterator<Item=impl DoubleEndedIterator<Item=&T> + ExactSizeIterator> + ExactSizeIterator + '_ {
        (0..self.len(major)).map(move |i| self.get(major, i))
    }

    pub fn iter_rows(&self) -> impl DoubleEndedIterator<Item=&[T]> + ExactSizeIterator + '_ {
        self.cells.chunks(self.shape.1)
    }

    const fn to_1d(&self, (r, c): (usize, usize)) -> usize {
        r * self.shape.1 + c
    }

    const fn to_2d(&self, i: usize) -> (usize, usize) {
        (i / self.shape.1, i % self.shape.1)
    }

    pub fn transposed(&mut self) -> Self where T: Clone {
        self.clone().into_transpose()
    }

    pub fn into_transpose(mut self) -> Self {
        let mut new_cells = Box::new_uninit_slice(self.cells.len());
        let new_shape = (self.shape.1, self.shape.0);

        // Walk cells backwards, removing from the end
        for s in (0..self.cells.len()).rev() {
            let (r, c) = self.to_2d(s);
            let t = c * new_shape.1 + r;
            let elem = self.cells.remove(s);
            new_cells[t].write(elem);
        }

        // Safety: all cells were initialized above
        self.cells = unsafe { new_cells.assume_init() }.into_vec();
        self.shape = new_shape;
        self
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

impl<T> Debug for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)?;
        writeln!(f, "shape: {:?}", self.shape)
    }
}

impl<T> Display for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in self.iter_rows() {
            (self.debug_fmt)(row, f)?;
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

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let start = self.to_1d((index, 0));
        let end = self.to_1d((index + 1, 0));
        &mut self.cells[start..end]
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let i = self.to_1d(index);
        &self.cells[i]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let i = self.to_1d(index);
        &mut self.cells[i]
    }
}

impl<T, R> From<&[R]> for Grid<T>
    where R: Borrow<[T]>, T: Clone + 'static, [T]: Debug
{
    fn from(value: &[R]) -> Self {
        let rows = value.len();
        let cols = if rows == 0 { 0 } else { value[0].borrow().len() };
        let mut cells: Vec<T> = Vec::with_capacity(rows * cols);
        cells.extend(
            value.iter().flat_map(|row| row.borrow().iter().cloned())
        );

        Grid {
            shape: (rows, cols),
            cells: cells,
            debug_fmt: Rc::new(debug_to_str),
        }
    }
}

pub fn read_grid_ascii<R>(reader: &mut R, padding: Option<u8>) -> Result<Grid<u8>, aoc::Error>
    where R: BufRead
{
    read_grid_with_transform(reader, padding, std::convert::identity, ascii_to_str)
}

// Optionally pad the grid edges with `padding` rows and columns for easier processing
pub fn read_grid_with_transform<R, T, F, D>(
    reader: &mut R,
    padding_value: Option<T>,
    mut transform: F,
    debug_fmt: D
) -> Result<Grid<T>, aoc::Error>
    where
        R: BufRead,
        T: Clone,
        F: FnMut(u8) -> T,
        D: 'static + Fn(&[T], &mut Formatter<'_>) -> fmt::Result,
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

    if expected_col_count == 0 {
        return Err(aoc::Error::EndOfFile);
    }

    if padding_value.is_some() {
        // Clone first row of padding
        let padding = cells.iter().take(expected_col_count).cloned().collect_vec();
        cells.extend(padding);
    }

    let shape = (cells.len() / expected_col_count, expected_col_count);
    cells.shrink_to_fit();
    Ok(Grid { cells, shape, debug_fmt: Rc::new(debug_fmt) })
}

#[cfg(test)]
mod tests {
    use crate::grid::Grid;

    #[test]
    fn transpose_row() {
        let mut g = Grid::new(1, 4);
        g[0][0] = 1;
        g[0][1] = 2;
        g[0][2] = 3;
        g[0][3] = 4;

        let mut expected = Grid::new(4, 1);
        expected[0][0] = 1;
        expected[1][0] = 2;
        expected[2][0] = 3;
        expected[3][0] = 4;

        let actual = g.into_transpose();
        assert_eq!(actual, expected);
    }

    #[test]
    fn transpose_col() {
        let mut g = Grid::new_ascii(5, 1);
        g[0][0] = b'1';
        g[1][0] = b'2';
        g[2][0] = b'3';
        g[3][0] = b'4';
        g[4][0] = b'5';

        let mut expected = Grid::new_ascii(1, 5);
        expected[0][0] = b'1';
        expected[0][1] = b'2';
        expected[0][2] = b'3';
        expected[0][3] = b'4';
        expected[0][4] = b'5';

        let actual = g.into_transpose();
        assert_eq!(actual, expected);
    }

    #[test]
    fn transpose() {
        let mut g = Grid::new_ascii(5, 4);
        for c in 0..4 {
            g[0][c] = b'1';
            g[1][c] = b'2';
            g[2][c] = b'3';
            g[3][c] = b'4';
            g[4][c] = b'5';
        }

        let mut expected = Grid::new_ascii(4, 5);
        for r in 0..expected.shape.0 {
            expected[r][0] = b'1';
            expected[r][1] = b'2';
            expected[r][2] = b'3';
            expected[r][3] = b'4';
            expected[r][4] = b'5';
        }

        let actual = g.into_transpose();
        assert_eq!(actual, expected);
    }

    #[test]
    fn from() {
        let slice: &[[u8; 4]] = [
            [b'1'; 4],
            [b'2'; 4],
            [b'3'; 4],
            [b'4'; 4],
            [b'5'; 4],
        ].as_slice();

        let g = Grid::from(slice);

        let mut expected = Grid::new_ascii(4, 5);
        for r in 0..expected.shape.0 {
            expected[r][0] = b'1';
            expected[r][1] = b'2';
            expected[r][2] = b'3';
            expected[r][3] = b'4';
            expected[r][4] = b'5';
        }

        let actual = g.into_transpose();
        assert_eq!(actual, expected);
    }
}

#[derive(Debug, EnumSetType)]
pub enum Way {
    Up,
    Right,
    Down,
    Left,
}

impl Way {
    pub const fn flipped(&self) -> Self {
        match self {
            Way::Up => Way::Down,
            Way::Right => Way::Left,
            Way::Down => Way::Up,
            Way::Left => Way::Right,
        }
    }

    pub const fn step(&self, pos: (usize, usize)) -> (usize, usize) {
        self.steps(pos, 1)
    }

    pub const fn steps(&self, pos: (usize, usize), steps: usize) -> (usize, usize) {
        match self {
            Way::Up => (pos.0 - steps, pos.1),
            Way::Right => (pos.0, pos.1 + steps),
            Way::Down => (pos.0 + steps, pos.1),
            Way::Left => (pos.0, pos.1 - steps),
        }
    }

    pub const fn axis_changing(&self) -> Axis {
        match self {
            Way::Up | Way::Down => Axis::Row,
            Way::Left | Way::Right => Axis::Column,
        }
    }
}
