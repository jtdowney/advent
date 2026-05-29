use std::num::ParseIntError;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day20)]
fn generator(input: &str) -> Result<Vec<(u32, u32)>, ParseIntError> {
    let mut ranges = input
        .lines()
        .map(|line| {
            let mut parts = line.split('-');
            let start = parts.next().unwrap().parse::<u32>()?;
            let end = parts.next().unwrap().parse::<u32>()?;
            Ok((start, end))
        })
        .collect::<Result<Vec<_>, _>>()?;
    ranges.sort_by_key(|&(start, _)| start);
    Ok(ranges)
}

#[aoc(day20, part1)]
fn part1(input: &[(u32, u32)]) -> u32 {
    input.iter().fold(
        u32::MIN,
        |acc, &(start, end)| {
            if start > acc { acc } else { acc.max(end + 1) }
        },
    )
}

#[aoc(day20, part2)]
fn part2(input: &[(u32, u32)]) -> u32 {
    let (_, count) = input
        .iter()
        .fold((u32::MIN, 0), |(acc, count), &(start, end)| {
            let next = end.saturating_add(1);
            if start > acc {
                (next, count + start - acc)
            } else {
                (acc.max(next), count)
            }
        });
    count
}
