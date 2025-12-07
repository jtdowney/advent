use std::collections::{HashMap, HashSet};

use anyhow::bail;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Splitter,
    Beam,
}

#[derive(Debug)]
struct Simulation {
    cells: Vec<Vec<Cell>>,
    start: (usize, usize),
}

#[aoc_generator(day7)]
fn generator(input: &str) -> anyhow::Result<Simulation> {
    let mut start = None;
    let cells = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => Cell::Empty,
                    '^' => Cell::Splitter,
                    'S' => {
                        start = Some((x, y));
                        Cell::Beam
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let Some(start) = start else {
        bail!("No start found");
    };

    Ok(Simulation { cells, start })
}

fn simulate(input: &Simulation, merge: bool) -> usize {
    let width = input.cells[0].len();
    let (start_x, start_y) = input.start;

    let (positions, split_count) = input.cells[start_y + 1..].iter().fold(
        (HashMap::from([(start_x, 1usize)]), 0usize),
        |(positions, split_count), row| {
            let splitters = row
                .iter()
                .enumerate()
                .filter_map(|(x, &cell)| (cell == Cell::Splitter).then_some(x))
                .collect::<HashSet<_>>();

            let row_splits = positions
                .iter()
                .filter_map(|(&x, &count)| splitters.contains(&x).then_some(count))
                .sum::<usize>();

            let next = positions
                .into_iter()
                .flat_map(|(x, count)| {
                    let options = if splitters.contains(&x) {
                        [x.checked_sub(1), Some(x + 1)]
                    } else {
                        [Some(x), None]
                    };

                    options
                        .into_iter()
                        .flatten()
                        .filter(|&nx| nx < width)
                        .map(move |nx| (nx, count))
                })
                .fold(HashMap::new(), |mut acc, (x, count)| {
                    *acc.entry(x).or_default() += count;
                    acc
                });

            let next = if merge {
                next.into_keys().map(|k| (k, 1)).collect()
            } else {
                next
            };

            (next, split_count + row_splits)
        },
    );

    if merge {
        split_count
    } else {
        positions.values().sum()
    }
}

#[aoc(day7, part1)]
fn part1(input: &Simulation) -> usize {
    simulate(input, true)
}

#[aoc(day7, part2)]
fn part2(input: &Simulation) -> usize {
    simulate(input, false)
}
