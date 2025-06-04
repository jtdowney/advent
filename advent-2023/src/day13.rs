use std::cmp::min;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{iproduct, izip};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Mirror {
    Vertical(usize),
    Horizontal(usize),
}

impl Mirror {
    fn value(&self) -> usize {
        match self {
            Mirror::Vertical(v) => *v,
            Mirror::Horizontal(v) => v * 100,
        }
    }
}

#[derive(Debug, Clone)]
struct Pattern {
    cells: Vec<Vec<char>>,
}

impl Pattern {
    fn toggle(&mut self, x: usize, y: usize) {
        if self.cells[y][x] == '.' {
            self.cells[y][x] = '#';
        } else {
            self.cells[y][x] = '.';
        }
    }

    fn mirrors(&self) -> Vec<Mirror> {
        let mut mirrors = vec![];
        let candidates = (1..self.cells[0].len())
            .filter(|&x| self.cells.iter().all(|row| row[x - 1] == row[x]))
            .collect::<Vec<_>>();
        for x in candidates {
            let size = min(x, self.cells[0].len() - x);
            if self
                .cells
                .iter()
                .all(|row| itertools::equal(&row[x - size..x], row[x..x + size].iter().rev()))
            {
                mirrors.push(Mirror::Vertical(x));
            }
        }

        let candidates = (1..self.cells.len())
            .filter(|&y| self.cells[y - 1] == self.cells[y])
            .collect::<Vec<_>>();
        for y in candidates {
            let size = min(y, self.cells.len() - y);
            if izip!(y - size..y, (y..y + size).rev()).all(|(i, j)| self.cells[i] == self.cells[j])
            {
                mirrors.push(Mirror::Horizontal(y));
            }
        }

        mirrors
    }
}

#[aoc_generator(day13)]
fn generator(input: &str) -> Vec<Pattern> {
    input
        .split("\n\n")
        .map(|pattern| {
            let cells = pattern
                .lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>();
            Pattern { cells }
        })
        .collect()
}

#[aoc(day13, part1)]
fn part1(input: &[Pattern]) -> usize {
    input
        .iter()
        .flat_map(Pattern::mirrors)
        .map(|mirror| mirror.value())
        .sum()
}

#[aoc(day13, part2)]
fn part2(input: &[Pattern]) -> usize {
    input
        .iter()
        .filter_map(|pattern| {
            let existing = pattern.mirrors();
            iproduct!(0..pattern.cells.len(), 0..pattern.cells[0].len()).find_map(move |(y, x)| {
                let mut next_pattern = pattern.clone();
                next_pattern.toggle(x, y);

                let mut next_mirrors = next_pattern.mirrors();
                next_mirrors.retain(|mirror| !existing.contains(mirror));
                next_mirrors.first().copied()
            })
        })
        .map(|mirror| mirror.value())
        .sum()
}
