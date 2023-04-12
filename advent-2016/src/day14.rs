use std::{collections::HashMap, iter};

use aoc_runner_derive::aoc;
use itertools::Itertools;
use md5::Digest;

fn memoized_hash(
    input: &str,
    counter: usize,
    stretch: bool,
    cache: &mut HashMap<usize, Digest>,
) -> Digest {
    if let Some(hash) = cache.get(&counter) {
        *hash
    } else {
        let mut hash = md5::compute(format!("{}{}", input, counter));
        if stretch {
            hash = (0..2016).fold(hash, |hash, _| md5::compute(format!("{:x}", hash)));
        }
        cache.insert(counter, hash);
        hash
    }
}

fn next_hash(
    input: &str,
    counter: usize,
    stretch: bool,
    cache: &mut HashMap<usize, Digest>,
) -> usize {
    for i in counter.. {
        let digest = memoized_hash(input, i, stretch, cache);
        let hex = format!("{:x}", digest);
        if let Some((x, _, _)) = hex
            .chars()
            .tuple_windows()
            .find(|(a, b, c)| a == b && b == c)
        {
            for j in 1..=1000 {
                let digest = memoized_hash(input, i + j, stretch, cache);
                let hex = format!("{:x}", digest);
                let five_match = hex
                    .chars()
                    .tuple_windows()
                    .any(|(a, b, c, d, e)| a == x && a == b && b == c && c == d && d == e);
                if five_match {
                    return i;
                }
            }
        }
    }

    unreachable!()
}

#[aoc(day14, part1)]
fn part1(input: &str) -> Option<usize> {
    let mut cache = HashMap::new();
    let mut counter = 0;
    iter::from_fn(|| {
        counter = next_hash(input, counter + 1, false, &mut cache);
        Some(counter)
    })
    .nth(63)
}

#[aoc(day14, part2)]
fn part2(input: &str) -> Option<usize> {
    let mut cache = HashMap::new();
    let mut counter = 0;
    iter::from_fn(|| {
        counter = next_hash(input, counter + 1, true, &mut cache);
        Some(counter)
    })
    .nth(63)
}
