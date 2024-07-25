use std::{collections::HashMap, iter};

type Point = (isize, isize);

const NEIGHBORS: [Point; 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug, Copy, Clone, PartialEq)]
enum Seat {
    Empty,
    Occupied,
}

#[derive(Debug, Clone, PartialEq)]
struct SeatingArea {
    seats: HashMap<Point, Seat>,
    width: isize,
    height: isize,
}

impl SeatingArea {
    fn count_occupied_immediate_neighbors(&self, (x, y): Point) -> usize {
        NEIGHBORS
            .iter()
            .map(|&(dx, dy)| (x + dx, y + dy))
            .filter(|n| {
                self.seats
                    .get(n)
                    .map(|&seat| seat == Seat::Occupied)
                    .unwrap_or_default()
            })
            .count()
    }

    fn count_occupied_line_of_sight_neighbors(&self, (x, y): Point) -> usize {
        NEIGHBORS
            .iter()
            .filter_map(|&(dx, dy)| {
                let neighbor = (x + dx, y + dy);
                iter::successors(Some(neighbor), |(nx, ny)| Some((nx + dx, ny + dy)))
                    .take_while(|&(nx, ny)| {
                        nx >= 0 && nx <= self.width && ny >= 0 && ny <= self.height
                    })
                    .find_map(|n| self.seats.get(&n))
            })
            .filter(|&seat| *seat == Seat::Occupied)
            .count()
    }

    fn step_immedate(&self) -> Self {
        let seats = self
            .seats
            .iter()
            .map(|(&point, &seat)| {
                let count = self.count_occupied_immediate_neighbors(point);
                let next = match (seat, count) {
                    (Seat::Empty, 0) => Seat::Occupied,
                    (Seat::Occupied, count) if count >= 4 => Seat::Empty,
                    (s, _) => s,
                };
                (point, next)
            })
            .collect();

        SeatingArea { seats, ..*self }
    }

    fn step_line_of_sight(&self) -> Self {
        let seats = self
            .seats
            .iter()
            .map(|(&point, &seat)| {
                let count = self.count_occupied_line_of_sight_neighbors(point);
                let next = match (seat, count) {
                    (Seat::Empty, 0) => Seat::Occupied,
                    (Seat::Occupied, count) if count >= 5 => Seat::Empty,
                    (s, _) => s,
                };
                (point, next)
            })
            .collect();

        SeatingArea { seats, ..*self }
    }
}

fn solve<F>(area: &SeatingArea, step: F) -> usize
where
    F: Fn(&SeatingArea) -> Option<SeatingArea>,
{
    let stable = iter::successors(Some(area.clone()), step)
        .try_fold(None, |prev: Option<SeatingArea>, next| match prev {
            Some(p) if p == next => Err(p),
            _ => Ok(Some(next)),
        })
        .unwrap_err();

    stable
        .seats
        .values()
        .filter(|&seat| *seat == Seat::Occupied)
        .count()
}

#[aoc_generator(day11)]
fn generator(input: &str) -> SeatingArea {
    let seats = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, ch)| ch != '.')
                .map(move |(x, _)| ((x as isize, y as isize), Seat::Empty))
        })
        .collect::<HashMap<Point, Seat>>();

    let width = seats.keys().map(|&(x, _)| x).max().unwrap();
    let height = seats.keys().map(|&(_, y)| y).max().unwrap();
    SeatingArea {
        seats,
        width,
        height,
    }
}

#[aoc(day11, part1)]
fn part1(area: &SeatingArea) -> usize {
    solve(area, |prev| Some(prev.step_immedate()))
}

#[aoc(day11, part2)]
fn part2(area: &SeatingArea) -> usize {
    solve(area, |prev| Some(prev.step_line_of_sight()))
}
