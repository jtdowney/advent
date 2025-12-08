use std::collections::HashMap;

use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point(u32, u32, u32);

impl Point {
    fn squared_distance(&self, other: &Point) -> u64 {
        let Point(x1, y1, z1) = self;
        let Point(x2, y2, z2) = other;
        let dx = u64::from(x1.abs_diff(*x2));
        let dy = u64::from(y1.abs_diff(*y2));
        let dz = u64::from(z1.abs_diff(*z2));
        dx.pow(2) + dy.pow(2) + dz.pow(2)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct PointPair(Point, Point);

impl PointPair {
    fn canonicalize(&self) -> PointPair {
        let PointPair(left, right) = *self;
        if left < right {
            PointPair(left, right)
        } else {
            PointPair(right, left)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct CircuitId(usize);

#[derive(Debug, Clone)]
struct Input {
    circuits: HashMap<Point, CircuitId>,
    distances: Vec<PointPair>,
}

#[aoc_generator(day8)]
fn generator(input: &str) -> anyhow::Result<Input> {
    let points = input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().context("bad point")?.parse()?;
            let y = parts.next().context("bad point")?.parse()?;
            let z = parts.next().context("bad point")?.parse()?;
            Ok(Point(x, y, z))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    let circuits = points
        .iter()
        .enumerate()
        .map(|(i, p)| (*p, CircuitId(i)))
        .collect();

    let distances = points
        .iter()
        .flat_map(|&p1| {
            points
                .iter()
                .filter(move |&p2| p1 != *p2)
                .map(move |&p2| (p1.squared_distance(&p2), PointPair(p1, p2).canonicalize()))
        })
        .unique()
        .sorted_unstable_by_key(|(distance, _)| *distance)
        .map(|(_, pair)| pair)
        .collect();

    Ok(Input {
        circuits,
        distances,
    })
}

fn merge_circuits(circuits: &mut HashMap<Point, CircuitId>, p1: &Point, p2: &Point) -> bool {
    let id1 = circuits[p1];
    let id2 = circuits[p2];

    if id1 == id2 {
        return false;
    }

    let min_id = id1.min(id2);
    let max_id = id1.max(id2);

    circuits
        .values_mut()
        .filter(|id| **id == max_id)
        .for_each(|id| *id = min_id);

    true
}

fn circuit_count(circuits: &HashMap<Point, CircuitId>) -> usize {
    circuits.values().unique().count()
}

#[aoc(day8, part1)]
fn part1(input: &Input) -> usize {
    let mut circuits = input.circuits.clone();

    for PointPair(p1, p2) in input.distances.iter().take(1000) {
        merge_circuits(&mut circuits, p1, p2);
    }

    circuits
        .values()
        .counts()
        .into_values()
        .sorted_unstable_by(|a, b| b.cmp(a))
        .take(3)
        .product()
}

#[aoc(day8, part2)]
fn part2(input: &Input) -> u64 {
    let mut circuits = input.circuits.clone();

    for PointPair(p1, p2) in &input.distances {
        if merge_circuits(&mut circuits, p1, p2) && circuit_count(&circuits) == 1 {
            let Point(x1, _, _) = p1;
            let Point(x2, _, _) = p2;
            return u64::from(*x1) * u64::from(*x2);
        }
    }

    unreachable!()
}
