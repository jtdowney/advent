use std::{cmp::min, collections::HashMap, iter, str::FromStr};

use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};

type CacheKey<'a> = (&'a [char], &'a [usize]);
type Cache<'a> = HashMap<CacheKey<'a>, u64>;

struct Record {
    conditions: Vec<char>,
    chunks: Vec<usize>,
}

impl Record {
    fn arrangements(&self) -> u64 {
        fn fits(conditions: &[char], start: usize, length: usize) -> bool {
            if let Some(&postfix) = conditions.get(start + length)
                && postfix == '#'
            {
                return false;
            }

            if conditions[..start].contains(&'#') {
                return false;
            }

            let window = &conditions[start..start + length];
            if window.contains(&'.') {
                return false;
            }

            true
        }

        fn search<'a>(conditions: &'a [char], chunks: &'a [usize], cache: &mut Cache<'a>) -> u64 {
            if let Some(&count) = cache.get(&(conditions, chunks)) {
                return count;
            }

            if chunks.is_empty() {
                if conditions.contains(&'#') {
                    return 0;
                }
                return 1;
            }

            let length = chunks[0];
            let rest_chunks = &chunks[1..];

            let mut count = 0;
            for start in 0..conditions.len() {
                if start + length > conditions.len() {
                    break;
                }

                let end = start + length;
                if fits(conditions, start, length) {
                    let next = min(end + 1, conditions.len());
                    count += search(&conditions[next..], rest_chunks, cache);
                }
            }

            cache.insert((conditions, chunks), count);
            count
        }

        let mut cache = HashMap::new();
        search(&self.conditions, &self.chunks, &mut cache)
    }

    fn unfold(&self) -> Self {
        let conditions = itertools::intersperse(iter::repeat_n(&self.conditions, 5), &vec!['?'])
            .flatten()
            .copied()
            .collect();
        let chunks = iter::repeat_n(&self.chunks, 5).flatten().copied().collect();
        Self { conditions, chunks }
    }
}

impl FromStr for Record {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (conditions, chunks) = s.split_once(' ').context("splitting space")?;
        let conditions = conditions.chars().collect();
        let chunks = chunks
            .split(',')
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        Ok(Self { conditions, chunks })
    }
}

#[aoc_generator(day12)]
fn generator(input: &str) -> anyhow::Result<Vec<Record>> {
    input.lines().map(str::parse).collect()
}

#[aoc(day12, part1)]
fn part1(input: &[Record]) -> u64 {
    input.iter().map(Record::arrangements).sum()
}

#[aoc(day12, part2)]
fn part2(input: &[Record]) -> u64 {
    input
        .iter()
        .map(|record| record.unfold().arrangements())
        .sum()
}
