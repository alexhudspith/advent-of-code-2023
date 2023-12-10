use std::fmt::{Display, Formatter};
use std::io::{BufRead, BufReader, Read};
use std::iter::{once, repeat};
use enumset::{EnumSet, EnumSetType};

use aoc::aoc_err;

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
    pub rows: Vec<Vec<Ways>>,
}

impl Grid {
    pub fn shape(&self) -> (usize, usize) {
        (self.rows.len(), self.rows[0].len())
    }

    pub fn start(&self) -> (usize, usize) {
        self.find(|tile| tile == Ways::all()).expect("Start missing")
    }

    pub fn ways_available(&self, pos: (usize, usize)) -> Ways {
        let mut ways = self.ways_in_grid(pos);
        for dir in ways {
            let neighbour = dir.step(pos);
            let back_ways = self.ways_in_grid(neighbour);
            if !back_ways.contains(dir.flipped()) {
                ways.remove(dir);
            }
        }

        if ways.len() < 2 { Ways::default() } else { ways }
    }

    fn ways_in_grid(&self, pos: (usize, usize)) -> Ways {
        self.rows[pos.0][pos.1]
    }

    fn find<F>(&self, mut pred: F) -> Option<(usize, usize)>
        where F: FnMut(Ways) -> bool
    {
        self.rows.iter().enumerate()
            .map(|(r, row)| (r, row.iter().position(|&tile| pred(tile))))
            .find(|(_, c_opt)| c_opt.is_some())
            .map(|(r, c_opt)| (r, c_opt.unwrap()))
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.rows.iter() {
            for &tile in row.iter() {
                write!(f, "{}", ways_to_tile(tile) as char)?;
            }
            writeln!(f)?;
        }
        Ok(())
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
    let mut rows: Vec<Vec<Ways>> = vec![vec![]];

    for line in BufReader::new(reader).lines() {
        let line = line?;
        if !line.is_empty() {
            let padded_row: Vec<Ways> = once(Ways::default())
                .chain(line.bytes().map(tile_to_ways))
                .chain(once(Ways::default()))
                .collect();
            rows.push(padded_row);
        }
    }

    let col_count = rows.get(1).ok_or_else(|| aoc_err("Empty grid"))?.len();
    let padding_row: Vec<Ways> = repeat(Ways::default()).take(col_count).collect();
    rows[0] = padding_row.clone();
    rows.push(padding_row);

    Ok(Grid { rows })
}
