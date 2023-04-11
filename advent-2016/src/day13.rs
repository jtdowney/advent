use std::collections::{HashSet, VecDeque};

use aoc_runner_derive::aoc;
use itertools::iproduct;

type Point = (u32, u32);
const STARTING_POINT: Point = (1, 1);
const ENDING_POINT: Point = (31, 39);
const MAX_STEPS: usize = 50;

fn is_open_space((x, y): Point, favorite_number: u32) -> bool {
    let value = x * x + 3 * x + 2 * x * y + y + y * y + favorite_number;
    value.count_ones() % 2 == 0
}

fn neighbors((x, y): Point, favorite_number: u32) -> impl Iterator<Item = Point> {
    iproduct!(-1..=1, -1..=1)
        .filter(|&(dx, dy)| dx == 0 || dy == 0)
        .filter(|&delta| delta != (0, 0))
        .map(move |(dx, dy)| (x.saturating_add_signed(dx), y.saturating_add_signed(dy)))
        .filter(move |&point| is_open_space(point, favorite_number))
}

#[aoc(day13, part1)]
fn part1(input: &str) -> anyhow::Result<usize> {
    let favorite_number = input.parse()?;

    let mut search = VecDeque::from_iter([(STARTING_POINT, 0)]);
    let mut visited = HashSet::new();

    while let Some((point, steps)) = search.pop_front() {
        if point == ENDING_POINT {
            return Ok(steps);
        }

        if !visited.insert(point) {
            continue;
        }

        for neighbor in neighbors(point, favorite_number) {
            search.push_back((neighbor, steps + 1));
        }
    }

    unreachable!()
}

#[aoc(day13, part2)]
fn part2(input: &str) -> anyhow::Result<usize> {
    let favorite_number = input.parse()?;

    let mut search = VecDeque::from_iter([(STARTING_POINT, 0)]);
    let mut visited = HashSet::new();

    while let Some((point, steps)) = search.pop_front() {
        if steps == MAX_STEPS {
            continue;
        }

        if !visited.insert(point) {
            continue;
        }

        for neighbor in neighbors(point, favorite_number) {
            search.push_back((neighbor, steps + 1));
        }
    }

    Ok(visited.len())
}
