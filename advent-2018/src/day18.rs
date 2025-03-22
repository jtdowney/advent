use std::{
    collections::HashMap,
    fmt::{self, Display},
};

use anyhow::bail;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Cell {
    Open,
    Tree,
    Lumberyard,
}

impl Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Cell::Open => '.',
            Cell::Tree => '|',
            Cell::Lumberyard => '#',
        };

        write!(f, "{}", c)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Grid(Vec<Vec<Cell>>);

impl Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Grid(cells) = self;
        for row in cells {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Grid {
    fn neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = Cell> + '_ {
        let Grid(cells) = self;

        [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ]
        .into_iter()
        .filter_map(move |(dx, dy)| {
            let x = x.checked_add_signed(dx);
            let y = y.checked_add_signed(dy);

            let (x, y) = x.zip(y)?;
            if x >= cells[0].len() || y >= cells.len() {
                None
            } else {
                Some(cells[y][x])
            }
        })
    }

    fn next(self) -> Self {
        let Grid(ref cells) = self;
        let mut next = cells.clone();

        for (y, row) in cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let neighbors = self.neighbors(x, y).counts();
                // dbg!(cell, &neighbors);

                next[y][x] = match cell {
                    Cell::Open => {
                        if neighbors.get(&Cell::Tree).copied().unwrap_or_default() >= 3 {
                            Cell::Tree
                        } else {
                            Cell::Open
                        }
                    }
                    Cell::Tree => {
                        if neighbors
                            .get(&Cell::Lumberyard)
                            .copied()
                            .unwrap_or_default()
                            >= 3
                        {
                            Cell::Lumberyard
                        } else {
                            Cell::Tree
                        }
                    }
                    Cell::Lumberyard => {
                        if neighbors
                            .get(&Cell::Lumberyard)
                            .copied()
                            .unwrap_or_default()
                            >= 1
                            && neighbors.get(&Cell::Tree).copied().unwrap_or_default() >= 1
                        {
                            Cell::Lumberyard
                        } else {
                            Cell::Open
                        }
                    }
                }
            }
        }

        Grid(next)
    }
}

#[aoc_generator(day18)]
fn generator(input: &str) -> anyhow::Result<Grid> {
    let cells = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    Ok(match c {
                        '.' => Cell::Open,
                        '|' => Cell::Tree,
                        '#' => Cell::Lumberyard,
                        _ => bail!("Invalid input at ({}, {}): {}", x, y, c),
                    })
                })
                .collect::<Result<Vec<Cell>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Grid(cells))
}

#[aoc(day18, part1)]
fn part1(input: &Grid) -> usize {
    let mut grid = input.clone();
    for _ in 0..10 {
        grid = grid.next();
    }

    let Grid(cells) = grid;
    let counts = cells.iter().flatten().counts();
    counts[&Cell::Tree] * counts[&Cell::Lumberyard]
}

#[aoc(day18, part2)]
fn part2(input: &Grid) -> usize {
    let mut grid = input.clone();
    let mut seen = HashMap::new();
    let mut cycle = None;

    for i in 0.. {
        if let Some(&prev) = seen.get(&grid) {
            cycle = Some((prev, i));
            break;
        }

        seen.insert(grid.clone(), i);
        grid = grid.next();
    }

    let (prev, i) = cycle.unwrap();
    let cycle_len = i - prev;
    let remaining = (1_000_000_000 - i) % cycle_len;

    for _ in 0..remaining {
        grid = grid.next();
    }

    let Grid(cells) = grid;
    let counts = cells.iter().flatten().counts();
    counts[&Cell::Tree] * counts[&Cell::Lumberyard]
}
