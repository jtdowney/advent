use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
    ops::{Index, IndexMut},
};

use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};

const PADDLE_Y: i64 = 21;

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
    halted: bool,
}

#[derive(Copy, Clone, PartialEq)]
enum ComputerResult {
    Output(i64),
    NeedInput,
    Halted,
}

impl Computer {
    fn new(memory: &[i64]) -> Computer {
        Computer {
            memory: memory.into(),
            ip: 0,
            rb: 0,
            inputs: VecDeque::new(),
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

    fn run(&mut self) -> ComputerResult {
        if self.halted {
            return ComputerResult::Halted;
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
                    if let Some(value) = self.inputs.pop_front() {
                        self.memory[destination] = value;
                        self.ip += 2;
                    } else {
                        return ComputerResult::NeedInput;
                    }
                }
                4 => {
                    let value = self.read_parameter(0);
                    self.ip += 2;
                    return ComputerResult::Output(value);
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
                    return ComputerResult::Halted;
                }
                _ => unreachable!("unknown opcode {}", opcode),
            }
        }
    }
}

type Point = (i64, i64);

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile {
    Blank = 0,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl From<i64> for Tile {
    fn from(source: i64) -> Self {
        match source {
            0 => Tile::Blank,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum PlayResult {
    Tile(Point, Tile),
    Score(i64),
    NeedInput,
    Halted,
}

fn play_game(game: &mut Computer) -> PlayResult {
    match game.run() {
        ComputerResult::Halted => PlayResult::Halted,
        ComputerResult::NeedInput => PlayResult::NeedInput,
        ComputerResult::Output(x) => {
            let y = match game.run() {
                ComputerResult::Halted => unreachable!(),
                ComputerResult::NeedInput => unreachable!(),
                ComputerResult::Output(y) => y,
            };

            let value = match game.run() {
                ComputerResult::Halted => unreachable!(),
                ComputerResult::NeedInput => unreachable!(),
                ComputerResult::Output(value) => value,
            };

            if x == -1 && y == 0 {
                PlayResult::Score(value)
            } else {
                PlayResult::Tile((x, y), value.into())
            }
        }
    }
}

fn find_paddle(grid: &HashMap<Point, Tile>) -> Option<Point> {
    grid.iter()
        .find(|&(_, t)| *t == Tile::Paddle)
        .map(|(&p, _)| p)
}

#[aoc_generator(day13)]
fn generate(input: &str) -> anyhow::Result<Vec<i64>> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse::<i64>().context("parse number"))
        .collect()
}

#[aoc(day13, part1)]
fn part1(memory: &[i64]) -> usize {
    let mut grid = HashMap::new();
    let mut game = Computer::new(memory);

    while let PlayResult::Tile(point, tile) = play_game(&mut game) {
        grid.insert(point, tile);
    }

    grid.values().filter(|&t| *t == Tile::Block).count()
}

#[aoc(day13, part2)]
fn part2(memory: &[i64]) -> i64 {
    let mut grid = HashMap::new();
    let mut game = Computer::new(memory);
    let mut score = 0;
    let mut ball_position: Option<Point> = None;
    let mut x_target = 0;

    game.memory[0] = 2;

    loop {
        match play_game(&mut game) {
            PlayResult::Tile(point, tile) => {
                grid.insert(point, tile);

                if tile == Tile::Ball {
                    if let Some(old_point) = ball_position {
                        x_target = if old_point.1 < point.1 {
                            let m = (old_point.1 - point.1) / (old_point.0 - point.0);
                            let b = point.1 - m * point.0;
                            (PADDLE_Y - b) / m
                        } else {
                            point.0
                        };

                        let (paddle_x, paddle_y) = find_paddle(&grid).unwrap();
                        if point.0 == paddle_x && point.1 == paddle_y - 1 {
                            x_target -= 1;
                        }
                    }

                    ball_position = Some(point);
                }
            }
            PlayResult::Score(value) => {
                score = value;
            }
            PlayResult::NeedInput => {
                let (x, _) = find_paddle(&grid).unwrap();
                match x.cmp(&x_target) {
                    Ordering::Greater => game.inputs.push_back(-1),
                    Ordering::Less => game.inputs.push_back(1),
                    Ordering::Equal => game.inputs.push_back(0),
                }
            }
            PlayResult::Halted => break,
        }
    }

    score
}
