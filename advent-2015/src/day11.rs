use std::collections::HashSet;

use aoc_runner_derive::aoc;
use itertools::Itertools;

fn password_candidates(input: &str) -> impl Iterator<Item = String> + '_ {
    std::iter::successors(Some(input.to_string()), |previous| {
        let mut next = previous.bytes().rev().collect::<Vec<u8>>();
        let mut position = 0;
        loop {
            next[position] += 1;
            if next[position] > b'z' {
                next[position] = b'a';
                position += 1;
            } else {
                break;
            }
        }

        next.reverse();
        let password = String::from_utf8(next).unwrap();
        Some(password)
    })
}

fn is_valid_password(password: &str) -> bool {
    let pairs = password
        .bytes()
        .tuple_windows()
        .filter(|(a, b)| a == b)
        .collect::<HashSet<(u8, u8)>>();
    password
        .bytes()
        .tuple_windows()
        .any(|(a, b, c)| a + 1 == b && b + 1 == c)
        && !(password.contains('i') || password.contains('o') || password.contains('u'))
        && pairs.len() >= 2
}

#[aoc(day11, part1)]
fn part1(input: &str) -> Option<String> {
    password_candidates(input).find(|p| is_valid_password(p))
}

#[aoc(day11, part2)]
fn part2(input: &str) -> Option<String> {
    password_candidates(input)
        .skip(1)
        .find(|p| is_valid_password(p))
}
