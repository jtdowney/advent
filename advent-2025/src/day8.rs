use std::{cmp::Ordering, collections::HashMap};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

#[derive(Debug, Clone)]
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    count: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            count: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }

        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        match self.rank[root_x].cmp(&self.rank[root_y]) {
            Ordering::Less => self.parent[root_x] = root_y,
            Ordering::Greater => self.parent[root_y] = root_x,
            Ordering::Equal => {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
            }
        }

        self.count -= 1;
        true
    }

    fn component_sizes(&mut self) -> impl Iterator<Item = usize> + '_ {
        let n = self.parent.len();
        (0..n).map(|i| self.find(i)).counts().into_values()
    }
}

#[derive(Debug, Clone)]
struct Input {
    point_index: HashMap<Point, usize>,
    edges: Vec<PointPair>,
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

    let point_index = points.iter().enumerate().map(|(i, p)| (*p, i)).collect();

    let edges = points
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

    Ok(Input { point_index, edges })
}

#[aoc(day8, part1)]
fn part1(input: &Input) -> usize {
    let mut uf = UnionFind::new(input.point_index.len());

    for PointPair(p1, p2) in input.edges.iter().take(1000) {
        let i1 = input.point_index[p1];
        let i2 = input.point_index[p2];
        uf.union(i1, i2);
    }

    uf.component_sizes()
        .sorted_unstable_by(|a, b| b.cmp(a))
        .take(3)
        .product()
}

#[aoc(day8, part2)]
fn part2(input: &Input) -> u64 {
    let mut uf = UnionFind::new(input.point_index.len());

    for PointPair(p1, p2) in &input.edges {
        let i1 = input.point_index[p1];
        let i2 = input.point_index[p2];
        if uf.union(i1, i2) && uf.count == 1 {
            let Point(x1, _, _) = *p1;
            let Point(x2, _, _) = *p2;
            return u64::from(x1) * u64::from(x2);
        }
    }

    unreachable!()
}
