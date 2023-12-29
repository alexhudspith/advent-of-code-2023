use std::collections::hash_map::Entry;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::io::{BufReader, Read};
use std::iter;

use fxhash::{FxBuildHasher, FxHashMap};
use indexmap::IndexSet;
use itertools::Itertools;
use petgraph::graph::NodeIndex;
use petgraph::prelude::EdgeRef;
use petgraph::Undirected;

use aoc::grid::{Way, Ways};

pub type Ordinate = u8;
pub type Coords = (Ordinate, Ordinate);
// Several graph types will work, but not petgraph::Graph which has unstable IDs upon removal
pub type Graph = petgraph::stable_graph::StableGraph<Coords, u16, Undirected, Ordinate>;
pub type NodeIx = NodeIndex<Ordinate>;

pub type Grid = aoc::grid::Grid<Tile>;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Tile {
    Path,
    Forest,
    Slope(Way),
}

impl TryFrom<u8> for Tile {
    type Error = u8;

    fn try_from(value: u8) -> Result<Tile, Self::Error> {
        let tile = match value {
            b'.' => Self::Path,
            b'#' => Self::Forest,
            b'<' => Self::Slope(Way::Left),
            b'>' => Self::Slope(Way::Right),
            b'^' => Self::Slope(Way::Up),
            b'v' => Self::Slope(Way::Down),
            _ => return Err(value)
        };

        Ok(tile)
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let chr = match self {
            Self::Path => b'.',
            Self::Forest => b'#',
            Self::Slope(Way::Left) => b'<',
            Self::Slope(Way::Right) => b'>',
            Self::Slope(Way::Up) => b'^',
            Self::Slope(Way::Down) => b'v',
        };
        write!(f, "{}", chr)
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct State {
    pos: Coords,
    way_in: Way,
    cost: u16
}

fn usz((r, c): Coords) -> (usize, usize) {
    (r as usize, c as usize)
}

fn neighbours(grid: &Grid, old_state: &State, use_slopes: bool) -> Vec<State> {
    let State { pos: old_pos, way_in: old_way_in, .. } = *old_state;

    Ways::all().iter()
        .filter(|&new_way| new_way != old_way_in.flipped())
        .flat_map(|new_way| {
            let new_pos = grid.step(old_pos, new_way)
                .filter(|&new_pos| grid[usz(new_pos)] != Tile::Forest)?;

            let new_tile = if use_slopes { grid[usz(new_pos)] } else { Tile::Path };
            slip_to_next(old_state, new_pos, new_way, new_tile)
        })
        .collect_vec()
}

fn slip_to_next(old_state: &State, new_pos: Coords, new_way: Way, new_tile: Tile) -> Option<State> {
    let Tile::Slope(slope) = new_tile else {
        return Some(State {
            pos: new_pos,
            way_in: new_way,
            cost: old_state.cost + 1
        });
    };

    (slope != new_way.flipped()).then_some(
        State {
            pos: slope.step(new_pos),
            way_in: slope,
            cost: old_state.cost + 2
        }
    )
}

fn collapse_unique_edge(graph: &mut Graph, node: NodeIx, edge_weight_offset: u16) -> NodeIx {
    let Ok(e) = graph.edges(node).exactly_one() else {
        // Not valid for collapse
        return node;
    };

    let edge_weight = e.weight() - edge_weight_offset;
    let other_node = e.target();
    let other_edges = graph.edges(other_node)
        .filter_map(|e| (e.target() != node && e.target() != node).then_some(e.id()))
        .collect_vec();
    for e in other_edges {
        *graph.edge_weight_mut(e).unwrap() += edge_weight;
    }

    // Note: Graph implementation must have stable node IDs
    graph.remove_node(node);
    other_node
}

/**
  Reduces a dense grid to a graph of junctions.
*/
pub fn reduce_grid(grid: &Grid, start: Coords, end: Coords) -> (Graph, NodeIx, NodeIx) {
    let mut graph = Graph::default();
    let start_node = graph.add_node(start);
    let end_node = graph.add_node(end);

    let mut junction_nodes_by_pos = FxHashMap::default();
    junction_nodes_by_pos.insert(end, end_node);
    let mut stack = Vec::new();
    stack.push((start_node, State { pos: start, way_in: Way::Down, cost: 0 }));

    while let Some((a_node_ix, a @ State { pos: a_pos, cost: a_cost, .. })) = stack.pop() {
        let mut b = a;
        // Walk corridors
        loop {
            let State { pos: b_pos, cost: b_cost, .. } = b;
            // Check for cycles and add the back-edge
            match junction_nodes_by_pos.entry(b_pos) {
                Entry::Vacant(e) => { e.insert(a_node_ix); }
                Entry::Occupied(_) if a_pos == b_pos => { break },
                Entry::Occupied(e) => {
                    let b_node_ix = *e.get();
                    graph.add_edge(a_node_ix, b_node_ix, b_cost - a_cost + 1);
                    break;
                }
            }

            let neighbours: Vec<State> = neighbours(grid, &b, false);
            match neighbours.len() {
                0 => { break },
                1 => { b = neighbours[0] }
                _ => {
                    let b_node_ix = graph.add_node(b_pos);
                    junction_nodes_by_pos.insert(b_pos, b_node_ix);
                    graph.add_edge(a_node_ix, b_node_ix, b_cost - a_cost + 1);
                    stack.extend(neighbours.into_iter().map(|n| (b_node_ix, n)));
                    break;
                }
            };
        }
    }

    // To prune the path search, remove the single-edged start and end nodes.
    // Push their costs to (start) next or (end) prev edges.
    let start_node = collapse_unique_edge(&mut graph, start_node, 1);
    let end_node = collapse_unique_edge(&mut graph, end_node, 0);

    (graph, start_node, end_node)
}

/**
    Depth-first path-walking iterator, beginning from state `start`,
    proceeding via function `neighbours`, and ending at `end`. Each state has a
    key given by the `key` function. Cycles are deemed to occur when a key is
    equal to one seen earlier _in the same path_.
 */
fn walk_paths<S, I, K, IF, KF>(start: S, end: K, mut neighbours: IF, mut key: KF) -> impl Iterator<Item=S>
    where
        S: Debug,
        I: IntoIterator<Item=S>,
        K: Copy + Eq + Hash + Debug,
        IF: FnMut(&S) -> I,
        KF: FnMut(&S) -> K,
{
    let mut path = IndexSet::with_hasher(FxBuildHasher::default());
    let mut stack = Vec::new();
    stack.push((1, start));

    iter::from_fn(move || {
        loop {
            let Some((depth, u)) = stack.pop() else {
                return None;
            };

            if depth <= path.len() {
                path.truncate(depth - 1);
            }

            let key_u = key(&u);
            if !path.insert(key_u) {
                // Cycle
                continue;
            }

            if key_u != end {
                let neighbours = neighbours(&u).into_iter();
                stack.extend(neighbours.map(|v| (depth + 1, v)));
            }

            return Some(u);
        }
    })
}

pub fn part1_longest_path(grid: &Grid, start: Coords, end: Coords) -> u16 {
    let dfs_iter = walk_paths(
        State { pos: start, way_in: Way::Down, cost: 0 },
        end,
        |state| neighbours(grid, state, true),
        |&State { pos, .. }| pos
    );

    dfs_iter
        .filter_map(|State { pos, cost, .. }| (pos == end).then_some(cost))
        .max()
        .expect("End node not reached")
}

pub fn part2_longest_path(graph: &Graph, start: NodeIx, end: NodeIx) -> u16 {
    let dfs_iter = walk_paths(
        (start, 0),
        end,
        |&(node, cost)| graph.edges(node).map(move |e| (e.target(), cost + e.weight())),
        |&(node, _cost)| node
    );

    dfs_iter
        .filter_map(|(node, cost)| (node == end).then_some(cost))
        .max()
        .expect("End node not reached")
}

pub fn read_grid<R: Read>(input: R) -> Result<Grid, aoc::Error> {
    let mut r = BufReader::new(input);
    aoc::grid::read_grid(&mut r, None)
}
