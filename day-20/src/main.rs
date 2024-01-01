use std::fs::File;
use std::io::{Read, Seek};
use day_20::parse::read_system;

fn part1<R: Read + Seek>(input: R) -> Result<usize, aoc::Error> {
    let mut system = read_system(input)?;
    let low_high = system.run_part1(1000);
    Ok(low_high.low * low_high.high)
}

fn part2<R: Read + Seek>(input: R) -> Result<usize, aoc::Error> {
    let mut system = read_system(input)?;
    let answer = system.run_part2();
    Ok(answer)
}

fn main() -> Result<(), aoc::Error> {
    let path = aoc::find_input_path("day-20");
    let mut f = File::open(path)?;
    // Answer: 777666211
    let answer = part1(&f)?;
    println!("Part 1: {answer}");
    f.rewind()?;
    // Answer: 243081086866483
    let answer = part2(&f)?;
    println!("Part 2: {answer}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use indoc::indoc;

    const EXAMPLE1: &str = indoc!{r"
        broadcaster -> a, b, c
        %a -> b
        %b -> c
        %c -> inv
        &inv -> a
    "};

    const EXAMPLE2: &str = indoc!{r"
        broadcaster -> a
        %a -> inv, con
        &inv -> b
        %b -> con
        &con -> output
    "};

    #[test]
    fn part1_example1() {
        let answer = part1(Cursor::new(EXAMPLE1)).unwrap();
        assert_eq!(answer, 32000000);
    }

    #[test]
    fn part1_example2() {
        let answer = part1(Cursor::new(EXAMPLE2)).unwrap();
        assert_eq!(answer, 11687500);
    }
}
