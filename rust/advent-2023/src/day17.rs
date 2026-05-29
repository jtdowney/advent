use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
};

use aoc_runner_derive::{aoc, aoc_generator};

type Point = (i16, i16);
type Map = HashMap<Point, u8>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next_position(&self, (x, y): Point) -> Point {
        match self {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        }
    }

    fn next_directions(&self) -> Vec<Self> {
        match self {
            Direction::Up | Direction::Down => vec![Direction::Left, Direction::Right],
            Direction::Left | Direction::Right => vec![Direction::Up, Direction::Down],
        }
    }
}

#[aoc_generator(day17)]
fn generator(input: &str) -> Map {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes()
                .enumerate()
                .map(move |(x, b)| ((x as i16, y as i16), b - b'0'))
        })
        .collect()
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Step {
    position: Point,
    direction: Direction,
    heat_lost: u32,
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        self.heat_lost.cmp(&other.heat_lost)
    }
}

fn search(map: &Map, min_steps: u8, max_steps: u8) -> u32 {
    let maxx = map.keys().map(|&(x, _)| x).max().unwrap();
    let maxy = map.keys().map(|&(_, y)| y).max().unwrap();
    let goal = (maxx, maxy);

    let mut heat_map = HashMap::new();
    let mut visited = HashSet::new();
    let mut search = BinaryHeap::new();
    search.push(Reverse(Step {
        position: (0, 0),
        direction: Direction::Up,
        heat_lost: 0,
    }));

    while let Some(Reverse(Step {
        position,
        direction,
        heat_lost,
    })) = search.pop()
    {
        if position == goal {
            return heat_lost;
        }

        if !visited.insert((position, direction)) {
            continue;
        }

        for next_direction in direction.next_directions() {
            let mut next_heat_loss = heat_lost;
            let mut next_position = position;
            for steps in 1..=max_steps {
                next_position = next_direction.next_position(next_position);
                if let Some(&loss) = map.get(&next_position) {
                    next_heat_loss += loss as u32;
                    if let Some(&existing) = heat_map.get(&next_position)
                        && existing < next_heat_loss
                    {
                        continue;
                    }

                    if steps >= min_steps {
                        heat_map.insert(position, next_heat_loss);
                        search.push(Reverse(Step {
                            position: next_position,
                            direction: next_direction,
                            heat_lost: next_heat_loss,
                        }))
                    }
                }
            }
        }
    }

    panic!("no answer found")
}

#[aoc(day17, part1)]
fn part1(input: &Map) -> u32 {
    search(input, 1, 3)
}

#[aoc(day17, part2)]
fn part2(input: &Map) -> u32 {
    search(input, 4, 10)
}
