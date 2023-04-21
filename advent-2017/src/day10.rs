use std::num::ParseIntError;

use aoc_runner_derive::aoc;

use crate::knot_hasher::KnotHasher;

#[aoc(day10, part1)]
fn part1(input: &str) -> Result<u32, ParseIntError> {
    let data = input
        .split(',')
        .map(|part| part.parse())
        .collect::<Result<Vec<u8>, _>>()?;
    let mut hasher = KnotHasher::default();
    hasher.mix_all(&data);

    let product = hasher.state.iter().take(2).map(|&n| u32::from(n)).product();
    Ok(product)
}

#[aoc(day10, part2)]
fn part2(input: &[u8]) -> String {
    let mut hasher = KnotHasher::default();
    hasher.hash(input).to_string()
}
