use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read, Seek};
use std::path::{Path, PathBuf};

use day_02::{Cubes, Game};

pub fn data_dir() -> PathBuf {
    Path::new(file!()).ancestors().nth(2).unwrap().join("data")
}

fn run<R, F>(input: R, mut game_score: F) -> io::Result<u64>
    where R: Read, F: FnMut(&Game) -> u64
{
    let lines = BufReader::new(input).lines();
    let mut total = 0;
    for line in lines {
        let line = line?;
        let game: Game = line.parse()
            .map_err(|_| io::Error::from(io::ErrorKind::InvalidData))?;
        total += game_score(&game);
    }

    Ok(total)
}

// Answer: 2006
fn part1(game: &Game) -> u64 {
    let cubes = Cubes { red: 12, green: 13, blue: 14 };
    if game.possible(cubes) { game.id } else { 0 }
}

// Answer: 84911
fn part2(game: &Game) -> u64 {
    game.min_cubes().power()
}

fn main() -> io::Result<()> {
    let path = data_dir().join("input.txt");
    let mut f = File::open(path)?;

    let total = run(&f, part1)?;
    println!("Part 1: {total}");
    f.rewind()?;
    let total = run(&f, part2)?;
    println!("Part 2: {total}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const EXAMPLE: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    #[test]
    fn part1_example() {
        let input = Cursor::new(EXAMPLE);
        assert_eq!(run(input, part1).unwrap(), 8);
    }

    #[test]
    fn part2_example() {
        let input = Cursor::new(EXAMPLE);
        assert_eq!(run(input, part2).unwrap(), 2286);
    }
}