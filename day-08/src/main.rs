use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek};

use itertools::Itertools;

type LeftRight = (String, String);

fn parse_line(line: String) -> Result<(String, LeftRight), aoc::Error> {
    let (source, _, left, right) = line
        .split_ascii_whitespace()
        .collect_tuple()
        .ok_or("Line is not <source> = (<left>, <right>)")?;
    let source = source.to_owned();
    let left = left.trim_matches(&['(', ',']).to_owned();
    let right = right.trim_matches(&[')']).to_owned();
    Ok((source, (left, right)))
}

fn read<R: Read>(input: R) -> Result<(String, HashMap<String, LeftRight>), aoc::Error> {
    let mut lines = BufReader::new(input).lines();
    let directions = lines.next().ok_or("No directions line")??.trim().to_string();
    let _blank = lines.next().ok_or("Expected blank line")??;
    let map = lines.process_results(|lines| {
        lines.map(parse_line).try_collect()
    })??;

    Ok((directions, map))
}

pub fn choose<'a>(left: &'a str, right: &'a str, direction: char) -> &'a str {
    match direction {
        'L' => left,
        'R' => right,
        _ => panic!("Bad direction"),
    }
}

pub fn run<R, P>(input: R, mut is_start_node: P) -> Result<usize, aoc::Error>
    where
        R: Read,
        P: FnMut(&str) -> bool
{
    let (directions, map) = read(input)?;

    let start_nodes = map.keys()
        .filter(|&k| is_start_node(k))
        .map(|s| s.as_str())
        .sorted()
        .collect_vec();

    let hops_to_z: Vec<usize> = start_nodes.iter()
        .flat_map(|node| hops_to_z(&directions, &map, node))
        .collect_vec();

    let lcm = hops_to_z
        .into_iter()
        .reduce(num::integer::lcm)
        .ok_or("No hops to Z")?;

    Ok(lcm)
}

fn hops_to_z(directions: &str, map: &HashMap<String, LeftRight>, start_node: &str) -> Vec<usize> {
    directions.chars()
        .enumerate()
        .cycle()
        .enumerate()
        .map(|(hop_ix, (dir_ix, dir))| (hop_ix, dir_ix, dir))
        .scan((start_node, false, HashSet::new()), |(node, stop, visited), (hop_ix, dir_ix, dir)| {
            if !(*visited).insert((dir_ix, node.to_string())) {
                // Cycle
                *stop = true;
            }

            if *stop {
                return None;
            }

            let yield_item = if (*node).ends_with('Z') { Some(hop_ix) } else { None };
            let (left, right) = &map[*node];
            if left == right && left == *node {
                // Simple cycle in both left and right
                *stop = true;
            }

            *node = choose(left, right, dir);
            Some(yield_item)
        })
        .flatten()
        .collect_vec()
}

// Answer: 14681
fn part1<R: Read>(input: R) -> Result<usize, aoc::Error> {
    run(input, |node: &str| node == "AAA")
}

// Answer: 14321394058031
fn part2<R: Read>(input: R) -> Result<usize, aoc::Error> {
    run(input, |node: &str| node.ends_with('A'))
}

fn main() -> Result<(), aoc::Error> {
    let path = aoc::find_input_path("day-08");
    let mut f = File::open(path)?;

    let answer = part1(&mut f)?;
    println!("Part 1: {answer}");
    f.rewind()?;
    let answer = part2(&mut f)?;
    println!("Part 2: {answer}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use indoc::indoc;

    const EXAMPLE1_1: &str = indoc! {"
        RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)
    "};

    const EXAMPLE1_2: &str = indoc! {"
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
    "};

    const EXAMPLE2: &str = indoc! {"
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
    "};

    #[test]
    fn part1_example1() {
        let total = part1(Cursor::new(EXAMPLE1_1)).unwrap();
        assert_eq!(total, 2);
    }

    #[test]
    fn part1_example2() {
        let total = part1(Cursor::new(EXAMPLE1_2)).unwrap();
        assert_eq!(total, 6);
    }

    #[test]
    fn part2_example() {
        let total = part2(Cursor::new(EXAMPLE2)).unwrap();
        assert_eq!(total, 6);
    }
}
