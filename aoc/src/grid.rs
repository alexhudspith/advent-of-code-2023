#![allow(clippy::redundant_field_names)]

use std::{fmt, iter};
use std::borrow::Borrow;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::io::BufRead;
use std::iter::repeat;
use std::ops::{Index, IndexMut};
use std::rc::Rc;
use std::str::FromStr;

use enumset::{EnumSet, EnumSetType};
use itertools::Itertools;

use crate as aoc;
use aoc::infallible;

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

pub type GridDisplay<T> = dyn Fn(&Grid<T>, &mut Formatter) -> fmt::Result;

#[derive(Clone)]
pub struct Grid<T=u8> {
    shape: (usize, usize),
    cells: Vec<T>,
    display: Rc<GridDisplay<T>>,
}

impl<T: Hash> Hash for Grid<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.shape.hash(state);
        self.cells.hash(state);
    }
}

impl Grid<u8> {
    pub fn new_ascii((rows, cols): (usize, usize)) -> Self {
        let cells = vec![b'.'; rows * cols];
        Self {
            shape: (rows, cols),
            cells: cells,
            display: Rc::new(ascii_fmt),
        }
    }
}

impl Grid<Option<Way>> {
    pub fn new_way_opt((rows, cols): (usize, usize)) -> Self {
        let cells = vec![None; rows * cols];

        Self {
            shape: (rows, cols),
            cells: cells,
            display: Rc::new(option_fmt),
        }
    }
}

impl Grid<Ways> {
    pub fn new_ways((rows, cols): (usize, usize)) -> Self {
        let cells = vec![Ways::empty(); rows * cols];

        Self {
            shape: (rows, cols),
            cells: cells,
            display: Rc::new(display_fmt),
        }
    }
}

impl<T> Grid<T> {
    pub fn new((rows, cols): (usize, usize)) -> Self
        where
            T: Default + Display + 'static,
    {
        let cells = iter::repeat_with(T::default).take(rows * cols).collect();
        Self {
            shape: (rows, cols),
            cells: cells,
            display: Rc::new(display_fmt),
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
        self.cells.chunks_exact(self.shape.1)
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

    pub fn fill(&mut self, value: T) where T: Clone {
        self.cells.fill(value);
    }

    pub fn step<I: num::PrimInt>(&self, pos: (I, I), way: Way) -> Option<(I, I)> {
        self.steps(pos, way, I::one())
    }

    pub fn steps<I: num::PrimInt>(&self, (r, c): (I, I), way: Way, count: I) -> Option<(I, I)> {
        let ru: usize = num::cast(r).unwrap();
        let cu: usize = num::cast(c).unwrap();

        match way {
            Way::Up if ru == 0 => None,
            Way::Left if cu == 0 => None,
            Way::Down if ru >= self.shape.0 - 1 => None,
            Way::Right if cu >= self.shape.1 - 1 => None,
            _ => Some(way.steps((r, c), count))
        }
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
        (self.display)(self, f)
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
    where
        R: Borrow<[T]>, T: Clone + Display + 'static
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
            display: Rc::new(display_fmt),
        }
    }
}


fn ascii_fmt(grid: &Grid<u8>, f: &mut Formatter) -> fmt::Result {
    let rows = grid.iter_rows().map(|row| String::from_utf8_lossy(row));
    if f.alternate() {
        for row in rows {
            writeln!(f, "{:#}", row)?;
        }
    } else {
        for row in rows {
            writeln!(f, "{}", row)?;
        }
    }
    Ok(())
}

fn option_fmt<T: Display>(grid: &Grid<Option<T>>, f: &mut Formatter) -> fmt::Result {
    if f.alternate() {
        for row in grid.iter_rows() {
            for cell_opt in row {
                if let Some(cell) = cell_opt {
                    write!(f, "{:#}", cell)?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
    } else {
        for row in grid.iter_rows() {
            for cell_opt in row {
                if let Some(cell) = cell_opt {
                    write!(f, "{}", cell)?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
    }
    Ok(())
}

fn display_fmt<T: Display>(grid: &Grid<T>, f: &mut Formatter) -> fmt::Result {
    if f.alternate() {
        for row in grid.iter_rows() {
            for cell in row {
                write!(f, "{:#}", cell)?;
            }
            writeln!(f)?;
        }
    } else {
        for row in grid.iter_rows() {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
    }
    Ok(())
}

pub fn read_grid_ascii<R>(reader: &mut R, padding: Option<u8>) -> Result<Grid<u8>, aoc::Error>
    where R: BufRead
{
    read_grid_with_transform(reader, padding, infallible(std::convert::identity), ascii_fmt)
}

pub fn read_grid<R, T>(reader: &mut R, padding: Option<T>) -> Result<Grid<T>, aoc::Error>
    where
        R: BufRead,
        T: Display + Clone + TryFrom<u8> + 'static,
        aoc::Error: From<T::Error>,
{
    read_grid_with_transform(reader, padding, T::try_from, display_fmt)
}

// Optionally pad the grid edges with `padding` rows and columns for easier processing
pub fn read_grid_with_transform<R, T, E, F, D>(
    reader: &mut R,
    padding_value: Option<T>,
    mut transform: F,
    display: D
) -> Result<Grid<T>, aoc::Error>
    where
        R: BufRead,
        T: Clone,
        F: FnMut(u8) -> Result<T, E>,
        aoc::Error: From<E>,
        D: 'static + Fn(&Grid<T>, &mut Formatter<'_>) -> fmt::Result,
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
            line.bytes().map(&mut transform).process_results(|ts| cells.extend(ts))?;
            cells.push(padding.clone());
        } else {
            line.bytes().map(&mut transform).process_results(|ts| cells.extend(ts))?;
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
    Ok(Grid { cells, shape, display: Rc::new(display) })
}

#[cfg(test)]
mod tests {
    use crate::grid::Grid;

    #[test]
    fn transpose_row() {
        let mut g = Grid::new((1, 4));
        g[0][0] = 1;
        g[0][1] = 2;
        g[0][2] = 3;
        g[0][3] = 4;

        let mut expected = Grid::new((4, 1));
        expected[0][0] = 1;
        expected[1][0] = 2;
        expected[2][0] = 3;
        expected[3][0] = 4;

        let actual = g.into_transpose();
        assert_eq!(actual, expected);
    }

    #[test]
    fn transpose_col() {
        let mut g = Grid::new_ascii((5, 1));
        g[0][0] = b'1';
        g[1][0] = b'2';
        g[2][0] = b'3';
        g[3][0] = b'4';
        g[4][0] = b'5';

        let mut expected = Grid::new_ascii((1, 5));
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
        let mut g = Grid::new_ascii((5, 4));
        for c in 0..4 {
            g[0][c] = b'1';
            g[1][c] = b'2';
            g[2][c] = b'3';
            g[3][c] = b'4';
            g[4][c] = b'5';
        }

        let mut expected = Grid::new_ascii((4, 5));
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

        let mut expected = Grid::new_ascii((4, 5));
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

pub type Ways = EnumSet<Way>;

#[derive(Debug, Hash, PartialOrd, Ord, EnumSetType)]
pub enum Way {
    Up,
    Right,
    Down,
    Left,
}

impl Way {
    pub fn all() -> Ways {
        Ways::all()
    }

    pub fn horizontals() -> Ways {
        Way::Left | Way::Right
    }

    pub fn verticals() -> Ways {
        Way::Up | Way::Down
    }

    pub const fn flipped(&self) -> Self {
        match self {
            Way::Up => Way::Down,
            Way::Right => Way::Left,
            Way::Down => Way::Up,
            Way::Left => Way::Right,
        }
    }

    pub const fn is_horizontal(&self) -> bool {
        // match for constness...
        match self {
            Way::Up => false,
            Way::Right => true,
            Way::Down => false,
            Way::Left => true,
        }
    }

    pub const fn is_vertical(&self) -> bool {
        !self.is_horizontal()
    }

    pub fn step<I: num::PrimInt>(&self, pos: (I, I)) -> (I, I) {
        self.steps(pos, I::one())
    }

    pub fn steps<I: num::PrimInt>(&self, (r, c): (I, I), count: I) -> (I, I) {
        match self {
            Way::Up => (r - count, c),
            Way::Right => (r, c + count),
            Way::Down => (r + count, c),
            Way::Left => (r, c - count),
        }
    }

    pub const fn axis_changing(&self) -> Axis {
        match self {
            Way::Up | Way::Down => Axis::Row,
            Way::Left | Way::Right => Axis::Column,
        }
    }

    pub const fn rotate_cw(&self) -> Self {
        match self {
            Way::Up => Way::Right,
            Way::Right => Way::Down,
            Way::Down => Way::Left,
            Way::Left => Way::Up,
        }
    }

    pub const fn rotate_ccw(&self) -> Self {
        match self {
            Way::Up => Way::Left,
            Way::Right => Way::Up,
            Way::Down => Way::Right,
            Way::Left => Way::Down,
        }
    }

    pub const fn mirror_45_pos(&self) -> Self {
        match self {
            Way::Up => Way::Right,
            Way::Right => Way::Up,
            Way::Down => Way::Left,
            Way::Left => Way::Down,
        }
    }

    pub const fn mirror_45_neg(&self) -> Self {
        match self {
            Way::Up => Way::Left,
            Way::Right => Way::Down,
            Way::Down => Way::Right,
            Way::Left => Way::Up,
        }
    }
}

impl TryFrom<char> for Way {
    type Error = char;

    fn try_from(value: char) -> Result<Way, char> {
        let result = match value {
            'U' | '↑' => Way::Up,
            'R' | '→' => Way::Right,
            'D' | '↓' => Way::Down,
            'L' | '←' => Way::Left,
            _ => { return Err(value); }
        };

        Ok(result)
    }
}

impl FromStr for Way {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let only: char = s.chars().exactly_one().map_err(|_| s)?;
        let result = Way::try_from(only)?;
        Ok(result)
    }
}

impl From<Way> for char {
    fn from(value: Way) -> Self {
        match value {
            Way::Up => '↑',
            Way::Right => '→',
            Way::Down => '↓',
            Way::Left => '←',
        }
    }
}

impl From<&Way> for char {
    fn from(value: &Way) -> Self {
        Self::from(*value)
    }
}

impl Display for Way {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let chr = if f.alternate() {
            format!("{:?}", self).chars().next().unwrap()
        } else {
            char::from(self)
        };
        write!(f, "{}", chr)
    }
}


#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct WayMap<V>([Option<V>; 4]);

impl<V> WayMap<V> {
    pub fn new() -> Self {
        Self([None, None, None, None])
    }

    pub fn with_all_default() -> Self where V: Default {
        Self([Some(V::default()), Some(V::default()), Some(V::default()), Some(V::default())])
    }

    pub fn with_all(v: V) -> Self where V: Clone {
        Self([Some(v.clone()), Some(v.clone()), Some(v.clone()), Some(v)])
    }

    pub fn len(&self) -> usize {
        self.0.iter().flatten().count()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn insert(&mut self, k: Way, v: V) -> bool {
        let bucket = &mut self.0[k as usize];
        let result = bucket.is_none();
        *bucket = Some(v);
        result
    }

    pub fn get(&self, k: &Way) -> Option<&V> {
        self.0[*k as usize].as_ref()
    }

    pub fn get_mut(&mut self, k: &Way) -> Option<&mut V> {
        self.0[*k as usize].as_mut()
    }

    pub fn iter(&self) -> impl Iterator<Item=(Way, &V)> {
        Ways::all().iter()
            .filter_map(|w|
                self.0[w as usize].as_ref().map(|v| (w, v))
            )
    }

    pub fn keys(&self) -> impl Iterator<Item=Way> + '_ {
        self.iter().map(|(k, _)| k)
    }

    pub fn values(&self) -> impl Iterator<Item=&V> + '_ {
        self.iter().map(|(_, v)| v)
    }
}

impl<V> Index<Way> for WayMap<V> {
    type Output = V;

    fn index(&self, index: Way) -> &Self::Output {
        self.get(&index).unwrap_or_else(|| panic!("No such key"))
    }
}

impl<V> IndexMut<Way> for WayMap<V> {
    fn index_mut(&mut self, index: Way) -> &mut Self::Output {
        self.get_mut(&index).unwrap_or_else(|| panic!("No such key"))
    }
}
