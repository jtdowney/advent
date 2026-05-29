use std::{collections::HashMap, iter, num::ParseIntError};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::iproduct;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn prev(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn advance_point(&self, (x, y): Point) -> Point {
        match *self {
            Direction::Down => (x, y - 1),
            Direction::Up => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        }
    }
}

type Point = (i32, i32);

#[aoc_generator(day3)]
fn generator(input: &str) -> Result<i32, ParseIntError> {
    input.parse()
}

#[aoc(day3, part1)]
fn part1(input: &i32) -> Option<i32> {
    let (map, _, _) = (1..=*input).fold(
        (HashMap::new(), (0i32, 0i32), Direction::Right),
        |(mut acc, point, direction), i| {
            acc.insert(point, i);

            let next_point = direction.advance_point(point);
            let (next_point, next_direction) = if acc.contains_key(&next_point) {
                (direction.prev().advance_point(point), direction)
            } else {
                (next_point, direction.next())
            };

            (acc, next_point, next_direction)
        },
    );

    map.iter().find_map(|((x, y), value)| {
        if *value == *input {
            Some(x.abs() + y.abs())
        } else {
            None
        }
    })
}

#[aoc(day3, part2)]
fn part2(input: &i32) -> Option<i32> {
    let mut point = (0i32, 0i32);
    let mut map = HashMap::new();
    map.insert(point, 1);

    let mut direction = Direction::Right;
    iter::from_fn(|| {
        let value = iproduct!(-1..=1, -1..=1)
            .map(|(dx, dy)| {
                let (x, y) = point;
                map.get(&(x + dx, y + dy)).copied().unwrap_or_default()
            })
            .sum();

        let next_point = direction.advance_point(point);
        let (next_point, next_direction) = if map.contains_key(&next_point) {
            (direction.prev().advance_point(point), direction)
        } else {
            (next_point, direction.next())
        };

        map.insert(point, value);
        point = next_point;
        direction = next_direction;

        Some(value)
    })
    .find(|&value| value > *input)
}
