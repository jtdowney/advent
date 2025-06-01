use std::{
    collections::{HashMap, HashSet},
    iter,
};

use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
fn generate(input: &str) -> Result<HashMap<String, String>> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(')');
            let value = parts.next().context("missing value")?.to_string();
            let key = parts.next().context("missing key")?.to_string();
            Ok((key, value))
        })
        .collect()
}

#[aoc(day6, part1)]
fn part1(orbits: &HashMap<String, String>) -> usize {
    orbits
        .keys()
        .map(|start| iter::successors(Some(start), |prev| orbits.get(prev.as_str())).count() - 1)
        .sum()
}

#[aoc(day6, part2)]
fn part2(orbits: &HashMap<String, String>) -> usize {
    let you_orbit = iter::successors(Some("YOU".to_string()), |prev| {
        orbits.get(prev.as_str()).cloned()
    })
    .collect::<Vec<String>>();
    let san_orbit = iter::successors(Some("SAN".to_string()), |prev| {
        orbits.get(prev.as_str()).cloned()
    })
    .collect::<Vec<String>>();

    let san_ancestors = san_orbit.iter().cloned().collect::<HashSet<String>>();
    let ancestor = you_orbit
        .iter()
        .find(|orbit| san_ancestors.contains(orbit.as_str()))
        .unwrap();

    let transfers_to_ancestor = you_orbit
        .iter()
        .take_while(|orbit| orbit != &ancestor)
        .count()
        - 1;
    let transfers_from_ancestor = san_orbit
        .iter()
        .take_while(|orbit| orbit != &ancestor)
        .count()
        - 1;
    transfers_to_ancestor + transfers_from_ancestor
}
