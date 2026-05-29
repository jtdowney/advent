use std::{clone::Clone, ops::RangeInclusive};

use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};

fn is_repetition(bytes: &[u8], chunks: usize) -> bool {
    if !bytes.len().is_multiple_of(chunks) {
        return false;
    }

    let chunk_size = bytes.len() / chunks;
    let first = &bytes[..chunk_size];

    bytes.chunks(chunk_size).all(|chunk| chunk == first)
}

#[aoc_generator(day2)]
fn generator(input: &str) -> anyhow::Result<Vec<RangeInclusive<u64>>> {
    input
        .trim()
        .split(',')
        .map(|part| {
            let (min, max) = part.trim().split_once('-').context("missing range")?;
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
        .filter(|&id| {
            let id_str = id.to_string();
            is_repetition(id_str.as_bytes(), 2)
        })
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &[RangeInclusive<u64>]) -> u64 {
    input
        .iter()
        .flat_map(Clone::clone)
        .filter(|&id| {
            let id_str = id.to_string();
            let bytes = id_str.as_bytes();
            (2..=bytes.len())
                .filter(|&size| bytes.len().is_multiple_of(size))
                .any(|chunks| is_repetition(bytes, chunks))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeated_ids_part1() {
        assert!(is_repetition(b"55", 2));
        assert!(is_repetition(b"6464", 2));
        assert!(is_repetition(b"123123", 2));
        assert!(is_repetition(b"1188511885", 2));
        assert!(!is_repetition(b"101", 2));
    }

    #[test]
    fn test_repeated_ids_part2() {
        assert!(is_repetition(b"12341234", 2));
        assert!(is_repetition(b"123123123", 3));
        assert!(is_repetition(b"1212121212", 5));
        assert!(is_repetition(b"1111111", 7));
        assert!(!is_repetition(b"12345", 2));
        assert!(!is_repetition(b"123456", 3));
    }
}
