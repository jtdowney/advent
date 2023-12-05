use std::{cmp::min, collections::HashMap, str::FromStr};

use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl FromStr for Category {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let category = match s {
            "seed" => Self::Seed,
            "soil" => Self::Soil,
            "fertilizer" => Self::Fertilizer,
            "water" => Self::Water,
            "light" => Self::Light,
            "temperature" => Self::Temperature,
            "humidity" => Self::Humidity,
            "location" => Self::Location,
            _ => anyhow::bail!("unknown category: {}", s),
        };

        Ok(category)
    }
}

struct RangeMap {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

impl FromStr for RangeMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_ascii_whitespace().map(str::parse).flatten();
        let destination_start = parts.next().context("missing destination start")?;
        let source_start = parts.next().context("missing source start")?;
        let length = parts.next().context("missing length")?;

        Ok(Self {
            destination_start,
            source_start,
            length,
        })
    }
}

impl RangeMap {
    fn lookup(&self, value: u64) -> Option<u64> {
        if value < self.source_start || value >= self.source_start + self.length {
            return None;
        }

        let offset = value - self.source_start;
        Some(self.destination_start + offset)
    }
}

struct CategoryMap {
    destination: Category,
    range_maps: Vec<RangeMap>,
}

impl CategoryMap {
    fn lookup(&self, value: u64) -> u64 {
        self.range_maps
            .iter()
            .find_map(|rm| rm.lookup(value))
            .unwrap_or(value)
    }
}

struct Almanac {
    seeds: Vec<u64>,
    category_maps: HashMap<Category, CategoryMap>,
}

#[aoc_generator(day5)]
fn generator(input: &str) -> anyhow::Result<Almanac> {
    let (seeds_line, rest) = input.split_once("\n\n").context("splitting seeds line")?;
    let (_, seeds) = seeds_line.split_once(':').context("splitting seeds")?;
    let seeds = seeds
        .split_ascii_whitespace()
        .map(str::parse)
        .collect::<Result<_, _>>()?;

    let category_maps = rest
        .split("\n\n")
        .try_fold(HashMap::new(), |mut acc, part| {
            let (label, mapping_lines) = part.split_once('\n').context("splitting label line")?;
            let (source, rest) = label.split_once("-to-").context("splitting label")?;
            let (destination, _) = rest.split_once(' ').context("splitting destination")?;
            let range_maps = mapping_lines
                .lines()
                .map(str::parse)
                .collect::<Result<_, _>>()?;

            acc.insert(
                source.parse()?,
                CategoryMap {
                    destination: destination.parse()?,
                    range_maps,
                },
            );

            anyhow::Ok(acc)
        })?;

    Ok(Almanac {
        seeds,
        category_maps,
    })
}

#[aoc(day5, part1)]
fn part1(input: &Almanac) -> Option<u64> {
    let mut lowest_value = u64::MAX;
    for value in &input.seeds {
        let mut current = *value;
        let mut source = Category::Seed;
        while let Some(map) = input.category_maps.get(&source) {
            current = map.lookup(current);
            source = map.destination;
        }

        lowest_value = min(lowest_value, current);
    }

    Some(lowest_value)
}

#[aoc(day5, part2)]
fn part2(input: &Almanac) -> Option<u64> {
    let mut lowest_value = u64::MAX;
    let pairs = input.seeds.chunks_exact(2);
    for pair in pairs {
        let start = pair[0];
        let length = pair[1];
        for value in start..(start + length) {
            let mut current = value;
            let mut source = Category::Seed;
            while let Some(map) = input.category_maps.get(&source) {
                current = map.lookup(current);
                source = map.destination;
            }

            lowest_value = min(lowest_value, current);
        }
    }

    Some(lowest_value)
}
