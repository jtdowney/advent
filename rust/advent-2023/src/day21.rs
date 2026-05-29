use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

type Position = (i32, i32);

#[derive(Debug, Clone)]
struct Garden {
    grid: HashMap<Position, char>,
    start: Position,
    width: i32,
    height: i32,
}

#[aoc_generator(day21)]
fn generator(input: &str) -> anyhow::Result<Garden> {
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len() as i32;
    let width = lines.first().map(|l| l.len() as i32).unwrap_or(0);

    let mut grid = HashMap::new();
    let mut start = (0, 0);

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let pos = (x as i32, y as i32);
            match ch {
                'S' => {
                    start = pos;
                    grid.insert(pos, '.');
                }
                '#' => {}
                _ => {
                    grid.insert(pos, ch);
                }
            }
        }
    }

    Ok(Garden {
        grid,
        start,
        width,
        height,
    })
}

#[aoc(day21, part1)]
fn part1(garden: &Garden) -> usize {
    const TARGET_STEPS: usize = 64;
    reachable_after_steps(garden.start, TARGET_STEPS, |pos| {
        garden.grid.contains_key(&pos)
    })
}

#[aoc(day21, part2)]
fn part2(garden: &Garden) -> usize {
    const TARGET_STEPS: usize = 26501365;
    let size = garden.width as usize;

    let remainder = TARGET_STEPS % size;
    let n = TARGET_STEPS / size;

    let values: Vec<usize> = (0..3)
        .map(|i| {
            let steps = remainder + i * size;
            reachable_after_steps(garden.start, steps, |(x, y)| {
                let wrapped = (x.rem_euclid(garden.width), y.rem_euclid(garden.height));
                garden.grid.contains_key(&wrapped)
            })
        })
        .collect();

    let a = (values[2] + values[0]).saturating_sub(2 * values[1]) / 2;
    let b = values[1].saturating_sub(values[0]).saturating_sub(a);
    let c = values[0];

    a * n * n + b * n + c
}

fn reachable_after_steps<F>(start: Position, steps: usize, is_valid: F) -> usize
where
    F: Fn(Position) -> bool,
{
    let mut positions = HashSet::new();
    positions.insert(start);

    (0..steps)
        .fold(positions, |current_positions, _| {
            current_positions
                .iter()
                .flat_map(|&(x, y)| {
                    [(0, 1), (0, -1), (1, 0), (-1, 0)]
                        .iter()
                        .map(move |(dx, dy)| (x + dx, y + dy))
                })
                .filter(|&pos| is_valid(pos))
                .collect()
        })
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn test_part1_small() {
        let garden = generator(EXAMPLE).unwrap();
        assert_eq!(
            reachable_after_steps(garden.start, 6, |pos| { garden.grid.contains_key(&pos) }),
            16
        );
    }

    #[test]
    fn test_part2_examples() {
        let garden = generator(EXAMPLE).unwrap();
        let test_cases = [(6, 16), (10, 50), (50, 1594), (100, 6536)];

        for (steps, expected) in test_cases {
            assert_eq!(
                reachable_after_steps(garden.start, steps, |(x, y)| {
                    let wrapped = (x.rem_euclid(garden.width), y.rem_euclid(garden.height));
                    garden.grid.contains_key(&wrapped)
                }),
                expected
            );
        }
    }
}
