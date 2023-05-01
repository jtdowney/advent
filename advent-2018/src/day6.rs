use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{iproduct, Itertools};

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash, PartialOrd, Ord)]
struct Point(i32, i32);

impl Point {
    fn distance(&self, Point(ox, oy): Point) -> i32 {
        let &Point(sx, sy) = self;
        (sx - ox).abs() + (sy - oy).abs()
    }
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let x = parts.next().context("unable to get part")?.trim().parse()?;
        let y = parts.next().context("unable to get part")?.trim().parse()?;

        Ok(Point(x, y))
    }
}

#[aoc_generator(day6)]
fn generator(input: &str) -> anyhow::Result<Vec<Point>> {
    input.lines().map(|line| line.parse()).collect()
}

#[aoc(day6, part1)]
fn part1(input: &[Point]) -> Option<usize> {
    let (startx, endx) = input
        .iter()
        .copied()
        .map(|Point(x, _)| x)
        .minmax()
        .into_option()
        .unwrap();
    let (starty, endy) = input
        .iter()
        .copied()
        .map(|Point(_, y)| y)
        .minmax()
        .into_option()
        .unwrap();

    let grid = iproduct!(startx..=endx, starty..=endy)
        .map(|(x, y)| Point(x, y))
        .map(|point| {
            let closest = input
                .iter()
                .cloned()
                .map(|p| (p, point.distance(p)))
                .sorted_by_key(|&(_, d)| d);

            let elements = closest.as_slice();
            match (elements[0], elements[1]) {
                ((_, d1), (_, d2)) if d1 == d2 => (point, None),
                ((p, _), _) => (point, Some(p)),
            }
        })
        .collect::<HashMap<Point, Option<Point>>>();

    let edge_points = grid
        .iter()
        .filter(|&(Point(x, y), _)| *x == startx || *x == endx || *y == starty || *y == endy)
        .filter_map(|(_, &point)| point)
        .collect::<HashSet<Point>>();

    grid.values()
        .filter_map(|&point| point)
        .filter(|point| !edge_points.contains(point))
        .fold(HashMap::new(), |mut acc, point| {
            *acc.entry(point).or_insert(0) += 1;
            acc
        })
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(_, count)| count)
}

#[aoc(day6, part2)]
fn part2(input: &[Point]) -> usize {
    let (startx, endx) = input
        .iter()
        .copied()
        .map(|Point(x, _)| x)
        .minmax()
        .into_option()
        .unwrap();
    let (starty, endy) = input
        .iter()
        .copied()
        .map(|Point(_, y)| y)
        .minmax()
        .into_option()
        .unwrap();

    iproduct!(startx..=endx, starty..=endy)
        .map(|(x, y)| Point(x, y))
        .filter(|point| input.iter().map(|&p| point.distance(p)).sum::<i32>() < 10000)
        .count()
}
