use std::{
    collections::{HashMap, HashSet},
    iter,
};

use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Default)]
struct Rules {
    pages_after: HashMap<u32, HashSet<u32>>,
    pages_before: HashMap<u32, HashSet<u32>>,
}

#[aoc_generator(day5)]
fn generator(input: &str) -> anyhow::Result<(Rules, Vec<Vec<u32>>)> {
    let (rules_input, candidates_input) = input.split_once("\n\n").context("missing split")?;
    let rules = rules_input
        .lines()
        .try_fold(Rules::default(), |mut acc, line| {
            let (before, after) = line.split_once('|').context("missing split")?;
            let before = before.parse().context("failed to parse before")?;
            let after = after.parse().context("failed to parse after")?;
            acc.pages_after.entry(before).or_default().insert(after);
            acc.pages_before.entry(after).or_default().insert(before);
            anyhow::Ok(acc)
        })?;

    let candidates = candidates_input
        .lines()
        .map(|line| line.split(',').map(|page| page.parse::<u32>()).collect())
        .collect::<Result<_, _>>()?;

    Ok((rules, candidates))
}

fn is_ordered_page(before: u32, after: u32, rules: &Rules) -> bool {
    rules
        .pages_after
        .get(&before)
        .map_or(false, |set| set.contains(&after))
        && rules
            .pages_before
            .get(&after)
            .map_or(false, |set| set.contains(&before))
}

fn is_ordered_candidate(pages: &[u32], rules: &Rules) -> bool {
    pages
        .iter()
        .tuple_windows()
        .all(|(&before, &after)| is_ordered_page(before, after, rules))
}

#[aoc(day5, part1)]
fn part1((rules, candidates): &(Rules, Vec<Vec<u32>>)) -> u32 {
    candidates
        .iter()
        .filter(|pages| is_ordered_candidate(pages, rules))
        .map(|pages| {
            let mid = pages.len() / 2;
            pages[mid]
        })
        .sum()
}

#[aoc(day5, part2)]
fn part2((rules, candidates): &(Rules, Vec<Vec<u32>>)) -> Option<u32> {
    fn permutations<'a>(pages: &'a [u32], rules: &'a Rules) -> impl Iterator<Item = Vec<u32>> + 'a {
        let pages = pages.to_vec();
        iter::successors(Some(pages), |pages| {
            let (i, _) = pages
                .iter()
                .tuple_windows()
                .find_position(|(&before, &after)| !is_ordered_page(before, after, rules))?;

            let mut next = pages.clone();
            next.swap(i, i + 1);

            Some(next)
        })
    }

    candidates
        .iter()
        .filter(|pages| !is_ordered_candidate(pages, rules))
        .map(|pages| {
            let next =
                permutations(pages, rules).find(|pages| is_ordered_candidate(pages, rules))?;
            let mid = next.len() / 2;
            Some(next[mid])
        })
        .sum()
}
