use std::io::Read;

use enumset::{EnumSet, EnumSetType};

use aoc::grid::{Grid, read_grid};

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
pub type Maze = Grid<Ways>;

pub fn start(maze: &Maze) -> (usize, usize) {
    maze.find(|&tile| tile == Ways::all()).expect("Start missing")
}

pub fn ways_available(maze: &Maze, pos: (usize, usize)) -> Ways {
    let mut ways: Ways = maze[pos];
    for dir in ways {
        let neighbour = dir.step(pos);
        let back_ways = maze[neighbour];
        if !back_ways.contains(dir.flipped()) {
            ways.remove(dir);
        }
    }

    if ways.len() < 2 { Ways::empty() } else { ways }
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
        BLANK => Ways::default(),
        _ => panic!("Invalid tile: {c}")
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
        .unwrap_or_else(|| panic!("Invalid ways: {ways:?}"))
        .1
}

pub fn read_maze<R: Read>(reader: R) -> Result<Maze, aoc::Error> {
    read_grid(reader, Some(Ways::empty()), tile_to_ways)
}
