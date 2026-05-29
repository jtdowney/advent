use std::{collections::HashMap, iter, ops::Index};

use anyhow::{Context, Result, bail};

type Point = (usize, usize);
const ORIGIN: Point = (0, 0);

#[derive(Copy, Clone, PartialEq)]
enum Square {
    Tree,
    Open,
}

struct Grid {
    pattern: HashMap<Point, Square>,
    pattern_width: usize,
    pattern_height: usize,
}

impl Grid {
    fn new(pattern: HashMap<Point, Square>) -> Result<Self> {
        let pattern_width = pattern
            .keys()
            .map(|&(x, _)| x)
            .max()
            .context("Grid has no width")?;
        let pattern_height = pattern
            .keys()
            .map(|&(_, y)| y)
            .max()
            .context("Grid has no height")?;
        Ok(Self {
            pattern,
            pattern_width,
            pattern_height,
        })
    }
}

impl Index<Point> for Grid {
    type Output = Square;

    fn index(&self, (x, y): Point) -> &Self::Output {
        let x = x % (self.pattern_width + 1);
        &self.pattern[&(x, y)]
    }
}

#[aoc_generator(day3)]
fn generator(input: &str) -> Result<Grid> {
    let mut pattern = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let square = match ch {
                '#' => Square::Tree,
                '.' => Square::Open,
                _ => bail!("Invalid character '{}' at position ({}, {})", ch, x, y),
            };
            pattern.insert((x, y), square);
        }
    }

    Grid::new(pattern)
}

fn path(dx: usize, dy: usize) -> impl Iterator<Item = Point> {
    iter::successors(Some(ORIGIN), move |(x, y)| Some((x + dx, y + dy)))
}

#[aoc(day3, part1)]
fn part1(grid: &Grid) -> usize {
    path(3, 1)
        .take_while(|&(_, y)| y <= grid.pattern_height)
        .filter(|&p| grid[p] == Square::Tree)
        .count()
}

#[aoc(day3, part2)]
fn part2(grid: &Grid) -> usize {
    let paths = &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    paths
        .iter()
        .map(|&(dx, dy)| {
            path(dx, dy)
                .take_while(|&(_, y)| y <= grid.pattern_height)
                .filter(|&p| grid[p] == Square::Tree)
                .count()
        })
        .product()
}
