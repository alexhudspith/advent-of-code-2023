use std::fs::File;
use std::io::{BufReader, Read, Seek};
use aoc::grid::{Grid, read_grid, Way, Ways};

type Tiles = Grid<Tile>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Tile {
    Border = b'X',
    Blank = b'.',
    VSplit = b'|',
    HSplit = b'-',
    Slash = b'/',
    Backslash = b'\\',
}

impl Tile {
    pub const fn all() -> [Tile; 6] {
        [
            Tile::Border,
            Tile::Blank,
            Tile::VSplit,
            Tile::HSplit,
            Tile::Slash,
            Tile::Backslash
        ]
    }

    pub fn next_ways(&self, way: Way) -> Ways {
        match self {
            Tile::Border => Ways::empty(),
            Tile::Blank => Ways::from(way),
            Tile::VSplit => Way::Up | Way::Down,
            Tile::HSplit => Way::Left | Way::Right,
            Tile::Slash => Ways::from(way.mirror_45_pos()),
            Tile::Backslash => Ways::from(way.mirror_45_neg()),
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

fn energized(history: &Grid<Ways>) -> usize {
    history.iter_rows()
        .flat_map(|row| row.iter())
        .filter(|ways| !ways.is_empty())
        .count()
}

fn solve(tiles: &Tiles, pos: (usize, usize), way: Way) -> usize {
    let mut history: Grid<Ways> = Grid::new(tiles.shape());
    let mut stack = vec![];
    stack.push((pos, way));

    while let Some((pos, way)) = stack.pop() {
        let tile = tiles[pos];
        let ways = tile.next_ways(way);
        if ways.is_empty() || !history[pos].insert(way) {
            continue;
        }

        for w in ways {
            stack.push((w.step(pos), w));
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
        .map(|(pos, way)| solve(tiles, pos, way))
        .max()
        .unwrap_or(0)
}

fn run<R: Read, F>(input: R, solve: F) -> Result<usize, aoc::Error>
    where
        F: FnOnce(&Tiles) -> usize
{
    let mut reader = BufReader::new(input);
    let tiles = read_grid(&mut reader, Some(Tile::Border))?;
    Ok(solve(&tiles))
}

fn main() -> Result<(), aoc::Error> {
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
