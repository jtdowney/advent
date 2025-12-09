use std::{collections::HashMap, ops::RangeInclusive};

use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(u32, u32);

impl Point {
    fn area(self, other: Self) -> u64 {
        let Self(x1, y1) = self;
        let Self(x2, y2) = other;
        u64::from(x1.abs_diff(x2) + 1) * u64::from(y1.abs_diff(y2) + 1)
    }
}

struct Polygon {
    scanline_ys: Vec<u32>,
    interval_cache: HashMap<u32, Vec<RangeInclusive<u32>>>,
}

impl Polygon {
    fn new(vertices: &[Point]) -> Self {
        let edges = vertices
            .iter()
            .copied()
            .circular_tuple_windows()
            .collect::<Vec<_>>();

        let vertex_ys = vertices.iter().map(|Point(_, y)| *y);
        let mut scanline_ys = vertex_ys.sorted_unstable().dedup().collect::<Vec<_>>();

        let samples = scanline_ys
            .iter()
            .tuple_windows()
            .filter_map(|(&a, &b)| (b > a + 1).then_some(a + 1))
            .collect::<Vec<_>>();

        scanline_ys.extend(samples);
        scanline_ys.sort_unstable();
        scanline_ys.dedup();

        let interval_cache = scanline_ys
            .iter()
            .map(|&y| (y, Self::compute_intervals_at_y(&edges, y)))
            .collect();

        Self {
            scanline_ys,
            interval_cache,
        }
    }

    fn compute_intervals_at_y(edges: &[(Point, Point)], y: u32) -> Vec<RangeInclusive<u32>> {
        let crossings = edges
            .iter()
            .filter_map(|&(Point(ax, ay), Point(bx, by))| {
                if ax != bx {
                    return None;
                }

                let (y_min, y_max) = if ay <= by { (ay, by) } else { (by, ay) };
                (y_min <= y && y < y_max).then_some(ax)
            })
            .sorted_unstable();

        let raycast_intervals = crossings.tuples().map(|(a, b)| a..=b);
        let horizontal_intervals = edges.iter().filter_map(|&(Point(ax, ay), Point(bx, by))| {
            if ay != by || ay != y {
                return None;
            }

            let (x_min, x_max) = if ax <= bx { (ax, bx) } else { (bx, ax) };
            Some(x_min..=x_max)
        });

        raycast_intervals
            .chain(horizontal_intervals)
            .sorted_unstable_by_key(|r| *r.start())
            .fold(vec![], |mut merged, range| {
                match merged.last_mut() {
                    Some(last) if *range.start() <= *last.end() + 1 => {
                        *last = *last.start()..=(*last.end()).max(*range.end());
                    }
                    _ => merged.push(range),
                }

                merged
            })
    }

    fn scanline_ys_in_range(&self, y1: u32, y2: u32) -> impl Iterator<Item = u32> + '_ {
        let start = self.scanline_ys.partition_point(|&y| y < y1);
        let end = self.scanline_ys.partition_point(|&y| y <= y2);
        self.scanline_ys[start..end].iter().copied()
    }

    fn contains_rectangle(&self, a: Point, b: Point) -> bool {
        let Point(ax, ay) = a;
        let Point(bx, by) = b;
        let (x1, x2) = if ax <= bx { (ax, bx) } else { (bx, ax) };
        let (y1, y2) = if ay <= by { (ay, by) } else { (by, ay) };

        self.scanline_ys_in_range(y1, y2).all(|y| {
            self.interval_cache.get(&y).is_some_and(|intervals| {
                intervals.iter().any(|r| r.contains(&x1) && r.contains(&x2))
            })
        })
    }
}

#[aoc_generator(day9)]
fn generator(input: &str) -> anyhow::Result<Vec<Point>> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(',').context("Invalid input")?;
            let x = left.parse()?;
            let y = right.parse()?;
            Ok(Point(x, y))
        })
        .collect()
}

#[aoc(day9, part1)]
fn part1(input: &[Point]) -> Option<u64> {
    input
        .iter()
        .copied()
        .tuple_combinations()
        .map(|(a, b)| a.area(b))
        .max()
}

#[aoc(day9, part2)]
fn part2(input: &[Point]) -> Option<u64> {
    let polygon = Polygon::new(input);

    input
        .iter()
        .copied()
        .tuple_combinations()
        .filter(|&(a, b)| polygon.contains_rectangle(a, b))
        .map(|(a, b)| a.area(b))
        .max()
}
