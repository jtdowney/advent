use std::{
    collections::{HashMap, VecDeque},
    num::ParseIntError,
    ops::{Index, IndexMut},
};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

#[derive(Debug)]
struct Memory {
    inner: HashMap<usize, i64>,
}

impl From<&[i64]> for Memory {
    fn from(source: &[i64]) -> Memory {
        Memory {
            inner: source.iter().cloned().enumerate().collect(),
        }
    }
}

impl Index<usize> for Memory {
    type Output = i64;
    fn index(&self, index: usize) -> &Self::Output {
        self.inner.get(&index).unwrap_or(&0)
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.inner.entry(index).or_insert(0)
    }
}

#[derive(Debug)]
struct Computer {
    memory: Memory,
    ip: usize,
    rb: usize,
    inputs: VecDeque<i64>,
    outputs: VecDeque<i64>,
    halted: bool,
}

impl Computer {
    fn new(memory: &[i64]) -> Self {
        Self {
            memory: memory.into(),
            ip: 0,
            rb: 0,
            inputs: VecDeque::new(),
            outputs: VecDeque::new(),
            halted: false,
        }
    }

    fn read_opcode(instruction: i64) -> i64 {
        instruction % 100
    }

    fn read_mode(instruction: i64, position: usize) -> ParameterMode {
        match (instruction / 10_i64.pow(position as u32 + 2)) % 10 {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => unreachable!(),
        }
    }

    fn read_destination(&self, position: usize) -> usize {
        let instruction = self.memory[self.ip];
        match Self::read_mode(instruction, position) {
            ParameterMode::Position => self.memory[self.ip + 1 + position] as usize,
            ParameterMode::Immediate => unreachable!(),
            ParameterMode::Relative => {
                ((self.rb as i64) + self.memory[self.ip + 1 + position]) as usize
            }
        }
    }

    fn read_parameter(&self, position: usize) -> i64 {
        let instruction = self.memory[self.ip];
        let value = self.memory[self.ip + 1 + position];
        match Self::read_mode(instruction, position) {
            ParameterMode::Position => self.memory[value as usize],
            ParameterMode::Immediate => value,
            ParameterMode::Relative => self.memory[((self.rb as i64) + value) as usize],
        }
    }

    fn run(&mut self) -> bool {
        if self.halted {
            return false;
        }

        loop {
            let instruction = self.memory[self.ip];
            let opcode = Self::read_opcode(instruction);
            match opcode {
                1 => {
                    let left = self.read_parameter(0);
                    let right = self.read_parameter(1);
                    let destination = self.read_destination(2);
                    self.memory[destination] = left + right;
                    self.ip += 4;
                }
                2 => {
                    let left = self.read_parameter(0);
                    let right = self.read_parameter(1);
                    let destination = self.read_destination(2);
                    self.memory[destination] = left * right;
                    self.ip += 4;
                }
                3 => {
                    if self.inputs.is_empty() {
                        return true;
                    }
                    let destination = self.read_destination(0);
                    self.memory[destination] = self.inputs.pop_front().unwrap();
                    self.ip += 2;
                }
                4 => {
                    let value = self.read_parameter(0);
                    self.outputs.push_back(value);
                    self.ip += 2;
                    return true;
                }
                5 => {
                    let cond = self.read_parameter(0);
                    self.ip = if cond != 0 {
                        self.read_parameter(1) as usize
                    } else {
                        self.ip + 3
                    };
                }
                6 => {
                    let cond = self.read_parameter(0);
                    self.ip = if cond == 0 {
                        self.read_parameter(1) as usize
                    } else {
                        self.ip + 3
                    };
                }
                7 => {
                    let left = self.read_parameter(0);
                    let right = self.read_parameter(1);
                    let destination = self.read_destination(2);
                    self.memory[destination] = if left < right { 1 } else { 0 };
                    self.ip += 4;
                }
                8 => {
                    let left = self.read_parameter(0);
                    let right = self.read_parameter(1);
                    let destination = self.read_destination(2);
                    self.memory[destination] = if left == right { 1 } else { 0 };
                    self.ip += 4;
                }
                9 => {
                    self.rb = ((self.rb as i64) + self.read_parameter(0)) as usize;
                    self.ip += 2;
                }
                99 => {
                    self.halted = true;
                    return false;
                }
                _ => unreachable!("unknown opcode {}", opcode),
            }
        }
    }

    fn run_until_halt(&mut self) {
        while !self.halted {
            self.run();
        }
    }
}

#[aoc_generator(day19)]
fn generator(input: &str) -> Result<Vec<i64>, ParseIntError> {
    input.trim().split(',').map(|s| s.parse::<i64>()).collect()
}

fn is_in_beam(memory: &[i64], x: i64, y: i64) -> bool {
    let mut computer = Computer::new(memory);
    computer.inputs.push_back(x);
    computer.inputs.push_back(y);
    computer.run_until_halt();
    computer.outputs.pop_front().unwrap() == 1
}

#[aoc(day19, part1)]
fn part1(input: &[i64]) -> usize {
    (0..50)
        .cartesian_product(0..50)
        .filter(|&(x, y)| is_in_beam(input, x, y))
        .count()
}

#[aoc(day19, part2)]
fn part2(input: &[i64]) -> usize {
    let size = 100;

    (size..)
        .scan(0, |last_x, y| {
            let x = (*last_x..)
                .find(|&x| is_in_beam(input, x, y + size - 1))
                .unwrap_or(*last_x);
            *last_x = x;
            Some((x, y))
        })
        .find(|&(x, y)| is_in_beam(input, x + size - 1, y))
        .map(|(x, y)| (x as usize) * 10000 + (y as usize))
        .expect("Could not find suitable position")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tractor_beam_at_origin() {
        let input = include_str!("../input/2019/day19.txt");
        let memory = generator(input).unwrap();
        assert!(is_in_beam(&memory, 0, 0));
    }

    #[test]
    fn test_part1_solution() {
        let input = include_str!("../input/2019/day19.txt");
        let memory = generator(input).unwrap();
        let result = part1(&memory);
        assert!(result > 0 && result < 2500);
    }

    #[test]
    fn test_is_in_beam() {
        let input = include_str!("../input/2019/day19.txt");
        let memory = generator(input).unwrap();
        assert!(is_in_beam(&memory, 0, 0));
        assert!(!is_in_beam(&memory, 0, 100));
        assert!(!is_in_beam(&memory, 100, 0));
    }

    #[test]
    fn test_part2_solution() {
        let input = include_str!("../input/2019/day19.txt");
        let memory = generator(input).unwrap();
        let result = part2(&memory);
        assert!(result > 0);
        let y = result % 10000;
        let x = result / 10000;
        assert!(x > 0 && y > 0 && x < 10000 && y < 10000);
    }
}
