use std::{num::ParseIntError, str::FromStr};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct BoardingPass(u16);

impl FromStr for BoardingPass {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = s
            .chars()
            .map(|ch| match ch {
                'F' => '0',
                'B' => '1',
                'L' => '0',
                'R' => '1',
                _ => unreachable!(),
            })
            .collect::<String>();
        u16::from_str_radix(&id, 2).map(BoardingPass)
    }
}

#[aoc_generator(day5)]
fn generator(input: &str) -> Vec<BoardingPass> {
    input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

#[aoc(day5, part1)]
fn part1(passes: &[BoardingPass]) -> u16 {
    passes.iter().max().map(|&BoardingPass(n)| n).unwrap()
}

#[aoc(day5, part2)]
fn part2(passes: &[BoardingPass]) -> u16 {
    let mut passes = passes.to_vec();
    passes.sort_unstable();

    let neighbors = passes
        .windows(2)
        .find(|parts| {
            let BoardingPass(a) = parts[0];
            let BoardingPass(b) = parts[1];
            b - a == 2
        })
        .map(|passes| passes[0].0)
        .unwrap();

    neighbors + 1
}
