use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::iproduct;

type Point = (i32, i32);

#[aoc_generator(day18)]
fn generator(input: &str) -> HashSet<Point> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn neighbors((x, y): Point) -> impl Iterator<Item = Point> {
    iproduct!((-1..=1), (-1..=1))
        .filter(|&(dx, dy)| !(dx == 0 && dy == 0))
        .map(move |(dx, dy)| (x + dx, y + dy))
}

fn step(lights: &HashSet<Point>) -> HashSet<Point> {
    iproduct!((0..100), (0..100))
        .filter(|&point @ (x, y)| {
            let neighbors = neighbors(point).filter(|&n| lights.contains(&n)).count();
            if lights.contains(&(x, y)) {
                neighbors == 2 || neighbors == 3
            } else {
                neighbors == 3
            }
        })
        .collect()
}

#[aoc(day18, part1)]
fn part1(input: &HashSet<Point>) -> usize {
    let mut lights = input.clone();
    for _ in 0..100 {
        lights = step(&lights);
    }

    lights.len()
}

#[aoc(day18, part2)]
fn part2(input: &HashSet<Point>) -> usize {
    let mut lights = input.clone();
    let stuck_on = [(0, 0), (0, 99), (99, 0), (99, 99)];

    for _ in 0..100 {
        lights.extend(stuck_on.iter().copied());
        lights = step(&lights);
    }

    lights.extend(stuck_on.iter().copied());
    lights.len()
}
