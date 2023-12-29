use std::fs::File;
use std::io::{Read, Seek};

use day_23::{Coords, Grid, Ordinate, Tile, read_grid, reduce_grid, part1_longest_path, part2_longest_path};

pub fn start(grid: &Grid) -> Coords {
    let r = 0;
    let c = grid[r].iter().position(|&tile| tile == Tile::Path).expect("No start tile");
    (r as Ordinate, c as Ordinate)
}

pub fn end(grid: &Grid) -> Coords {
    let r = grid.shape().0 - 1;
    let c = grid[r].iter().rposition(|&tile| tile == Tile::Path).expect("No end tile");
    (r as Ordinate, c as Ordinate)
}

fn part1<R: Read>(input: R) -> Result<u16, aoc::Error> {
    let grid = read_grid(input)?;
    let (start, end) = (start(&grid), end(&grid));
    let answer = part1_longest_path(&grid, start, end);

    Ok(answer)
}

fn part2<R: Read>(input: R) -> Result<u16, aoc::Error> {
    let grid = read_grid(input)?;
    let start = start(&grid);
    let end = end(&grid);
    let (graph, start, end) = reduce_grid(&grid, start, end);
    let answer = part2_longest_path(&graph, start, end);
    Ok(answer)
}

fn main() -> Result<(), aoc::Error> {
    let path = aoc::find_input_path("day-23");
    let mut f = File::open(path)?;
    // Answer: 2238
    let answer = part1(&f)?;
    println!("Part 1: {answer}");
    f.rewind()?;
    // Answer: 6398
    let answer = part2(&f)?;
    println!("Part 2: {answer}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use indoc::indoc;

    const EXAMPLE: &str = indoc!{"
        #.#####################
        #.......#########...###
        #######.#########.#.###
        ###.....#.>.>.###.#.###
        ###v#####.#v#.###.#.###
        ###.>...#.#.#.....#...#
        ###v###.#.#.#########.#
        ###...#.#.#.......#...#
        #####.#.#.#######.#.###
        #.....#.#.#.......#...#
        #.#####.#.#.#########v#
        #.#...#...#...###...>.#
        #.#.#v#######v###.###v#
        #...#.>.#...>.>.#.###.#
        #####v#.#.###v#.#.###.#
        #.....#...#...#.#.#...#
        #.#########.###.#.#.###
        #...###...#...#...#.###
        ###.###.#.###v#####v###
        #...#...#.#.>.>.#.>.###
        #.###.###.#.###.#.#v###
        #.....###...###...#...#
        #####################.#
    "};

    const EXAMPLE_REDUCED: &str = indoc!{"
        #.#####################
        #.......#########...###
        #######.#########.#.###
        ###.....#.........#.###
        ###.#####.###.#####.###
        ###...................#
        #####################.#
    "};

    #[test]
    fn part1_example() {
        let answer = part1(Cursor::new(EXAMPLE)).unwrap();
        assert_eq!(answer, 94);
    }

    #[test]
    fn part2_example() {
        let answer = part2(Cursor::new(EXAMPLE)).unwrap();
        assert_eq!(answer, 154);
    }

    #[test]
    fn part2_example_reduced() {
        let answer = part2(Cursor::new(EXAMPLE_REDUCED)).unwrap();
        assert_eq!(answer, 42);
    }
}
