use std::{collections::HashMap, iter};

use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug)]
struct Node {
    weight: u32,
    children: Vec<String>,
}

#[aoc_generator(day7)]
fn generator(input: &str) -> anyhow::Result<HashMap<String, Node>> {
    input.lines().try_fold(HashMap::new(), |mut acc, line| {
        let mut parts = line.split_whitespace();
        let name = parts.next().context("unable to parse name")?.to_string();
        let weight = parts.next().context("unable to parse weight")?;
        let weight = weight[1..weight.len() - 1].parse()?;
        let children = if let Some("->") = parts.next() {
            let children = parts.join(" ");
            children.split(',').map(|s| s.trim().to_string()).collect()
        } else {
            Vec::new()
        };

        acc.insert(name, Node { weight, children });
        Ok(acc)
    })
}

fn find_root(input: &HashMap<String, Node>) -> Option<String> {
    input
        .keys()
        .find(|name| input.values().all(|node| !node.children.contains(name)))
        .cloned()
}

fn find_weight(input: &HashMap<String, Node>, name: &str) -> Option<u32> {
    let node = input.get(name)?;
    let weight = node.weight;
    let children_weight = node
        .children
        .iter()
        .map(|name| find_weight(input, name))
        .sum::<Option<u32>>()?;
    Some(weight + children_weight)
}

fn find_mismatched_child(input: &HashMap<String, Node>, name: &str) -> Option<(String, u32)> {
    input
        .get(name)?
        .children
        .iter()
        .filter_map(|name| Some((name, find_weight(input, name)?)))
        .fold(HashMap::<_, Vec<_>>::new(), |mut acc, (name, weight)| {
            acc.entry(weight).or_default().push(name);
            acc
        })
        .iter()
        .combinations(2)
        .find_map(|parts| {
            let (&(&aw, ref af), &(&bw, ref bf)) = (&parts[0], &parts[1]);
            if af.len() == 1 {
                Some((af[0].to_string(), aw - bw))
            } else if bf.len() == 1 {
                Some((bf[0].to_string(), bw - aw))
            } else {
                None
            }
        })
}

#[aoc(day7, part1)]
fn part1(input: &HashMap<String, Node>) -> Option<String> {
    find_root(input)
}

#[aoc(day7, part2)]
fn part2(input: &HashMap<String, Node>) -> Option<u32> {
    let root = find_root(input)?;
    iter::successors(Some((root, 0)), |(name, _)| {
        find_mismatched_child(input, name)
    })
    .last()
    .and_then(|(name, delta)| {
        let weight = input.get(&name)?.weight;
        Some(weight - delta)
    })
}
