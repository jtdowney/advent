use std::{collections::HashMap, iter};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Point = (i32, i32);

#[aoc_generator(day8)]
fn generator(input: &str) -> (HashMap<char, Vec<Point>>, Point) {
    let antennas =
        input
            .lines()
            .enumerate()
            .fold(HashMap::<char, Vec<Point>>::new(), |mut acc, (y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|&(_, c)| c != '.')
                    .for_each(|(x, c)| {
                        acc.entry(c).or_default().push((x as i32, y as i32));
                    });

                acc
            });
    let maxy = input.lines().count() as i32;
    let maxx = input.lines().next().unwrap().len() as i32;

    (antennas, (maxx, maxy))
}

fn in_bounds((x, y): Point, (maxx, maxy): Point) -> bool {
    x >= 0 && x < maxx && y >= 0 && y < maxy
}

#[aoc(day8, part1)]
fn part1((antennas, bounds): &(HashMap<char, Vec<Point>>, Point)) -> usize {
    antennas
        .values()
        .flat_map(|points| {
            points
                .iter()
                .tuple_combinations()
                .flat_map(|((x1, y1), (x2, y2))| {
                    let dx = x2 - x1;
                    let dy = y2 - y1;
                    [(x1 - dx, y1 - dy), (x2 + dx, y2 + dy)]
                })
        })
        .filter(|&point| in_bounds(point, *bounds))
        .unique()
        .count()
}

#[aoc(day8, part2)]
fn part2((antennas, bounds): &(HashMap<char, Vec<Point>>, Point)) -> usize {
    antennas
        .values()
        .flat_map(|points| {
            points
                .iter()
                .tuple_combinations()
                .flat_map(|(p1 @ (x1, y1), p2 @ (x2, y2))| {
                    let dx = x2 - x1;
                    let dy = y2 - y1;
                    iter::successors(Some(*p1), move |&(x, y)| Some((x - dx, y - dy)))
                        .take_while(|&point| in_bounds(point, *bounds))
                        .chain(
                            iter::successors(Some(*p2), move |&(x, y)| Some((x + dx, y + dy)))
                                .take_while(|&point| in_bounds(point, *bounds)),
                        )
                })
        })
        .unique()
        .count()
}
