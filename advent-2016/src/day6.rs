use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day6)]
fn generator(input: &str) -> Vec<Vec<char>> {
    let mut columns = vec![vec![]; input.lines().next().unwrap().len()];

    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            columns[i].push(c);
        }
    }

    columns
}

#[aoc(day6, part1)]
fn part1(input: &[Vec<char>]) -> String {
    input
        .iter()
        .filter_map(|column| {
            column
                .iter()
                .counts()
                .into_iter()
                .max_by_key(|&(_, count)| count)
                .map(|(c, _)| c)
        })
        .collect()
}

#[aoc(day6, part2)]
fn part2(input: &[Vec<char>]) -> String {
    input
        .iter()
        .filter_map(|column| {
            column
                .iter()
                .counts()
                .into_iter()
                .min_by_key(|&(_, count)| count)
                .map(|(c, _)| c)
        })
        .collect()
}
