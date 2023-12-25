use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

use itertools::Itertools;
use petgraph::Undirected;
use petgraph::data::DataMap;
use petgraph::graph::EdgeIndex;
use rand::Rng;
use rayon::prelude::*;

use aoc::parse::OkOrErr;

type Graph = petgraph::Graph<u32, (), Undirected>;

fn read_graph<R: Read>(input: R) -> Result<Graph, aoc::Error> {
    let lines = BufReader::new(input).lines();
    let mut graph = Graph::new_undirected();
    let mut nodes = HashMap::new();
    for line in lines {
        let line = line?;
        if line.is_empty() {
            continue;
        }

        let (lhs, others) = line.split(':')
            .map(str::trim)
            .collect_tuple()
            .ok_or_err(&line)?;

        let mut add_node = |g: &mut Graph, n: &str| *nodes.entry(n.to_string())
            .or_insert_with(|| {
                g.add_node(1)
            });

        let n1 = add_node(&mut graph, lhs);
        for rhs in others.split_ascii_whitespace() {
            let n2 = add_node(&mut graph, rhs);
            graph.update_edge(n1, n2, ());
        }
    }

    Ok(graph)
}

fn contract_edge(graph: &mut Graph, edge_ix: EdgeIndex) {
    let (a, b) = graph.edge_endpoints(edge_ix)
        .unwrap_or_else(|| panic!("Bad edge_ix: {edge_ix:?}"));

    let b_weight = *graph.node_weight(b)
        .unwrap_or_else(|| panic!("Bad node_ix: {b:?}"));

    let mut walker = graph.neighbors(b).detach();
    while let Some(b_neigh) = walker.next_node(graph) {
        if b_neigh != a && b_neigh != b {
            graph.add_edge(a, b_neigh, ());
        }
    }

    if let Some(a_weight) = graph.node_weight_mut(a) {
        *a_weight += b_weight;
    }

    graph.remove_node(b);
}

fn contracted_graph(graph: &Graph) -> Graph {
    let mut graph = graph.clone();
    let mut rng = rand::thread_rng();
    while graph.node_count() > 2 {
        let edge_ix = rng.gen_range(0..graph.edge_count());
        contract_edge(&mut graph, EdgeIndex::new(edge_ix));
    }

    graph
}

fn part1<R: Read>(input: R) -> Result<u32, aoc::Error> {
    let original_graph = read_graph(input)?;

    // https://en.wikipedia.org/wiki/Karger%27s_algorithm
    // Repeatedly contract random edges until only 2 nodes remain
    let answer: (u32, u32) = rayon::iter::repeat(())
        .map(|_| contracted_graph(&original_graph))
        .find_any(|g| g.edge_count() <= 3)
        .and_then(|g| g.node_weights().copied().collect_tuple())
        .expect("Expected exactly 2 connected components in the graph");

    Ok(answer.0 * answer.1)
}

fn main() -> Result<(), aoc::Error> {
    let path = aoc::find_input_path("day-25");
    let f = File::open(path)?;
    // Answer: 601344
    let answer = part1(&f)?;
    println!("Part 1: {answer}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {r"
        jqt: rhn xhk nvd
        rsh: frs pzl lsr
        xhk: hfx
        cmg: qnr nvd lhk bvb
        rhn: xhk bvb hfx
        bvb: xhk hfx
        pzl: lsr hfx nvd
        qnr: nvd
        ntq: jqt hfx bvb xhk
        nvd: lhk
        lsr: lhk
        rzs: qnr cmg lsr rsh
        frs: qnr lhk lsr
    "};

    #[test]
    fn part1_example() {
        let answer = part1(Cursor::new(EXAMPLE)).unwrap();
        assert_eq!(answer, 54);
    }
}
