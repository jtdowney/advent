use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

type Point = (i16, i16);
type Grid = HashMap<Point, State>;

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn reverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    fn forward(&self, (x, y): Point) -> Point {
        match self {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        }
    }
}

#[aoc_generator(day22)]
fn generator(input: &str) -> Grid {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                let point = (x as i16, y as i16);
                let state = if c == '#' {
                    State::Infected
                } else {
                    State::Clean
                };
                (point, state)
            })
        })
        .collect()
}

#[aoc(day22, part1)]
fn part1(input: &Grid) -> usize {
    let mut grid = input.clone();
    let side_length = (grid.len() as f32).sqrt() as i16;

    let mut direction = Direction::Up;
    let mut position = (side_length / 2, side_length / 2);
    let mut infections = 0;

    for _ in 0..10_000 {
        let state = grid.entry(position).or_insert(State::Clean);
        if *state == State::Infected {
            direction = direction.turn_right();
            *state = State::Clean;
        } else {
            direction = direction.turn_left();
            *state = State::Infected;
            infections += 1;
        }
        position = direction.forward(position);
    }

    infections
}

#[aoc(day22, part2)]
fn part2(input: &Grid) -> usize {
    let mut grid = input.clone();
    let side_length = (grid.len() as f32).sqrt() as i16;

    let mut direction = Direction::Up;
    let mut position = (side_length / 2, side_length / 2);
    let mut infections = 0;

    for _ in 0..10_000_000 {
        let state = grid.entry(position).or_insert(State::Clean);
        match *state {
            State::Clean => {
                direction = direction.turn_left();
                *state = State::Weakened;
            }
            State::Weakened => {
                *state = State::Infected;
                infections += 1;
            }
            State::Infected => {
                direction = direction.turn_right();
                *state = State::Flagged;
            }
            State::Flagged => {
                direction = direction.reverse();
                *state = State::Clean;
            }
        }

        position = direction.forward(position);
    }

    infections
}
