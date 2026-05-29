use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

use anyhow::anyhow;
use aoc_runner_derive::{aoc, aoc_generator};
use winnow::{
    ascii::{dec_uint, space1},
    combinator::{alt, delimited, repeat, separated, seq},
    prelude::*,
};
use z3::{Optimize, SatResult, ast::Int};

type Button = Vec<usize>;

#[derive(Debug)]
struct Machine {
    target_lights: Vec<bool>,
    buttons: Vec<Button>,
    joltage: Vec<u64>,
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

    fn min_presses_joltage(&self) -> Option<u64> {
        let opt = Optimize::new();

        let button_vars = self
            .buttons
            .iter()
            .enumerate()
            .map(|(i, _)| Int::new_const(format!("b{i}")))
            .collect::<Vec<_>>();

        for var in &button_vars {
            opt.assert(&var.ge(Int::from_u64(0)));
        }

        for (counter_idx, &target) in self.joltage.iter().enumerate() {
            let sum = self
                .buttons
                .iter()
                .zip(&button_vars)
                .filter(|(button, _)| button.contains(&counter_idx))
                .map(|(_, var)| var.clone())
                .sum::<Int>();

            let target_val = Int::from_u64(target);
            opt.assert(&sum.eq(&target_val));
        }

        let total = button_vars.iter().cloned().sum::<Int>();
        opt.minimize(&total);

        match opt.check(&[]) {
            SatResult::Sat => {
                let model = opt.get_model()?;
                let result = model.eval(&total, false)?;
                result.as_u64()
            }
            _ => None,
        }
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

        fn joltage(input: &mut &str) -> winnow::Result<Vec<u64>> {
            delimited("{", separated(1.., dec_uint::<_, u64, _>, ","), "}").parse_next(input)
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
fn part2(input: &[Machine]) -> Option<u64> {
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
