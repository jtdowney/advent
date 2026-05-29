use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

type Point = (i32, i32);

#[aoc_generator(day10)]
fn generator(input: &str) -> anyhow::Result<HashMap<Point, u32>> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                let height = c.to_digit(10).expect("invalid digit");
                Ok(((x as i32, y as i32), height))
            })
        })
        .collect()
}

fn neighbors((x, y): Point) -> impl Iterator<Item = Point> {
    [(0, 1), (1, 0), (0, -1), (-1, 0)]
        .iter()
        .map(move |&(dx, dy)| (x + dx, y + dy))
}

fn solve(input: &HashMap<Point, u32>) -> (usize, usize) {
    struct Search {
        start: Point,
        current: Point,
    }

    let mut search = input
        .iter()
        .filter_map(|(&point, &height)| {
            if height == 0 {
                Some(Search {
                    start: point,
                    current: point,
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut trails = HashMap::<Point, HashSet<Point>>::new();
    let mut ratings = HashMap::<Point, usize>::new();
    while let Some(Search { start, current }) = search.pop() {
        let Some(&height) = input.get(&current) else {
            continue;
        };

        if height == 9 {
            trails.entry(start).or_default().insert(current);
            *ratings.entry(start).or_default() += 1;
            continue;
        }

        let candidates = neighbors(current).filter(|&neighbor| match input.get(&neighbor) {
            Some(&neighbor_height) => neighbor_height == height + 1,
            None => false,
        });
        for neighbor in candidates {
            search.push(Search {
                start,
                current: neighbor,
            });
        }
    }

    let score = trails.values().map(HashSet::len).sum();
    let rating = ratings.values().sum();

    (score, rating)
}

#[aoc(day10, part1)]
fn part1(input: &HashMap<Point, u32>) -> usize {
    let (score, _) = solve(input);
    score
}

#[aoc(day10, part2)]
fn part2(input: &HashMap<Point, u32>) -> usize {
    let (_, rating) = solve(input);
    rating
}
