use std::fs::File;
use std::io::{Read, Seek};
use itertools::Itertools;

use aoc::error::aoc_err;
use day_08::*;

pub fn run<R, P>(input: R, mut is_start_node: P) -> Result<usize, aoc::error::Error>
    where
        R: Read,
        P: FnMut(Node) -> bool
{
    let graph = read_graph(input)?;
    let start_nodes = graph.nodes()
        .filter(|&n| is_start_node(n))
        .sorted()
        .collect_vec();

    if start_nodes.is_empty() {
        return Err(aoc_err("No start nodes"));
    }

    let hops_to_z: Vec<usize> = start_nodes.into_iter()
        .flat_map(|node| hops_to_z(&graph, node))
        .collect_vec();

    let lcm = hops_to_z
        .into_iter()
        .reduce(num::integer::lcm)
        .ok_or("No hops to Z")?;

    Ok(lcm)
}

fn hops_to_z(graph: &Graph, start_node: Node) -> Vec<usize> {
    graph.iter_at(start_node)
        .enumerate()
        .filter(|&(_, node)| node.ends_with(b'Z'))
        .map(|(hop_ix, _)| hop_ix + 1)
        .collect_vec()
}

fn part1_fn() -> Result<impl FnMut(Node) -> bool, aoc::error::Error> {
    let aaa: Node = "AAA".parse()?;
    Ok(move |node: Node| node == aaa)
}

fn part2_fn() -> Result<impl FnMut(Node) -> bool, aoc::error::Error> {
    Ok(|node: Node| node.ends_with(b'A'))
}

fn main() -> Result<(), aoc::error::Error> {
    let path = aoc::find_input_path("day-08");
    let mut f = File::open(path)?;

    // Answer: 14681
    let answer = run(&mut f, part1_fn()?)?;
    println!("Part 1: {answer}");
    f.rewind()?;
    // Answer: 14321394058031
    let answer = run(&mut f, part2_fn()?)?;
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
        let part1 = part1_fn().unwrap();
        let total = run(Cursor::new(EXAMPLE1_1), part1).unwrap();
        assert_eq!(total, 2);
    }

    #[test]
    fn part1_example2() {
        let part1 = part1_fn().unwrap();
        let total = run(Cursor::new(EXAMPLE1_2), part1).unwrap();
        assert_eq!(total, 6);
    }

    #[test]
    fn part2_example() {
        let part2 = part2_fn().unwrap();
        let total = run(Cursor::new(EXAMPLE2), part2).unwrap();
        assert_eq!(total, 6);
    }
}
