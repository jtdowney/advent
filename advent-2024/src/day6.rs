use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rayon::prelude::*;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Point(i32, i32);

impl Add<Direction> for Point {
    type Output = Self;

    fn add(self, direction: Direction) -> Self::Output {
        let Point(x, y) = self;
        match direction {
            Direction::Up => Point(x, y - 1),
            Direction::Down => Point(x, y + 1),
            Direction::Left => Point(x - 1, y),
            Direction::Right => Point(x + 1, y),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Cell {
    Free,
    Obstruction,
}

#[derive(Clone, Debug, Default)]
struct Simulation {
    cells: HashMap<Point, Cell>,
    visited: HashSet<(Point, Direction)>,
    current: Point,
    direction: Direction,
    looping: bool,
    outside: bool,
}

impl Simulation {
    fn step(&mut self) {
        self.looping = !self.visited.insert((self.current, self.direction));
        let next = self.current + self.direction;
        match self.cells.get(&next) {
            Some(Cell::Free) => self.current = next,
            Some(Cell::Obstruction) => self.direction = self.direction.next(),
            None => self.outside = true,
        }
    }

    fn run(&mut self) {
        while !self.outside && !self.looping {
            self.step();
        }
    }
}

#[aoc_generator(day6)]
fn generator(input: &str) -> Simulation {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| (Point(x as i32, y as i32), c))
        })
        .fold(Simulation::default(), |mut acc, (point, c)| {
            match c {
                '.' => {
                    acc.cells.insert(point, Cell::Free);
                }
                '#' => {
                    acc.cells.insert(point, Cell::Obstruction);
                }
                '^' => {
                    acc.cells.insert(point, Cell::Free);
                    acc.current = point;
                }
                _ => unimplemented!("Unexpected character: {}", c),
            }

            acc
        })
}

#[aoc(day6, part1)]
fn part1(input: &Simulation) -> usize {
    let mut simulation = input.clone();
    simulation.run();
    simulation
        .visited
        .iter()
        .unique_by(|(point, _)| *point)
        .count()
}

#[aoc(day6, part2)]
fn part2(input: &Simulation) -> usize {
    let mut original = input.clone();
    original.run();

    let candidates = original
        .visited
        .iter()
        .filter(|&&(point, _)| point != input.current)
        .map(|&(point, _)| point)
        .collect::<HashSet<_>>();
    candidates
        .par_iter()
        .filter(|&point| {
            let mut simulation = input.clone();
            simulation.cells.insert(*point, Cell::Obstruction);
            simulation.run();
            simulation.looping
        })
        .count()
}
