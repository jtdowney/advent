use std::str::FromStr;

use anyhow::{Context, ensure};
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy)]
struct Blueprint {
    id: usize,
    ore_robot_ore: u8,
    clay_robot_ore: u8,
    obsidian_robot_ore: u8,
    obsidian_robot_clay: u8,
    geode_robot_ore: u8,
    geode_robot_obsidian: u8,
}

impl Blueprint {
    fn max_ore_needed(&self) -> u16 {
        [
            self.ore_robot_ore,
            self.clay_robot_ore,
            self.obsidian_robot_ore,
            self.geode_robot_ore,
        ]
        .into_iter()
        .max()
        .unwrap() as u16
    }
}

impl FromStr for Blueprint {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        ensure!(parts.len() >= 32, "Invalid blueprint format");

        Ok(Blueprint {
            id: parts[1]
                .trim_end_matches(':')
                .parse()
                .context("Invalid ID")?,
            ore_robot_ore: parts[6].parse().context("Invalid ore robot ore cost")?,
            clay_robot_ore: parts[12].parse().context("Invalid clay robot ore cost")?,
            obsidian_robot_ore: parts[18]
                .parse()
                .context("Invalid obsidian robot ore cost")?,
            obsidian_robot_clay: parts[21]
                .parse()
                .context("Invalid obsidian robot clay cost")?,
            geode_robot_ore: parts[27].parse().context("Invalid geode robot ore cost")?,
            geode_robot_obsidian: parts[30]
                .parse()
                .context("Invalid geode robot obsidian cost")?,
        })
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct State {
    time: u8,
    ore: u16,
    clay: u16,
    obsidian: u16,
    geodes: u16,
    ore_robots: u16,
    clay_robots: u16,
    obsidian_robots: u16,
    geode_robots: u16,
}

impl State {
    fn advance_time(&self) -> Self {
        Self {
            time: self.time + 1,
            ore: self.ore + self.ore_robots,
            clay: self.clay + self.clay_robots,
            obsidian: self.obsidian + self.obsidian_robots,
            geodes: self.geodes + self.geode_robots,
            ..*self
        }
    }

    fn build_ore_robot(&self, cost: u16) -> Self {
        let mut next = self.advance_time();
        next.ore -= cost;
        next.ore_robots += 1;
        next
    }

    fn build_clay_robot(&self, cost: u16) -> Self {
        let mut next = self.advance_time();
        next.ore -= cost;
        next.clay_robots += 1;
        next
    }

    fn build_obsidian_robot(&self, ore_cost: u16, clay_cost: u16) -> Self {
        let mut next = self.advance_time();
        next.ore -= ore_cost;
        next.clay -= clay_cost;
        next.obsidian_robots += 1;
        next
    }

    fn build_geode_robot(&self, ore_cost: u16, obsidian_cost: u16) -> Self {
        let mut next = self.advance_time();
        next.ore -= ore_cost;
        next.obsidian -= obsidian_cost;
        next.geode_robots += 1;
        next
    }

    fn max_possible_geodes(&self, time_left: u8) -> u16 {
        self.geodes
            + self.geode_robots * time_left as u16
            + (time_left as u16 * (time_left as u16 - 1)) / 2
    }
}

#[aoc_generator(day19)]
fn generator(input: &str) -> anyhow::Result<Vec<Blueprint>> {
    input.lines().map(|line| line.parse()).collect()
}

fn max_geodes(blueprint: &Blueprint, max_time: u8) -> u16 {
    let mut best = 0;
    let mut stack = vec![State {
        time: 0,
        ore_robots: 1,
        ..Default::default()
    }];

    let max_ore_needed = blueprint.max_ore_needed();

    while let Some(state) = stack.pop() {
        if state.time == max_time {
            best = best.max(state.geodes);
            continue;
        }

        let time_left = max_time - state.time;

        if state.max_possible_geodes(time_left) <= best {
            continue;
        }

        if state.ore >= blueprint.geode_robot_ore as u16
            && state.obsidian >= blueprint.geode_robot_obsidian as u16
        {
            stack.push(state.build_geode_robot(
                blueprint.geode_robot_ore as u16,
                blueprint.geode_robot_obsidian as u16,
            ));
            continue;
        }

        if state.ore >= blueprint.obsidian_robot_ore as u16
            && state.clay >= blueprint.obsidian_robot_clay as u16
            && state.obsidian_robots < blueprint.geode_robot_obsidian as u16
        {
            stack.push(state.build_obsidian_robot(
                blueprint.obsidian_robot_ore as u16,
                blueprint.obsidian_robot_clay as u16,
            ));
        }

        if state.ore >= blueprint.clay_robot_ore as u16
            && state.clay_robots < blueprint.obsidian_robot_clay as u16
        {
            stack.push(state.build_clay_robot(blueprint.clay_robot_ore as u16));
        }

        if state.ore >= blueprint.ore_robot_ore as u16 && state.ore_robots < max_ore_needed {
            stack.push(state.build_ore_robot(blueprint.ore_robot_ore as u16));
        }

        if state.ore < max_ore_needed * 2 || state.clay < blueprint.obsidian_robot_clay as u16 * 2 {
            stack.push(state.advance_time());
        }
    }

    best
}

#[aoc(day19, part1)]
fn part1(blueprints: &[Blueprint]) -> usize {
    blueprints
        .iter()
        .map(|blueprint| {
            let geodes = max_geodes(blueprint, 24) as usize;
            blueprint.id * geodes
        })
        .sum()
}

#[aoc(day19, part2)]
fn part2(blueprints: &[Blueprint]) -> usize {
    blueprints
        .iter()
        .take(3)
        .map(|blueprint| max_geodes(blueprint, 32) as usize)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn test_part1() {
        let input = generator(EXAMPLE).unwrap();
        assert_eq!(part1(&input), 33);
    }

    #[test]
    fn test_part2() {
        let input = generator(EXAMPLE).unwrap();
        assert_eq!(part2(&input), 56 * 62);
    }
}
