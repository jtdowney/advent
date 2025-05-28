use std::{collections::VecDeque, num::ParseIntError};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day19)]
fn generator(input: &str) -> Result<u32, ParseIntError> {
    input.parse()
}

#[aoc(day19, part1)]
fn part1(input: &u32) -> u32 {
    let n = *input;
    let l = n - (1 << (31 - n.leading_zeros()));
    2 * l + 1
}

#[aoc(day19, part2)]
fn part2(input: &u32) -> u32 {
    let n = *input;
    let mut left = (1..=n / 2 + 1).collect::<VecDeque<_>>();
    let mut right = (n.div_ceil(2) + 1..=n).collect::<VecDeque<_>>();
    loop {
        if right.len() >= left.len() {
            right.pop_front();
            if right.is_empty() {
                return left[0];
            }
        } else {
            left.pop_back();
        }
        left.push_back(right.pop_front().unwrap());
        right.push_back(left.pop_front().unwrap());
    }
}
