use std::fs::File;
use std::io::{Read, Seek};

use day_22::read_bricks;

fn part1<R: Read>(input: R) -> Result<u64, aoc::Error> {
    let mut bricks = read_bricks(input)?;
    bricks.settle();

    let total = bricks.iter()
        .filter(|&support| bricks.falls_without(support).is_empty())
        .count();

    Ok(total as u64)
}

fn part2<R: Read>(input: R) -> Result<u64, aoc::Error> {
    let mut bricks = read_bricks(input)?;
    bricks.settle();

    let total: usize = bricks.iter()
        .map(|brick| {
            bricks.falling(Some(brick))
                .filter(|&(_, fall)| fall > 0)
                .count()
        })
        .sum();

    Ok(total as u64)
}

fn main() -> Result<(), aoc::Error> {
    let path = aoc::find_input_path("day-22");
    let mut f = File::open(path)?;
    // Answer: 430
    let answer = part1(&f)?;
    println!("Part 1: {answer}");
    f.rewind()?;
    // Answer: 60558
    let answer = part2(&f)?;
    println!("Part 2: {answer}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use indoc::indoc;

    const EXAMPLE: &str = indoc!{r"
        1,0,1~1,2,1
        0,0,2~2,0,2
        0,2,3~2,2,3
        0,0,4~0,2,4
        2,0,5~2,2,5
        0,1,6~2,1,6
        1,1,8~1,1,9
    "};

    #[test]
    fn part1_example() {
        let answer = part1(Cursor::new(EXAMPLE)).unwrap();
        assert_eq!(answer, 5);
    }

    #[test]
    fn part2_example() {
        let answer = part2(Cursor::new(EXAMPLE)).unwrap();
        assert_eq!(answer, 7);
    }
}
