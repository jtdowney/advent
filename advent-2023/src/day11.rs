use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Point = (i64, i64);
type Map = HashSet<Point>;

#[aoc_generator(day11)]
fn generator(input: &str) -> Map {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some((x as i64, y as i64))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn distance((ax, ay): &Point, (bx, by): &Point) -> i64 {
    (ax - bx).abs() + (ay - by).abs()
}

fn expand(map: &Map, amount: i64) -> Option<Map> {
    let maxx = map.iter().map(|&(x, _)| x).max()?;
    let maxy = map.iter().map(|&(_, y)| y).max()?;
    let emptyx = (0..=maxx)
        .filter(|&search| !map.iter().any(|&(x, _)| x == search))
        .collect_vec();
    let emptyy = (0..=maxy)
        .filter(|&search| !map.iter().any(|&(_, y)| y == search))
        .collect_vec();

    let expanded = map
        .iter()
        .map(|(x, y)| {
            let dx = emptyx.iter().filter(|&ex| ex < x).count() as i64;
            let dy = emptyy.iter().filter(|&ey| ey < y).count() as i64;

            let dx = dx * (amount - 1);
            let dy = dy * (amount - 1);

            (x + dx, y + dy)
        })
        .collect();
    Some(expanded)
}

#[aoc(day11, part1)]
fn part1(input: &Map) -> Option<i64> {
    let expanded = expand(input, 2)?;
    let answer = expanded
        .iter()
        .tuple_combinations()
        .map(|(a, b)| distance(a, b))
        .sum();
    Some(answer)
}

#[aoc(day11, part2)]
fn part2(input: &Map) -> Option<i64> {
    let expanded = expand(input, 1_000_000)?;
    let answer = expanded
        .iter()
        .tuple_combinations()
        .map(|(a, b)| distance(a, b))
        .sum();
    Some(answer)
}
