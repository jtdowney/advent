use std::num::ParseIntError;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Report = Vec<i32>;

#[aoc_generator(day2)]
fn generator(input: &str) -> Result<Vec<Report>, ParseIntError> {
    input
        .lines()
        .map(|line| line.split_whitespace().map(|s| s.parse()).collect())
        .collect()
}

fn is_safe(report: &Report) -> bool {
    fn is_valid_diff(a: i32, b: i32) -> bool {
        let diff = a.abs_diff(b);
        (1..=3).contains(&diff)
    }

    let mut trend = None;

    for (&a, &b) in report.iter().tuple_windows() {
        if !is_valid_diff(a, b) {
            return false;
        }

        match trend {
            Some(true) if a > b => return false,
            Some(false) if a < b => return false,
            None => trend = Some(a <= b),
            _ => continue,
        }
    }

    true
}

fn permutations(report: &Report) -> impl Iterator<Item = Report> + '_ {
    (0..report.len())
        .map(move |i| {
            let mut reduced = report.clone();
            reduced.remove(i);
            reduced
        })
        .chain([report.clone()])
}

#[aoc(day2, part1)]
fn part1(input: &[Report]) -> usize {
    input.iter().filter(|report| is_safe(report)).count()
}

#[aoc(day2, part2)]
fn part2(input: &[Report]) -> usize {
    input
        .iter()
        .filter(|&report| permutations(report).any(|reduced| is_safe(&reduced)))
        .count()
}
