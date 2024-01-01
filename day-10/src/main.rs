use std::collections::HashSet;
use std::fs::File;
use aoc::grid::{Way, Ways};

use day_10::{Maze, read_maze, ways_available, maze_pipe_loop};

pub fn part1(maze: &Maze) -> Result<usize, aoc::error::Error> {
    let distance = maze_pipe_loop(maze)?.len();
    Ok(distance.div_ceil(2))
}

pub fn part2(maze: &Maze) -> Result<usize, aoc::error::Error>
{
    let pipe_loop: HashSet<_> = maze_pipe_loop(maze)?.into_iter().collect();

    let mut count = 0;
    for r in 0..maze.shape().0 {
        let mut inside = false;
        for c in 0..maze.shape().1 {
            let ways = if pipe_loop.contains(&(r, c)) {
                ways_available(maze, (r, c))
            } else {
                Ways::empty()
            };

            if ways.is_empty() && inside {
                count += 1;
            }

            if ways.contains(Way::Down) {
                inside = !inside;
            }
        }
    }

    Ok(count)
}

fn main() -> Result<(), aoc::error::Error> {
    let path = aoc::find_input_path("day-10");
    let f = File::open(path)?;
    let maze = read_maze(f)?;

    // Answer: 7107
    let answer = part1(&maze)?;
    println!("Part 1: {answer}");
    // Answer: 281
    let answer = part2(&maze)?;
    println!("Part 2: {answer}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use indoc::indoc;

    const EXAMPLE1_1: &str = indoc! {"
        .....
        .S-7.
        .|.|.
        .L-J.
        .....
    "};

    const EXAMPLE1_2: &str = indoc! {"
        ..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...
    "};

    const EXAMPLE2_1: &str = indoc! {"
        ...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........
    "};

    const EXAMPLE2_2: &str = indoc! {"
        .F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ...
    "};

    const EXAMPLE2_3: &str = indoc! {"
        FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L
    "};

    #[test]
    fn part1_example1() {
        let r = Cursor::new(EXAMPLE1_1);
        let maze = read_maze(r).unwrap();
        let v = part1(&maze).unwrap();
        assert_eq!(v, 4);
    }

    #[test]
    fn part1_example2() {
        let r = Cursor::new(EXAMPLE1_2);
        let maze = read_maze(r).unwrap();
        let v = part1(&maze).unwrap();
        assert_eq!(v, 8);
    }

    #[test]
    fn part2_example1() {
        let r = Cursor::new(EXAMPLE2_1);
        let maze = read_maze(r).unwrap();
        let v = part2(&maze).unwrap();
        assert_eq!(v, 4);
    }

    #[test]
    fn part2_example2() {
        let r = Cursor::new(EXAMPLE2_2);
        let maze = read_maze(r).unwrap();
        let v = part2(&maze).unwrap();
        assert_eq!(v, 8);
    }

    #[test]
    fn part2_example3() {
        let r = Cursor::new(EXAMPLE2_3);
        let maze = read_maze(r).unwrap();
        let v = part2(&maze).unwrap();
        assert_eq!(v, 10);
    }
}
