use std::{cmp, collections::HashMap};

use aoc_runner_derive::{aoc, aoc_generator};

use crate::intcode::{ComputerState, StepResult, parse_program, step};

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
    state: ComputerState,
    direction: Direction,
    location: Point,
}

impl Robot {
    fn new(program: &[i64]) -> Robot {
        Robot {
            state: ComputerState::new(program),
            direction: Direction::Up,
            location: (0, 0),
        }
    }

    fn run_robot(&mut self, current_color: Color) -> Option<(Color, Direction)> {
        self.state.inputs.push_back(current_color.into());

        // Get color output
        let color = loop {
            let (new_state, result) = step(self.state.clone());
            self.state = new_state;

            match result {
                StepResult::Output(value) => break value,
                StepResult::Continue => continue,
                _ => return None,
            }
        };

        // Get turn output
        let turn = loop {
            let (new_state, result) = step(self.state.clone());
            self.state = new_state;

            match result {
                StepResult::Output(value) => break value,
                StepResult::Continue => continue,
                _ => return None,
            }
        };

        let new_color = color.into();
        let new_direction = match turn {
            0 => self.direction.turn_left(),
            1 => self.direction.turn_right(),
            _ => unreachable!(),
        };
        self.direction = new_direction;
        self.move_forward();
        Some((new_color, new_direction))
    }

    fn move_forward(&mut self) {
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
    parse_program(input)
}

#[aoc(day11, part1)]
fn part1(program: &[i64]) -> usize {
    let mut robot = Robot::new(program);
    let mut grid = HashMap::new();

    loop {
        let current_location = robot.location;
        let current_color = *grid.get(&current_location).unwrap_or(&Color::Black);
        if let Some((next_color, _)) = robot.run_robot(current_color) {
            grid.insert(current_location, next_color);
        } else {
            break;
        }
    }

    grid.len()
}

#[aoc(day11, part2)]
fn part2(program: &[i64]) -> String {
    let mut robot = Robot::new(program);
    let mut grid = HashMap::new();
    grid.insert((0, 0), Color::White);

    loop {
        let current_location = robot.location;
        let current_color = *grid.get(&current_location).unwrap_or(&Color::Black);
        if let Some((next_color, _)) = robot.run_robot(current_color) {
            grid.insert(current_location, next_color);
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
