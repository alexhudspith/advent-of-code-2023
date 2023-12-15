use std::collections::HashMap;
use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::{BufReader, Read, Seek};

use itertools::Itertools;

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
    let mut load = vec![0; grid.len(axis.other())];
    let mut total_load = 0;

    for rc in iter_grid(grid, way) {
        let (i, j) = if axis == Axis::Row { rc } else { (rc.1, rc.0) };
        match grid[rc] {
            b'O' => {
                load[j] += grid.len(axis) - i - 1;
            },
            b'#' => {
                total_load += load[j];
                load[j] = 0;
            },
            _ => {},
        }
    }

    total_load
}

pub fn hash<T: Hash>(value: &T) -> u64 {
    let mut h = DefaultHasher::new();
    value.hash(&mut h);
    h.finish()
}

fn spin(grid: &mut Grid) {
    tilt(grid, Way::Up);
    tilt(grid, Way::Left);
    tilt(grid, Way::Down);
    tilt(grid, Way::Right);
}

fn part1(mut grid: Grid) -> usize {
    tilt(&mut grid, Way::Up);
    calc_load(&grid, Way::Up)
}

fn part2(mut grid: Grid) -> usize {
    const SPINS: usize = 1_000_000_000;

    let mut grids_seen: HashMap<_, (usize, usize)> = HashMap::new();
    let (mut cycle_start, mut cycle_end) = (0, SPINS);
    for j in 0..SPINS {
        spin(&mut grid);
        let h = hash(&grid);
        let load = calc_load(&grid, Way::Up);
        if let Some((i, _)) = grids_seen.insert(h, (j, load)) {
            (cycle_start, cycle_end) = (i, j);
            break;
        }
    }

    let cycle_len = cycle_end - cycle_start;
    let target_ix = ((SPINS - 1 - cycle_start) % cycle_len) + cycle_start;
    eprintln!("Cycle at {cycle_start:?} -> {cycle_end:?}, need {target_ix:?}");

    let &(_, load) = grids_seen.values()
        .find(|(i, _load)| *i == target_ix)
        .unwrap();
    load
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
