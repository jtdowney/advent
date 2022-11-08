use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day5)]
fn generator(input: &str) -> Vec<String> {
    input.lines().map(str::to_string).collect()
}

#[aoc(day5, part1)]
fn part1(input: &[String]) -> usize {
    let vowels = "aeiou".chars().collect::<HashSet<_>>();
    input
	.iter()
	.filter(|line| line.chars().filter(|c| vowels.contains(c)).count() >= 3)
	.filter(|line| {
	    line.chars()
		.scan(None, |state, c| {
		    let value = *state == Some(c);
		    *state = Some(c);
		    Some(value)
		})
		.any(|b| b)
	})
	.filter(|line| {
	    !(line.contains("ab")
		|| line.contains("cd")
		|| line.contains("pq")
		|| line.contains("xy"))
	})
	.count()
}

#[derive(Default)]
struct State {
    pairs: HashMap<(char, char), usize>,
    previous: Option<char>,
}

#[aoc(day5, part2)]
fn part2(input: &[String]) -> usize {
    input
	.iter()
	.filter(|line| {
	    let State { pairs, .. } =
		line.chars()
		    .tuple_windows()
		    .fold(State::default(), |mut acc, (a, b)| {
			if a == b && acc.previous == Some(a) {
			    return acc;
			}

			*acc.pairs.entry((a, b)).or_default() += 1;
			acc.previous = Some(a);
			acc
		    });
	    pairs.iter().any(|(_, &count)| count >= 2)
	})
	.filter(|line| line.chars().tuple_windows().any(|(a, _, b)| a == b))
	.count()
}
