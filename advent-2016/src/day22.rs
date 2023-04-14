use std::{
    collections::HashMap,
    fmt::{self, Display},
};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::Regex;

type Point = (usize, usize);

struct Node {
    used: usize,
    available: usize,
}

impl Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.used == 0 {
            write!(f, "_")?;
        } else if self.used >= 100 {
            write!(f, "#")?;
        } else {
            write!(f, ".")?;
        }

        Ok(())
    }
}

#[aoc_generator(day22)]
fn generator(input: &str) -> anyhow::Result<HashMap<Point, Node>> {
    let regex = Regex::new(r"node-x(\d+)-y(\d+)\s+(?:\d+)T\s+(\d+)T\s+(\d+)T")?;
    input
        .lines()
        .skip(2)
        .map(|line| {
            let captures = regex
                .captures(line)
                .ok_or_else(|| anyhow::anyhow!("Invalid line: {}", line))?;
            let x = captures[1].parse()?;
            let y = captures[2].parse()?;
            let used = captures[3].parse()?;
            let available = captures[4].parse()?;
            Ok(((x, y), Node { used, available }))
        })
        .collect()
}

#[aoc(day22, part1)]
fn part1(input: &HashMap<Point, Node>) -> usize {
    input
        .values()
        .permutations(2)
        .filter(|nodes| {
            let a = nodes[0];
            let b = nodes[1];
            a.used > 0 && a.used <= b.available
        })
        .count()
}

#[aoc(day22, part2)]
fn part2(input: &HashMap<Point, Node>) -> usize {
    let (maxx, maxy) = input.keys().max().unwrap();
    for y in 0..=*maxy {
        for x in 0..=*maxx {
            if y == 0 && x == *maxx {
                print!("G");
            } else {
                print!("{}", input[&(x, y)]);
            }
        }

        println!();
    }

    0
}
