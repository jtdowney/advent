use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

use anyhow::anyhow;
use aoc_runner_derive::{aoc, aoc_generator};
use winnow::{
    ascii::{dec_int, dec_uint, space1},
    combinator::{alt, delimited, repeat, separated, seq},
    prelude::*,
};

type Button = Vec<usize>;
type Matrix = Vec<Vec<i32>>;

#[derive(Debug)]
struct Machine {
    target_lights: Vec<bool>,
    buttons: Vec<Button>,
    joltage: Vec<i32>,
}

impl Machine {
    fn min_presses_lights(&self) -> u32 {
        let n = self.target_lights.len();
        let start = vec![false; n];

        let mut visited = HashSet::new();
        let mut search = VecDeque::new();
        visited.insert(start.clone());
        search.push_back((start, 0));

        while let Some((state, presses)) = search.pop_front() {
            if state == self.target_lights {
                return presses;
            }

            for button in &self.buttons {
                let next_state = state
                    .iter()
                    .enumerate()
                    .map(|(i, &on)| if button.contains(&i) { !on } else { on })
                    .collect::<Vec<_>>();

                if visited.insert(next_state.clone()) {
                    search.push_back((next_state, presses + 1));
                }
            }
        }

        unreachable!()
    }

    fn min_presses_joltage(&self) -> Option<i32> {
        let max_val = *self.joltage.iter().max()?;
        Solver::new(&self.buttons, &self.joltage).solve(max_val)
    }
}

struct Solver {
    matrix: Matrix,
    pivot_cols: Vec<usize>,
    free_cols: Vec<usize>,
    button_size: usize,
}

impl Solver {
    fn new(buttons: &[Button], joltage: &[i32]) -> Self {
        let button_size = buttons.len();
        let matrix = Self::build_matrix(buttons, joltage);
        let (matrix, pivot_cols) = Self::gaussian_elimination(matrix, button_size);
        let free_cols = (0..button_size)
            .filter(|c| !pivot_cols.contains(c))
            .collect();

        Self {
            matrix,
            pivot_cols,
            free_cols,
            button_size,
        }
    }

    fn build_matrix(buttons: &[Button], joltage: &[i32]) -> Matrix {
        let button_size = buttons.len();
        joltage
            .iter()
            .enumerate()
            .map(|(i, &j)| {
                let mut row = (0..button_size)
                    .map(|b| i32::from(buttons[b].contains(&i)))
                    .collect::<Vec<_>>();
                row.push(j);
                row
            })
            .collect()
    }

    fn gaussian_elimination(mut matrix: Matrix, button_size: usize) -> (Matrix, Vec<usize>) {
        let row_count = matrix.len();

        let (_, pivot_cols) =
            (0..button_size).fold((0, vec![]), |(pivot_row, mut pivot_cols), pivot_col| {
                let Some(swap_row) = (pivot_row..row_count).find(|&r| matrix[r][pivot_col] != 0)
                else {
                    return (pivot_row, pivot_cols);
                };

                matrix.swap(pivot_row, swap_row);
                pivot_cols.push(pivot_col);

                let pivot_val = matrix[pivot_row][pivot_col];
                let pivot_row_vals = matrix[pivot_row].clone();

                let rows = matrix
                    .iter_mut()
                    .skip(pivot_row + 1)
                    .filter(|row| row[pivot_col] != 0);
                for row in rows {
                    let factor = row[pivot_col];
                    for (cell, &pivot_cell) in row.iter_mut().zip(&pivot_row_vals) {
                        *cell = *cell * pivot_val - pivot_cell * factor;
                    }
                }

                (pivot_row + 1, pivot_cols)
            });

        (matrix, pivot_cols)
    }

    fn try_solve(&self, free_vals: &[i32]) -> Option<i32> {
        let mut solution = vec![0; self.button_size];
        for (&col, &val) in self.free_cols.iter().zip(free_vals) {
            solution[col] = val;
        }

        for (row, &pcol) in self.pivot_cols.iter().enumerate().rev() {
            let sum = self.matrix[row][self.button_size]
                - ((pcol + 1)..self.button_size)
                    .map(|c| self.matrix[row][c] * solution[c])
                    .sum::<i32>();

            if sum % self.matrix[row][pcol] != 0 {
                return None;
            }
            solution[pcol] = sum / self.matrix[row][pcol];
        }

        solution
            .iter()
            .all(|&x| x >= 0)
            .then(|| solution.iter().sum())
    }

    fn search(&self, free_vals: &[i32], max_val: i32, current_best: Option<i32>) -> Option<i32> {
        if free_vals.len() == self.free_cols.len() {
            return self.try_solve(free_vals);
        }

        let current_sum = free_vals.iter().sum::<i32>();
        (0..=max_val)
            .take_while(|&val| current_best.is_none_or(|b| current_sum + val < b))
            .fold(current_best, |best, val| {
                let mut next_vals = free_vals.to_vec();
                next_vals.push(val);

                let result = self.search(&next_vals, max_val, best);
                match (best, result) {
                    (Some(b), Some(r)) => Some(b.min(r)),
                    (None, r) => r,
                    (b, None) => b,
                }
            })
    }

    fn solve(&self, max_val: i32) -> Option<i32> {
        self.search(&[], max_val, None)
    }
}

impl FromStr for Machine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn machine(input: &mut &str) -> winnow::Result<Machine> {
            seq!(Machine {
                target_lights: target_lights,
                _: space1,
                buttons: buttons,
                _: space1,
                joltage: joltage,
            })
            .parse_next(input)
        }

        fn target_lights(input: &mut &str) -> winnow::Result<Vec<bool>> {
            delimited(
                "[",
                repeat(1.., alt((".".value(false), "#".value(true)))),
                "]",
            )
            .parse_next(input)
        }

        fn button(input: &mut &str) -> winnow::Result<Button> {
            delimited("(", separated(1.., dec_uint::<_, usize, _>, ","), ")").parse_next(input)
        }

        fn buttons(input: &mut &str) -> winnow::Result<Vec<Button>> {
            separated(1.., button, " ").parse_next(input)
        }

        fn joltage(input: &mut &str) -> winnow::Result<Vec<i32>> {
            delimited("{", separated(1.., dec_int::<_, i32, _>, ","), "}").parse_next(input)
        }

        machine.parse(s).map_err(|e| anyhow!("parse error:\n{e}"))
    }
}

#[aoc_generator(day10)]
fn generator(input: &str) -> anyhow::Result<Vec<Machine>> {
    input.lines().map(str::parse).collect()
}

#[aoc(day10, part1)]
fn part1(input: &[Machine]) -> u32 {
    input.iter().map(Machine::min_presses_lights).sum()
}

#[aoc(day10, part2)]
fn part2(input: &[Machine]) -> Option<i32> {
    input.iter().map(Machine::min_presses_joltage).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_machine() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let machine = input.parse::<Machine>().unwrap();
        assert_eq!(machine.target_lights, vec![false, true, true, false]);
        assert_eq!(
            machine.buttons,
            vec![
                vec![3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![0, 2],
                vec![0, 1]
            ]
        );
        assert_eq!(machine.joltage, vec![3, 5, 4, 7]);
    }
}
