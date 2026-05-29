use std::collections::{HashMap, HashSet};

use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day12)]
fn generator(input: &str) -> anyhow::Result<HashMap<u32, HashSet<u32>>> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(" <-> ");
            let id = parts.next().context("unable to find id")?.parse()?;
            let connected = parts
                .next()
                .context("unable to find connections")?
                .split(", ")
                .map(|id| id.parse())
                .collect::<Result<_, _>>()?;
            Ok((id, connected))
        })
        .collect()
}

#[aoc(day12, part1)]
fn part1(input: &HashMap<u32, HashSet<u32>>) -> usize {
    let mut connected = HashSet::new();
    connected.insert(0);

    let mut last_size = 0;
    while last_size != connected.len() {
        last_size = connected.len();

        for (origin, connections) in input {
            if connected.contains(origin) {
                connected.extend(connections);
            }
        }
    }

    connected.len()
}

#[aoc(day12, part2)]
fn part2(input: &HashMap<u32, HashSet<u32>>) -> usize {
    let mut connected: Vec<HashSet<u32>> = vec![];
    let mut need_search = false;

    loop {
        for origin in input.keys() {
            if !connected.iter().any(|set| set.contains(origin)) {
                connected.push(HashSet::from_iter([*origin]));
                need_search = true;
                break;
            }
        }

        if !need_search {
            break;
        } else {
            need_search = false;
        }

        let mut last_size = 0;
        let last_set = connected.last_mut().unwrap();
        while last_size != last_set.len() {
            last_size = last_set.len();

            for (origin, connections) in input {
                if last_set.contains(origin) {
                    last_set.extend(connections);
                }
            }
        }
    }

    connected.len()
}
