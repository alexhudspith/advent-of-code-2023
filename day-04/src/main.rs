use std::fs::File;
use std::io::{Read, Seek};
use day_04::{Cards, read_cards};

fn part1(cards: Cards) -> u64 {
    cards.iter()
        .filter(|card| card.win_count != 0)
        .map(|card| 2_u64.pow(card.win_count - 1))
        .sum()
}

fn part2(mut cards: Cards) -> u64 {
    (0..cards.len())
        .map(|i| {
            let (copies, win_count) = (cards[i].copies, cards[i].win_count);
            let cards_after = &mut cards[(i + 1)..][..win_count as usize];
            cards_after.iter_mut().for_each(|c| c.copies += copies);
            copies as u64
        })
        .sum()
}

fn run<R, F>(input: R, mut score: F) -> Result<u64, aoc::error::Error>
    where R: Read, F: FnMut(Cards) -> u64
{
    let cards = read_cards(input)?;
    Ok(score(cards))
}

fn main() -> Result<(), aoc::error::Error> {
    let path = aoc::find_input_path("day-04");
    let mut f = File::open(path)?;

    // Answer: 21105
    let total = run(&f, part1)?;
    println!("Part 1: {}", total);
    f.rewind()?;
    // Answer: 5329815
    let total = run(&f, part2)?;
    println!("Part 2: {}", total);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "};

    #[test]
    fn part1_example() {
        let f = Cursor::new(EXAMPLE);
        let total = run(f, part1).unwrap();
        assert_eq!(total, 13);
    }

    #[test]
    fn part2_example() {
        let f = Cursor::new(EXAMPLE);
        let total = run(f, part2).unwrap();
        assert_eq!(total, 30);
    }
}
