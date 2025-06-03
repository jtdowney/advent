use aoc_runner_derive::{aoc, aoc_generator};

use crate::intcode::{parse_program, run_with_inputs};

fn check_position(program: &[i64], x: i64, y: i64) -> bool {
    let outputs = run_with_inputs(program, &[x, y]);
    !outputs.is_empty() && outputs[0] == 1
}

#[aoc_generator(day19)]
fn generate(input: &str) -> anyhow::Result<Vec<i64>> {
    parse_program(input)
}

#[aoc(day19, part1)]
fn part1(program: &[i64]) -> usize {
    let mut count = 0;
    for y in 0..50 {
        for x in 0..50 {
            if check_position(program, x, y) {
                count += 1;
            }
        }
    }
    count
}

#[aoc(day19, part2)]
fn part2(program: &[i64]) -> i64 {
    let mut x = 0;
    let mut y = 100;

    loop {
        while !check_position(program, x, y) {
            x += 1;
        }

        if check_position(program, x + 99, y - 99) {
            return 10000 * x + (y - 99);
        }

        y += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_in_beam() {
        let program = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        assert!(!check_position(&program, 0, 0));
    }

    #[test]
    fn test_tractor_beam_at_origin() {
        let program = vec![3, 9, 1, 9, 10, 10, 4, 10, 99, 0, 0];
        assert!(!check_position(&program, 0, 0));
    }
}
