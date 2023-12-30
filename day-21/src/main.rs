use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Read, Seek};
use std::iter;

use aoc::CollectArray;

use aoc::grid::{read_grid_ascii, Ways};

type Grid = aoc::grid::Grid<u8>;

type Ordinate = isize;
type Coords = (Ordinate, Ordinate);

fn signed((r, c): (usize, usize)) -> Coords {
    (r as Ordinate, c as Ordinate)
}

fn unsigned((r, c): Coords) -> (usize, usize) {
    (r as usize, c as usize)
}

fn wrapped(grid: &Grid, (r, c): Coords) -> (usize, usize) {
    let (rows, cols) = signed(grid.shape());
    let r = r.rem_euclid(rows);
    let c = c.rem_euclid(cols);
    (r as usize, c as usize)
}

fn start(grid: &Grid) -> Coords {
    for (r, row) in grid.iter_rows().enumerate() {
        for (c, &tile) in row.iter().enumerate() {
            if tile == b'S' {
                return signed((r, c));
            }
        }
    }

    panic!("No start tile");
}

fn quadratic_fit(y: [f64; 3]) -> [f64; 3] {
    // y = a₀x² + a₁x + a₂
    // Solve for (a₀, a₁, a₂) given satisfying (x, y) = (0, y₀), (1, y₁), (2, y₂)
    //
    // Output a = V⁻¹y where V is the Vandemonde matrix
    //     ┌          ┐   ┌       ┐
    //     │ x₀² x₀ 1 │   │ 0 0 1 │
    // V = │ x₁² x₁ 1 │ = │ 1 1 1 │
    //     │ x₂² x₂ 1 │   │ 4 2 1 │
    //     └          ┘   └       ┘
    // https://en.wikipedia.org/wiki/Vandermonde_matrix#Applications
    // https://en.wikipedia.org/wiki/Cramer%27s_rule
    let det_v = -2.0;
    let det_v0 = -y[0] + 2.0 * y[1] - y[2];
    let det_v1 = 3.0 * y[0] - 4.0 * y[1] + y[2];
    let det_v2 = -2.0 * y[0];
    [det_v0 / det_v, det_v1 / det_v, det_v2 / det_v]
}

fn bfs_level_size<'g, F, I>(grid: &'g Grid, start: Coords, neighbours: F) -> impl Iterator<Item=usize> + 'g
    where
        F: Fn(&'g Grid, Coords) -> I + 'g,
        I: Iterator<Item=Coords>
{
    let mut level_nodes = HashSet::new();
    level_nodes.insert(start);

    iter::repeat(()).scan(level_nodes, move |next, _| {
        *next = next.iter().flat_map(|&pos| neighbours(grid, pos)).collect();
        Some(next.len())
    })
}

fn neighbours_part1(grid: &Grid, prev: Coords) -> impl Iterator<Item=Coords> + '_ {
    Ways::all().iter()
        .filter_map(move |way| grid.step(unsigned(prev), way))
        .filter(|&pos| grid[pos] != b'#')
        .map(signed)
}

fn neighbours_part2(grid: &Grid, prev: Coords) -> impl Iterator<Item=Coords> + '_ {
    Ways::all().iter()
        .map(move |way| way.step(prev))
        .filter(|&pos| grid[wrapped(grid, pos)] != b'#')
}

fn part1(grid: &Grid, max_dist: usize) -> usize {
    let start = start(grid);
    bfs_level_size(grid, start, neighbours_part1)
        .take(max_dist)
        .last()
        .expect("Expect at least the start position")
}

#[cfg(test)]
fn part2_test(grid: &Grid, max_dist: usize) -> usize {
    let start = start(grid);
    bfs_level_size(grid, start, neighbours_part2)
        .take(max_dist)
        .last()
        .unwrap()
}

fn part2_real(grid: &Grid, max_dist: usize) -> usize {
    let grid_size = grid.shape().0;
    let grid_size_half = grid_size / 2;
    assert_eq!(grid_size, 131, "Part 2 requires specially crafted input 🤷");

    let start = (grid_size_half, grid_size_half);
    assert_eq!(grid[start], b'S', "Start must be in the centre 🤷");

    let y = bfs_level_size(grid, signed(start), neighbours_part2)
        .enumerate()
        .filter(|&(i, _)| (i + 1) % grid_size == grid_size_half)
        .take(3)
        .map(|(_, d)| d as f64)
        .try_collect_array()
        .expect("Expected at least 3 levels");

    let n = ((max_dist - grid_size_half) / grid_size) as Ordinate;
    let a = quadratic_fit(y).map(|v| v as isize);
    let result = a[0] * n * n + a[1] * n + a[2];

    assert!(result >= 0);
    result as usize
}

fn run<R: Read, F>(input: R, solve: F) -> Result<usize, aoc::Error>
    where
        R: Read,
        F: FnOnce(&Grid) -> usize
{
    let mut reader = BufReader::new(input);
    let grid = read_grid_ascii(&mut reader, None)?;
    Ok(solve(&grid))
}

fn main() -> Result<(), aoc::Error> {
    let path = aoc::find_input_path("day-21");
    let mut f = File::open(path)?;
    // Answer: 3746
    let answer = run(&f, |g| part1(g, 64))?;
    println!("Part 1: {answer}");
    f.rewind()?;
    // Answer: 623540829615589
    let answer = run(&f, |g| part2_real(g, 26501365))?;
    println!("Part 2: {answer}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use indoc::indoc;

    const EXAMPLE: &str = indoc!{"
        ...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ...........
    "};

    #[test]
    fn part1_example() {
        let answer = run(Cursor::new(EXAMPLE), |g| part1(g, 6)).unwrap();
        assert_eq!(answer, 16);
    }

    #[test]
    fn part2_example_0006() {
        let answer = run(Cursor::new(EXAMPLE), |g| part2_test(g, 6)).unwrap();
        assert_eq!(answer, 16);
    }

    #[test]
    fn part2_example_0010() {
        let answer = run(Cursor::new(EXAMPLE), |g| part2_test(g, 10)).unwrap();
        assert_eq!(answer, 50);
    }

    #[test]
    fn part2_example_0050() {
        let answer = run(Cursor::new(EXAMPLE), |g| part2_test(g, 50)).unwrap();
        assert_eq!(answer, 1594);
    }

    #[test]
    fn part2_example_0100() {
        let answer = run(Cursor::new(EXAMPLE), |g| part2_test(g, 100)).unwrap();
        assert_eq!(answer, 6536);
    }
}
