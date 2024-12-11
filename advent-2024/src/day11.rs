use std::{collections::HashMap, num::ParseIntError};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day11)]
fn generator(input: &str) -> Result<Vec<u64>, ParseIntError> {
    input.split_ascii_whitespace().map(str::parse).collect()
}

fn split_stone(stone: u64) -> (u64, u64) {
    let stone = stone.to_string();
    let half = stone.len() / 2;
    let (left, right) = stone.split_at(half);
    (left.parse().unwrap(), right.parse().unwrap())
}

fn count_stones(stones: &[u64], steps: usize, cache: &mut HashMap<(u64, usize), usize>) -> usize {
    if steps == 0 {
        return stones.len();
    }

    stones
        .iter()
        .map(|&stone| {
            if let Some(&value) = cache.get(&(stone, steps)) {
                value
            } else {
                let value = match stone {
                    0 => count_stones(&[1], steps - 1, cache),
                    s if s.to_string().len() % 2 == 0 => {
                        let (left, right) = split_stone(stone);
                        count_stones(&[left, right], steps - 1, cache)
                    }
                    s => count_stones(&[s * 2024], steps - 1, cache),
                };
                cache.insert((stone, steps), value);
                value
            }
        })
        .sum()
}

#[aoc(day11, part1)]
fn part1(input: &[u64]) -> usize {
    let mut cache = HashMap::new();
    count_stones(input, 25, &mut cache)
}

#[aoc(day11, part2)]
fn part2(input: &[u64]) -> usize {
    let mut cache = HashMap::new();
    count_stones(input, 75, &mut cache)
}
