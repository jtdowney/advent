use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};

type Rules = HashMap<u32, HashSet<u32>>;

#[derive(Clone, Copy, Eq, PartialEq)]
struct Page {
    value: u32,
    rules: &'static Rules,
}

impl Ord for Page {
    fn cmp(&self, other: &Self) -> Ordering {
        let before = self
            .rules
            .get(&self.value)
            .is_some_and(|set| set.contains(&other.value));
        if before {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl PartialOrd for Page {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Candidate {
    pages: Vec<u32>,
    rules: &'static Rules,
}

impl Candidate {
    fn is_sorted(&self) -> bool {
        self.pages().is_sorted()
    }

    fn sorted(&self) -> Self {
        let mut pages = self.pages().collect::<Vec<_>>();
        pages.sort();

        Self {
            pages: pages.into_iter().map(|page| page.value).collect(),
            rules: self.rules,
        }
    }

    fn pages(&self) -> impl Iterator<Item = Page> + '_ {
        self.pages.iter().map(move |&value| Page {
            value,
            rules: self.rules,
        })
    }

    fn midpoint(&self) -> u32 {
        let mid = self.pages.len() / 2;
        self.pages[mid]
    }
}

#[aoc_generator(day5)]
fn generator(input: &str) -> anyhow::Result<Vec<Candidate>> {
    let (rules_input, candidates_input) = input.split_once("\n\n").context("missing split")?;
    let rules = rules_input
        .lines()
        .try_fold(Rules::default(), |mut acc, line| {
            let (before, after) = line.split_once('|').context("missing split")?;
            let before = before.parse().context("failed to parse before")?;
            let after = after.parse().context("failed to parse after")?;
            acc.entry(before).or_default().insert(after);
            anyhow::Ok(acc)
        })?;
    let rules = Box::leak(Box::new(rules));

    let candidates = candidates_input
        .lines()
        .map(|line| {
            let pages = line
                .split(',')
                .map(str::parse::<u32>)
                .collect::<Result<Vec<_>, _>>()?;
            anyhow::Ok(Candidate { pages, rules })
        })
        .collect::<Result<_, _>>()?;

    Ok(candidates)
}

#[aoc(day5, part1)]
fn part1(input: &[Candidate]) -> u32 {
    input
        .iter()
        .filter(|candidate| candidate.is_sorted())
        .map(Candidate::midpoint)
        .sum()
}

#[aoc(day5, part2)]
fn part2(input: &[Candidate]) -> u32 {
    input
        .iter()
        .filter(|candidate| !candidate.is_sorted())
        .map(|candidate| {
            let next = candidate.sorted();
            next.midpoint()
        })
        .sum()
}
