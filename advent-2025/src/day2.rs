use std::{clone::Clone, ops::RangeInclusive};

use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};

fn valid(id: u64, chunks: usize) -> bool {
    let id = id.to_string();
    let bytes = id.as_bytes();

    if !bytes.len().is_multiple_of(chunks) {
        return true;
    }

    let chunk_size = bytes.len() / chunks;
    let first = &bytes[..chunk_size];

    !bytes.chunks(chunk_size).all(|chunk| chunk == first)
}

#[aoc_generator(day2)]
fn generator(input: &str) -> anyhow::Result<Vec<RangeInclusive<u64>>> {
    input
        .split(',')
        .map(|part| {
            let (min, max) = part.split_once('-').context("missing range")?;
            let range = min.parse()?..=max.parse()?;
            anyhow::Ok(range)
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[RangeInclusive<u64>]) -> u64 {
    input
        .iter()
        .flat_map(Clone::clone)
        .filter(|id| !valid(*id, 2))
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &[RangeInclusive<u64>]) -> u64 {
    input
        .iter()
        .flat_map(Clone::clone)
        .filter(|id| {
            let s = id.to_string();
            (2..=s.len()).any(|chunks| s.len() % chunks == 0 && !valid(*id, chunks))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeated_ids_part1() {
        assert!(!valid(55, 2));
        assert!(!valid(6464, 2));
        assert!(!valid(123_123, 2));
        assert!(!valid(1_188_511_885, 2));
        assert!(valid(101, 2));
    }

    #[test]
    fn test_repeated_ids_part2() {
        assert!(!valid(12_341_234, 2));
        assert!(!valid(123_123_123, 3));
        assert!(!valid(1_212_121_212, 5));
        assert!(!valid(1_111_111, 7));
        assert!(valid(12345, 2));
        assert!(valid(123_456, 3));
    }
}
