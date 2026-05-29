use std::{
    collections::{HashMap, HashSet},
    f64::consts::{FRAC_PI_2, PI},
};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Point = (u16, u16);

fn distance(a: Point, b: Point) -> f64 {
    ((f64::from(a.0) - f64::from(b.0)).powi(2) + (f64::from(a.1) - f64::from(b.1)).powi(2)).sqrt()
}

fn is_blocked(segment: &[Point], points: &HashSet<Point>) -> bool {
    let a = *segment.first().unwrap();
    let b = *segment.last().unwrap();
    let ab = distance(a, b);

    points.iter().filter(|&c| *c != a && *c != b).any(|&c| {
        let ac = distance(a, c);
        let bc = distance(b, c);
        // Using epsilon comparison instead of approx crate
        (ac + bc - ab).abs() < 1e-10
    })
}

fn find_station(asteroids: &HashSet<Point>) -> (Point, usize) {
    let counts: HashMap<Point, usize> = asteroids
        .iter()
        .cloned()
        .combinations(2)
        .filter(|segment| !is_blocked(segment, asteroids))
        .flatten()
        .fold(HashMap::new(), |mut acc, point| {
            *acc.entry(point).or_insert(0) += 1;
            acc
        });

    counts.into_iter().max_by_key(|&(_, count)| count).unwrap()
}

#[aoc_generator(day10)]
fn generate(input: &str) -> anyhow::Result<HashSet<Point>> {
    let asteroids = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, ch)| ch == '#')
                .map(move |(x, _)| (x as u16, y as u16))
                .collect::<Vec<Point>>()
        })
        .collect::<HashSet<Point>>();
    Ok(asteroids)
}

#[aoc(day10, part1)]
fn part1(input: &HashSet<Point>) -> usize {
    let (_, count) = find_station(input);
    count
}

#[aoc(day10, part2)]
fn part2(input: &HashSet<Point>) -> u16 {
    let (station, _) = find_station(input);
    let targets = input
        .iter()
        .filter(|&p| *p != station)
        .map(|&p| {
            let x = f64::from(p.0) - f64::from(station.0);
            let y = f64::from(p.1) - f64::from(station.1);
            let mut theta = y.atan2(x) + FRAC_PI_2;
            if theta < 0.0 {
                theta += 2.0 * PI;
            }

            (p, theta)
        })
        .sorted_by(|&(a, _), &(b, _)| {
            distance(station, a)
                .partial_cmp(&distance(station, b))
                .unwrap()
        })
        .sorted_by(|&(_, a), &(_, b)| a.partial_cmp(&b).unwrap())
        .chunk_by(|&(_, theta)| theta)
        .into_iter()
        .flat_map(|(_, group)| {
            group
                .enumerate()
                .map(|(i, (p, theta))| (p, (f64::from(i as u16) * PI * 2.0) + theta))
                .collect::<Vec<(Point, f64)>>()
        })
        .sorted_by(|&(_, a), &(_, b)| a.partial_cmp(&b).unwrap())
        .collect::<Vec<(Point, f64)>>();

    let ((x, y), _) = targets[199];
    x * 100 + y
}
