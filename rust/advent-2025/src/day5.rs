use std::ops::RangeInclusive;

use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};

struct Input {
    ranges: Vec<RangeInclusive<u64>>,
    ids: Vec<u64>,
}

#[aoc_generator(day5)]
fn generator(input: &str) -> anyhow::Result<Input> {
    let (ranges_str, ids_str) = input.split_once("\n\n").context("expected blank line")?;

    let ranges = ranges_str
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            let range = start.parse::<u64>()?..=end.parse()?;
            Ok(range)
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    let ids = ids_str
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<u64>, _>>()?;

    Ok(Input { ranges, ids })
}

#[aoc(day5, part1)]
fn part1(input: &Input) -> usize {
    input
        .ids
        .iter()
        .filter(|id| input.ranges.iter().any(|range| range.contains(id)))
        .count()
}

#[aoc(day5, part2)]
fn part2(input: &Input) -> u64 {
    let mut ranges = input.ranges.clone();
    ranges.sort_by_key(|r| *r.start());

    ranges
        .into_iter()
        .fold(Vec::<RangeInclusive<u64>>::new(), |mut merged, range| {
            match merged.last_mut() {
                Some(last) if *last.end() >= *range.start() => {
                    *last = *last.start()..=(*last.end()).max(*range.end());
                }
                _ => merged.push(range),
            }
            merged
        })
        .iter()
        .map(|r| r.end() - r.start() + 1)
        .sum()
}
