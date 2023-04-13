use std::iter;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Debug)]
struct Row {
    tiles: Vec<bool>,
}

impl Row {
    fn next(&self) -> Row {
        let tiles = (0..self.tiles.len())
            .map(|i| {
                let left = if i == 0 { false } else { self.tiles[i - 1] };
                let right = self.tiles.get(i + 1).copied().unwrap_or(false);

                (left && !right) || (!left && right)
            })
            .collect();

        Row { tiles }
    }
}

#[aoc_generator(day18)]
fn generator(input: &str) -> Row {
    let tiles = input.chars().map(|c| c == '^').collect();
    Row { tiles }
}

#[aoc(day18, part1)]
fn part1(input: &Row) -> usize {
    iter::successors(Some(input.clone()), |row| Some(row.next()))
        .take(40)
        .map(|row| row.tiles.iter().filter(|&&b| !b).count())
        .sum()
}

#[aoc(day18, part2)]
fn part2(input: &Row) -> usize {
    iter::successors(Some(input.clone()), |row| Some(row.next()))
        .take(400000)
        .map(|row| row.tiles.iter().filter(|&&b| !b).count())
        .sum()
}
