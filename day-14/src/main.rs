use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::{BufReader, Read, Seek};
use std::iter;

use itertools::Itertools;
use aoc::cycle::{Cycle, NoCycle, find_in_cycle};

use aoc::grid::{Axis, read_grid_ascii, Way};

type Grid = aoc::grid::Grid<u8>;

fn iter_grid(grid: &Grid, way: Way) -> impl Iterator<Item=(usize, usize)> {
    let row_iter = 0..grid.len(Axis::Row);
    let col_iter = 0..grid.len(Axis::Column);

    let result: Box<dyn Iterator<Item=_>> = match way {
        // Bottom-right to top-left, reverse direction
        Way::Up | Way::Left => Box::new(
            row_iter.rev().cartesian_product(col_iter.rev())
        ),
        // Top-left to bottom-right, forward direction
        Way::Down | Way::Right => Box::new(
            row_iter.cartesian_product(col_iter)
        ),
    };

    result
}

fn tilt(grid: &mut Grid, way: Way) {
    let tilt_axis = way.axis_changing();
    let mut round_rocks_tracked = vec![0; grid.len(tilt_axis.other())];

    for rc in iter_grid(grid, way) {
        let j = if tilt_axis == Axis::Row { rc.1 } else { rc.0 };
        match grid[rc] {
            b'O' => {
                // Remove but track rock
                round_rocks_tracked[j] += 1;
                grid[rc] = b'.';
            },
            b'#' => {
                // Place tracked rocks
                for i in 1..=round_rocks_tracked[j] {
                    let rock = way.flipped().steps(rc, i);
                    grid[rock] = b'O';
                }

                round_rocks_tracked[j] = 0;
            },
            _ => {},
        }
    }
}

fn calc_load(grid: &Grid, way: Way) -> usize {
    let axis = way.axis_changing();

    iter_grid(grid, way).map(|rc| {
        let i = if axis == Axis::Row { rc.0 } else { rc.1 };
        if grid[rc] == b'O' { grid.len(axis) - i - 1 } else { 0 }
    }).sum()
}

pub fn hash(value: &Grid) -> u64 {
    let mut h = DefaultHasher::new();
    value.hash(&mut h);
    h.finish()
}

fn spin(grid: &mut Grid) {
    for way in [Way::Up, Way::Left, Way::Down, Way::Right] {
        tilt(grid, way);
    }
}

fn part1(mut grid: Grid) -> usize {
    tilt(&mut grid, Way::Up);
    calc_load(&grid, Way::Up)
}

fn part2(mut grid: Grid) -> usize {
    const SPINS: usize = 1_000_000_000;

    let grid_loads = iter::repeat(()).map(|_| {
        spin(&mut grid);
        (hash(&grid), calc_load(&grid, Way::Up))
    });

    match find_in_cycle(grid_loads, SPINS - 1) {
        Ok(Cycle { target_equiv: (_ix, load), ..}) => load,
        Err(NoCycle { target: (_ix, load), ..}) => load,
    }
}

fn run<R: Read, F>(input: R, solve: F) -> Result<usize, aoc::Error>
    where
        R: Read,
        F: FnOnce(Grid) -> usize
{
    let mut reader = BufReader::new(input);
    let grid = read_grid_ascii(&mut reader, Some(b'#'))?;
    Ok(solve(grid))
}

fn main() -> Result<(), aoc::Error> {
    let path = aoc::find_input_path("day-14");
    let mut f = File::open(path)?;

    // Answer: 105982
    let answer = run(&f, part1)?;
    println!("Part 1: {answer}");
    f.rewind()?;
    // Answer: 85175
    let answer = run(&f, part2)?;
    println!("Part 2: {answer}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....
    "};

    #[test]
    fn part1_example() {
        let r = Cursor::new(EXAMPLE);
        let answer = run(r, part1).unwrap();
        assert_eq!(answer, 136);
    }

    #[test]
    fn part2_example() {
        let r = Cursor::new(EXAMPLE);
        let answer = run(r, part2).unwrap();
        assert_eq!(answer, 64);
    }
}
