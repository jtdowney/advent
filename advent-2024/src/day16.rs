use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotate_clockwise(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn rotate_counterclockwise(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    fn delta(self) -> (i32, i32) {
        match self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    pos: Position,
    dir: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    cost: u32,
    state: State,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Maze {
    grid: Vec<Vec<char>>,
    start: Position,
    end: Position,
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Forward,
    RotateClockwise,
    RotateCounterclockwise,
}

impl Move {
    fn apply(self, state: State, maze: &Maze) -> Option<(State, u32)> {
        match self {
            Move::Forward => self.apply_forward(state, maze),
            Move::RotateClockwise => Some((
                State {
                    pos: state.pos,
                    dir: state.dir.rotate_clockwise(),
                },
                1000,
            )),
            Move::RotateCounterclockwise => Some((
                State {
                    pos: state.pos,
                    dir: state.dir.rotate_counterclockwise(),
                },
                1000,
            )),
        }
    }

    fn apply_forward(self, state: State, maze: &Maze) -> Option<(State, u32)> {
        let (dr, dc) = state.dir.delta();
        let new_row = state.pos.row.saturating_add_signed(dr as isize);
        let new_col = state.pos.col.saturating_add_signed(dc as isize);

        let is_valid_position = new_row != usize::MAX
            && new_col != usize::MAX
            && new_row < maze.grid.len()
            && new_col < maze.grid[0].len()
            && maze.grid[new_row][new_col] != '#';

        if is_valid_position {
            Some((
                State {
                    pos: Position {
                        row: new_row,
                        col: new_col,
                    },
                    dir: state.dir,
                },
                1,
            ))
        } else {
            None
        }
    }
}

const MOVES: [Move; 3] = [
    Move::Forward,
    Move::RotateClockwise,
    Move::RotateCounterclockwise,
];

fn find_position(grid: &[Vec<char>], target: char) -> Option<Position> {
    grid.iter().enumerate().find_map(|(row, line)| {
        line.iter()
            .enumerate()
            .find(|(_, ch)| **ch == target)
            .map(|(col, _)| Position { row, col })
    })
}

#[aoc_generator(day16)]
fn generator(input: &str) -> Option<Maze> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let start = find_position(&grid, 'S')?;
    let end = find_position(&grid, 'E')?;

    Some(Maze { start, end, grid })
}

fn find_optimal_path_tiles(maze: &Maze) -> HashSet<Position> {
    let mut heap = BinaryHeap::new();
    let mut distances = HashMap::new();
    let mut predecessors: HashMap<State, Vec<State>> = HashMap::new();

    let initial_state = State {
        pos: maze.start,
        dir: Direction::East,
    };

    heap.push(Node {
        cost: 0,
        state: initial_state,
    });
    distances.insert(initial_state, 0);

    let mut min_end_cost = u32::MAX;
    let mut end_states = Vec::new();

    while let Some(Node { cost, state }) = heap.pop() {
        if let Some(&dist) = distances.get(&state) {
            if cost > dist {
                continue;
            }
        }

        if state.pos == maze.end {
            match cost.cmp(&min_end_cost) {
                Ordering::Less => {
                    min_end_cost = cost;
                    end_states.clear();
                    end_states.push(state);
                }
                Ordering::Equal => {
                    end_states.push(state);
                }
                Ordering::Greater => {}
            }
            continue;
        }

        for &move_type in &MOVES {
            if let Some((new_state, move_cost)) = move_type.apply(state, maze) {
                let new_cost = cost + move_cost;
                let current_best = *distances.get(&new_state).unwrap_or(&u32::MAX);

                match new_cost.cmp(&current_best) {
                    Ordering::Less => {
                        distances.insert(new_state, new_cost);
                        predecessors.insert(new_state, vec![state]);
                        heap.push(Node {
                            cost: new_cost,
                            state: new_state,
                        });
                    }
                    Ordering::Equal => {
                        predecessors.entry(new_state).or_default().push(state);
                    }
                    Ordering::Greater => {}
                }
            }
        }
    }

    let mut optimal_positions = HashSet::new();
    let mut queue = end_states.clone();
    let mut visited = end_states.into_iter().collect::<HashSet<_>>();

    while let Some(current_state) = queue.pop() {
        optimal_positions.insert(current_state.pos);

        if let Some(preds) = predecessors.get(&current_state) {
            for &pred in preds {
                if !visited.contains(&pred) {
                    visited.insert(pred);
                    queue.push(pred);
                }
            }
        }
    }

    optimal_positions
}

fn find_shortest_path(maze: &Maze) -> u32 {
    let mut heap = BinaryHeap::new();
    let mut distances = HashMap::new();

    let initial_state = State {
        pos: maze.start,
        dir: Direction::East,
    };

    heap.push(Node {
        cost: 0,
        state: initial_state,
    });
    distances.insert(initial_state, 0);

    while let Some(Node { cost, state }) = heap.pop() {
        if state.pos == maze.end {
            return cost;
        }

        if let Some(&dist) = distances.get(&state) {
            if cost > dist {
                continue;
            }
        }

        for &move_type in &MOVES {
            if let Some((new_state, move_cost)) = move_type.apply(state, maze) {
                let new_cost = cost + move_cost;

                if new_cost < *distances.get(&new_state).unwrap_or(&u32::MAX) {
                    distances.insert(new_state, new_cost);
                    heap.push(Node {
                        cost: new_cost,
                        state: new_state,
                    });
                }
            }
        }
    }

    u32::MAX
}

#[aoc(day16, part1)]
fn part1(maze: &Maze) -> u32 {
    find_shortest_path(maze)
}

#[aoc(day16, part2)]
fn part2(maze: &Maze) -> usize {
    find_optimal_path_tiles(maze).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let input = r"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

        let maze = generator(input).unwrap();
        assert_eq!(part1(&maze), 7036);
    }

    #[test]
    fn test_part1_example2() {
        let input = r"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

        let maze = generator(input).unwrap();
        assert_eq!(part1(&maze), 11048);
    }

    #[test]
    fn test_part2_example1() {
        let input = r"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

        let maze = generator(input).unwrap();
        assert_eq!(part2(&maze), 45);
    }

    #[test]
    fn test_part2_example2() {
        let input = r"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

        let maze = generator(input).unwrap();
        assert_eq!(part2(&maze), 64);
    }
}
