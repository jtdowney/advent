use std::collections::HashSet;

use anyhow::bail;
use aoc_runner_derive::{aoc, aoc_generator};

type Cache = Vec<Vec<usize>>;

#[derive(Debug)]
struct Simulation {
    width: usize,
    splitter_rows: Vec<HashSet<usize>>,
    start_x: usize,
}

#[aoc_generator(day7)]
fn generator(input: &str) -> anyhow::Result<Simulation> {
    let mut start = None;
    let mut width = 0;

    let splitter_rows: Vec<HashSet<usize>> = input
        .lines()
        .enumerate()
        .filter_map(|(y, line)| {
            width = line.len();
            let splitters: HashSet<usize> = line
                .chars()
                .enumerate()
                .filter_map(|(x, c)| match c {
                    '^' => Some(x),
                    'S' => {
                        start = Some((x, y));
                        None
                    }
                    _ => None,
                })
                .collect();

            (!splitters.is_empty()).then_some(splitters)
        })
        .collect();

    let Some((start_x, _)) = start else {
        bail!("No start found");
    };

    Ok(Simulation {
        width,
        splitter_rows,
        start_x,
    })
}

#[aoc(day7, part1)]
fn part1(input: &Simulation) -> usize {
    let width = input.width;

    let (_, split_count) = input.splitter_rows.iter().fold(
        (HashSet::from([input.start_x]), 0usize),
        |(positions, split_count), splitters| {
            let row_splits = positions.intersection(splitters).count();

            let next = positions
                .into_iter()
                .flat_map(|x| {
                    if splitters.contains(&x) {
                        [x.checked_sub(1), Some(x + 1)]
                    } else {
                        [Some(x), None]
                    }
                    .into_iter()
                    .flatten()
                    .filter(|&nx| nx < width)
                })
                .collect();

            (next, split_count + row_splits)
        },
    );

    split_count
}

fn ways(x: usize, row_idx: usize, input: &Simulation, cache: &mut Cache) -> usize {
    if row_idx >= input.splitter_rows.len() {
        return 1;
    }

    if cache[x][row_idx] > 0 {
        return cache[x][row_idx];
    }

    let result = if input.splitter_rows[row_idx].contains(&x) {
        let left = x
            .checked_sub(1)
            .map_or(0, |lx| ways(lx, row_idx + 1, input, cache));
        let right = if x + 1 < input.width {
            ways(x + 1, row_idx + 1, input, cache)
        } else {
            0
        };
        left + right
    } else {
        ways(x, row_idx + 1, input, cache)
    };

    cache[x][row_idx] = result;
    result
}

#[aoc(day7, part2)]
fn part2(input: &Simulation) -> usize {
    let num_rows = input.splitter_rows.len();
    let mut cache = vec![vec![0; num_rows + 1]; input.width];
    ways(input.start_x, 0, input, &mut cache)
}
