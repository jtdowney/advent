use std::collections::{HashMap, HashSet};

use anyhow::anyhow;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{Itertools, iproduct};
use nom::{
    Finish, IResult, Parser,
    bytes::complete::tag,
    character::complete::{i64, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};

const TARGET_ROW: i64 = 2000000;
const LIMIT: i64 = 4000000;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Point(i64, i64);

impl Point {
    fn distance(&self, Point(bx, by): Point) -> i64 {
        let Point(ax, ay) = *self;
        (ax - bx).abs() + (ay - by).abs()
    }
}

impl From<RotatedPoint> for Point {
    fn from(RotatedPoint(rx, ry): RotatedPoint) -> Self {
        let y = (rx + ry) / 2;
        Self(ry - y, y)
    }
}

struct RotatedPoint(i64, i64);

impl From<Point> for RotatedPoint {
    fn from(Point(x, y): Point) -> Self {
        Self(y - x, y + x)
    }
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    map(
        separated_pair(
            preceded(tag("x="), i64),
            tag(", "),
            preceded(tag("y="), i64),
        ),
        |(x, y)| Point(x, y),
    )
    .parse(input)
}

fn parse_input(input: &str) -> IResult<&str, HashMap<Point, Point>> {
    let line = map(
        (
            tag("Sensor at "),
            parse_point,
            tag(": closest beacon is at "),
            parse_point,
        ),
        |(_, a, _, b)| (a, b),
    );

    map(separated_list1(line_ending, line), HashMap::from_iter).parse(input)
}

#[aoc_generator(day15)]
fn generator(input: &str) -> anyhow::Result<HashMap<Point, Point>> {
    parse_input(input)
        .finish()
        .map_err(|e| anyhow!("error parsing: {}", e))
        .map(|(_, o)| o)
}

#[aoc(day15, part1)]
fn part1(input: &HashMap<Point, Point>) -> usize {
    let beacons = input.iter().map(|(_, &b)| b).collect::<HashSet<Point>>();
    let distances = input
        .iter()
        .map(|(&s, &b)| (s, s.distance(b)))
        .collect::<HashMap<Point, i64>>();
    let (minx, maxx) = distances
        .iter()
        .flat_map(|(Point(x, _), d)| [x - d, x + d])
        .minmax()
        .into_option()
        .unwrap();

    (minx..=maxx)
        .filter(|&tx| {
            let t = Point(tx, TARGET_ROW);
            !beacons.contains(&t) && distances.iter().any(|(&s, &d)| s.distance(t) <= d)
        })
        .count()
}

#[aoc(day15, part2)]
fn part2(input: &HashMap<Point, Point>) -> Option<i64> {
    let distances = input
        .iter()
        .map(|(&s, &b)| (s, s.distance(b)))
        .collect::<HashMap<Point, i64>>();
    let corners = distances
        .iter()
        .flat_map(|(&s, d)| {
            let RotatedPoint(rx, ry) = RotatedPoint::from(s);
            [(rx + d, ry + d), (rx - d, ry - d)]
        })
        .collect::<Vec<_>>();
    let searchx = corners
        .iter()
        .tuple_combinations()
        .filter(|((ax, _), (bx, _))| (ax - bx).abs() == 2)
        .map(|((ax, _), (bx, _))| ax.max(bx) - 1)
        .collect::<HashSet<_>>();
    let searchy = corners
        .iter()
        .tuple_combinations()
        .filter(|((_, ay), (_, by))| (ay - by).abs() == 2)
        .map(|((_, ay), (_, by))| ay.max(by) - 1)
        .collect::<HashSet<_>>();
    iproduct!(searchx.iter(), searchy.iter())
        .map(|(&rx, &ry)| Point::from(RotatedPoint(rx, ry)))
        .filter(|&Point(x, y)| (0..=LIMIT).contains(&x) && (0..=LIMIT).contains(&y))
        .find(|&p| distances.iter().all(|(&s, &d)| s.distance(p) > d))
        .map(|Point(x, y)| x * 4000000 + y)
}
