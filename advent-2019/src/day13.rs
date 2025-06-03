use std::{cmp::Ordering, collections::HashMap};

use aoc_runner_derive::{aoc, aoc_generator};

use crate::intcode::{ComputerState, StepResult, parse_program, step};

const PADDLE_Y: i64 = 21;

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

fn play_game(state: &mut ComputerState) -> PlayResult {
    loop {
        let (new_state, result) = step(state.clone());
        *state = new_state;

        match result {
            StepResult::Output(x) => {
                let y = loop {
                    let (new_state, result) = step(state.clone());
                    *state = new_state;
                    match result {
                        StepResult::Output(y) => break y,
                        StepResult::Continue => continue,
                        _ => unreachable!(),
                    }
                };

                let value = loop {
                    let (new_state, result) = step(state.clone());
                    *state = new_state;
                    match result {
                        StepResult::Output(value) => break value,
                        StepResult::Continue => continue,
                        _ => unreachable!(),
                    }
                };

                if x == -1 && y == 0 {
                    return PlayResult::Score(value);
                } else {
                    return PlayResult::Tile((x, y), value.into());
                }
            }
            StepResult::NeedInput => return PlayResult::NeedInput,
            StepResult::Halted => return PlayResult::Halted,
            StepResult::Continue => continue,
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
    parse_program(input)
}

#[aoc(day13, part1)]
fn part1(program: &[i64]) -> usize {
    let mut grid = HashMap::new();
    let mut state = ComputerState::new(program);

    while let PlayResult::Tile(point, tile) = play_game(&mut state) {
        grid.insert(point, tile);
    }

    grid.values().filter(|&t| *t == Tile::Block).count()
}

#[aoc(day13, part2)]
fn part2(program: &[i64]) -> i64 {
    let mut grid = HashMap::new();
    let mut state = ComputerState::new(program);
    let mut score = 0;
    let mut ball_position: Option<Point> = None;
    let mut x_target = 0;

    state.memory[0] = 2;

    loop {
        match play_game(&mut state) {
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
                    Ordering::Greater => state.inputs.push_back(-1),
                    Ordering::Less => state.inputs.push_back(1),
                    Ordering::Equal => state.inputs.push_back(0),
                }
            }
            PlayResult::Halted => break,
        }
    }

    score
}
