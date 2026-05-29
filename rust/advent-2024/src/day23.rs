use std::collections::{HashMap, HashSet};

use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day23)]
fn generator(input: &str) -> Result<Vec<(String, String)>> {
    Ok(input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('-').unwrap();
            (a.to_string(), b.to_string())
        })
        .collect())
}

fn build_graph(connections: &[(String, String)]) -> HashMap<&str, HashSet<&str>> {
    connections
        .iter()
        .fold(HashMap::new(), |mut graph, (a, b)| {
            graph.entry(a.as_str()).or_default().insert(b.as_str());
            graph.entry(b.as_str()).or_default().insert(a.as_str());
            graph
        })
}

#[aoc(day23, part1)]
fn part1(connections: &[(String, String)]) -> usize {
    let graph = build_graph(connections);

    graph
        .keys()
        .copied()
        .tuple_combinations()
        .filter(|(a, b, c)| {
            graph.get(a).is_some_and(|n| n.contains(b) && n.contains(c))
                && graph.get(b).is_some_and(|n| n.contains(c))
                && [a, b, c].iter().any(|name| name.starts_with('t'))
        })
        .count()
}

#[aoc(day23, part2)]
fn part2(connections: &[(String, String)]) -> String {
    let graph = build_graph(connections);
    let mut max_clique = Vec::new();
    let all_nodes: HashSet<&str> = graph.keys().copied().collect();

    bron_kerbosch(
        &graph,
        HashSet::new(),
        all_nodes,
        HashSet::new(),
        &mut max_clique,
    );

    max_clique.into_iter().sorted().join(",")
}

fn bron_kerbosch<'a>(
    graph: &HashMap<&'a str, HashSet<&'a str>>,
    r: HashSet<&'a str>,
    mut p: HashSet<&'a str>,
    mut x: HashSet<&'a str>,
    max_clique: &mut Vec<&'a str>,
) {
    if p.is_empty() && x.is_empty() {
        if r.len() > max_clique.len() {
            *max_clique = r.into_iter().collect();
        }
        return;
    }

    let pivot = p
        .union(&x)
        .max_by_key(|&&v| {
            graph
                .get(v)
                .map_or(0, |neighbors| neighbors.intersection(&p).count())
        })
        .copied();

    let candidates: Vec<&str> = match pivot.and_then(|v| graph.get(v)) {
        Some(neighbors) => p.difference(neighbors).copied().collect(),
        None => p.iter().copied().collect(),
    };

    for &v in &candidates {
        if let Some(neighbors) = graph.get(v) {
            let mut new_r = r.clone();
            new_r.insert(v);

            let new_p = p.intersection(neighbors).copied().collect();
            let new_x = x.intersection(neighbors).copied().collect();

            bron_kerbosch(graph, new_r, new_p, new_x, max_clique);
        }

        p.remove(v);
        x.insert(v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn part1_example() {
        let connections = generator(EXAMPLE).unwrap();
        assert_eq!(part1(&connections), 7);
    }

    #[test]
    fn part2_example() {
        let connections = generator(EXAMPLE).unwrap();
        assert_eq!(part2(&connections), "co,de,ka,ta");
    }
}
