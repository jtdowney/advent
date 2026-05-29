use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day22)]
fn generator(input: &str) -> anyhow::Result<Vec<u64>> {
    input
        .lines()
        .map(|line| line.parse().map_err(Into::into))
        .collect()
}

fn mix(secret: u64, value: u64) -> u64 {
    secret ^ value
}

fn prune(secret: u64) -> u64 {
    secret % 16_777_216
}

fn evolve(secret: u64) -> u64 {
    let step1 = prune(mix(secret, secret * 64));
    let step2 = prune(mix(step1, step1 / 32));
    prune(mix(step2, step2 * 2048))
}

fn nth_secret(initial: u64, n: usize) -> u64 {
    (0..n).fold(initial, |secret, _| evolve(secret))
}

fn get_price(secret: u64) -> u8 {
    (secret % 10) as u8
}

fn get_price_changes(prices: &[u8]) -> Vec<i8> {
    prices
        .iter()
        .copied()
        .tuple_windows()
        .map(|(a, b)| (b as i8).saturating_sub_unsigned(a))
        .collect()
}

#[aoc(day22, part1)]
fn part1(initial_secrets: &[u64]) -> u64 {
    initial_secrets
        .iter()
        .map(|&secret| nth_secret(secret, 2000))
        .sum()
}

#[aoc(day22, part2)]
fn part2(initial_secrets: &[u64]) -> u64 {
    let sequence_totals = initial_secrets
        .iter()
        .flat_map(|&initial| {
            let prices: Vec<u8> = std::iter::successors(Some(initial), |&s| Some(evolve(s)))
                .take(2001)
                .map(get_price)
                .collect();

            let changes = get_price_changes(&prices);

            let mut seen_sequences = HashMap::new();

            changes
                .windows(4)
                .enumerate()
                .filter_map(|(i, window)| {
                    let sequence = window.to_vec();
                    if seen_sequences.contains_key(&sequence) {
                        None
                    } else {
                        let price = u64::from(prices[i + 4]);
                        seen_sequences.insert(sequence.clone(), price);
                        Some((sequence, price))
                    }
                })
                .collect::<Vec<_>>()
        })
        .fold(HashMap::new(), |mut totals, (sequence, price)| {
            *totals.entry(sequence).or_insert(0) += price;
            totals
        });

    sequence_totals.values().max().copied().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "1
10
100
2024";

    #[test]
    fn test_generator() {
        let secrets = generator(EXAMPLE_INPUT).unwrap();
        assert_eq!(secrets, vec![1, 10, 100, 2024]);
    }

    #[test]
    fn test_mix_operation() {
        let secret = 42;
        let value = 15;
        assert_eq!(mix(secret, value), 37);
    }

    #[test]
    fn test_prune_operation() {
        let secret = 100000000;
        assert_eq!(prune(secret), 16113920);
    }

    #[test]
    fn test_evolve_secret() {
        let mut secret = 123;
        secret = evolve(secret);
        assert_eq!(secret, 15887950);

        secret = evolve(secret);
        assert_eq!(secret, 16495136);

        secret = evolve(secret);
        assert_eq!(secret, 527345);

        secret = evolve(secret);
        assert_eq!(secret, 704524);

        secret = evolve(secret);
        assert_eq!(secret, 1553684);

        secret = evolve(secret);
        assert_eq!(secret, 12683156);

        secret = evolve(secret);
        assert_eq!(secret, 11100544);

        secret = evolve(secret);
        assert_eq!(secret, 12249484);

        secret = evolve(secret);
        assert_eq!(secret, 7753432);

        secret = evolve(secret);
        assert_eq!(secret, 5908254);
    }

    #[test]
    fn test_nth_secret() {
        assert_eq!(nth_secret(1, 2000), 8685429);
        assert_eq!(nth_secret(10, 2000), 4700978);
        assert_eq!(nth_secret(100, 2000), 15273692);
        assert_eq!(nth_secret(2024, 2000), 8667524);
    }

    #[test]
    fn test_part1() {
        let secrets = generator(EXAMPLE_INPUT).unwrap();
        assert_eq!(part1(&secrets), 37327623);
    }

    #[test]
    fn test_get_price() {
        assert_eq!(get_price(123), 3);
        assert_eq!(get_price(15887950), 0);
        assert_eq!(get_price(16495136), 6);
    }

    #[test]
    fn test_price_changes_sequence() {
        let mut secret = 123;
        let mut prices = vec![get_price(secret)];

        for _ in 0..9 {
            secret = evolve(secret);
            prices.push(get_price(secret));
        }

        assert_eq!(prices, vec![3, 0, 6, 5, 4, 4, 6, 4, 4, 2]);

        let changes = get_price_changes(&prices);
        assert_eq!(changes[0], -3);
        assert_eq!(changes[1], 6);
        assert_eq!(changes[2], -1);
        assert_eq!(changes[3], -1);
        assert_eq!(changes[4], 0);
        assert_eq!(changes[5], 2);
        assert_eq!(changes[6], -2);
        assert_eq!(changes[7], 0);
        assert_eq!(changes[8], -2);
    }

    #[test]
    fn test_part2() {
        let secrets = vec![1, 2, 3, 2024];
        assert_eq!(part2(&secrets), 23);
    }
}
