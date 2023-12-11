use std::fmt::{Display, Formatter};
use std::io::{BufRead, BufReader, Read};
use std::iter::{once, repeat};
use std::ops::Index;
use enumset::{EnumSet, EnumSetType};
use itertools::Itertools;

const BLANK: u8 = b'.';
const START: u8 = b'S';

#[derive(Debug, EnumSetType)]
pub enum Way {
    Up,
    Right,
    Down,
    Left,
}

impl Way {
    pub fn flipped(&self) -> Self {
        match self {
            Way::Up => Way::Down,
            Way::Right => Way::Left,
            Way::Down => Way::Up,
            Way::Left => Way::Right,
        }
    }

    pub fn step(&self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            Way::Up => (pos.0 - 1, pos.1),
            Way::Right => (pos.0, pos.1 + 1),
            Way::Down => (pos.0 + 1, pos.1),
            Way::Left => (pos.0, pos.1 - 1),
        }
    }
}

pub type Ways = EnumSet<Way>;

#[derive(Debug, Default, Clone)]
pub struct Grid {
    cells: Vec<Ways>,
    shape: (usize, usize),
}

impl Grid {
    pub fn shape(&self) -> (usize, usize) {
        self.shape
    }

    pub fn start(&self) -> (usize, usize) {
        self.find(|tile| tile == Ways::all()).expect("Start missing")
    }

    pub fn ways_available(&self, pos: (usize, usize)) -> Ways {
        let mut ways = self[pos];
        for dir in ways {
            let neighbour = dir.step(pos);
            let back_ways = self[neighbour];
            if !back_ways.contains(dir.flipped()) {
                ways.remove(dir);
            }
        }

        if ways.len() < 2 { Ways::default() } else { ways }
    }

    fn find<F>(&self, mut pred: F) -> Option<(usize, usize)>
        where F: FnMut(Ways) -> bool
    {
        let (i, _) = self.cells.iter().enumerate().find(|&(_, &tile)| pred(tile))?;
        Some((i / self.shape.1, i % self.shape.0))
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for c in 0..self.shape.1 {
            for r in 0..self.shape.0 {
                write!(f, "{}", ways_to_tile(self[(r, c)]) as char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Index<usize> for Grid {
    type Output = [Ways];

    fn index(&self, index: usize) -> &Self::Output {
        let start = index * self.shape.1;
        let end = start + self.shape.1;
        &self.cells[start..end]
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = Ways;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.cells[index.0 * self.shape.1 + index.1]
    }
}

fn tile_to_ways(c: u8) -> Ways {
    match c {
        b'|' => Way::Up | Way::Down,
        b'-' => Way::Left | Way::Right,
        b'J' => Way::Up | Way::Left,
        b'L' => Way::Up | Way::Right,
        b'7' => Way::Down | Way::Left,
        b'F' => Way::Down | Way::Right,
        START => Ways::all(),
        _ => Ways::default(),
    }
}

fn ways_to_tile(ways: Ways) -> u8 {
    [
        (Way::Up | Way::Down, b'|'),
        (Way::Left | Way::Right, b'-'),
        (Way::Up | Way::Left, b'J'),
        (Way::Up | Way::Right, b'L'),
        (Way::Down | Way::Left, b'7'),
        (Way::Down | Way::Right, b'F'),
        (Ways::all(), START),
        (Ways::default(), BLANK),
    ].into_iter()
        .find(|&(w, _pipe)| ways == w)
        .unwrap().1
}

pub fn read_grid<R: Read>(reader: R) -> Result<Grid, aoc::Error> {
    // Pad the grid edges with '.'' rows and columns for easier processing
    let mut cells: Vec<Ways> = vec![];
    let mut col_count = 0;
    for line in BufReader::new(reader).lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }

        if cells.is_empty() {
            // First row all padding
            col_count = line.len() + 2;
            let padding = repeat(Ways::default()).take(col_count);
            cells.extend(padding);
        }

        if col_count != line.len() + 2 {
            return Err("Ragged lines".into());
        }

        // First/last column padding
        cells.extend(
            once(Ways::default())
                .chain(line.bytes().map(tile_to_ways))
                .chain(once(Ways::default()))
        );
    }

    let padding: Vec<Ways> = cells.iter().take(col_count).copied().collect_vec();
    cells.extend(padding);
    let shape = (cells.len() / col_count, col_count);
    Ok(Grid { cells, shape })
}
