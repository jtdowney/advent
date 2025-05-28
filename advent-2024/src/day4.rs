use std::{collections::HashMap, iter};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{Itertools, iproduct};

type Position = (i32, i32);
type Grid = HashMap<Position, char>;

const NEIGHBORS: &[Position] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[aoc_generator(day4)]
fn generator(input: &str) -> Grid {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i32, y as i32), c))
        })
        .collect()
}

fn sequence(
    grid: &Grid,
    position: Position,
    (dx, dy): Position,
) -> impl Iterator<Item = char> + '_ {
    iter::successors(Some(position), move |&(x, y)| {
        if grid.contains_key(&(x + dx, y + dy)) {
            Some((x + dx, y + dy))
        } else {
            None
        }
    })
    .flat_map(|p| grid.get(&p))
    .copied()
}

fn sequence_strings(
    grid: &Grid,
    position @ (x, y): Position,
    length: usize,
) -> impl Iterator<Item = (Position, Position, String)> + '_ {
    NEIGHBORS.iter().map(move |&neighbor @ (dx, dy)| {
        let value = sequence(grid, position, neighbor).take(length).collect();
        (position, (x + dx, y + dy), value)
    })
}

fn calculate_bounds(grid: &Grid) -> (i32, i32, i32, i32) {
    let (minx, maxx) = grid.keys().map(|&(x, _)| x).minmax().into_option().unwrap();
    let (miny, maxy) = grid.keys().map(|&(_, y)| y).minmax().into_option().unwrap();
    (minx, maxx, miny, maxy)
}

#[aoc(day4, part1)]
fn part1(grid: &Grid) -> usize {
    let (minx, maxx, miny, maxy) = calculate_bounds(grid);
    iproduct!(minx..=maxx, miny..=maxy)
        .flat_map(|position| sequence_strings(grid, position, 4))
        .filter(|(_, _, value)| value == "XMAS")
        .count()
}

#[aoc(day4, part2)]
fn part2(grid: &Grid) -> usize {
    fn is_crossing((x1, y1): Position, (x2, y2): Position) -> bool {
        (x1 == x2 || y1 == y2) && (x1.abs_diff(x2) == 2 || y1.abs_diff(y2) == 2)
    }

    let (minx, maxx, miny, maxy) = calculate_bounds(grid);
    let candidates = iproduct!(minx..=maxx, miny..=maxy)
        .flat_map(|position| sequence_strings(grid, position, 3))
        .filter(|(_, _, value)| value == "MAS")
        .fold(
            HashMap::<Position, Vec<Position>>::new(),
            |mut acc, (position, middle, _)| {
                acc.entry(middle).or_default().push(position);
                acc
            },
        );

    candidates
        .values()
        .flat_map(|starts| {
            starts
                .iter()
                .tuple_combinations()
                .filter(|&(&a, &b)| is_crossing(a, b))
        })
        .count()
}
