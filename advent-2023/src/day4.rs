use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

struct Game {
    winning_numbers: HashSet<u16>,
    my_numbers: HashSet<u16>,
}

#[aoc_generator(day4)]
fn generator(input: &str) -> anyhow::Result<Vec<Game>> {
    input
        .lines()
        .map(|line| {
            let mut iter = line.split_ascii_whitespace().skip(2);
            let winning_numbers = iter
                .by_ref()
                .take_while(|&s| s != "|")
                .map(str::parse)
                .collect::<Result<_, _>>()?;
            let my_numbers = iter.map(str::parse).collect::<Result<_, _>>()?;

            Ok(Game {
                winning_numbers,
                my_numbers,
            })
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(input: &[Game]) -> usize {
    input
        .iter()
        .map(|game| {
            let count = (&game.winning_numbers & &game.my_numbers).len() as u32;
            if count > 0 {
                1 << (count - 1)
            } else {
                0
            }
        })
        .sum()
}

#[aoc(day4, part2)]
fn part2(input: &[Game]) -> usize {
    let mut copies = vec![1; input.len()];
    for (i, game) in input.iter().enumerate() {
        let count = (&game.winning_numbers & &game.my_numbers).len();
        for offset in 1..=count {
            copies[i + offset] += copies[i];
        }
    }

    copies.iter().sum()
}
