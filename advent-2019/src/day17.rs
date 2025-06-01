use std::{
    collections::{HashMap, VecDeque},
    num::ParseIntError,
    ops::{Index, IndexMut},
};

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
        match Computer::read_mode(instruction, position) {
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
        match Computer::read_mode(instruction, position) {
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

    fn run_until_prompt(&mut self, prompt_char: char) -> String {
        let mut output_chars = Vec::new();
        loop {
            self.run();
            while let Some(output) = self.outputs.pop_front() {
                output_chars.push(output as u8 as char);
            }
            if self.halted || output_chars.last() == Some(&prompt_char) {
                break;
            }
        }

        output_chars.into_iter().collect()
    }

    fn send_input(&mut self, input: &str) {
        input
            .chars()
            .for_each(|ch| self.inputs.push_back(ch as i64));
    }
}

#[aoc_generator(day17)]
fn parse(input: &str) -> Result<Vec<i64>, ParseIntError> {
    input.trim().split(',').map(|s| s.parse::<i64>()).collect()
}

#[aoc(day17, part1)]
fn part1(memory: &[i64]) -> i32 {
    let mut computer = Computer::new(memory);
    computer.run_until_halt();

    let grid: Vec<Vec<char>> = computer
        .outputs
        .iter()
        .map(|&code| code as u8 as char)
        .collect::<String>()
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let height = grid.len();
    let width = grid[0].len();

    (1..height - 1)
        .flat_map(|y| (1..width - 1).map(move |x| (x, y)))
        .filter(|&(x, y)| {
            grid[y][x] == '#'
                && grid[y - 1][x] == '#'
                && grid[y + 1][x] == '#'
                && grid[y][x - 1] == '#'
                && grid[y][x + 1] == '#'
        })
        .map(|(x, y)| x * y)
        .sum::<usize>() as i32
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn offset(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Movement {
    Left,
    Right,
    Forward(usize),
}

impl std::fmt::Display for Movement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Movement::Left => write!(f, "L"),
            Movement::Right => write!(f, "R"),
            Movement::Forward(n) => write!(f, "{}", n),
        }
    }
}

fn find_robot(grid: &[Vec<char>]) -> Option<((i32, i32), Direction)> {
    grid.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, &ch)| match ch {
                '^' => Some(((x as i32, y as i32), Direction::Up)),
                'v' => Some(((x as i32, y as i32), Direction::Down)),
                '<' => Some(((x as i32, y as i32), Direction::Left)),
                '>' => Some(((x as i32, y as i32), Direction::Right)),
                _ => None,
            })
        })
        .next()
}

fn is_scaffold(grid: &[Vec<char>], pos: (i32, i32)) -> bool {
    let (x, y) = pos;
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    x >= 0 && x < width && y >= 0 && y < height && grid[y as usize][x as usize] == '#'
}

fn count_forward_steps(
    grid: &[Vec<char>],
    start_pos: (i32, i32),
    dir: Direction,
) -> (usize, (i32, i32)) {
    let (dx, dy) = dir.offset();
    (0..)
        .map(|i| {
            let new_pos = (start_pos.0 + dx * (i + 1), start_pos.1 + dy * (i + 1));
            (i, new_pos)
        })
        .take_while(|&(_, pos)| is_scaffold(grid, pos))
        .last()
        .map(|(steps, pos)| ((steps + 1) as usize, pos))
        .unwrap_or((0, start_pos))
}

fn find_path(grid: &[Vec<char>]) -> Vec<Movement> {
    let (mut pos, mut dir) = find_robot(grid).expect("Robot not found");
    let mut path = Vec::new();

    loop {
        let (steps, new_pos) = count_forward_steps(grid, pos, dir);

        if steps > 0 {
            path.push(Movement::Forward(steps));
            pos = new_pos;
        } else {
            let left_dir = dir.turn_left();
            let right_dir = dir.turn_right();
            let (ldx, ldy) = left_dir.offset();
            let (rdx, rdy) = right_dir.offset();

            if is_scaffold(grid, (pos.0 + ldx, pos.1 + ldy)) {
                path.push(Movement::Left);
                dir = left_dir;
            } else if is_scaffold(grid, (pos.0 + rdx, pos.1 + rdy)) {
                path.push(Movement::Right);
                dir = right_dir;
            } else {
                break;
            }
        }
    }

    path
}

fn try_compress(moves: &[String], patterns: &[&[String]]) -> Option<Vec<String>> {
    if moves.is_empty() {
        return Some(Vec::new());
    }

    patterns.iter().enumerate().find_map(|(idx, pattern)| {
        if moves.starts_with(pattern) {
            try_compress(&moves[pattern.len()..], patterns).map(|mut result| {
                result.insert(0, ['A', 'B', 'C'][idx].to_string());
                result
            })
        } else {
            None
        }
    })
}

fn compress_path(path: &[Movement]) -> Option<(Vec<String>, String, String, String)> {
    let moves: Vec<String> = path.iter().map(|m| m.to_string()).collect();

    for a_len in 2..=10.min(moves.len()) {
        let a_pattern = &moves[0..a_len];
        let a_str = a_pattern.join(",");
        if a_str.len() > 20 {
            break;
        }

        for b_start in a_len..moves.len() {
            for b_len in 2..=10.min(moves.len() - b_start) {
                let b_pattern = &moves[b_start..b_start + b_len];
                let b_str = b_pattern.join(",");
                if b_str.len() > 20 || b_pattern == a_pattern {
                    continue;
                }

                for c_start in 0..moves.len() {
                    for c_len in 2..=10.min(moves.len() - c_start) {
                        let c_pattern = &moves[c_start..c_start + c_len];
                        let c_str = c_pattern.join(",");
                        if c_str.len() > 20 || c_pattern == a_pattern || c_pattern == b_pattern {
                            continue;
                        }

                        let patterns = vec![a_pattern, b_pattern, c_pattern];
                        if let Some(main_routine) = try_compress(&moves, &patterns) {
                            if main_routine.len() <= 10 && main_routine.join(",").len() <= 20 {
                                return Some((main_routine, a_str, b_str, c_str));
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

#[aoc(day17, part2)]
fn part2(memory: &[i64]) -> i64 {
    let mut memory = memory.to_vec();
    memory[0] = 2;

    let mut computer = Computer::new(&memory);

    let initial_output = computer.run_until_prompt(':');
    let map_end = initial_output.find("Main:").unwrap_or(initial_output.len());
    let grid: Vec<Vec<char>> = initial_output[..map_end]
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let path = find_path(&grid);
    let (main_routine, a_str, b_str, c_str) =
        compress_path(&path).expect("Could not compress path");

    computer.send_input(&(main_routine.join(",") + "\n"));
    computer.run_until_prompt(':');

    computer.send_input(&(a_str + "\n"));
    computer.run_until_prompt(':');

    computer.send_input(&(b_str + "\n"));
    computer.run_until_prompt(':');

    computer.send_input(&(c_str + "\n"));
    computer.run_until_prompt('?');

    computer.send_input("n\n");

    let mut last_output = 0;
    while !computer.halted {
        computer.run();
        computer
            .outputs
            .drain(..)
            .filter(|&output| output > 127)
            .for_each(|output| last_output = output);
    }

    last_output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let example = "..#..........
..#..........
#######...###
#.#...#...#.#
#############
..#...#...#..
..#####...^..";

        let grid: Vec<Vec<char>> = example.lines().map(|line| line.chars().collect()).collect();
        let height = grid.len();
        let width = grid[0].len();

        let sum: usize = (1..height - 1)
            .flat_map(|y| (1..width - 1).map(move |x| (x, y)))
            .filter(|&(x, y)| {
                grid[y][x] == '#'
                    && grid[y - 1][x] == '#'
                    && grid[y + 1][x] == '#'
                    && grid[y][x - 1] == '#'
                    && grid[y][x + 1] == '#'
            })
            .map(|(x, y)| x * y)
            .sum();

        assert_eq!(sum, 76);
    }

    #[test]
    fn test_path_finding() {
        let example = "#######...#####
#.....#...#...#
#.....#...#...#
......#...#...#
......#...###.#
......#.....#.#
^########...#.#
......#.#...#.#
......#########
........#...#..
....#########..
....#...#......
....#...#......
....#...#......
....#####......";

        let grid: Vec<Vec<char>> = example.lines().map(|line| line.chars().collect()).collect();

        let path = find_path(&grid);
        let moves: String = path
            .iter()
            .map(|m| m.to_string())
            .collect::<Vec<_>>()
            .join(",");

        eprintln!("Path: {}", moves);
        assert!(!path.is_empty());

        let result = compress_path(&path);
        assert!(result.is_some());
        let (main, a, b, c) = result.unwrap();
        eprintln!("Main: {}", main.join(","));
        eprintln!("A: {}", a);
        eprintln!("B: {}", b);
        eprintln!("C: {}", c);
    }
}
