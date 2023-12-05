#![feature(is_sorted)]

use std::fs::File;
use std::path::{Path, PathBuf};

use itertools::Itertools;

use day_05::*;

pub fn data_dir() -> PathBuf {
    Path::new(file!()).ancestors().nth(2).unwrap().join("data")
}


fn follow_maps(seeds: &[Range], maps: &[SeedMap]) -> Vec<Range> {
    // let ranges = seeds.iter().copied().sorted().collect_vec();
    let ranges = seeds.to_vec();
    let dest_ranges = maps.iter().fold(ranges, |ranges, map: &SeedMap| map.get_many_ordered(&ranges));
    assert!(dest_ranges.is_sorted());
    dest_ranges
}

// Answer: 324724204
fn part1(seed_numbers: &[u64], maps: &[SeedMap]) -> u64 {
    seed_numbers.iter()
        .map(|&seed| [Range::from_start_len(seed, 1)])
        .map(|seeds| follow_maps(&seeds, maps))
        .inspect(|dest_ranges| assert_eq!(dest_ranges.len(), 1))
        .map(|dest_ranges| dest_ranges[0])
        .min()
        .unwrap()
        .start()
}

// Answer: 104070862
fn part2(seed_numbers: &[u64], maps: &[SeedMap]) -> u64 {
    let seeds: Vec<Range> = seed_numbers.iter()
        .tuples::<(_, _)>()
        .map(|(&start, &len)| Range::from_start_len(start, len))
        .collect_vec();

    let result = follow_maps(&seeds, maps);
    assert!(result.is_sorted());
    result[0].start()
}

fn main() -> Result<(), anyhow::Error> {
    let f = File::open(data_dir().join("input.txt"))?;

    let (seeds, maps) = parse::read_seed_maps(&f)?;
    let answer = part1(&seeds, &maps);
    println!("Part 1: {}", answer);
    let answer = part2(&seeds, &maps);
    println!("Part 2: {}", answer);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
    "};

    #[test]
    fn part1_example() {
        let f = Cursor::new(EXAMPLE);
        let (seeds, maps) = parse::read_seed_maps(f).unwrap();
        let total = part1(&seeds, &maps);
        assert_eq!(total, 35);
    }

    #[test]
    fn part2_example() {
        let f = Cursor::new(EXAMPLE);
        let (seeds, maps) = parse::read_seed_maps(f).unwrap();
        let total = part2(&seeds, &maps);
        assert_eq!(total, 46);
    }
}
