use std::{collections::VecDeque, num::ParseIntError};

use aoc_runner_derive::{aoc, aoc_generator};

struct Scoreboard {
    position: usize,
    scores: Vec<char>,
    workers: (usize, usize),
}

impl Default for Scoreboard {
    fn default() -> Self {
        Scoreboard {
            position: 0,
            scores: vec!['3', '7'],
            workers: (0, 1),
        }
    }
}

impl Iterator for Scoreboard {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.scores.len() {
            let (left, right) = self.workers;
            let left_score = self.scores[left].to_digit(10).unwrap() as usize;
            let right_score = self.scores[right].to_digit(10).unwrap() as usize;
            let score = left_score + right_score;
            self.scores.extend(score.to_string().chars());

            let left = (left + left_score + 1) % self.scores.len();
            let right = (right + right_score + 1) % self.scores.len();

            self.workers = (left, right);
        }

        let item = self.scores.get(self.position).cloned();
        self.position += 1;

        item
    }
}

#[aoc_generator(day14)]
fn generator(input: &str) -> Result<usize, ParseIntError> {
    input.parse()
}

#[aoc(day14, part1)]
fn part1(input: &usize) -> String {
    let scoreboard = Scoreboard::default();
    scoreboard.skip(*input).take(10).collect()
}

#[aoc(day14, part2)]
fn part2(input: &usize) -> usize {
    let scoreboard = Scoreboard::default();
    let input_chars = input.to_string().chars().collect::<Vec<char>>();
    scoreboard
        .enumerate()
        .try_fold(VecDeque::new(), |mut acc, (i, score)| {
            acc.push_back(score);
            if acc.len() > input_chars.len() {
                let _ = acc.pop_front();
            }

            if acc == input_chars {
                Err(i - input_chars.len() + 1)
            } else {
                Ok(acc)
            }
        })
        .unwrap_err()
}
