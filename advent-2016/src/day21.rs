use anyhow::anyhow;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::{Finish, IResult, Parser};

const START_WORD: &str = "abcdefgh";
const END_WORD: &str = "fbgdceah";

#[derive(Debug)]
enum Operation {
    SwapPosition(usize, usize),
    SwapLetter(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateBasedOn(char),
    Reverse(usize, usize),
    Move(usize, usize),
}

impl Operation {
    fn transform(&self, input: &[char]) -> Vec<char> {
        let mut result = input.to_vec();
        match self {
            Operation::SwapPosition(x, y) => result.swap(*x, *y),
            Operation::SwapLetter(x, y) => {
                let x = result.iter().position(|&c| c == *x).unwrap();
                let y = result.iter().position(|&c| c == *y).unwrap();
                result.swap(x, y);
            }
            Operation::RotateLeft(x) => {
                result.rotate_left(*x);
            }
            Operation::RotateRight(x) => {
                result.rotate_right(*x);
            }
            Operation::RotateBasedOn(x) => {
                let x = result.iter().position(|&c| c == *x).unwrap();
                result.rotate_right(1);
                result.rotate_right(x);

                if x >= 4 {
                    result.rotate_right(1);
                }
            }
            Operation::Reverse(x, y) => {
                result[*x..=*y].reverse();
            }
            Operation::Move(x, y) => {
                let c = result.remove(*x);
                result.insert(*y, c);
            }
        }

        result
    }
}

fn scramble(input: &str, operations: &[Operation]) -> String {
    let password = input.chars().collect::<Vec<_>>();
    operations
        .iter()
        .fold(password, |acc, op| op.transform(&acc))
        .iter()
        .collect()
}

fn operation(input: &str) -> IResult<&str, Operation> {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{anychar, u32},
        combinator::map,
    };
    use Operation::*;

    let swap_position = map(
        (tag("swap position "), u32, tag(" with position "), u32),
        |(_, x, _, y)| SwapPosition(x as usize, y as usize),
    );
    let swap_letter = map(
        (tag("swap letter "), anychar, tag(" with letter "), anychar),
        |(_, x, _, y)| SwapLetter(x, y),
    );
    let rotate_left = map((tag("rotate left "), u32), |(_, x)| {
        RotateLeft(x as usize)
    });
    let rotate_right = map((tag("rotate right "), u32), |(_, x)| {
        RotateRight(x as usize)
    });
    let rotate_based_on = map(
        (tag("rotate based on position of letter "), anychar),
        |(_, x)| RotateBasedOn(x),
    );
    let reverse = map(
        (tag("reverse positions "), u32, tag(" through "), u32),
        |(_, x, _, y)| Reverse(x as usize, y as usize),
    );
    let move_position = map(
        (tag("move position "), u32, tag(" to position "), u32),
        |(_, x, _, y)| Move(x as usize, y as usize),
    );
    alt((
        swap_position,
        swap_letter,
        rotate_left,
        rotate_right,
        rotate_based_on,
        reverse,
        move_position,
    )).parse(input)
}

#[aoc_generator(day21)]
fn generator(input: &str) -> anyhow::Result<Vec<Operation>> {
    input
        .lines()
        .map(|line| {
            operation
                .parse(line)
                .finish()
                .map(|(_, op)| op)
                .map_err(|_| anyhow!("Invalid operation: {}", line))
        })
        .collect()
}

#[aoc(day21, part1)]
fn part1(input: &[Operation]) -> String {
    scramble(START_WORD, input)
}

#[aoc(day21, part2)]
fn part2(input: &[Operation]) -> Option<String> {
    START_WORD
        .chars()
        .permutations(START_WORD.len())
        .map(|candidate| candidate.iter().collect::<String>())
        .find(|candidate| scramble(candidate, input) == END_WORD)
}
