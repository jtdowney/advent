use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use eyre::eyre;
use itertools::{Itertools, iproduct};
use nom::{
    Finish, IResult, Parser,
    bytes::complete::tag,
    character::complete::{char, i16, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
};

type Point = (i16, i16);
type Grid = HashSet<Point>;

const DROP_POINT: Point = (500, 0);

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Point>>> {
    let parse_point = separated_pair(i16, char(','), i16);
    let parse_line = separated_list1(tag(" -> "), parse_point);
    separated_list1(line_ending, parse_line).parse(input)
}

#[aoc_generator(day14)]
fn generator(input: &str) -> eyre::Result<Grid> {
    let lines = parse_input(input)
        .finish()
        .map_err(|e| eyre!("error parsing: {}", e))
        .map(|(_, o)| o)?;
    let grid = lines
        .into_iter()
        .flat_map(|line| {
            line.into_iter()
                .tuple_windows()
                .flat_map(|((ax, ay), (bx, by))| {
                    let sx = ax.min(bx);
                    let ex = ax.max(bx);
                    let sy = ay.min(by);
                    let ey = ay.max(by);
                    iproduct!(sx..=ex, sy..=ey)
                })
        })
        .collect();

    Ok(grid)
}

fn simulate<P>(mut grid: Grid, goal: P) -> usize
where
    P: Fn(Point) -> bool,
{
    let maxy = grid.iter().map(|&(_, y)| y).max().unwrap();
    let floor = maxy + 2;

    for round in 1.. {
        let mut position = DROP_POINT;
        loop {
            let (x, y) = position;
            let next_position = [(0, 1), (-1, 1), (1, 1)]
                .iter()
                .map(|(dx, dy)| (x + dx, y + dy))
                .take_while(|&(_, y)| y < floor)
                .find(|p| !grid.contains(p));
            if let Some(next) = next_position {
                position = next;
            } else {
                if goal(position) {
                    return round;
                }

                grid.insert(position);
                break;
            }
        }
    }

    unreachable!()
}

#[aoc(day14, part1)]
fn part1(input: &Grid) -> usize {
    let grid = input.clone();
    let goal = grid.iter().map(|&(_, y)| y).max().unwrap();
    simulate(grid, |(_, y)| y >= goal) - 1
}

#[aoc(day14, part2)]
fn part2(input: &Grid) -> usize {
    let grid = input.clone();
    simulate(grid, |point| point == DROP_POINT)
}
