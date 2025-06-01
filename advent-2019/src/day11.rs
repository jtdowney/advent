use std::{
    cmp,
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

#[derive(Clone, Copy, Debug, PartialEq)]
enum Color {
    White,
    Black,
}

impl From<i64> for Color {
    fn from(source: i64) -> Color {
        match source {
            0 => Color::Black,
            1 => Color::White,
            _ => unreachable!(),
        }
    }
}

impl From<Color> for i64 {
    fn from(color: Color) -> i64 {
        match color {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_left(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Down,
        }
    }
}

type Point = (i32, i32);

struct Robot {
    computer: Computer,
    direction: Direction,
    location: Point,
}

impl Robot {
    fn new(memory: &[i64]) -> Robot {
        Robot {
            computer: Computer::new(memory),
            direction: Direction::Up,
            location: (0, 0),
        }
    }

    fn is_halted(&self) -> bool {
        self.computer.halted
    }

    fn next_color(&mut self, current: Color) -> Option<Color> {
        self.computer.inputs.push_back(current.into());
        self.computer.run();
        if self.is_halted() {
            return None;
        }

        let color_value = self.computer.outputs.pop_front().expect("new color");
        Some(color_value.into())
    }

    fn move_forward(&mut self) {
        self.computer.run();
        let direction_value = self.computer.outputs.pop_front().expect("new direction");
        self.direction = match direction_value {
            0 => self.direction.turn_left(),
            1 => self.direction.turn_right(),
            _ => unreachable!(),
        };

        self.location = match self.direction {
            Direction::Up => (self.location.0, self.location.1 - 1),
            Direction::Left => (self.location.0 - 1, self.location.1),
            Direction::Down => (self.location.0, self.location.1 + 1),
            Direction::Right => (self.location.0 + 1, self.location.1),
        };
    }
}

#[aoc_generator(day11)]
fn generate(input: &str) -> anyhow::Result<Vec<i64>> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse::<i64>().context("parse number"))
        .collect()
}

#[aoc(day11, part1)]
fn part1(memory: &[i64]) -> usize {
    let mut robot = Robot::new(memory);
    let mut grid = HashMap::new();

    loop {
        if robot.is_halted() {
            break;
        }

        let current_color = *grid.get(&robot.location).unwrap_or(&Color::Black);
        if let Some(next_color) = robot.next_color(current_color) {
            *grid.entry(robot.location).or_insert(Color::Black) = next_color;
            robot.move_forward();
        } else {
            break;
        }
    }

    grid.len()
}

#[aoc(day11, part2)]
fn part2(memory: &[i64]) -> String {
    let mut robot = Robot::new(memory);
    let mut grid = HashMap::new();
    grid.insert((0, 0), Color::White);

    loop {
        if robot.is_halted() {
            break;
        }

        let current_color = *grid.get(&robot.location).unwrap_or(&Color::Black);
        if let Some(next_color) = robot.next_color(current_color) {
            *grid.entry(robot.location).or_insert(Color::Black) = next_color;
            robot.move_forward();
        } else {
            break;
        }
    }

    let (top_left, bottom_right) = grid.keys().fold(
        ((i32::MAX, i32::MAX), (i32::MIN, i32::MIN)),
        |((x_min, y_min), (x_max, y_max)), &(x, y)| {
            (
                (cmp::min(x_min, x), cmp::min(y_min, y)),
                (cmp::max(x_max, x), cmp::max(y_max, y)),
            )
        },
    );

    let mut result = String::new();
    result.push('\n');
    for y in top_left.1..=bottom_right.1 {
        for x in top_left.0..=bottom_right.0 {
            match grid.get(&(x, y)).unwrap_or(&Color::Black) {
                Color::Black => result.push(' '),
                Color::White => result.push('█'),
            }
        }
        result.push('\n');
    }

    result
}
