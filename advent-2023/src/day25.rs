use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet, VecDeque},
    iter,
};

use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Graph = HashMap<String, HashSet<String>>;

#[aoc_generator(day25)]
fn generator(input: &str) -> Result<Graph> {
    input
        .lines()
        .map(|line| {
            let (node, connections) = line
                .split_once(": ")
                .ok_or_else(|| anyhow::anyhow!("Invalid line format"))?;
            anyhow::Ok((
                node.to_string(),
                connections
                    .split_whitespace()
                    .map(String::from)
                    .collect::<Vec<_>>(),
            ))
        })
        .try_fold(Graph::new(), |mut graph, result| {
            let (node, connections) = result?;
            for conn in connections {
                graph.entry(node.clone()).or_default().insert(conn.clone());
                graph.entry(conn).or_default().insert(node.clone());
            }
            Ok(graph)
        })
}

fn count_reachable(
    graph: &Graph,
    start: &str,
    excluded_edges: &HashSet<(String, String)>,
) -> usize {
    let mut visited = HashSet::from([start.to_string()]);
    let mut queue = VecDeque::from([start.to_string()]);

    while let Some(node) = queue.pop_front() {
        if let Some(neighbors) = graph.get(&node) {
            neighbors
                .iter()
                .filter(|neighbor| {
                    let edge = normalize_edge(&node, neighbor);
                    !excluded_edges.contains(&edge)
                })
                .filter(|neighbor| visited.insert((*neighbor).clone()))
                .for_each(|neighbor| queue.push_back(neighbor.clone()));
        }
    }

    visited.len()
}

fn normalize_edge(a: &str, b: &str) -> (String, String) {
    if a < b {
        (a.to_string(), b.to_string())
    } else {
        (b.to_string(), a.to_string())
    }
}

#[aoc(day25, part1)]
fn part1(graph: &Graph) -> Option<usize> {
    let nodes: Vec<_> = graph.keys().cloned().collect();
    let total_nodes = nodes.len();

    let edge_count = count_edge_usage(graph, &nodes);

    let mut edges: Vec<_> = edge_count.into_iter().collect();
    edges.sort_unstable_by_key(|(_, count)| Reverse(*count));

    (0..edges.len().min(15))
        .combinations(3)
        .find_map(|indices| {
            let excluded: HashSet<_> = indices.iter().map(|&i| edges[i].0.clone()).collect();

            let reachable = count_reachable(graph, &nodes[0], &excluded);

            (reachable < total_nodes && reachable > 0)
                .then(|| reachable * (total_nodes - reachable))
        })
}

fn count_edge_usage(graph: &Graph, nodes: &[String]) -> HashMap<(String, String), usize> {
    let total_nodes = nodes.len();

    let node_pairs: Vec<(usize, usize)> = if total_nodes < 50 {
        (0..total_nodes)
            .cartesian_product(0..total_nodes)
            .filter(|(i, j)| i != j)
            .collect()
    } else {
        let sample_size = (total_nodes * 20).min(2000);
        (0..total_nodes)
            .cycle()
            .zip((0..total_nodes).cycle().skip(total_nodes / 3))
            .take(sample_size)
            .filter(|(i, j)| i != j)
            .collect()
    };

    node_pairs
        .into_iter()
        .filter_map(|(src_idx, dst_idx)| find_path(graph, &nodes[src_idx], &nodes[dst_idx]))
        .flat_map(|path| {
            path.windows(2)
                .map(|w| normalize_edge(&w[0], &w[1]))
                .collect::<Vec<_>>()
        })
        .fold(HashMap::new(), |mut counts, edge| {
            *counts.entry(edge).or_insert(0) += 1;
            counts
        })
}

fn find_path(graph: &Graph, start: &str, end: &str) -> Option<Vec<String>> {
    let mut queue = VecDeque::from([start.to_string()]);
    let mut parent = HashMap::from([(start.to_string(), start.to_string())]);

    while let Some(node) = queue.pop_front() {
        if node == end {
            return Some(reconstruct_path(&parent, start, end));
        }

        if let Some(neighbors) = graph.get(&node) {
            let new_neighbors: Vec<_> = neighbors
                .iter()
                .filter(|neighbor| !parent.contains_key(*neighbor))
                .cloned()
                .collect();

            for neighbor in new_neighbors {
                parent.insert(neighbor.clone(), node.clone());
                queue.push_back(neighbor);
            }
        }
    }

    None
}

fn reconstruct_path(parent: &HashMap<String, String>, start: &str, end: &str) -> Vec<String> {
    iter::successors(Some(end.to_string()), |current| {
        (current != start).then(|| parent[current].clone())
    })
    .collect::<Vec<_>>()
    .into_iter()
    .rev()
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "jqt: rhn xhk nvd
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
frs: qnr lhk lsr";

    #[test]
    fn test_generator() {
        let graph = generator(EXAMPLE).unwrap();

        // Check that graph has correct number of nodes
        assert_eq!(graph.len(), 15);

        // Check some specific connections
        assert!(graph.get("jqt").unwrap().contains("rhn"));
        assert!(graph.get("jqt").unwrap().contains("xhk"));
        assert!(graph.get("jqt").unwrap().contains("nvd"));

        // Check bidirectional connections
        assert!(graph.get("rhn").unwrap().contains("jqt"));
        assert!(graph.get("xhk").unwrap().contains("jqt"));
        assert!(graph.get("nvd").unwrap().contains("jqt"));
    }

    #[test]
    fn test_part1() {
        let graph = generator(EXAMPLE).unwrap();
        assert_eq!(part1(&graph), Some(54));
    }
}
