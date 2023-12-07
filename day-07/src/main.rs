use std::fs::File;
use std::io::{Read, Seek};
use day_07::*;

fn run<R: Read>(input: R, use_jokers: bool) -> Result<u32, aoc::Error> {
    let mut hand_bids = read_hand_bids(input, use_jokers)?;
    hand_bids.sort_by_key(|&(hand, _)| hand);

    let total = hand_bids.iter().enumerate()
        .map(|(i, (_, bid))| (i + 1) as u32 * bid)
        .sum();

    Ok(total)
}

// Answer: 248569531
fn part1<R: Read>(input: R) -> Result<u32, aoc::Error> {
    run(input, false)
}

// Answer: 250382098
fn part2<R: Read>(input: R) -> Result<u32, aoc::Error> {
    run(input, true)
}

fn main() -> Result<(), aoc::Error> {
    let path = aoc::find_input_path("day-07");
    let mut f = File::open(path)?;

    let answer = part1(&f)?;
    println!("Part 1: {answer}");
    f.rewind()?;
    let answer = part2(&f)?;
    println!("Part 2: {answer}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "};

    #[test]
    fn part1_example() {
        let total = part1(Cursor::new(EXAMPLE)).unwrap();
        assert_eq!(total, 6440);
    }

    #[test]
    fn part2_example() {
        let total = part2(Cursor::new(EXAMPLE)).unwrap();
        assert_eq!(total, 5905);
    }
}
