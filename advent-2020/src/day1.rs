use itertools::Itertools;

const SEARCH_VALUE: u32 = 2020;

#[aoc_generator(day1)]
fn generator(input: &str) -> Vec<u32> {
    input.lines().map(str::parse).map(Result::unwrap).collect()
}

fn solve(size: usize, input: &[u32]) -> u32 {
    let entries = input
        .iter()
        .combinations(size)
        .find(|entries| entries.iter().copied().sum::<u32>() == SEARCH_VALUE)
        .unwrap();

    entries.iter().copied().product()
}

#[aoc(day1, part1)]
fn part1(input: &[u32]) -> u32 {
    solve(2, input)
}

#[aoc(day1, part2)]
fn part2(input: &[u32]) -> u32 {
    solve(3, input)
}
