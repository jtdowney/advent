use std::collections::HashSet;

use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};

type Component = (u32, u32);

#[derive(Clone, Debug, Default)]
struct Bridge {
    components: Vec<Component>,
    port: u32,
    strength: u32,
}

impl Bridge {
    fn is_compatible(&self, (a, b): Component) -> bool {
        a == self.port || b == self.port
    }
}

#[aoc_generator(day24)]
fn generator(input: &str) -> anyhow::Result<HashSet<Component>> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split('/');
            let a = parts.next().context("unable to get part")?.parse::<u32>()?;
            let b = parts.next().context("unable to get part")?.parse::<u32>()?;
            Ok((a, b))
        })
        .collect()
}

struct State {
    components: HashSet<Component>,
    bridge: Bridge,
}

#[aoc(day24, part1)]
fn part1(input: &HashSet<Component>) -> u32 {
    let mut search = vec![State {
        components: input.clone(),
        bridge: Bridge::default(),
    }];

    let mut max_strength = 0;
    while let Some(current) = search.pop() {
        let next_components = current
            .components
            .iter()
            .filter(|&c| current.bridge.is_compatible(*c));
        for &component in next_components {
            let (a, b) = component;
            let mut next_bridge = current.bridge.clone();
            next_bridge.components.push(component);
            next_bridge.port = if a == next_bridge.port { b } else { a };
            next_bridge.strength += a + b;

            max_strength = max_strength.max(next_bridge.strength);

            let mut next_components = current.components.clone();
            next_components.remove(&component);

            search.push(State {
                components: next_components,
                bridge: next_bridge,
            });
        }
    }

    max_strength
}

#[aoc(day24, part2)]
fn part2(input: &HashSet<Component>) -> Option<u32> {
    let mut search = vec![State {
        components: input.clone(),
        bridge: Bridge::default(),
    }];

    let mut bridges: Vec<Bridge> = vec![];
    while let Some(current) = search.pop() {
        match bridges.last().map(|b| b.components.len()) {
            Some(len) if current.bridge.components.len() > len => {
                bridges.clear();
                bridges.push(current.bridge.clone());
            }
            Some(len) if current.bridge.components.len() == len => {
                bridges.push(current.bridge.clone());
            }
            None => bridges.push(current.bridge.clone()),
            _ => {}
        }

        let next_components = current
            .components
            .iter()
            .filter(|&c| current.bridge.is_compatible(*c));
        for &component in next_components {
            let (a, b) = component;
            let mut next_bridge = current.bridge.clone();
            next_bridge.components.push(component);
            next_bridge.port = if a == next_bridge.port { b } else { a };
            next_bridge.strength += a + b;

            let mut next_components = current.components.clone();
            next_components.remove(&component);

            search.push(State {
                components: next_components,
                bridge: next_bridge,
            });
        }
    }

    bridges.iter().map(|b| b.strength).max()
}
