use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufReader, Read, Seek};
use enumset::{EnumSet, EnumSetType};
use aoc::grid::{Grid, read_grid, Way, Ways};

type Tiles = Grid<Tile>;

#[derive(Debug, EnumSetType)]
enum Pathway {
    A, B
}

impl From<bool> for Pathway {
    fn from(value: bool) -> Self {
        if value { Self::A } else { Self::B }
    }
}

impl Display for Pathway {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if *self == Self::A { 'A' } else { 'B' })
    }
}

type Pathways = EnumSet<Pathway>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Tile {
    Border = b'X',
    Blank = b'.',
    VSplit = b'|',
    HSplit = b'-',
    FwdMirror = b'/',
    BackMirror = b'\\',
}

impl Tile {
    pub const fn all() -> [Tile; 6] {
        [Tile::Border, Tile::Blank, Tile::VSplit, Tile::HSplit, Tile::FwdMirror, Tile::BackMirror]
    }

    pub fn ways_out(&self, way_in: Way) -> (Ways, Pathway) {
        match self {
            // A if beam splits, B if straight through
            Tile::VSplit if way_in.is_horizontal() => (Way::verticals(), Pathway::A),
            Tile::HSplit if way_in.is_vertical() => (Way::horizontals(), Pathway::A),
            Tile::VSplit => (way_in.into(), Pathway::B),
            Tile::HSplit => (way_in.into(), Pathway::B),
            // A if hit top side, B if hit bottom side
            Tile::FwdMirror => {
                let pathway_a = (Way::Right | Way::Down).contains(way_in);
                (way_in.mirror_45_pos().into(), pathway_a.into())
            }
            Tile::BackMirror => {
                let pathway_a = (Way::Left | Way::Down).contains(way_in);
                (way_in.mirror_45_neg().into(), pathway_a.into())
            }
            // A if runs vertical, B if runs horizontal
            Tile::Blank => (way_in.into(), way_in.is_vertical().into()),
            Tile::Border => (Ways::empty(), way_in.is_vertical().into()),
        }
    }
}

impl TryFrom<u8> for Tile {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        for t in Tile::all() {
            if t as u8 == value {
                return Ok(t);
            }
        }
        Err(value)
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8 as char)
    }
}

fn energized(history: &Grid<Pathways>) -> usize {
    history.iter_rows()
        .flat_map(|row| row.iter())
        .filter(|sides| !sides.is_empty())
        .count()
}

fn solve(tiles: &Tiles, pos: (usize, usize), way_in: Way) -> usize {
    let mut history: Grid<Pathways> = Grid::new(tiles.shape());
    let mut stack = vec![];
    stack.push((pos, way_in));

    'stack:
    while let Some((mut pos, mut way_in)) = stack.pop() {
        let (mut ways_out, mut side) = tiles[pos].ways_out(way_in);
        while ways_out.len() == 1 {
            // Optimized path
            if !history[pos].insert(side) {
                continue 'stack;
            }
            way_in = ways_out.iter().next().unwrap();
            pos = way_in.step(pos);
            (ways_out, side) = tiles[pos].ways_out(way_in);
        }

        if ways_out.is_empty() || !history[pos].insert(side) {
            continue;
        }

        for way_out in ways_out {
            stack.push((way_out.step(pos), way_out));
        }
    }

    energized(&history)
}

fn perimeter((rows, cols): (usize, usize)) -> impl Iterator<Item=((usize, usize), Way)> {
    let top = (1..cols - 1).map(move |c| ((1, c), Way::Down));
    let bottom = (1..cols - 1).map(move |c| ((rows - 2, c), Way::Up));
    let left = (1..rows - 1).map(move |r| ((r, 1), Way::Right));
    let right = (1..rows - 1).map(move |r| ((r, cols - 2), Way::Left));

    top.chain(bottom).chain(left).chain(right)
}

fn part1(tiles: &Tiles) -> usize {
    solve(tiles, (1, 1), Way::Right)
}

fn part2(tiles: &Tiles) -> usize {
    perimeter(tiles.shape())
        .map(|(pos, way_in)| solve(tiles, pos, way_in))
        .max()
        .unwrap_or(0)
}

fn run<R: Read, F>(input: R, solve: F) -> Result<usize, aoc::error::Error>
    where
        F: FnOnce(&Tiles) -> usize
{
    let mut reader = BufReader::new(input);
    let tiles = read_grid(&mut reader, Some(Tile::Border))?;
    Ok(solve(&tiles))
}

fn main() -> Result<(), aoc::error::Error> {
    let path = aoc::find_input_path("day-16");
    let mut f = File::open(path)?;

    // Answer: 7860
    let answer = run(&f, part1)?;
    println!("Part 1: {answer}");
    f.rewind()?;
    // Answer: 8331
    let answer = run(&f, part2)?;
    println!("Part 2: {answer}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use indoc::indoc;

    const EXAMPLE: &str = indoc!{r"
        .|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|....
    "};

    #[test]
    fn part1_example() {
        let answer = run(Cursor::new(EXAMPLE), part1).unwrap();
        assert_eq!(answer, 46);
    }

    #[test]
    fn part2_example() {
        let answer = run(Cursor::new(EXAMPLE), part2).unwrap();
        assert_eq!(answer, 51);
    }
}
