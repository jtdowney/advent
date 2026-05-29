use std::collections::HashSet;

use itertools::Itertools;

#[aoc_generator(day6)]
fn generator(input: &str) -> Vec<Vec<HashSet<char>>> {
    input
        .split("\n\n")
        .map(|group| group.lines().map(|line| line.chars().collect()).collect())
        .collect()
}

#[aoc(day6, part1)]
fn part1(answers: &[Vec<HashSet<char>>]) -> usize {
    answers
        .iter()
        .map(|group| group.iter().flatten().unique().count())
        .sum()
}

#[aoc(day6, part2)]
fn part2(answers: &[Vec<HashSet<char>>]) -> usize {
    answers
        .iter()
        .map(|group| {
            ('a'..='z')
                .filter(|c| group.iter().all(|a| a.contains(c)))
                .count()
        })
        .sum()
}
