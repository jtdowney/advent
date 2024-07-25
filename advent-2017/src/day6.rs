use std::{
    collections::{HashMap, HashSet},
    num::ParseIntError,
};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
fn generator(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.split_whitespace().map(|line| line.parse()).collect()
}

fn rebalance(banks: &mut [u32]) {
    let banks_len = banks.len();
    let (max_index, _) =
        banks
            .iter()
            .enumerate()
            .fold((0, 0), |(max_index, max_value), (index, &value)| {
                if value > max_value {
                    (index, value)
                } else {
                    (max_index, max_value)
                }
            });
    let mut blocks = banks[max_index];
    banks[max_index] = 0;

    let mut index = max_index + 1;
    while blocks > 0 {
        banks[index % banks_len] += 1;
        blocks -= 1;
        index += 1;
    }
}

#[aoc(day6, part1)]
fn part1(input: &[u32]) -> Option<usize> {
    let mut banks = input.to_vec();
    let mut seen = HashSet::new();
    (1..).find(|_| {
        rebalance(&mut banks);
        !seen.insert(banks.clone())
    })
}

#[aoc(day6, part2)]
fn part2(input: &[u32]) -> Option<usize> {
    let mut banks = input.to_vec();
    let mut seen = HashMap::new();
    (1..).find_map(|i| {
        rebalance(&mut banks);
        seen.insert(banks.clone(), i).map(|j| i - j)
    })
}
