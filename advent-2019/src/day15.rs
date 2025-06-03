use std::collections::{HashMap, HashSet, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};

use crate::intcode::{ComputerState, parse_program, run_interactive};

type Point = (i32, i32);

#[derive(Clone)]
struct State {
    computer: ComputerState,
    position: Point,
    steps: usize,
}

fn get_next_position((x, y): Point, direction: i64) -> Point {
    match direction {
        1 => (x, y - 1),
        2 => (x, y + 1),
        3 => (x - 1, y),
        4 => (x + 1, y),
        _ => unreachable!(),
    }
}

fn try_move(computer: ComputerState, direction: i64) -> Option<(ComputerState, i64)> {
    let (new_computer, output) = run_interactive(computer, direction);
    output.map(|status| (new_computer, status))
}

fn explore_all_directions<F>(state: State, mut process_result: F)
where
    F: FnMut(Point, ComputerState, i64),
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
    parse_program(input)
}

#[aoc(day15, part1)]
fn part1(program: &[i64]) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    let initial_state = State {
        computer: ComputerState::new(program),
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

fn explore_map(program: &[i64]) -> (HashMap<Point, Tile>, Point) {
    let mut map = HashMap::new();
    let mut queue = VecDeque::new();
    let mut oxygen_position = (0, 0);

    let initial_state = State {
        computer: ComputerState::new(program),
        position: (0, 0),
        steps: 0,
    };

    queue.push_back(initial_state);
    map.insert((0, 0), Tile::Open);

    while let Some(state) = queue.pop_front() {
        explore_all_directions(state.clone(), |next_pos, next_computer, status| {
            if map.contains_key(&next_pos) {
                return;
            }

            match status {
                0 => {
                    map.insert(next_pos, Tile::Wall);
                }
                1 => {
                    map.insert(next_pos, Tile::Open);
                    queue.push_back(State {
                        computer: next_computer,
                        position: next_pos,
                        steps: 0,
                    });
                }
                2 => {
                    map.insert(next_pos, Tile::OxygenSystem);
                    oxygen_position = next_pos;
                    queue.push_back(State {
                        computer: next_computer,
                        position: next_pos,
                        steps: 0,
                    });
                }
                _ => unreachable!(),
            }
        });
    }

    (map, oxygen_position)
}

fn simulate_oxygen_spread(map: &HashMap<Point, Tile>, start: Point) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut max_minutes = 0;

    queue.push_back((start, 0));
    visited.insert(start);

    while let Some((position, minutes)) = queue.pop_front() {
        max_minutes = max_minutes.max(minutes);

        for direction in 1..=4 {
            let next_pos = get_next_position(position, direction);

            if visited.contains(&next_pos) {
                continue;
            }

            if let Some(&tile) = map.get(&next_pos) {
                if tile != Tile::Wall {
                    visited.insert(next_pos);
                    queue.push_back((next_pos, minutes + 1));
                }
            }
        }
    }

    max_minutes
}

#[aoc(day15, part2)]
fn part2(program: &[i64]) -> usize {
    let (map, oxygen_position) = explore_map(program);
    simulate_oxygen_spread(&map, oxygen_position)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_next_position() {
        assert_eq!(get_next_position((0, 0), 1), (0, -1));
        assert_eq!(get_next_position((0, 0), 2), (0, 1));
        assert_eq!(get_next_position((0, 0), 3), (-1, 0));
        assert_eq!(get_next_position((0, 0), 4), (1, 0));
    }

    #[test]
    fn test_simulate_oxygen_spread_simple() {
        let mut map = HashMap::new();
        map.insert((0, 0), Tile::OxygenSystem);
        map.insert((1, 0), Tile::Open);
        map.insert((0, 1), Tile::Open);
        map.insert((-1, 0), Tile::Wall);
        map.insert((0, -1), Tile::Wall);

        assert_eq!(simulate_oxygen_spread(&map, (0, 0)), 1);
    }

    #[test]
    fn test_simulate_oxygen_spread_line() {
        let mut map = HashMap::new();
        for i in 0..5 {
            map.insert((i, 0), Tile::Open);
        }
        map.insert((2, 0), Tile::OxygenSystem);

        assert_eq!(simulate_oxygen_spread(&map, (2, 0)), 2);
    }

    #[test]
    fn test_simulate_oxygen_spread_example() {
        let mut map = HashMap::new();

        // Create a small room
        for x in -2..=2 {
            for y in -2..=2 {
                map.insert((x, y), Tile::Open);
            }
        }

        // Add walls around it
        for x in -3..=3 {
            map.insert((x, -3), Tile::Wall);
            map.insert((x, 3), Tile::Wall);
        }
        for y in -3..=3 {
            map.insert((-3, y), Tile::Wall);
            map.insert((3, y), Tile::Wall);
        }

        // Oxygen starts in center
        map.insert((0, 0), Tile::OxygenSystem);

        assert_eq!(simulate_oxygen_spread(&map, (0, 0)), 4);
    }
}
