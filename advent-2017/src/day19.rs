use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

type Point = (u16, u16);
type Map = HashMap<Point, char>;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn reverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Clone, Debug)]
struct State<'a> {
    map: &'a Map,
    steps: usize,
    letters: String,
    point: Point,
    direction: Direction,
}

impl<'a> From<&'a Map> for State<'a> {
    fn from(map: &'a Map) -> Self {
        let point = map
            .keys()
            .find_map(|point @ (_, y)| if *y == 0 { Some(*point) } else { None })
            .unwrap();

        Self {
            map,
            steps: 0,
            letters: String::new(),
            point,
            direction: Direction::Down,
        }
    }
}

fn neighbors((x, y): Point) -> impl Iterator<Item = (Point, Direction)> {
    [
        ((-1, 0), Direction::Left),
        ((1, 0), Direction::Right),
        ((0, -1), Direction::Up),
        ((0, 1), Direction::Down),
    ]
    .iter()
    .map(move |&((dx, dy), direction)| {
        (
            (x.saturating_add_signed(dx), y.saturating_add_signed(dy)),
            direction,
        )
    })
}

impl<'a> State<'a> {
    fn walk(&mut self) {
        loop {
            let (x, y) = self.point;
            let Some(c) = self.map.get(&self.point).copied() else {
                break;
            };

            let next_direction = if c == '+' {
                let Some(direction) = neighbors(self.point).find_map(|(point, direction)| {
                    if self.map.contains_key(&point) && direction != self.direction.reverse() {
                        Some(direction)
                    } else {
                        None
                    }
                }) else {
                    break;
                };

                direction
            } else {
                self.direction
            };

            let next_point = match next_direction {
                Direction::Up => (x, y.saturating_sub(1)),
                Direction::Down => (x, y.saturating_add(1)),
                Direction::Left => (x.saturating_sub(1), y),
                Direction::Right => (x.saturating_add(1), y),
            };

            if c.is_ascii_alphabetic() {
                self.letters.push(c);
            }

            self.steps += 1;
            self.point = next_point;
            self.direction = next_direction;
        }
    }
}

#[aoc_generator(day19)]
fn generator(input: &str) -> Map {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c.is_ascii_whitespace() {
                    None
                } else {
                    Some(((x as u16, y as u16), c))
                }
            })
        })
        .collect()
}

#[aoc(day19, part1)]
fn part1(input: &Map) -> String {
    let mut state = State::from(input);
    state.walk();
    state.letters
}

#[aoc(day19, part2)]
fn part2(input: &Map) -> usize {
    let mut state = State::from(input);
    state.walk();
    state.steps
}
