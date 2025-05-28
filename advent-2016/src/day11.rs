use std::collections::{BTreeSet, HashSet, VecDeque};

use anyhow::bail;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{Itertools, iproduct};
use regex::Regex;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Part {
    Microchip(String),
    Generator(String),
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
struct Floor {
    parts: BTreeSet<Part>,
}

impl Floor {
    fn is_valid(&self) -> bool {
        self.parts.iter().all(|part| match part {
            Part::Microchip(microchip) => {
                self.parts.contains(&Part::Generator(microchip.clone()))
                    || !self
                        .parts
                        .iter()
                        .any(|part| matches!(part, Part::Generator(_)))
            }
            Part::Generator(_) => true,
        })
    }

    fn is_empty(&self) -> bool {
        self.parts.is_empty()
    }
}

#[aoc_generator(day11)]
fn generator(input: &str) -> anyhow::Result<Vec<Floor>> {
    let regex = Regex::new(
        r"(?:(?:(?P<microchip>\w+)-compatible microchip)|(?:(?P<generator>\w+) generator))",
    )?;

    input
        .lines()
        .map(|line| {
            regex
                .captures_iter(line)
                .try_fold(Floor::default(), |mut floor, cap| {
                    if let Some(microchip) = cap.name("microchip") {
                        floor
                            .parts
                            .insert(Part::Microchip(microchip.as_str().to_string()));
                    } else if let Some(generator) = cap.name("generator") {
                        floor
                            .parts
                            .insert(Part::Generator(generator.as_str().to_string()));
                    } else {
                        bail!("Invalid input");
                    }

                    Ok(floor)
                })
        })
        .collect()
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Step {
    count: usize,
    current_floor: usize,
    floors: Vec<Floor>,
}

impl Step {
    fn is_valid(&self) -> bool {
        self.floors.iter().all(Floor::is_valid)
    }

    fn is_complete(&self) -> bool {
        self.floors
            .iter()
            .take(self.floors.len() - 1)
            .all(|floor| floor.is_empty())
    }
}

fn solve(input: &[Floor]) -> usize {
    let floors = input.to_vec();
    let mut visited = HashSet::new();
    let mut search = VecDeque::from_iter([Step {
        count: 0,
        current_floor: 0,
        floors,
    }]);

    while let Some(step) = search.pop_front() {
        if step.is_complete() {
            return step.count;
        }

        if !visited.insert((step.current_floor, step.floors.clone())) {
            continue;
        }

        let next_floors = [-1, 1]
            .iter()
            .map(|&d| step.current_floor.saturating_add_signed(d))
            .filter(|&f| f != step.current_floor && f < step.floors.len())
            .collect_vec();
        for (next_floor, size) in iproduct!(next_floors, 1..=2) {
            let part_groups = step.floors[step.current_floor]
                .parts
                .iter()
                .combinations(size);
            for part_group in part_groups {
                let mut next_step = step.clone();
                next_step.count += 1;
                next_step.current_floor = next_floor;
                for part in part_group {
                    next_step.floors[step.current_floor].parts.remove(part);
                    next_step.floors[next_floor].parts.insert(part.clone());
                }

                if next_step.is_valid() {
                    search.push_back(next_step);
                }
            }
        }
    }

    unreachable!()
}

#[aoc(day11, part1)]
fn part1(input: &[Floor]) -> usize {
    solve(input)
}

#[aoc(day11, part2)]
fn part2(input: &[Floor]) -> usize {
    let mut floors = input.to_vec();
    for name in ["elerium", "dilithium"].iter() {
        floors[0].parts.insert(Part::Microchip(name.to_string()));
        floors[0].parts.insert(Part::Generator(name.to_string()));
    }

    solve(&floors)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_microchip_alone() {
        let mut floor = Floor::default();
        floor.parts.insert(Part::Microchip("hydrogen".to_string()));
        assert!(floor.is_valid());
    }

    #[test]
    fn test_microchip_with_its_generator() {
        let mut floor = Floor::default();
        floor.parts.insert(Part::Microchip("hydrogen".to_string()));
        floor.parts.insert(Part::Generator("hydrogen".to_string()));
        assert!(floor.is_valid());
    }

    #[test]
    fn test_microchip_with_another_generator() {
        let mut floor = Floor::default();
        floor.parts.insert(Part::Microchip("hydrogen".to_string()));
        floor.parts.insert(Part::Generator("lithium".to_string()));
        assert!(!floor.is_valid());
    }
}
