use std::str::FromStr;

use anyhow::{Result, bail};
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
enum Instruction {
    DealIntoNewStack,
    CutN(i32),
    DealWithIncrement(usize),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if s == "deal into new stack" {
            Ok(Instruction::DealIntoNewStack)
        } else if let Some(n) = s.strip_prefix("cut ") {
            Ok(Instruction::CutN(n.parse()?))
        } else if let Some(n) = s.strip_prefix("deal with increment ") {
            Ok(Instruction::DealWithIncrement(n.parse()?))
        } else {
            bail!("Unknown instruction: {}", s);
        }
    }
}

#[aoc_generator(day22)]
fn parse(input: &str) -> Result<Vec<Instruction>> {
    input.lines().map(str::parse).collect()
}

#[aoc(day22, part1)]
fn part1(instructions: &[Instruction]) -> Result<usize> {
    const DECK_SIZE: i64 = 10007;
    const CARD: i64 = 2019;

    let (a, b) = compose_operations(instructions, DECK_SIZE);
    let position = mod_add(mod_mul(a, CARD, DECK_SIZE), b, DECK_SIZE);

    Ok(position as usize)
}

#[aoc(day22, part2)]
fn part2(instructions: &[Instruction]) -> Result<i64> {
    const DECK_SIZE: i64 = 119315717514047;
    const SHUFFLE_COUNT: i64 = 101741582076661;
    const TARGET_POSITION: i64 = 2020;

    let (a, b) = compose_operations(instructions, DECK_SIZE);
    let (final_a, final_b) = power_transform(a, b, SHUFFLE_COUNT, DECK_SIZE);

    let inv_a = mod_inverse(final_a, DECK_SIZE);
    let result = mod_sub(
        mod_mul(TARGET_POSITION, inv_a, DECK_SIZE),
        mod_mul(final_b, inv_a, DECK_SIZE),
        DECK_SIZE,
    );

    Ok(result)
}

fn compose_operations(instructions: &[Instruction], deck_size: i64) -> (i64, i64) {
    instructions.iter().fold((1, 0), |(a, b), instruction| {
        let (new_a, new_b) = match instruction {
            Instruction::DealIntoNewStack => (mod_sub(0, 1, deck_size), mod_sub(0, 1, deck_size)),
            Instruction::CutN(n) => (1, mod_sub(0, *n as i64, deck_size)),
            Instruction::DealWithIncrement(n) => (*n as i64, 0),
        };

        (
            mod_mul(new_a, a, deck_size),
            mod_add(mod_mul(new_a, b, deck_size), new_b, deck_size),
        )
    })
}

fn power_transform(a: i64, b: i64, k: i64, m: i64) -> (i64, i64) {
    match k {
        0 => (1, 0),
        1 => (a, b),
        _ => {
            let a_k = mod_pow(a, k, m);
            let b_k = if a == 1 {
                mod_mul(k, b, m)
            } else {
                let numerator = mod_mul(b, mod_sub(a_k, 1, m), m);
                let denominator = mod_sub(a, 1, m);
                mod_mul(numerator, mod_inverse(denominator, m), m)
            };
            (a_k, b_k)
        }
    }
}

fn mod_add(a: i64, b: i64, m: i64) -> i64 {
    ((a % m + b % m) % m + m) % m
}

fn mod_sub(a: i64, b: i64, m: i64) -> i64 {
    ((a % m - b % m) % m + m) % m
}

fn mod_mul(a: i64, b: i64, m: i64) -> i64 {
    let (mut a, mut b) = (a % m, b % m);
    let mut result = 0;

    while b > 0 {
        if b & 1 == 1 {
            result = (result + a) % m;
        }
        a = (a * 2) % m;
        b >>= 1;
    }

    result
}

fn mod_pow(base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result = 1;
    let mut base = base % m;

    while exp > 0 {
        if exp & 1 == 1 {
            result = mod_mul(result, base, m);
        }
        base = mod_mul(base, base, m);
        exp >>= 1;
    }

    result
}

fn mod_inverse(a: i64, m: i64) -> i64 {
    let (mut old_r, mut r) = (a, m);
    let (mut old_s, mut s) = (1, 0);

    while r != 0 {
        let quotient = old_r / r;
        let (new_r, new_s) = (old_r - quotient * r, old_s - quotient * s);
        old_r = r;
        r = new_r;
        old_s = s;
        s = new_s;
    }

    ((old_s % m) + m) % m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instructions() {
        let input = "deal with increment 7
deal into new stack
cut -2";
        let instructions = parse(input).unwrap();
        assert_eq!(instructions.len(), 3);
        match &instructions[0] {
            Instruction::DealWithIncrement(n) => assert_eq!(*n, 7),
            _ => panic!("Expected DealWithIncrement"),
        }
        match &instructions[1] {
            Instruction::DealIntoNewStack => {}
            _ => panic!("Expected DealIntoNewStack"),
        }
        match &instructions[2] {
            Instruction::CutN(n) => assert_eq!(*n, -2),
            _ => panic!("Expected CutN"),
        }
    }

    #[test]
    fn test_deal_into_new_stack() {
        let deck = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let result = apply_shuffle(deck, &Instruction::DealIntoNewStack);
        assert_eq!(result, vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn test_cut_positive() {
        let deck = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let result = apply_shuffle(deck, &Instruction::CutN(3));
        assert_eq!(result, vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
    }

    #[test]
    fn test_cut_negative() {
        let deck = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let result = apply_shuffle(deck, &Instruction::CutN(-4));
        assert_eq!(result, vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_deal_with_increment() {
        let deck = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let result = apply_shuffle(deck, &Instruction::DealWithIncrement(3));
        assert_eq!(result, vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
    }

    #[test]
    fn test_example_1() {
        let input = "deal with increment 7
deal into new stack
deal into new stack";
        let instructions = parse(input).unwrap();
        let deck: Vec<usize> = (0..10).collect();
        let result = instructions
            .iter()
            .fold(deck, |deck, instruction| apply_shuffle(deck, instruction));
        assert_eq!(result, vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
    }

    #[test]
    fn test_example_2() {
        let input = "cut 6
deal with increment 7
deal into new stack";
        let instructions = parse(input).unwrap();
        let deck: Vec<usize> = (0..10).collect();
        let result = instructions.iter().fold(deck, apply_shuffle);
        assert_eq!(result, vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
    }

    #[test]
    fn test_example_3() {
        let input = "deal with increment 7
deal with increment 9
cut -2";
        let instructions = parse(input).unwrap();
        let deck: Vec<usize> = (0..10).collect();
        let result = instructions.iter().fold(deck, apply_shuffle);
        assert_eq!(result, vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);
    }

    #[test]
    fn test_example_4() {
        let input = "deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1";
        let instructions = parse(input).unwrap();
        let deck: Vec<usize> = (0..10).collect();
        let result = instructions.iter().fold(deck, apply_shuffle);
        assert_eq!(result, vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
    }

    fn apply_shuffle(deck: Vec<usize>, instruction: &Instruction) -> Vec<usize> {
        match instruction {
            Instruction::DealIntoNewStack => deck.into_iter().rev().collect(),
            Instruction::CutN(n) => {
                let len = deck.len();
                let cut_point = ((n % len as i32) + len as i32) % len as i32;
                deck.into_iter()
                    .cycle()
                    .skip(cut_point as usize)
                    .take(len)
                    .collect()
            }
            Instruction::DealWithIncrement(inc) => {
                let len = deck.len();
                let mut result = vec![0; len];
                deck.into_iter().enumerate().for_each(|(i, card)| {
                    result[(i * inc) % len] = card;
                });
                result
            }
        }
    }

    #[test]
    fn test_modular_arithmetic() {
        assert_eq!(mod_pow(3, 4, 7), 4);
        assert_eq!(mod_pow(2, 10, 1000), 24);

        assert_eq!(mod_inverse(3, 7), 5);
        assert_eq!(mod_inverse(7, 11), 8);

        assert_eq!(mod_mul(999999999999, 999999999999, 1000000007), 49014001);

        let m = 10007;
        let (a1, b1) = (3, 5);
        let (a2, b2) = (7, 11);
        let expected_a = mod_mul(a1, a2, m);
        let expected_b = mod_add(mod_mul(a1, b2, m), b1, m);
        assert_eq!(expected_a, 21);
        assert_eq!(expected_b, 38);
    }
}
