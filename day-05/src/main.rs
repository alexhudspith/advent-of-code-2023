use std::fs::File;

use itertools::Itertools;
use aoc::range::Range;

use day_05::*;

fn follow_maps(seeds: &[Range], maps: &[SeedMap]) -> Vec<Range> {
    let ranges = seeds.iter().copied().sorted().collect_vec();
    maps.iter().fold(ranges, |ranges, map: &SeedMap| map.get_many_ordered(&ranges))
}

// Answer: 324724204
fn part1(seed_numbers: &[u64], maps: &[SeedMap]) -> u64 {
    let seeds: Vec<Range> = seed_numbers.iter()
        .map(|&seed| Range::from_start_len(seed, 1))
        .collect_vec();

    let sorted_result = follow_maps(&seeds, maps);
    assert_eq!(sorted_result.len(), seed_numbers.len());
    sorted_result[0].start()
}

// Answer: 104070862
fn part2(seed_numbers: &[u64], maps: &[SeedMap]) -> u64 {
    let seeds: Vec<Range> = seed_numbers.iter()
        .tuples::<(_, _)>()
        .map(|(&start, &len)| Range::from_start_len(start, len))
        .collect_vec();

    let sorted_result = follow_maps(&seeds, maps);
    sorted_result[0].start()
}

fn main() -> Result<(), aoc::Error> {
    let path = aoc::find_input_path("day-05");
    let f = File::open(path)?;

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
