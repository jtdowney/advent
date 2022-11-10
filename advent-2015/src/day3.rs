use std::collections::HashSet;

use aoc_runner_derive::aoc;

type Position = (isize, isize);

#[aoc(day3, part1)]
fn part1(input: &str) -> usize {
    input
        .chars()
        .scan(Position::default(), |state, c| {
            let current = *state;
            let (x, y) = *state;
            *state = match c {
                '>' => (x + 1, y),
                '<' => (x - 1, y),
                '^' => (x, y + 1),
                'v' => (x, y - 1),
                _ => unreachable!(),
            };
            Some(current)
        })
        .collect::<HashSet<_>>()
        .len()
}

#[aoc(day3, part2)]
fn part2(input: &str) -> usize {
    input
        .chars()
        .enumerate()
        .scan(
            (Position::default(), Position::default()),
            |(left, right), (i, c)| {
                let current_left = *left;
                let current_right = *right;

                let active = if i % 2 == 0 { left } else { right };
                let (x, y) = *active;

                *active = match c {
                    '>' => (x + 1, y),
                    '<' => (x - 1, y),
                    '^' => (x, y + 1),
                    'v' => (x, y - 1),
                    _ => unreachable!(),
                };

                Some([current_left, current_right])
            },
        )
        .flatten()
        .collect::<HashSet<_>>()
        .len()
}
