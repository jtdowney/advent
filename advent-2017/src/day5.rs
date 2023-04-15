use std::num::ParseIntError;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
fn generator(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.lines().map(|line| line.parse()).collect()
}

#[aoc(day5, part1)]
fn part1(input: &[i32]) -> usize {
    let mut input = input.to_vec();
    let mut steps = 0;
    let mut pc = 0;

    while let Some(offset) = input.get_mut(pc) {
        pc = pc.saturating_add_signed(*offset as isize);
        *offset += 1;
        steps += 1;
    }

    steps
}

#[aoc(day5, part2)]
fn part2(input: &[i32]) -> usize {
    let mut input = input.to_vec();
    let mut steps = 0;
    let mut pc = 0;

    while let Some(offset) = input.get_mut(pc) {
        pc = pc.saturating_add_signed(*offset as isize);

        if *offset >= 3 {
            *offset -= 1;
        } else {
            *offset += 1;
        }

        steps += 1;
    }

    steps
}
