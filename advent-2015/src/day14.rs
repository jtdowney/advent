use std::{collections::HashMap, str::FromStr};

use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};
use once_cell::sync::Lazy;
use regex::Regex;

static REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(?P<name>.+) can fly (?P<speed>\d+) km/s for (?P<time>\d+) seconds, but then must rest for (?P<rest>\d+) seconds.$").unwrap()
});

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Reindeer {
    speed: usize,
    time: usize,
    rest: usize,
}

impl FromStr for Reindeer {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let captures = REGEX
            .captures(s)
            .context(format!("unable to match line: {s}"))?;
        let speed = captures
            .name("speed")
            .and_then(|v| v.as_str().parse().ok())
            .context("unable to read speed")?;
        let time = captures
            .name("time")
            .and_then(|v| v.as_str().parse().ok())
            .context("unable to read time")?;
        let rest = captures
            .name("rest")
            .and_then(|v| v.as_str().parse().ok())
            .context("unable to read rest")?;
        Ok(Reindeer { speed, time, rest })
    }
}

#[derive(Debug, Default)]
enum Status {
    #[default]
    Flying,
    Resting,
}

#[derive(Debug, Default)]
struct State {
    status: Status,
    remaining: usize,
    distance: usize,
    points: usize,
}

#[aoc_generator(day14)]
fn generator(input: &str) -> anyhow::Result<Vec<Reindeer>> {
    input.lines().map(str::parse).collect()
}

fn simulate(input: &[Reindeer]) -> HashMap<Reindeer, State> {
    let state = input
        .iter()
        .cloned()
        .map(|r| {
            let state = State {
                remaining: r.time,
                ..Default::default()
            };
            (r, state)
        })
        .collect::<HashMap<Reindeer, State>>();

    (0..2503).fold(state, |mut state, _| {
        for (reindeer, state) in state.iter_mut() {
            if state.remaining == 0 {
                match state.status {
                    Status::Flying => {
                        state.status = Status::Resting;
                        state.remaining = reindeer.rest;
                    }
                    Status::Resting => {
                        state.status = Status::Flying;
                        state.remaining = reindeer.time;
                    }
                }
            }

            if let Status::Flying = state.status {
                state.distance += reindeer.speed;
            }

            state.remaining -= 1;
        }

        let top_distance = state.values().map(|s| s.distance).max().unwrap();
        let winners = state.iter_mut().filter(|(_, s)| s.distance == top_distance);
        for (_, state) in winners {
            state.points += 1;
        }

        state
    })
}

#[aoc(day14, part1)]
fn part1(input: &[Reindeer]) -> usize {
    let state = simulate(input);
    state.into_values().map(|s| s.distance).max().unwrap()
}

#[aoc(day14, part2)]
fn part2(input: &[Reindeer]) -> usize {
    let state = simulate(input);
    state.into_values().map(|s| s.points).max().unwrap()
}
