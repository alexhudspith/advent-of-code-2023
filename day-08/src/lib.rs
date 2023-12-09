use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::io::{BufRead, BufReader, Read};
use std::{iter, str};
use std::ops::Index;
use std::str::FromStr;
use itertools::Itertools;
use aoc::{aoc_err, CollectArray, some_ok_or};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction { Left = 0, Right = 1 }

impl<T> Index<Direction> for [T; 2] {
    type Output = T;

    fn index(&self, index: Direction) -> &T {
        &self[index as usize]
    }
}

impl<T> Index<Direction> for (T, T) {
    type Output = T;

    fn index(&self, index: Direction) -> &T {
        match index {
            Direction::Left => &self.0,
            Direction::Right => &self.1,

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
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err(s.to_string()),
        }
    }
}

const N: usize = 3;

#[repr(align(4))]
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
    directions: Vec<Direction>,
    edges: HashMap<Node, (Node, Node)>,
}

impl Graph {
    pub fn nodes(&self) -> impl Iterator<Item=Node> + '_ {
        self.edges.keys().copied()
    }

    pub fn iter_at(&self, start_node: Node) -> impl Iterator<Item=Node> + '_ {
        let mut visited_node_dirs: HashSet<(usize, Node)> = HashSet::with_capacity(22_000);
        let mut node = start_node;
        let mut dir_iter = self.directions.iter().copied().enumerate().cycle();

        iter::from_fn(move || {
            let (dir_ix, dir) = dir_iter.next().expect("Iterator should be inifinite");
            if !visited_node_dirs.insert((dir_ix, node)) {
                if cfg!(debug_assertions) {
                    eprintln!("{start_node:?}: Cycle at {node:?}, dir_ix {dir_ix} {dir:?}");
                }
                // Exhausted
                return None;
            }

            let (left, right) = self.edges[&node];
            if left == right && left == node {
                if cfg!(debug_assertions) {
                    eprintln!("Left & right self-loops at node {node:?}");
                }
                // Exhausted
                return None;
            }

            node = [left, right][dir];
            Some(node)
        })
    }
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

fn verify(edges: &HashMap<Node, (Node, Node)>) -> bool {
    edges.values()
        .all(|(left, right)|
            edges.contains_key(left) && edges.contains_key(right)
        )
}

pub fn read_graph<R: Read>(input: R) -> Result<Graph, aoc::Error> {
    let mut lines = BufReader::new(input).lines();
    let directions_line = some_ok_or(lines.next(), "No directions line")?;
    let directions = directions_line
        .trim()
        .chars()
        .map(Direction::try_from)
        .try_collect()?;
    let _blank = some_ok_or(lines.next(), "Expected blank line")?;
    let node_to_adj_pair = lines.process_results(|lines| {
        lines.map(parse_line).try_collect()
    })??;

    if !verify(&node_to_adj_pair) {
        return Err(aoc_err("Edge refers to missing node"))
    }

    Ok(Graph { directions, edges: node_to_adj_pair })
}
