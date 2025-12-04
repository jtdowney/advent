use std::{collections::HashSet, iter};

use aoc_runner_derive::{aoc, aoc_generator};

type Point = (i16, i16);

fn neighbors((x, y): Point, grid: &HashSet<Point>) -> impl Iterator<Item = Point> + '_ {
    [
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
    .into_iter()
    .filter(move |&p| grid.contains(&p))
}

fn is_removable(point: Point, grid: &HashSet<Point>) -> bool {
    neighbors(point, grid).count() < 4
}

#[aoc_generator(day4)]
fn generator(input: &str) -> HashSet<Point> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '@' {
                    Some((x as i16, y as i16))
                } else {
                    None
                }
            })
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(input: &HashSet<Point>) -> usize {
    input
        .iter()
        .copied()
        .filter(|&p| is_removable(p, input))
        .count()
}

#[aoc(day4, part2)]
fn part2(input: &HashSet<Point>) -> Option<usize> {
    iter::successors(Some(input.clone()), |grid| {
        let mut next = grid.clone();
        next.retain(|&p| !is_removable(p, grid));

        if grid != &next { Some(next) } else { None }
    })
    .last()
    .map(|grid| input.len() - grid.len())
}
