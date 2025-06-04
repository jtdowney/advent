use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Point = (i32, i32);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    const ALL: [Self; 4] = [Self::North, Self::South, Self::West, Self::East];

    fn delta(&self) -> Point {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
            Direction::East => (0, 1),
        }
    }

    fn check_positions(&self) -> [Point; 3] {
        match self {
            Direction::North => [(-1, -1), (-1, 0), (-1, 1)],
            Direction::South => [(1, -1), (1, 0), (1, 1)],
            Direction::West => [(-1, -1), (0, -1), (1, -1)],
            Direction::East => [(-1, 1), (0, 1), (1, 1)],
        }
    }
}

#[aoc_generator(day23)]
fn generator(input: &str) -> HashSet<Point> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, ch)| *ch == '#')
                .map(move |(col, _)| (row as i32, col as i32))
        })
        .collect()
}

fn neighbors(pos: Point) -> impl Iterator<Item = Point> {
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .into_iter()
    .map(move |(dr, dc)| (pos.0 + dr, pos.1 + dc))
}

fn has_neighbor(elves: &HashSet<Point>, pos: Point) -> bool {
    neighbors(pos).any(|p| elves.contains(&p))
}

fn can_move_direction(elves: &HashSet<Point>, pos: Point, dir: Direction) -> bool {
    dir.check_positions()
        .iter()
        .all(|&(dr, dc)| !elves.contains(&(pos.0 + dr, pos.1 + dc)))
}

fn simulate_round(elves: &mut HashSet<Point>, round: usize) -> bool {
    let proposals: HashMap<Point, Vec<Point>> = elves
        .iter()
        .filter(|&&elf| has_neighbor(elves, elf))
        .filter_map(|&elf| {
            Direction::ALL
                .iter()
                .cycle()
                .skip(round % 4)
                .take(4)
                .find(|&&dir| can_move_direction(elves, elf, dir))
                .map(|&dir| {
                    let (dr, dc) = dir.delta();
                    ((elf.0 + dr, elf.1 + dc), elf)
                })
        })
        .fold(HashMap::new(), |mut acc, (new_pos, old_pos)| {
            acc.entry(new_pos).or_default().push(old_pos);
            acc
        });

    if proposals.is_empty() {
        return false;
    }

    proposals
        .into_iter()
        .filter(|(_, old_positions)| old_positions.len() == 1)
        .for_each(|(new_pos, old_positions)| {
            elves.remove(&old_positions[0]);
            elves.insert(new_pos);
        });

    true
}

fn bounding_box(elves: &HashSet<Point>) -> (Point, Point) {
    let (min_r, max_r) = elves
        .iter()
        .map(|&(r, _)| r)
        .minmax()
        .into_option()
        .unwrap();
    let (min_c, max_c) = elves
        .iter()
        .map(|&(_, c)| c)
        .minmax()
        .into_option()
        .unwrap();
    ((min_r, min_c), (max_r, max_c))
}

#[aoc(day23, part1)]
fn part1(input: &HashSet<Point>) -> i32 {
    let mut elves = input.clone();

    (0..10).for_each(|round| {
        simulate_round(&mut elves, round);
    });

    let ((min_r, min_c), (max_r, max_c)) = bounding_box(&elves);
    (max_r - min_r + 1) * (max_c - min_c + 1) - elves.len() as i32
}

#[aoc(day23, part2)]
fn part2(input: &HashSet<Point>) -> usize {
    let mut elves = input.clone();

    (0..)
        .find(|&round| !simulate_round(&mut elves, round))
        .unwrap()
        + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";
        let parsed = generator(input);
        assert_eq!(part1(&parsed), 110);
    }

    #[test]
    fn test_part2() {
        let input = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";
        let parsed = generator(input);
        assert_eq!(part2(&parsed), 20);
    }
}
