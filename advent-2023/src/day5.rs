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
    destination_start: u32,
    source_start: u32,
    length: u32,
}

impl FromStr for RangeMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .split_ascii_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;
        let destination_start = parts[0];
        let source_start = parts[1];
        let length = parts[2];

        Ok(Self {
            destination_start,
            source_start,
            length,
        })
    }
}

impl RangeMap {
    fn lookup(&self, value: u32) -> Option<u32> {
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
    fn map_value(&self, value: u32) -> u32 {
        self.range_maps
            .iter()
            .find_map(|rm| rm.lookup(value))
            .unwrap_or(value)
    }

    fn map_range(&self, (start, length): (u32, u32)) -> Vec<(u32, u32)> {
        let candidate = self
            .range_maps
            .iter()
            .find(|rm| start >= rm.source_start && start < rm.source_start + rm.length);
        if let Some(range) = candidate {
            let current_end = start + length;
            let range_end = range.source_start + range.length;
            let dest_start = range.lookup(start).unwrap();
            if current_end <= range_end {
                return vec![(dest_start, length)];
            }

            let overage = current_end - range_end;
            let dest_length = range_end - start;

            let mut result = vec![(dest_start, dest_length)];
            let rest = self.map_range((range_end, overage));
            result.extend_from_slice(&rest);
            result
        } else {
            vec![(start, length)]
        }
    }
}

struct Almanac {
    seeds: Vec<u32>,
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
            let mut range_maps = mapping_lines
                .lines()
                .map(str::parse)
                .collect::<Result<Vec<RangeMap>, _>>()?;
            range_maps.sort_unstable_by_key(|rm| rm.source_start);

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
fn part1(input: &Almanac) -> Option<u32> {
    let mut lowest_value = u32::MAX;
    for value in &input.seeds {
        let mut current = *value;
        let mut source = Category::Seed;
        while let Some(map) = input.category_maps.get(&source) {
            current = map.map_value(current);
            source = map.destination;
        }

        lowest_value = min(lowest_value, current);
    }

    Some(lowest_value)
}

#[aoc(day5, part2)]
fn part2(input: &Almanac) -> Option<u32> {
    let mut lowest_value = u32::MAX;
    let pairs = input.seeds.chunks_exact(2);
    for pair in pairs {
        let start = pair[0];
        let length = pair[1];

        let mut current = vec![(start, length)];
        let mut source = Category::Seed;
        while let Some(map) = input.category_maps.get(&source) {
            current = current
                .iter()
                .flat_map(|&range| map.map_range(range))
                .collect();
            source = map.destination;
        }

        let pair_lowest = current
            .iter()
            .map(|&(start, _)| start)
            .min()
            .unwrap_or(u32::MAX);

        lowest_value = min(lowest_value, pair_lowest);
    }

    Some(lowest_value)
}
