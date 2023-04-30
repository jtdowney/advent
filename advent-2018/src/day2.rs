use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day2)]
fn generator(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[aoc(day2, part1)]
fn part1(input: &[String]) -> usize {
    let (twos, threes) = input.iter().fold((0, 0), |(mut twos, mut threes), line| {
        let freqs = line.chars().counts();

        if freqs.values().any(|&n| n == 2) {
            twos += 1;
        }

        if freqs.values().any(|&n| n == 3) {
            threes += 1;
        }

        (twos, threes)
    });

    twos * threes
}

#[aoc(day2, part2)]
fn part2(input: &[String]) -> Option<String> {
    input
        .iter()
        .tuple_combinations()
        .find(|(item1, item2)| {
            let edits = item1
                .chars()
                .zip(item2.chars())
                .filter(|(c1, c2)| c1 != c2)
                .count();
            edits == 1
        })
        .map(|(item1, item2)| {
            item1
                .chars()
                .zip(item2.chars())
                .filter(|(c1, c2)| c1 == c2)
                .map(|(c, _)| c)
                .collect()
        })
}
