use std::num::ParseIntError;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day17)]
fn generator(input: &str) -> Result<usize, ParseIntError> {
    input.parse()
}

#[aoc(day17, part1)]
fn part1(input: &usize) -> usize {
    let size = *input;
    let mut current_position = 0;
    let mut buffer = vec![0];
    for i in 1..2018 {
        current_position = ((current_position + size) % buffer.len()) + 1;
        buffer.insert(current_position, i);
    }

    buffer[current_position + 1]
}

#[aoc(day17, part2)]
fn part2(input: &usize) -> usize {
    let size = *input;
    let mut current_position = 0;
    let mut value = 0;
    for i in 1..=50_000_000 {
        current_position = ((current_position + size) % i) + 1;

        if current_position == 1 {
            value = i;
        }
    }

    value
}
