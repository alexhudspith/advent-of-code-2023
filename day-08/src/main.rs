#![feature(iter_array_chunks)]

use std::collections::HashSet;
use std::fs::File;
use std::io::{Read, Seek};
use itertools::Itertools;

use aoc::{aoc_err};
use day_08::*;

pub fn run<R, P>(input: R, mut is_start_node: P) -> Result<usize, aoc::Error>
    where
        R: Read,
        P: FnMut(Node) -> bool
{
    let graph = read_graph(input)?;

    let start_nodes = graph.edges.keys()
        .filter(|&&k| is_start_node(k))
        .sorted()
        .collect_vec();

    if start_nodes.is_empty() {
        return Err(aoc_err("No start nodes"));
    }

    let hops_to_z: Vec<usize> = start_nodes.into_iter()
        .flat_map(|&node| hops_to_z(&graph, node))
        .collect_vec();

    let lcm = hops_to_z
        .into_iter()
        .reduce(num::integer::lcm)
        .ok_or("No hops to Z")?;

    Ok(lcm)
}

fn hops_to_z(graph: &Graph, start_node: Node) -> Vec<usize> {
    let mut visited = HashSet::new();
    let mut hops_to_z = Vec::new();
    let mut node = start_node;
    let iter = graph.directions.iter()
        .enumerate()
        .cycle()
        .enumerate()
        .map(|(hop_ix, (dir_ix, &dir))| (hop_ix, dir_ix, dir));

    for (hop_ix, dir_ix, dir) in iter {
        if !visited.insert((dir_ix, node)) {
            // Found a cycle
            break;
        }

        if node.ends_with(b'Z') {
            hops_to_z.push(hop_ix);
        }

        let (left, right) = graph.edges[&node];
        if left == right && left == node {
            // Found a simple cycle in both left and right
            break;
        }
        node = dir.choose(left, right);
    }

    hops_to_z
}

// Answer: 14681
fn part1<R: Read>(input: R) -> Result<usize, aoc::Error> {
    let aaa: Node = "AAA".parse()?;
    run(input, |node: Node| node == aaa)
}

// Answer: 14321394058031
fn part2<R: Read>(input: R) -> Result<usize, aoc::Error> {
    run(input, |node: Node| node.ends_with(b'A'))
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
