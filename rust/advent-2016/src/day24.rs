use std::collections::{HashMap, HashSet, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Cell {
    Empty,
    Wall,
    Goal(u8),
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '#' => Cell::Wall,
            '.' => Cell::Empty,
            c if c.is_ascii_digit() => Cell::Goal(c as u8 - b'0'),
            _ => panic!("Invalid cell: {}", c),
        }
    }
}

type Point = (usize, usize);
type Map = HashMap<Point, Cell>;

#[aoc_generator(day24)]
fn generator(input: &str) -> Map {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                let cell = Cell::from(c);
                let point = (x, y);
                (point, cell)
            })
        })
        .collect()
}

fn find_goal(map: &Map, goal: u8) -> Option<Point> {
    map.iter().find_map(|(point, cell)| {
        if cell == &Cell::Goal(goal) {
            Some(*point)
        } else {
            None
        }
    })
}

fn distance(map: &Map, start: Point, end: Point) -> usize {
    let mut search = VecDeque::from_iter([(start, 0)]);
    let mut visited = HashSet::new();
    while let Some((point, steps)) = search.pop_front() {
        if point == end {
            return steps;
        }

        if !visited.insert(point) {
            continue;
        }

        let (x, y) = point;
        for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_point = (x.saturating_add_signed(*dx), y.saturating_add_signed(*dy));
            if let Some(cell) = map.get(&new_point)
                && *cell != Cell::Wall
            {
                search.push_back((new_point, steps + 1));
            }
        }
    }

    unreachable!()
}

fn solve(map: &Map, return_to_start: bool) -> Option<usize> {
    let start = find_goal(map, 0)?;
    let goals = map
        .iter()
        .filter(|&(point, _)| *point != start)
        .filter_map(|(point, cell)| match cell {
            Cell::Goal(_) => Some(*point),
            _ => None,
        })
        .collect::<Vec<_>>();

    goals
        .iter()
        .permutations(goals.len())
        .map(|mut path| {
            if return_to_start {
                path.push(&start);
            }

            let (_, steps) = path.into_iter().fold((start, 0), |(point, steps), &goal| {
                let distance = distance(map, point, goal);
                (goal, steps + distance)
            });
            steps
        })
        .min()
}

#[aoc(day24, part1)]
fn part1(map: &Map) -> Option<usize> {
    solve(map, false)
}

#[aoc(day24, part2)]
fn part2(map: &Map) -> Option<usize> {
    solve(map, true)
}
