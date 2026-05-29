use std::{cmp::Ordering, collections::BinaryHeap, str::FromStr};

use anyhow::{Result, anyhow};
use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    Finish, Parser,
    bytes::complete::tag,
    character::complete::{char, i64},
    combinator::map,
    sequence::{delimited, preceded, separated_pair},
};

#[derive(Debug, Clone, PartialEq)]
struct Nanobot {
    x: i64,
    y: i64,
    z: i64,
    r: i64,
}

impl FromStr for Nanobot {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parser = map(
            separated_pair(
                delimited(
                    tag("pos=<"),
                    (
                        i64::<_, nom::error::Error<_>>,
                        char(','),
                        i64,
                        char(','),
                        i64,
                    ),
                    tag(">"),
                ),
                tag(", "),
                preceded(tag("r="), i64),
            ),
            |((x, _, y, _, z), r)| Nanobot { x, y, z, r },
        );

        parser
            .parse(s)
            .finish()
            .map_err(|e| anyhow!("Failed to parse nanobot: {}", e))
            .map(|(_, nanobot)| nanobot)
    }
}

impl Nanobot {
    fn manhattan_distance(&self, other: &Nanobot) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }

    fn manhattan_distance_to_point(&self, x: i64, y: i64, z: i64) -> i64 {
        (self.x - x).abs() + (self.y - y).abs() + (self.z - z).abs()
    }

    fn in_range(&self, other: &Nanobot) -> bool {
        self.manhattan_distance(other) <= self.r
    }

    fn in_range_of_point(&self, x: i64, y: i64, z: i64) -> bool {
        self.manhattan_distance_to_point(x, y, z) <= self.r
    }
}

#[aoc_generator(day23)]
fn generator(input: &str) -> Result<Vec<Nanobot>> {
    input.lines().map(str::parse).collect()
}

#[aoc(day23, part1)]
fn part1(input: &[Nanobot]) -> usize {
    let strongest = input
        .iter()
        .max_by_key(|bot| bot.r)
        .expect("No nanobots found");

    input.iter().filter(|bot| strongest.in_range(bot)).count()
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct SearchBox {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
    min_z: i64,
    max_z: i64,
    min_possible_count: usize,
    max_possible_count: usize,
    min_distance: i64,
}

impl Ord for SearchBox {
    fn cmp(&self, other: &Self) -> Ordering {
        self.max_possible_count
            .cmp(&other.max_possible_count)
            .then(other.min_distance.cmp(&self.min_distance))
    }
}

impl PartialOrd for SearchBox {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl SearchBox {
    fn new(
        min_x: i64,
        max_x: i64,
        min_y: i64,
        max_y: i64,
        min_z: i64,
        max_z: i64,
        nanobots: &[Nanobot],
    ) -> Self {
        let mut min_possible_count = 0;
        let mut max_possible_count = 0;

        for bot in nanobots {
            let closest_x = bot.x.max(min_x).min(max_x);
            let closest_y = bot.y.max(min_y).min(max_y);
            let closest_z = bot.z.max(min_z).min(max_z);
            let min_dist_to_box =
                (bot.x - closest_x).abs() + (bot.y - closest_y).abs() + (bot.z - closest_z).abs();

            if min_dist_to_box <= bot.r {
                max_possible_count += 1;

                let max_dist_to_box = (bot.x - min_x).abs().max((bot.x - max_x).abs())
                    + (bot.y - min_y).abs().max((bot.y - max_y).abs())
                    + (bot.z - min_z).abs().max((bot.z - max_z).abs());

                if max_dist_to_box <= bot.r {
                    min_possible_count += 1;
                }
            }
        }

        let min_distance = if min_x > 0 {
            min_x
        } else if max_x < 0 {
            -max_x
        } else {
            0
        } + if min_y > 0 {
            min_y
        } else if max_y < 0 {
            -max_y
        } else {
            0
        } + if min_z > 0 {
            min_z
        } else if max_z < 0 {
            -max_z
        } else {
            0
        };

        SearchBox {
            min_x,
            max_x,
            min_y,
            max_y,
            min_z,
            max_z,
            min_possible_count,
            max_possible_count,
            min_distance,
        }
    }

    fn is_point(&self) -> bool {
        self.min_x == self.max_x && self.min_y == self.max_y && self.min_z == self.max_z
    }

    fn split(&self, nanobots: &[Nanobot]) -> Vec<SearchBox> {
        if self.is_point() {
            return vec![];
        }

        let mut boxes = Vec::new();
        let mid_x = (self.min_x + self.max_x) / 2;
        let mid_y = (self.min_y + self.max_y) / 2;
        let mid_z = (self.min_z + self.max_z) / 2;

        for &(min_x, max_x) in &[(self.min_x, mid_x), (mid_x + 1, self.max_x)] {
            if min_x > max_x {
                continue;
            }
            for &(min_y, max_y) in &[(self.min_y, mid_y), (mid_y + 1, self.max_y)] {
                if min_y > max_y {
                    continue;
                }
                for &(min_z, max_z) in &[(self.min_z, mid_z), (mid_z + 1, self.max_z)] {
                    if min_z > max_z {
                        continue;
                    }
                    boxes.push(SearchBox::new(
                        min_x, max_x, min_y, max_y, min_z, max_z, nanobots,
                    ));
                }
            }
        }

        boxes
    }
}

#[aoc(day23, part2)]
fn part2(input: &[Nanobot]) -> i64 {
    let min_x = input.iter().map(|bot| bot.x - bot.r).min().unwrap();
    let max_x = input.iter().map(|bot| bot.x + bot.r).max().unwrap();
    let min_y = input.iter().map(|bot| bot.y - bot.r).min().unwrap();
    let max_y = input.iter().map(|bot| bot.y + bot.r).max().unwrap();
    let min_z = input.iter().map(|bot| bot.z - bot.r).min().unwrap();
    let max_z = input.iter().map(|bot| bot.z + bot.r).max().unwrap();

    let mut best_count = 0;
    let mut best_distance = i64::MAX;

    let mut heap = BinaryHeap::new();
    heap.push(SearchBox::new(
        min_x, max_x, min_y, max_y, min_z, max_z, input,
    ));

    while let Some(search_box) = heap.pop() {
        if search_box.max_possible_count < best_count {
            continue;
        }

        if search_box.is_point() {
            let count = input
                .iter()
                .filter(|bot| {
                    bot.in_range_of_point(search_box.min_x, search_box.min_y, search_box.min_z)
                })
                .count();

            let distance = search_box.min_x.abs() + search_box.min_y.abs() + search_box.min_z.abs();

            if count > best_count || (count == best_count && distance < best_distance) {
                best_count = count;
                best_distance = distance;
            }
        } else {
            for sub_box in search_box.split(input) {
                if sub_box.max_possible_count >= best_count {
                    heap.push(sub_box);
                }
            }
        }
    }

    best_distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let input = "pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1";

        let nanobots = generator(input).unwrap();
        let result = part1(&nanobots);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_example_part2() {
        let input = "pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5";

        let nanobots = generator(input).unwrap();

        let count_at_12_12_12 = nanobots
            .iter()
            .filter(|bot| bot.in_range_of_point(12, 12, 12))
            .count();
        assert_eq!(count_at_12_12_12, 5);

        let result = part2(&nanobots);
        assert_eq!(result, 36);
    }
}
