use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy)]
struct Race {
    time: u16,
    distance: u16,
}

#[aoc_generator(day6)]
fn generator(input: &str) -> anyhow::Result<Vec<Race>> {
    let (times, distances) = input.split_once('\n').context("no newline")?;
    let (_, times) = times.split_once(':').context("no colon")?;
    let (_, distances) = distances.split_once(':').context("no colon")?;

    times
        .split_ascii_whitespace()
        .map(str::parse)
        .zip(distances.split_whitespace().map(str::parse))
        .map(|(time, distance)| {
            Ok(Race {
                time: time?,
                distance: distance?,
            })
        })
        .collect()
}

#[aoc(day6, part1)]
fn part1(input: &[Race]) -> usize {
    input
        .iter()
        .map(|&Race { time, distance }| {
            (1..time)
                .filter(|speed| {
                    let remaining = time - speed;
                    remaining * speed > distance
                })
                .count()
        })
        .product()
}

#[aoc(day6, part2)]
fn part2(input: &[Race]) -> anyhow::Result<usize> {
    let time = input
        .iter()
        .map(|&Race { time, .. }| time.to_string())
        .collect::<String>()
        .parse::<u64>()?;
    let distance = input
        .iter()
        .map(|&Race { distance, .. }| distance.to_string())
        .collect::<String>()
        .parse::<u64>()?;

    let count = (1..time)
        .filter(|speed| {
            let remaining = time - speed;
            remaining * speed > distance
        })
        .count();
    Ok(count)
}
