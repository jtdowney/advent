use std::{collections::HashMap, iter};

use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};
use take_until::TakeUntilExt;

type Point = (i32, i32);

const DIRECTIONS: [Point; 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn neighbors_in_direction((x, y): Point, (dx, dy): Point) -> impl Iterator<Item = Point> {
    iter::successors(Some((x + dx, y + dy)), move |(x, y)| Some((x + dx, y + dy)))
}

#[aoc_generator(day8)]
fn generator(input: &str) -> anyhow::Result<HashMap<Point, u32>> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                let value = c.to_digit(10)?;
                Some(((x as i32, y as i32), value))
            })
        })
        .collect::<Option<_>>()
        .context("reading input")
}

#[aoc(day8, part1)]
fn part1(input: &HashMap<Point, u32>) -> usize {
    input
        .iter()
        .filter(|&(point, h)| {
            DIRECTIONS.iter().any(|&delta| {
                neighbors_in_direction(*point, delta)
                    .take_while(|neighbor| input.contains_key(neighbor))
                    .all(|neighbor| input[&neighbor] < *h)
            })
        })
        .count()
}

#[aoc(day8, part2)]
fn part2(input: &HashMap<Point, u32>) -> Option<usize> {
    input
        .iter()
        .map(|(point, h)| {
            DIRECTIONS
                .iter()
                .map(|&delta| {
                    neighbors_in_direction(*point, delta)
                        .take_while(|neighbor| input.contains_key(neighbor))
                        .take_until(|neighbor| input[neighbor] >= *h)
                        .count()
                })
                .product::<usize>()
        })
        .max()
}
