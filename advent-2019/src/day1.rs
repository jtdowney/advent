use std::iter;

use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn generate(input: &str) -> anyhow::Result<Vec<i32>> {
    input
        .lines()
        .map(|line| line.parse::<i32>().context("parse number"))
        .collect()
}

fn calculate_fuel(mass: i32) -> i32 {
    (mass / 3) - 2
}

#[aoc(day1, part1)]
fn part1(input: &[i32]) -> i32 {
    input.iter().map(|&mass| calculate_fuel(mass)).sum()
}

#[aoc(day1, part2)]
fn part2(input: &[i32]) -> i32 {
    input
        .iter()
        .map(|&mass| {
            let fuel = calculate_fuel(mass);
            iter::successors(Some(fuel), |&prev_mass| {
                let fuel = calculate_fuel(prev_mass);
                Some(fuel)
            })
            .take_while(|fuel| fuel.is_positive())
            .sum::<i32>()
        })
        .sum()
}
