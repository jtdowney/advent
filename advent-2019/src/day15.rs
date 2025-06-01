use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::{Index, IndexMut},
};

use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};

enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
        instruction % 100
    }

    fn read_mode(instruction: i64, position: usize) -> ParameterMode {
        let mode = (instruction / 10_i64.pow(position as u32 + 2)) % 10;
        match mode {
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
                let source = self.ip + position + 1;
                let base = self.rb as i64;
                let offset = self.memory[source];
                (base + offset) as usize
            }
        }
    }

    fn read_parameter(&self, position: usize) -> i64 {
        let instruction = self.memory[self.ip];
        let source = self.ip + position + 1;
        match Self::read_mode(instruction, position) {
            ParameterMode::Position => self.memory[self.memory[source] as usize],
            ParameterMode::Immediate => self.memory[source],
            ParameterMode::Relative => {
                let base = self.rb as i64;
                let offset = self.memory[source];
                self.memory[(base + offset) as usize]
            }
        }
    }

    fn run(&mut self) {
        if self.halted {
            panic!("halted");
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

type Point = (i32, i32);

#[derive(Clone)]
struct State {
    computer: Computer,
    position: Point,
    steps: usize,
}

fn get_next_position(position: Point, direction: i64) -> Point {
    let (x, y) = position;
    match direction {
        1 => (x, y - 1),
        2 => (x, y + 1),
        3 => (x - 1, y),
        4 => (x + 1, y),
        _ => unreachable!(),
    }
}

fn try_move(mut computer: Computer, direction: i64) -> Option<(Computer, i64)> {
    computer.inputs.push_back(direction);
    computer.run();
    computer
        .outputs
        .pop_front()
        .map(|status| (computer, status))
}

fn explore_all_directions<F>(state: State, mut process_result: F)
where
    F: FnMut(Point, Computer, i64),
{
    (1..=4).for_each(|direction| {
        let next_pos = get_next_position(state.position, direction);
        if let Some((next_computer, status)) = try_move(state.computer.clone(), direction) {
            process_result(next_pos, next_computer, status);
        }
    });
}

#[aoc_generator(day15)]
fn generate(input: &str) -> anyhow::Result<Vec<i64>> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse::<i64>().context("parse number"))
        .collect()
}

#[aoc(day15, part1)]
fn part1(input: &[i64]) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    let initial_state = State {
        computer: Computer::new(input),
        position: (0, 0),
        steps: 0,
    };

    queue.push_back(initial_state);
    visited.insert((0, 0));

    while let Some(state) = queue.pop_front() {
        for direction in 1..=4 {
            let next_pos = get_next_position(state.position, direction);

            if visited.contains(&next_pos) {
                continue;
            }

            if let Some((next_computer, status)) = try_move(state.computer.clone(), direction) {
                match status {
                    0 => {
                        visited.insert(next_pos);
                    }
                    1 => {
                        visited.insert(next_pos);
                        queue.push_back(State {
                            computer: next_computer,
                            position: next_pos,
                            steps: state.steps + 1,
                        });
                    }
                    2 => {
                        return state.steps + 1;
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    panic!("Could not find oxygen system");
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Wall,
    Open,
    OxygenSystem,
}

fn explore_map(memory: &[i64]) -> (HashMap<Point, Tile>, Point) {
    let mut map = HashMap::new();
    let mut queue = VecDeque::new();
    let mut oxygen_position = (0, 0);

    let initial_state = State {
        computer: Computer::new(memory),
        position: (0, 0),
        steps: 0,
    };

    map.insert((0, 0), Tile::Open);
    queue.push_back(initial_state);

    while let Some(state) = queue.pop_front() {
        explore_all_directions(state, |next_pos, next_computer, status| {
            if map.contains_key(&next_pos) {
                return;
            }

            let tile = match status {
                0 => Tile::Wall,
                1 => Tile::Open,
                2 => Tile::OxygenSystem,
                _ => unreachable!(),
            };

            map.insert(next_pos, tile);

            if status != 0 {
                if status == 2 {
                    oxygen_position = next_pos;
                }
                queue.push_back(State {
                    computer: next_computer,
                    position: next_pos,
                    steps: 0,
                });
            }
        });
    }

    (map, oxygen_position)
}

fn simulate_oxygen_spread(map: &HashMap<Point, Tile>, oxygen_start: Point) -> usize {
    let mut oxygen_locations = HashSet::new();
    let mut queue = VecDeque::new();

    oxygen_locations.insert(oxygen_start);
    queue.push_back((oxygen_start, 0));

    let mut max_time = 0;

    while let Some((position, time)) = queue.pop_front() {
        max_time = max_time.max(time);

        let neighbors = (1..=4)
            .map(|dir| get_next_position(position, dir))
            .filter(|pos| !oxygen_locations.contains(pos))
            .filter(|pos| map.get(pos).is_some_and(|&tile| tile != Tile::Wall))
            .collect::<Vec<_>>();

        for next_pos in neighbors {
            oxygen_locations.insert(next_pos);
            queue.push_back((next_pos, time + 1));
        }
    }

    max_time
}

#[aoc(day15, part2)]
fn part2(input: &[i64]) -> usize {
    let (map, oxygen_position) = explore_map(input);
    simulate_oxygen_spread(&map, oxygen_position)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_next_position() {
        let pos = (0, 0);
        assert_eq!(get_next_position(pos, 1), (0, -1));
        assert_eq!(get_next_position(pos, 2), (0, 1));
        assert_eq!(get_next_position(pos, 3), (-1, 0));
        assert_eq!(get_next_position(pos, 4), (1, 0));
    }

    #[test]
    fn test_simulate_oxygen_spread_simple() {
        let mut map = HashMap::new();
        map.insert((0, 0), Tile::OxygenSystem);
        map.insert((0, -1), Tile::Wall);
        map.insert((0, 1), Tile::Wall);
        map.insert((-1, 0), Tile::Wall);
        map.insert((1, 0), Tile::Wall);

        assert_eq!(simulate_oxygen_spread(&map, (0, 0)), 0);
    }

    #[test]
    fn test_simulate_oxygen_spread_line() {
        let mut map = HashMap::new();
        map.insert((0, 0), Tile::OxygenSystem);
        map.insert((-1, 0), Tile::Open);
        map.insert((-2, 0), Tile::Open);
        map.insert((1, 0), Tile::Open);
        map.insert((2, 0), Tile::Open);

        assert_eq!(simulate_oxygen_spread(&map, (0, 0)), 2);
    }

    #[test]
    fn test_simulate_oxygen_spread_example() {
        let mut map = HashMap::new();

        map.insert((-1, -3), Tile::Wall);
        map.insert((0, -3), Tile::Wall);

        map.insert((-2, -2), Tile::Wall);
        map.insert((-1, -2), Tile::Open);
        map.insert((0, -2), Tile::Open);
        map.insert((1, -2), Tile::Wall);
        map.insert((2, -2), Tile::Wall);

        map.insert((-2, -1), Tile::Wall);
        map.insert((-1, -1), Tile::Open);
        map.insert((0, -1), Tile::Wall);
        map.insert((1, -1), Tile::Open);
        map.insert((2, -1), Tile::Open);
        map.insert((3, -1), Tile::Wall);

        map.insert((-2, 0), Tile::Wall);
        map.insert((-1, 0), Tile::Open);
        map.insert((0, 0), Tile::OxygenSystem);
        map.insert((1, 0), Tile::Open);
        map.insert((2, 0), Tile::Wall);

        map.insert((-1, 1), Tile::Wall);
        map.insert((0, 1), Tile::Wall);
        map.insert((1, 1), Tile::Wall);

        assert_eq!(simulate_oxygen_spread(&map, (0, 0)), 4);
    }
}
