use std::io::Read;

use enumset::{EnumSet, EnumSetType};

use aoc::grid::{Grid, read_grid_with_transform};

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
    maze.position(|&tile| tile == Ways::all()).expect("Start missing")
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
        BLANK => Ways::empty(),
        _ => panic!("Invalid tile: {c}")
    }
}

#[allow(dead_code)]
fn ways_to_tile(ways: Ways) -> u8 {
    [
        (Way::Up | Way::Down, b'|'),
        (Way::Left | Way::Right, b'-'),
        (Way::Up | Way::Left, b'J'),
        (Way::Up | Way::Right, b'L'),
        (Way::Down | Way::Left, b'7'),
        (Way::Down | Way::Right, b'F'),
        (Ways::all(), START),
        (Ways::empty(), BLANK),
    ].into_iter()
        .find(|&(w, _pipe)| ways == w)
        .unwrap_or_else(|| panic!("Invalid ways: {ways:?}"))
        .1
}

fn ways_to_graphic(ways: Ways) -> char {
    [
        (Way::Up | Way::Down, '│'),
        (Way::Left | Way::Right, '─'),
        (Way::Up | Way::Left, '┘'),
        (Way::Up | Way::Right, '└'),
        (Way::Down | Way::Left, '┐'),
        (Way::Down | Way::Right, '┌'),
        (Ways::all(), '▒'),
        (Ways::empty(), ' '),
    ].into_iter()
        .find(|&(w, _pipe)| ways == w)
        .unwrap_or_else(|| panic!("Invalid ways: {ways:?}"))
        .1
}

pub fn maze_pipe_loop(maze: &Maze) -> Result<Vec<(usize, usize)>, aoc::Error> {
    let mut main_loop = Vec::new();
    let start = start(maze);
    let mut pos = start;
    let mut prev_way: Option<Way> = None;
    loop {
        main_loop.push(pos);

        let mut ways = ways_available(maze, pos);
        if let Some(last_direction) = prev_way {
            ways.remove(last_direction.flipped());
        }

        if start != pos && ways.len() != 1 {
            return Err(format!("Expect one way, have {:?}", ways).into());
        }

        let way = ways.into_iter().next().unwrap();
        pos = way.step(pos);
        if pos == start {
            break;
        }

        prev_way = Some(way);
    }

    Ok(main_loop)
}

pub fn read_maze<R: Read>(reader: R) -> Result<Maze, aoc::Error> {
    read_grid_with_transform(reader, Some(Ways::empty()), tile_to_ways, |&w| ways_to_graphic(w))
}
