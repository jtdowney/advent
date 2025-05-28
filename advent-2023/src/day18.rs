use std::str::FromStr;

use anyhow::{anyhow, bail};
use aoc_runner_derive::{aoc, aoc_generator};

type Point = (i64, i64);

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(Direction::Right),
            '1' => Ok(Direction::Down),
            '2' => Ok(Direction::Left),
            '3' => Ok(Direction::Up),
            'D' => Ok(Direction::Down),
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            'U' => Ok(Direction::Up),
            _ => bail!("unknown direction: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct DigPlan {
    direction: Direction,
    length: i64,
}

#[derive(Debug, Clone, Copy)]
struct Instruction((DigPlan, DigPlan));

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use nom::{
            bytes::complete::{tag, take},
            character::complete::{anychar, i64, space1},
            combinator::{map, map_res},
            sequence::{delimited, pair},
            Finish, IResult, Parser,
        };

        fn direction(input: &str) -> IResult<&str, Direction> {
            map_res(anychar, Direction::try_from).parse(input)
        }

        fn hex_number(input: &str) -> IResult<&str, i64> {
            map_res(take(5usize), |s| i64::from_str_radix(s, 16)).parse(input)
        }

        map(
            (direction, space1, i64, space1, delimited(tag("(#"), pair(hex_number, direction), tag(")"))),
            |(direction, _, length, _, (length2, direction2))| {
                Instruction((
                    DigPlan { direction, length },
                    DigPlan {
                        direction: direction2,
                        length: length2,
                    },
                ))
            },
        ).parse(input)
        .finish()
        .map(|(_, o)| o)
        .map_err(|e| anyhow!("parse error: {:?}", e))
    }
}

#[aoc_generator(day18)]
fn generator(input: &str) -> anyhow::Result<Vec<Instruction>> {
    input.lines().map(str::parse).collect()
}

fn verticies(plans: &[DigPlan]) -> (Vec<Point>, i64) {
    let mut position = (0, 0);
    let mut perimeter = 0;
    let mut vertices = Vec::new();

    for DigPlan { direction, length } in plans {
        let (x, y) = position;
        position = match direction {
            Direction::Up => (x, y - length),
            Direction::Down => (x, y + length),
            Direction::Left => (x - length, y),
            Direction::Right => (x + length, y),
        };

        vertices.push(position);
        perimeter += length;
    }

    (vertices, perimeter)
}

fn shoelace_formula(points: &[Point]) -> i64 {
    let mut area = 0;
    let n = points.len();

    for i in 0..n {
        let j = (i + 1) % n;
        let (ax, ay) = points[i];
        let (bx, by) = points[j];
        area += (ay + by) * (ax - bx);
    }

    area.abs() / 2
}

fn picks_formula(verticies: &[Point], perimeter: i64) -> i64 {
    let area = shoelace_formula(verticies);
    let interior_count = area - perimeter / 2 + 1;
    interior_count + perimeter
}

#[aoc(day18, part1)]
fn part1(input: &[Instruction]) -> i64 {
    let plans = input
        .iter()
        .map(|&Instruction((plan, _))| plan)
        .collect::<Vec<_>>();
    let (verticies, perimeter) = verticies(&plans);
    picks_formula(&verticies, perimeter)
}

#[aoc(day18, part2)]
fn part2(input: &[Instruction]) -> i64 {
    let plans = input
        .iter()
        .map(|&Instruction((_, plan))| plan)
        .collect::<Vec<_>>();
    let (verticies, perimeter) = verticies(&plans);
    picks_formula(&verticies, perimeter)
}
