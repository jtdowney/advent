use std::collections::HashMap;

use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};
use num::Integer;

const START: &str = "AAA";
const END: &str = "ZZZ";

type Map = HashMap<String, (String, String)>;

#[aoc_generator(day8)]
fn generator(input: &str) -> anyhow::Result<(String, Map)> {
    let (instructions, rest) = input.split_once("\n\n").context("splitting instructions")?;
    let instructions = instructions.to_owned();

    let map = rest
        .lines()
        .map(|line| {
            let (node, next_steps) = line.split_once(" = ").context("splitting equals")?;
            let (left, right) = next_steps.split_once(", ").context("splitting comma")?;

            let node = node.to_owned();
            let left = left.trim_start_matches('(').to_owned();
            let right = right.trim_end_matches(')').to_owned();
            Ok((node, (left, right)))
        })
        .collect::<anyhow::Result<Map>>()?;

    Ok((instructions, map))
}

#[aoc(day8, part1)]
fn part1(input: &(String, Map)) -> usize {
    let (instructions, map) = input;
    instructions
        .chars()
        .cycle()
        .scan(START, |state, instruction| {
            if *state == END {
                return None;
            }

            map.get(*state).map(|(left, right)| {
                if instruction == 'L' {
                    *state = left.as_str();
                } else {
                    *state = right.as_str();
                }

                instruction
            })
        })
        .count()
}

#[aoc(day8, part2)]
fn part2(input: &(String, Map)) -> Option<usize> {
    let (instructions, map) = input;
    let start_nodes = map
        .keys()
        .filter(|key| key.ends_with('A'))
        .cloned()
        .collect::<Vec<String>>();

    start_nodes
        .iter()
        .map(|start| {
            instructions
                .chars()
                .cycle()
                .scan(start.as_str(), |state, instruction| {
                    if state.ends_with('Z') {
                        return None;
                    }

                    map.get(*state).map(|(left, right)| {
                        if instruction == 'L' {
                            *state = left.as_str();
                        } else {
                            *state = right.as_str();
                        }

                        instruction
                    })
                })
                .count()
        })
        .reduce(|acc, length| acc.lcm(&length))
}
