use std::str::FromStr;

use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
struct Schematic {
    heights: Vec<usize>,
    is_lock: bool,
}

impl FromStr for Schematic {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let lines: Vec<&str> = s.lines().collect();
        let is_lock = lines
            .first()
            .is_some_and(|line| line.chars().all(|c| c == '#'));
        let width = lines.first().map_or(0, |line| line.len());

        let heights = (0..width)
            .map(|col| {
                let column_chars: Vec<char> = lines
                    .iter()
                    .filter_map(|line| line.chars().nth(col))
                    .collect();

                if is_lock {
                    column_chars
                        .iter()
                        .take_while(|&&c| c == '#')
                        .count()
                        .saturating_sub(1)
                } else {
                    column_chars
                        .iter()
                        .rev()
                        .take_while(|&&c| c == '#')
                        .count()
                        .saturating_sub(1)
                }
            })
            .collect();

        Ok(Self { heights, is_lock })
    }
}

#[aoc_generator(day25)]
fn generator(input: &str) -> Result<Vec<Schematic>> {
    input.trim().split("\n\n").map(str::parse).collect()
}

fn fits(lock: &Schematic, key: &Schematic) -> bool {
    lock.heights.len() == key.heights.len()
        && lock
            .heights
            .iter()
            .zip(&key.heights)
            .all(|(lock_h, key_h)| lock_h + key_h <= 5)
}

#[aoc(day25, part1)]
fn part1(schematics: &[Schematic]) -> Result<usize> {
    let (locks, keys): (Vec<_>, Vec<_>) = schematics.iter().partition(|s| s.is_lock);

    Ok(locks
        .iter()
        .flat_map(|lock| keys.iter().filter(move |key| fits(lock, key)))
        .count())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    #[test]
    fn test_part1() {
        let schematics = generator(EXAMPLE).unwrap();
        assert_eq!(part1(&schematics).unwrap(), 3);
    }
}
