use std::collections::{HashMap, HashSet};

use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
struct Input {
    initial_state: HashSet<isize>,
    rules: HashMap<String, bool>,
}

#[aoc_generator(day12)]
fn generator(input: &str) -> anyhow::Result<Input> {
    let mut lines = input.lines();
    let initial_state = lines
        .next()
        .and_then(|line| {
            let mut parts = line.split(": ");
            let initial_state = parts.nth(1)?;
            Some(
                initial_state
                    .chars()
                    .enumerate()
                    .filter(|&(_, c)| c == '#')
                    .map(|(i, _)| i as isize)
                    .collect(),
            )
        })
        .context("unable to get initial state")?;

    let _ = lines.next();
    let rules = lines
        .map(|line| {
            let mut parts = line.split(" => ");
            let matches = parts.next()?.into();
            let next = parts.next().and_then(|s| s.chars().next())?;
            Some((matches, next == '#'))
        })
        .collect::<Option<HashMap<String, bool>>>()
        .context("unable to get rules")?;

    Ok(Input {
        initial_state,
        rules,
    })
}

fn next_generation(state: &HashSet<isize>, rules: &HashMap<String, bool>) -> HashSet<isize> {
    let mut next_state = HashSet::new();
    let min = state.iter().min().unwrap();
    let max = state.iter().max().unwrap();
    for i in min - 2..=max + 2 {
        let mut matches = String::new();
        for j in i - 2..=i + 2 {
            matches.push(if state.contains(&j) { '#' } else { '.' });
        }
        if let Some(&b) = rules.get(&matches)
            && b
        {
            next_state.insert(i);
        }
    }

    next_state
}

#[aoc(day12, part1)]
fn part1(input: &Input) -> isize {
    let state = (0..20).fold(input.initial_state.clone(), |state, _| {
        next_generation(&state, &input.rules)
    });

    state.iter().sum()
}

#[aoc(day12, part2)]
fn part2(input: &Input) -> Option<isize> {
    (1..)
        .scan(
            (0, 0, 0, input.initial_state.clone()),
            |(_, last_growth, last_sum, last_state), generation| {
                let state = next_generation(last_state, &input.rules);
                let sum = state.iter().sum::<isize>();
                let growth = sum - *last_sum;
                if *last_growth == growth {
                    None
                } else {
                    *last_growth = growth;
                    *last_sum = sum;
                    *last_state = state;
                    Some((generation, growth, sum))
                }
            },
        )
        .last()
        .map(|(generation, growth, sum)| {
            let rest = (50_000_000_000isize - generation) * growth;
            sum + rest
        })
}
