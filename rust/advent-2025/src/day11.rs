use std::collections::HashMap;

use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day11)]
fn generator(input: &str) -> anyhow::Result<HashMap<String, Vec<String>>> {
    input
        .lines()
        .map(|line| {
            let (name, path) = line.split_once(": ").context("invalid device")?;
            Ok((
                name.to_string(),
                path.split_whitespace().map(String::from).collect(),
            ))
        })
        .collect()
}

fn count_paths(
    name: &str,
    dac: bool,
    fft: bool,
    devices: &HashMap<String, Vec<String>>,
    cache: &mut HashMap<(String, bool, bool), u64>,
) -> u64 {
    let dac = dac || name == "dac";
    let fft = fft || name == "fft";

    if name == "out" {
        return u64::from(dac && fft);
    }

    let cache_key = (name.to_string(), dac, fft);
    if let Some(&cached) = cache.get(&cache_key) {
        return cached;
    }

    let result = devices
        .get(name)
        .map(|neighbors| {
            neighbors
                .iter()
                .map(|n| count_paths(n, dac, fft, devices, cache))
                .sum()
        })
        .unwrap_or_default();

    cache.insert(cache_key, result);
    result
}

#[aoc(day11, part1)]
fn part1(input: &HashMap<String, Vec<String>>) -> u64 {
    let mut cache = HashMap::new();
    count_paths("you", true, true, input, &mut cache)
}

#[aoc(day11, part2)]
fn part2(input: &HashMap<String, Vec<String>>) -> u64 {
    let mut cache = HashMap::new();
    count_paths("svr", false, false, input, &mut cache)
}
