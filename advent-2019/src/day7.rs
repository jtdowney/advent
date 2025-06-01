use std::{collections::VecDeque, iter};

use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

enum ParameterMode {
    Position,
    Immediate,
}

#[derive(Debug)]
struct Amplifier {
    memory: Vec<isize>,
    ip: usize,
    inputs: VecDeque<isize>,
    outputs: VecDeque<isize>,
    halted: bool,
}

impl Amplifier {
    fn new(memory: &[isize]) -> Amplifier {
        Amplifier {
            memory: memory.to_vec(),
            ip: 0,
            inputs: VecDeque::new(),
            outputs: VecDeque::new(),
            halted: false,
        }
    }

    fn read_opcode(instruction: isize) -> isize {
        let instruction_string = instruction.to_string();
        let length = instruction_string.len();
        if length <= 2 {
            instruction
        } else {
            instruction.to_string()[length - 2..].parse().unwrap()
        }
    }

    fn read_mode(instruction: isize, position: usize) -> ParameterMode {
        let instruction_string = instruction.to_string();
        let length = instruction_string.len();
        let offset = 3 + position;
        if length < offset {
            ParameterMode::Position
        } else {
            let offset = length - offset;
            let mode = instruction.to_string()[offset..=offset].parse().unwrap();
            match mode {
                0 => ParameterMode::Position,
                1 => ParameterMode::Immediate,
                _ => unreachable!(),
            }
        }
    }

    fn read_destination(&self, position: usize) -> usize {
        self.memory[self.ip + 1 + position] as usize
    }

    fn read_parameter(&self, position: usize) -> isize {
        let instruction = self.memory[self.ip];
        let source = self.ip + position + 1;
        match Amplifier::read_mode(instruction, position) {
            ParameterMode::Position => self.memory[self.memory[source] as usize],
            ParameterMode::Immediate => self.memory[source],
        }
    }

    fn run(&mut self) {
        if self.halted {
            panic!("halted");
        }

        loop {
            let instruction = self.memory[self.ip];
            let opcode = Amplifier::read_opcode(instruction);
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
                99 => {
                    self.halted = true;
                    return;
                }
                _ => unreachable!("unknown opcode {} state {:?}", opcode, self),
            }
        }
    }
}

#[aoc_generator(day7)]
fn generate(input: &str) -> Result<Vec<isize>> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse::<isize>().context("Failed to parse number"))
        .collect()
}

#[aoc(day7, part1)]
fn part1(memory: &[isize]) -> isize {
    (0..=4)
        .permutations(5)
        .map(|phases| {
            phases.iter().fold(0, |acc, &phase| {
                let mut amp = Amplifier::new(memory);
                amp.inputs.push_back(phase);
                amp.inputs.push_back(acc);
                amp.run();
                amp.outputs.pop_front().expect("output")
            })
        })
        .max()
        .unwrap()
}

#[aoc(day7, part2)]
fn part2(memory: &[isize]) -> isize {
    (5..=9)
        .permutations(5)
        .map(|phases| {
            let mut amps = phases
                .iter()
                .map(|&phase| {
                    let mut amp = Amplifier::new(memory);
                    amp.inputs.push_back(phase);
                    amp
                })
                .collect::<Vec<Amplifier>>();
            iter::successors(Some(0), |&input| {
                phases.iter().enumerate().try_fold(input, |acc, (i, _)| {
                    amps[i].inputs.push_back(acc);
                    amps[i].run();
                    if amps[i].halted {
                        None
                    } else {
                        Some(amps[i].outputs.pop_front().expect("output"))
                    }
                })
            })
            .last()
            .unwrap()
        })
        .max()
        .unwrap()
}
