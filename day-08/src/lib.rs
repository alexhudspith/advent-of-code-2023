use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::io::{BufRead, BufReader, Read};
use std::str;
use std::str::FromStr;
use itertools::Itertools;
use aoc::{CollectArray, expect_next_ok};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Left,
    Right
}

impl Direction {
    pub fn choose(&self, left: Node, right: Node) -> Node {
        match self {
            Self::Left => left,
            Self::Right => right,
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(value.to_string()),
        }
    }
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(s.to_string()),
        }
    }
}

const N: usize = 3;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Node([u8; N]);

impl Node {
    pub fn ends_with(&self, b: u8) -> bool {
        self.0.ends_with(&[b])
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", str::from_utf8(&self.0).unwrap())
    }
}

impl FromStr for Node {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x: [u8; N] = s.bytes().pad_using(N, |_| b'_').try_collect_array()?;
        Ok(Node(x))
    }
}

pub struct Graph {
    pub directions: Vec<Direction>,
    pub edges: HashMap<Node, (Node, Node)>,
}

fn parse_line(line: String) -> Result<(Node, (Node, Node)), aoc::Error> {
    let (source, _, left, right) = line
        .split_ascii_whitespace()
        .collect_tuple()
        .ok_or("Line is not <source> = (<left>, <right>)")?;
    let source: Node = source.parse()?;
    let left: Node = left.trim_matches(&['(', ',']).parse()?;
    let right: Node = right.trim_matches(&[')']).parse()?;
    Ok((source, (left, right)))
}

pub fn read_graph<R: Read>(input: R) -> Result<Graph, aoc::Error> {
    let mut lines = BufReader::new(input).lines();
    let directions_line = expect_next_ok(&mut lines, "No directions line")?;
    let directions = directions_line
        .trim()
        .chars()
        .map(Direction::try_from)
        .try_collect()?;
    let _blank = expect_next_ok(&mut lines, "Expected blank line")?;
    let node_to_adj_pair = lines.process_results(|lines| {
        lines.map(parse_line).try_collect()
    })??;

    Ok(Graph { directions, edges: node_to_adj_pair })
}
