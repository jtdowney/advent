use std::{iter::Sum, ops::Add, str::FromStr};

use anyhow::bail;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Copy, Clone, Debug)]
enum Direction {
    North,
    NorthEast,
    NorthWest,
    South,
    SouthEast,
    SouthWest,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Direction, Self::Err> {
        let direction = match value {
            "n" => Direction::North,
            "ne" => Direction::NorthEast,
            "nw" => Direction::NorthWest,
            "s" => Direction::South,
            "se" => Direction::SouthEast,
            "sw" => Direction::SouthWest,
            _ => bail!("unknown direction: {}", value),
        };

        Ok(direction)
    }
}

impl Direction {
    fn movement(&self) -> Hex {
        match *self {
            Direction::North => Hex(1, 0, -1),
            Direction::NorthEast => Hex(1, -1, 0),
            Direction::NorthWest => Hex(0, 1, -1),
            Direction::South => Hex(-1, 0, 1),
            Direction::SouthEast => Hex(0, -1, 1),
            Direction::SouthWest => Hex(-1, 1, 0),
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
struct Hex(i32, i32, i32);

impl Add for Hex {
    type Output = Hex;

    fn add(self, Hex(oa, ob, oc): Hex) -> Self {
        let Hex(sa, sb, sc) = self;
        Hex(sa + oa, sb + ob, sc + oc)
    }
}

impl Sum for Hex {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |acc, i| acc + i)
    }
}

impl Hex {
    fn distance(&self, Hex(oa, ob, oc): Hex) -> i32 {
        let Hex(sa, sb, sc) = self;
        ((sa - oa).abs() + (sb - ob).abs() + (sc - oc).abs()) / 2
    }
}

#[aoc_generator(day11)]
fn generator(input: &str) -> anyhow::Result<Vec<Direction>> {
    input.split(',').map(|s| s.parse()).collect()
}

#[aoc(day11, part1)]
fn part1(input: &[Direction]) -> i32 {
    let start = Hex::default();
    let end = input.iter().map(|d| d.movement()).sum::<Hex>();
    start.distance(end)
}

#[aoc(day11, part2)]
fn part2(input: &[Direction]) -> i32 {
    let start = Hex::default();
    let (max, _) = input
        .iter()
        .fold((0, start), |(max_distance, end), direction| {
            let end = end + direction.movement();
            let distance = start.distance(end);
            (max_distance.max(distance), end)
        });

    max
}
