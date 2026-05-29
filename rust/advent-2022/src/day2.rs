use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Copy, Clone)]
enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl Outcome {
    fn score(self) -> usize {
        self as usize
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Play {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

impl From<i8> for Play {
    fn from(play: i8) -> Self {
        match play {
            0 => Play::Rock,
            1 => Play::Paper,
            2 => Play::Scissors,
            _ => unreachable!(),
        }
    }
}

impl From<char> for Play {
    fn from(play: char) -> Self {
        match play {
            'A' | 'X' => Play::Rock,
            'B' | 'Y' => Play::Paper,
            'C' | 'Z' => Play::Scissors,
            _ => unreachable!(),
        }
    }
}

impl Play {
    fn find_loss(self) -> Play {
        let value = self as i8;
        let loss = (value - 1).rem_euclid(3);
        Play::from(loss)
    }

    fn find_win(self) -> Play {
        let value = self as i8;
        let win = (value + 1).rem_euclid(3);
        Play::from(win)
    }

    fn play(self, other: Play) -> Outcome {
        match (self, other) {
            (a, b) if a == b => Outcome::Draw,
            (a, b) if a.find_win() == b => Outcome::Loss,
            (a, b) if a.find_loss() == b => Outcome::Win,
            _ => unreachable!(),
        }
    }

    fn score(self) -> usize {
        (self as usize) + 1
    }
}

#[aoc_generator(day2)]
fn generator(input: &str) -> anyhow::Result<Vec<(char, char)>> {
    input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let left = chars.next().context("getting left")?;
            let right = chars.nth(1).context("getting right")?;
            Ok((left, right))
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[(char, char)]) -> usize {
    input
        .iter()
        .map(|&(opponent, you)| {
            let opponent = Play::from(opponent);
            let you = Play::from(you);
            let outcome = you.play(opponent);
            you.score() + outcome.score()
        })
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &[(char, char)]) -> usize {
    input
        .iter()
        .map(|&(opponent, you)| {
            let opponent = Play::from(opponent);
            let you = match you {
                'X' => opponent.find_loss(),
                'Y' => opponent,
                'Z' => opponent.find_win(),
                _ => unreachable!(),
            };

            let outcome = you.play(opponent);
            you.score() + outcome.score()
        })
        .sum()
}
