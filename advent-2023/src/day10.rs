use std::{
    cmp::min,
    collections::{HashMap, HashSet},
};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::iproduct;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn reverse(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::West => Direction::East,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Cell(Vec<Direction>);

impl Cell {
    fn connects(&self, direction: &Direction) -> bool {
        self.0.contains(direction)
    }
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '|' => Cell(vec![Direction::North, Direction::South]),
            '-' => Cell(vec![Direction::West, Direction::East]),
            'L' => Cell(vec![Direction::North, Direction::East]),
            'J' => Cell(vec![Direction::North, Direction::West]),
            '7' => Cell(vec![Direction::South, Direction::West]),
            'F' => Cell(vec![Direction::South, Direction::East]),
            _ => panic!("Invalid cell: {value}"),
        }
    }
}

type Point = (i16, i16);
type Map = HashMap<Point, Cell>;

#[aoc_generator(day10)]
fn generator(input: &str) -> (Point, Map) {
    let mut start_point = Point::default();
    let mut map = Map::new();

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '.' => continue,
                'S' => {
                    start_point = (x as i16, y as i16);
                }
                c => {
                    map.insert((x as i16, y as i16), Cell::from(c));
                }
            }
        }
    }

    let (startx, starty) = start_point;
    let start_directions = [
        ((-1, 0), Direction::West),
        ((1, 0), Direction::East),
        ((0, -1), Direction::North),
        ((0, 1), Direction::South),
    ]
    .iter()
    .filter_map(|((dx, dy), direction)| {
        let np = (startx + dx, starty + dy);
        map.get(&np).and_then(|neighbor| {
            if neighbor.connects(&direction.reverse()) {
                Some(direction)
            } else {
                None
            }
        })
    })
    .copied()
    .collect::<Vec<Direction>>();

    map.insert(start_point, Cell(start_directions));
    (start_point, map)
}

fn neighbors(point @ (x, y): Point, map: &Map) -> impl Iterator<Item = Point> + '_ {
    let cell = &map[&point];

    [
        ((0, -1), Direction::North),
        ((0, 1), Direction::South),
        ((1, 0), Direction::East),
        ((-1, 0), Direction::West),
    ]
    .iter()
    .filter_map(move |((dx, dy), direction)| {
        if !cell.connects(direction) {
            return None;
        }

        let np = (x + dx, y + dy);
        map.get(&np).and_then(|neighbor| {
            if neighbor.connects(&direction.reverse()) {
                Some(np)
            } else {
                None
            }
        })
    })
}

#[aoc(day10, part1)]
fn part1((start_point, map): &(Point, Map)) -> Option<usize> {
    let mut visited = HashMap::new();
    let mut search = vec![(*start_point, 0)];
    while let Some((point, steps)) = search.pop() {
        visited
            .entry(point)
            .and_modify(|e| {
                *e = min(*e, steps);
            })
            .or_insert(steps);

        for neighbor in neighbors(point, map) {
            if let Some(&visited_steps) = visited.get(&neighbor) {
                if visited_steps < steps {
                    continue;
                }
            }

            search.push((neighbor, steps + 1))
        }
    }

    visited.into_values().max()
}

#[aoc(day10, part2)]
fn part2((start_point, map): &(Point, Map)) -> Option<i16> {
    let mut loop_points = HashSet::new();
    let mut search = vec![*start_point];
    while let Some(point) = search.pop() {
        if !loop_points.insert(point) {
            continue;
        }

        search.extend(neighbors(point, map));
    }

    let maxx = loop_points.iter().map(|&(x, _)| x).max()?;
    let maxy = loop_points.iter().map(|&(_, y)| y).max()?;
    let (_, count) = iproduct!(0..=maxy, 0..=maxx).fold((false, 0), |(in_loop, count), (y, x)| {
        let current = (x, y);
        if loop_points.contains(&current) {
            let Cell(directions) = &map[&current];
            if directions.contains(&Direction::North) {
                (!in_loop, count)
            } else {
                (in_loop, count)
            }
        } else if in_loop {
            (in_loop, count + 1)
        } else {
            (in_loop, count)
        }
    });

    Some(count)
}
