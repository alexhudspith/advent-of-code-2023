use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use itertools::Itertools;

const GALAXY: u8 = b'#';

#[derive(Debug, Clone, PartialEq, Eq)]
struct Image {
    galaxies: Vec<(usize, usize)>,
    shape: (usize, usize),
}

impl Image {
    fn expanded(&self, factor: usize) -> Self {
        let mut row_factors = vec![factor - 1; self.shape.0];
        let mut col_factors = vec![factor - 1; self.shape.1];
        for &(r, c) in &self.galaxies {
            row_factors[r] = 0;
            col_factors[c] = 0;
        }

        let row_add = cumulative_sum(row_factors);
        let col_add = cumulative_sum(col_factors);

        let galaxies = self.galaxies.iter()
            .map(|&(r, c)| (r + row_add[r], c + col_add[c]))
            .collect_vec();
        let shape = (
            self.shape.0 + row_add.last().unwrap_or(&0),
            self.shape.1 + col_add.last().unwrap_or(&0)
        );

        Self { galaxies, shape }
    }
}

fn cumulative_sum<I: IntoIterator<Item=usize>>(v: I) -> Vec<usize> {
    v.into_iter().scan(0, |acc, v| {
        *acc += v;
        Some(*acc)
    }).collect_vec()
}

fn read_image<R: Read>(reader: R) -> Result<Image, aoc::Error> {
    let mut galaxies: Vec<(usize, usize)> = vec![];
    let mut col_count = 0;
    let mut row_count = 0;
    for line in BufReader::new(reader).lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }

        if galaxies.is_empty() {
            col_count = line.len();
        }

        if col_count != line.len() {
            return Err("Ragged lines".into());
        }

        let galaxy_iter = line.bytes()
            .enumerate()
            .filter(|&(_, chr)| chr == GALAXY)
            .map(|(c, _)| (row_count, c));
        galaxies.extend(galaxy_iter);
        row_count += 1;
    }

    let shape = (row_count, col_count);
    Ok(Image { galaxies, shape })
}

fn run(image: &Image, expansion_factor: usize) -> usize {
    let image = image.expanded(expansion_factor);
    let first = image.galaxies.iter();
    let second = image.galaxies.iter();
    first.cartesian_product(second)
        .filter(|(a, b)| b > a)
        .map(|(&(r1, c1), &(r2, c2))| r2.abs_diff(r1) + c2.abs_diff(c1))
        .sum()
}

// Answer: 10422930
fn part1(image: &Image) -> usize {
    run(image, 2)
}

// Answer: 699909023130
fn part2(image: &Image) -> usize {
    run(image, 1_000_000)
}

fn main() -> Result<(), aoc::Error> {
    let path = aoc::find_input_path("day-11");
    let f = File::open(path)?;

    let image = read_image(f)?;
    let answer = part1(&image);
    println!("Part 1: {answer}");
    let answer = part2(&image);
    println!("Part 2: {answer}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    "};

    #[test]
    fn part1_example() {
        let r = Cursor::new(EXAMPLE);
        let image = read_image(r).unwrap();
        let v = run(&image, 2);
        assert_eq!(v, 374);
    }

    #[test]
    fn part2_example1() {
        let r = Cursor::new(EXAMPLE);
        let image = read_image(r).unwrap();
        let v = run(&image, 10);
        assert_eq!(v, 1030);
    }

    #[test]
    fn part2_example2() {
        let r = Cursor::new(EXAMPLE);
        let image = read_image(r).unwrap();
        let v = run(&image, 100);
        assert_eq!(v, 8410);
    }
}
