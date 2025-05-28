use std::{collections::HashMap, iter};

use aoc_runner_derive::{aoc, aoc_generator};
use eyre::eyre;
use itertools::{iproduct, Itertools};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, u32},
    combinator::map,
    multi::separated_list1,
    Finish, IResult, Parser,
};
use smallvec::{smallvec, SmallVec};

const START: &str = "AA";

#[derive(Debug, Default)]
struct Input {
    names: Vec<String>,
    flow_rates: Vec<u32>,
    connections: Vec<Vec<String>>,
    distances: HashMap<(usize, usize), u8>,
}

fn parse_valve(input: &str) -> IResult<&str, (String, u32, Vec<String>)> {
    let tunnels = separated_list1(tag(", "), map(alpha1, String::from));

    map(
        (
            tag("Valve "),
            alpha1,
            tag(" has flow rate="),
            u32,
            alt((
                tag("; tunnel leads to valve "),
                tag("; tunnels lead to valves "),
            )),
            tunnels,
        ),
        |(_, name, _, flow_rate, _, connections)| (String::from(name), flow_rate, connections),
    ).parse(input)
}

fn compute_distances(names: &[String], connections: &[Vec<String>]) -> HashMap<(usize, usize), u8> {
    let length = names.len();
    let mut distances = names
        .iter()
        .enumerate()
        .flat_map(|(i, _)| {
            iter::once(((i, i), 0)).chain(connections[i].iter().map(move |connection| {
                let j = names.iter().position(|n| n == connection).unwrap();
                ((i, j), 1)
            }))
        })
        .collect::<HashMap<(usize, usize), u8>>();

    for (k, j, i) in iproduct!(0..length, 0..length, 0..length) {
        let current = distances.get(&(i, j)).copied().unwrap_or(u8::MAX);
        let tentative = distances
            .get(&(i, k))
            .copied()
            .unwrap_or(u8::MAX)
            .saturating_add(distances.get(&(k, j)).copied().unwrap_or(u8::MAX));
        distances.insert((i, j), current.min(tentative));
    }

    distances
}

#[aoc_generator(day16)]
fn generator(input: &str) -> eyre::Result<Input> {
    let mut result = input
        .lines()
        .map(|line| {
            parse_valve(line)
                .finish()
                .map_err(|e| eyre!("Error parsing {}: {}", line, e))
                .map(|(_, o)| o)
        })
        .try_fold(Input::default(), |mut acc, item| {
            let (name, flow_rate, connections) = item?;
            acc.names.push(name);
            acc.flow_rates.push(flow_rate);
            acc.connections.push(connections);
            Ok::<_, eyre::Report>(acc)
        })?;
    result.distances = compute_distances(&result.names, &result.connections);
    Ok(result)
}

fn search(
    input: &Input,
    current: usize,
    candidates: Vec<usize>,
    time: u8,
    elephant: bool,
    cache: &mut HashMap<(usize, Vec<usize>, bool, u8), u32>,
) -> u32 {
    if let Some(&value) = cache.get(&(current, candidates.clone(), elephant, time)) {
        return value;
    }

    let result = candidates
        .iter()
        .positions(|&c| input.distances[&(current, c)] < time)
        .flat_map(|i| {
            let mut results: SmallVec<[u32; 2]> = smallvec![];
            let mut next_candidates = candidates.clone();
            let next = next_candidates.swap_remove(i);
            let next_time_remaining = time - input.distances[&(current, next)] - 1;
            let next_rate = input.flow_rates[next] * next_time_remaining as u32;
            let future_rate = search(
                input,
                next,
                next_candidates,
                next_time_remaining,
                elephant,
                cache,
            );

            let answer = next_rate + future_rate;
            results.push(answer);

            if elephant {
                let start = input.names.iter().position(|n| n == "AA").unwrap();
                let answer = search(input, start, candidates.clone(), 26, false, cache);
                results.push(answer);
            }

            results
        })
        .max()
        .unwrap_or_default();
    cache.insert((current, candidates, elephant, time), result);
    result
}

#[aoc(day16, part1)]
fn part1(input: &Input) -> u32 {
    let candidates = input.flow_rates.iter().positions(|&r| r > 0).collect_vec();
    let start = input.names.iter().position(|n| n == START).unwrap();
    let mut cache = HashMap::new();
    search(input, start, candidates, 30, false, &mut cache)
}

#[aoc(day16, part2)]
fn part2(input: &Input) -> u32 {
    let candidates = input.flow_rates.iter().positions(|&r| r > 0).collect_vec();
    let start = input.names.iter().position(|n| n == START).unwrap();
    let mut cache = HashMap::new();
    search(input, start, candidates, 26, true, &mut cache)
}
