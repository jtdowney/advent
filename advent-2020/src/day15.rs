use std::{collections::HashMap, iter};

fn solve(target_round: usize, seen: &HashMap<usize, usize>) -> usize {
    let mut seen = seen.clone();
    let target = target_round - seen.len();

    let mut round = seen.len();
    iter::successors(Some(0), |number| {
        let next = match seen.get(number) {
            Some(last_seen) => round - last_seen,
            None => 0,
        };

        seen.insert(*number, round);
        round += 1;

        Some(next)
    })
    .take(target)
    .last()
    .unwrap()
}

#[aoc_generator(day15)]
fn generator(input: &str) -> HashMap<usize, usize> {
    input
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .enumerate()
        .map(|(i, s)| (s, i))
        .collect::<HashMap<usize, usize>>()
}

#[aoc(day15, part1)]
fn part1(seen: &HashMap<usize, usize>) -> usize {
    solve(2020, &seen)
}

#[aoc(day15, part2)]
fn part2(seen: &HashMap<usize, usize>) -> usize {
    solve(30000000, &seen)
}
