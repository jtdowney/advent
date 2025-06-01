use std::{
    collections::{HashMap, VecDeque},
    ops::{Index, IndexMut},
};

use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};

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
        let inner = source
            .iter()
            .cloned()
            .enumerate()
            .collect::<HashMap<usize, i64>>();
        Memory { inner }
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
    fn new(memory: &[i64]) -> Computer {
        Computer {
            memory: memory.into(),
            ip: 0,
            rb: 0,
            inputs: VecDeque::new(),
            outputs: VecDeque::new(),
            halted: false,
        }
    }

    fn read_opcode(instruction: i64) -> i64 {
        let instruction_string = instruction.to_string();
        let length = instruction_string.len();
        if length <= 2 {
            instruction
        } else {
            instruction.to_string()[length - 2..].parse().unwrap()
        }
    }

    fn read_mode(instruction: i64, position: usize) -> ParameterMode {
        let instruction_string = instruction.to_string();
        let length = instruction_string.len();
        let offset = 3 + position;
        if length < offset {
            ParameterMode::Position
        } else {
            let offset = length - offset;
            let mode = &instruction.to_string()[offset..=offset];
            match mode {
                "0" => ParameterMode::Position,
                "1" => ParameterMode::Immediate,
                "2" => ParameterMode::Relative,
                _ => unreachable!(),
            }
        }
    }

    fn read_destination(&self, position: usize) -> usize {
        let instruction = self.memory[self.ip];
        match Computer::read_mode(instruction, position) {
            ParameterMode::Position => self.memory[self.ip + 1 + position] as usize,
            ParameterMode::Immediate => unreachable!(),
            ParameterMode::Relative => {
                let source = self.ip + position + 1;
                let base = self.rb as i64;
                let offset = self.memory[source];
                let address = base + offset;
                address as usize
            }
        }
    }

    fn read_parameter(&self, position: usize) -> i64 {
        let instruction = self.memory[self.ip];
        let source = self.ip + position + 1;
        match Computer::read_mode(instruction, position) {
            ParameterMode::Position => self.memory[self.memory[source] as usize],
            ParameterMode::Immediate => self.memory[source],
            ParameterMode::Relative => {
                let base = self.rb as i64;
                let offset = self.memory[source];
                let address = base + offset;
                self.memory[address as usize]
            }
        }
    }

    fn run(&mut self) {
        if self.halted {
            panic!("halted");
        }

        loop {
            let instruction = self.memory[self.ip];
            let opcode = Computer::read_opcode(instruction);
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
                    let destination = self.read_destination(0);
                    self.memory[destination] = self.inputs.pop_front().expect("no more inputs");
                    self.ip += 2;
                }
                4 => {
                    let value = self.read_parameter(0);
                    self.outputs.push_back(value);
                    self.ip += 2;
                    return;
                }
                5 => {
                    let cond = self.read_parameter(0);
                    if cond != 0 {
                        self.ip = self.read_parameter(1) as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                6 => {
                    let cond = self.read_parameter(0);
                    if cond == 0 {
                        self.ip = self.read_parameter(1) as usize;
                    } else {
                        self.ip += 3;
                    }
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
                    let base = self.rb as i64;
                    let offset = self.read_parameter(0);
                    self.rb = (base + offset) as usize;
                    self.ip += 2;
                }
                99 => {
                    self.halted = true;
                    return;
                }
                _ => unreachable!("unknown opcode {}", opcode),
            }
        }
    }
}

#[aoc_generator(day9)]
fn generate(input: &str) -> anyhow::Result<Vec<i64>> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse::<i64>().context("parse number"))
        .collect()
}

#[aoc(day9, part1)]
fn part1(memory: &[i64]) -> i64 {
    let mut computer = Computer::new(memory);
    computer.inputs.push_back(1);

    while !computer.halted {
        computer.run();
    }

    computer.outputs.pop_front().unwrap()
}

#[aoc(day9, part2)]
fn part2(memory: &[i64]) -> i64 {
    let mut computer = Computer::new(memory);
    computer.inputs.push_back(2);

    while !computer.halted {
        computer.run();
    }

    computer.outputs.pop_front().unwrap()
}
