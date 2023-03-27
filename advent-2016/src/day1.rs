use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[aoc_generator(day1)]
fn generator(input: &str) -> anyhow::Result<Vec<(Direction, i32)>> {
    input
        .split(", ")
        .map(|part| {
            let (turn, distance) = part.split_at(1);
            let distance = distance.parse::<i32>()?;
            Ok((turn, distance))
        })
        .scan(Direction::North, |direction, part| {
            let (turn, distance) = match part {
                Ok(part) => part,
                Err(e) => return Some(Err(e)),
            };

            *direction = match turn {
                "R" => match direction {
                    Direction::North => Direction::East,
                    Direction::East => Direction::South,
                    Direction::South => Direction::West,
                    Direction::West => Direction::North,
                },
                "L" => match direction {
                    Direction::North => Direction::West,
                    Direction::West => Direction::South,
                    Direction::South => Direction::East,
                    Direction::East => Direction::North,
                },
                _ => unreachable!(),
            };
            Some(Ok((*direction, distance)))
        })
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &[(Direction, i32)]) -> i32 {
    let (x, y) = input
        .iter()
        .fold((0, 0), |(x, y), (direction, distance)| match direction {
            Direction::North => (x, y + distance),
            Direction::South => (x, y - distance),
            Direction::East => (x + distance, y),
            Direction::West => (x - distance, y),
        });

    x.abs() + y.abs()
}

#[aoc(day1, part2)]
fn part2(input: &[(Direction, i32)]) -> i32 {
    let mut visited = HashSet::new();
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    for &(direction, distance) in input {
        for _ in 0..distance {
            match direction {
                Direction::North => y += 1,
                Direction::South => y -= 1,
                Direction::East => x += 1,
                Direction::West => x -= 1,
            }

            if !visited.insert((x, y)) {
                return x.abs() + y.abs();
            }
        }
    }

    unreachable!()
}
