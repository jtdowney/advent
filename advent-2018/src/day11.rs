use std::{collections::HashMap, num::ParseIntError};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::iproduct;

type Point = (i32, i32);
type Grid = HashMap<Point, i32>;

fn power_level((x, y): Point, serial: i32) -> i32 {
    let rack_id = x + 10;
    let mut power = rack_id * y;
    power += serial;
    power *= rack_id;
    power = (power / 100) % 10;
    power - 5
}

#[aoc_generator(day11)]
fn generator(input: &str) -> Result<Grid, ParseIntError> {
    let serial = input.parse()?;
    let grid = iproduct!(1..=300, 1..=300)
        .map(|p| (p, power_level(p, serial)))
        .collect();
    Ok(grid)
}

#[aoc(day11, part1)]
fn part1(input: &Grid) -> Option<String> {
    input
        .keys()
        .map(|&point @ (x, y)| {
            let score = iproduct!(0..3, 0..3)
                .map(|(dx, dy)| (x + dx, y + dy))
                .filter_map(|p| input.get(&p).copied())
                .sum::<i32>();
            (point, score)
        })
        .max_by_key(|&(_, score)| score)
        .map(|((x, y), _)| format!("{},{}", x, y))
}

#[aoc(day11, part2)]
fn part2(input: &Grid) -> Option<String> {
    let cache = iproduct!(1..=300, 1..=300).fold(HashMap::new(), |mut acc, point @ (x, y)| {
        let prev: i32 = acc.get(&(x, y - 1)).cloned().unwrap_or_default();
        let row = (1..=x).map(|i| input[&(i, y)]).sum::<i32>();
        let score = prev + row;

        acc.insert(point, score);
        acc
    });

    iproduct!(1..=300, 1..=300)
        .flat_map(|(x, y)| {
            let end = 300 - x.max(y);
            (0..=end).map(move |i| ((x, y), i))
        })
        .max_by_key(|((x, y), i)| {
            let target = cache[&(x + i, y + i)];
            let top = cache.get(&(x + i, y - 1)).copied().unwrap_or_default();
            let side = cache.get(&(x - 1, y + i)).copied().unwrap_or_default();
            let overage = cache.get(&(x - 1, y - 1)).copied().unwrap_or_default();

            target - top - side + overage
        })
        .map(|((x, y), i)| format!("{},{},{}", x, y, i + 1))
}
