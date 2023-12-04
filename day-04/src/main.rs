use std::fs::File;
use std::io;
use std::io::{Read, Seek};
use std::path::{Path, PathBuf};

use day_04::Game;

pub fn data_dir() -> PathBuf {
    Path::new(file!()).ancestors().nth(2).unwrap().join("data")
}

fn run<R, F>(input: R, mut game_score: F) -> io::Result<u64>
    where R: Read, F: FnMut(Game) -> u64
{
    let game = Game::read(input)?;
    Ok(game_score(game))
}

// Answer: 21105
fn part1(game: Game) -> u64 {
    game.cards.iter().fold(0, |acc, card| {
        acc + if card.win_count == 0 { 0 } else { 2_u64.pow(card.win_count - 1) }
    })
}

// Answer: 5329815
fn part2(mut game: Game) -> u64 {
    loop {
        let mut won_cards = false;
        for i in 0..game.cards.len() {
            let (cards_before_incl, cards_after) = game.cards.split_at_mut(i + 1);
            let card = cards_before_incl.iter_mut().last().unwrap();
            let copies = card.copies_left;
            if copies != 0 {
                card.copies_kept += copies;
                card.copies_left = 0;
            }
            let win_count = if copies == 0 { 0 } else { card.win_count };
            if win_count == 0 {
                continue;
            }

            won_cards = true;
            for c in &mut cards_after[..win_count as usize] {
                c.copies_left += copies;
            }
        };

        if !won_cards {
            break;
        }
    }

    game.cards.iter().map(|card| card.copies_kept as u64).sum()
}

fn main() -> Result<(), anyhow::Error> {
    let mut f = File::open(data_dir().join("input.txt"))?;

    let total = run(&f, part1)?;
    println!("Part 1: {}", total);
    f.rewind()?;
    let total = run(&f, part2)?;
    println!("Part 2: {}", total);

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use indoc::indoc;

    use super::*;

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
