use std::collections::{HashMap, VecDeque};

use anyhow::anyhow;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
fn generator(input: &str) -> anyhow::Result<(usize, usize)> {
    let mut parts = input.split_whitespace();
    let players = parts
        .next()
        .and_then(|p| p.parse::<usize>().ok())
        .ok_or_else(|| anyhow!("Invalid players"))?;
    let marbles = parts
        .nth(5)
        .and_then(|p| p.parse::<usize>().ok())
        .ok_or_else(|| anyhow!("Invalid marbles"))?;

    Ok((players, marbles))
}

fn play(players: usize, marbles: usize) -> HashMap<usize, usize> {
    let mut scores = HashMap::new();
    let mut circle = VecDeque::with_capacity(marbles);
    circle.push_front(0);

    for (marble, player) in (1..=marbles).zip((1..=players).cycle()) {
        if marble % 23 == 0 {
            let mut tail = circle.split_off(circle.len() - 7);
            let scored = tail.pop_front().unwrap();

            for &item in tail.iter().rev() {
                circle.push_front(item);
            }

            *scores.entry(player).or_default() += marble + scored;
        } else {
            for _ in 0..2 {
                let current = circle.pop_front().unwrap();
                circle.push_back(current);
            }

            circle.push_front(marble);
        }
    }

    scores
}

#[aoc(day9, part1)]
fn part1(&(players, marbles): &(usize, usize)) -> Option<usize> {
    let scores = play(players, marbles);
    scores.into_values().max()
}

#[aoc(day9, part2)]
fn part2(&(players, marbles): &(usize, usize)) -> Option<usize> {
    let scores = play(players, marbles * 100);
    scores.into_values().max()
}
