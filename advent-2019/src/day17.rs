use aoc_runner_derive::{aoc, aoc_generator};

use crate::intcode::{
    ComputerState, ascii_to_codes, collect_ascii_output, parse_program, run_to_completion,
};

#[aoc_generator(day17)]
fn generate(input: &str) -> anyhow::Result<Vec<i64>> {
    parse_program(input)
}

#[aoc(day17, part1)]
fn part1(program: &[i64]) -> i32 {
    let state = ComputerState::new(program);
    let output = collect_ascii_output(state);

    let grid: Vec<Vec<char>> = output
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
    fn turn_left(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn delta(self) -> (i32, i32) {
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

fn find_robot(grid: &[Vec<char>]) -> ((usize, usize), Direction) {
    for (y, row) in grid.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            let direction = match ch {
                '^' => Some(Direction::Up),
                'v' => Some(Direction::Down),
                '<' => Some(Direction::Left),
                '>' => Some(Direction::Right),
                _ => None,
            };
            if let Some(dir) = direction {
                return ((x, y), dir);
            }
        }
    }
    panic!("Robot not found");
}

fn find_path(grid: &[Vec<char>]) -> Vec<Movement> {
    let mut path = Vec::new();
    let ((mut x, mut y), mut dir) = find_robot(grid);
    let height = grid.len();
    let width = grid[0].len();

    loop {
        // Try to move forward
        let (dx, dy) = dir.delta();
        let mut steps = 0;

        loop {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if nx < 0 || ny < 0 || nx >= width as i32 || ny >= height as i32 {
                break;
            }

            let nx = nx as usize;
            let ny = ny as usize;

            if grid[ny][nx] != '#' {
                break;
            }

            x = nx;
            y = ny;
            steps += 1;
        }

        if steps > 0 {
            path.push(Movement::Forward(steps));
        }

        // Try to turn
        let left_dir = dir.turn_left();
        let (ldx, ldy) = left_dir.delta();
        let lx = x as i32 + ldx;
        let ly = y as i32 + ldy;

        let right_dir = dir.turn_right();
        let (rdx, rdy) = right_dir.delta();
        let rx = x as i32 + rdx;
        let ry = y as i32 + rdy;

        if lx >= 0
            && ly >= 0
            && lx < width as i32
            && ly < height as i32
            && grid[ly as usize][lx as usize] == '#'
        {
            path.push(Movement::Left);
            dir = left_dir;
        } else if rx >= 0
            && ry >= 0
            && rx < width as i32
            && ry < height as i32
            && grid[ry as usize][rx as usize] == '#'
        {
            path.push(Movement::Right);
            dir = right_dir;
        } else {
            break;
        }
    }

    path
}

fn compress_path(path: &[Movement]) -> Option<(Vec<String>, String)> {
    // Try different function lengths
    for a_len in 1..=10 {
        for b_len in 1..=10 {
            for c_len in 1..=10 {
                let a = &path[0..a_len];

                // Find first occurrence of a pattern that's not A
                let mut b_start = None;
                let mut i = 0;
                while i < path.len() {
                    if i + a_len <= path.len() && &path[i..i + a_len] == a {
                        i += a_len;
                    } else {
                        b_start = Some(i);
                        break;
                    }
                }

                if let Some(b_start) = b_start {
                    if b_start + b_len > path.len() {
                        continue;
                    }
                    let b = &path[b_start..b_start + b_len];

                    // Find first occurrence of a pattern that's not A or B
                    let mut c_start = None;
                    let mut i = 0;
                    while i < path.len() {
                        if i + a_len <= path.len() && &path[i..i + a_len] == a {
                            i += a_len;
                        } else if i + b_len <= path.len() && &path[i..i + b_len] == b {
                            i += b_len;
                        } else {
                            c_start = Some(i);
                            break;
                        }
                    }

                    if let Some(c_start) = c_start {
                        if c_start + c_len > path.len() {
                            continue;
                        }
                        let c = &path[c_start..c_start + c_len];

                        // Check if we can represent the entire path using A, B, C
                        let mut main_routine = Vec::new();
                        let mut i = 0;
                        while i < path.len() {
                            if i + a_len <= path.len() && &path[i..i + a_len] == a {
                                main_routine.push('A');
                                i += a_len;
                            } else if i + b_len <= path.len() && &path[i..i + b_len] == b {
                                main_routine.push('B');
                                i += b_len;
                            } else if i + c_len <= path.len() && &path[i..i + c_len] == c {
                                main_routine.push('C');
                                i += c_len;
                            } else {
                                break;
                            }
                        }

                        if i == path.len() && main_routine.len() <= 10 {
                            let main = main_routine
                                .iter()
                                .map(|&ch| ch.to_string())
                                .collect::<Vec<_>>()
                                .join(",");

                            let format_function = |func: &[Movement]| {
                                func.iter()
                                    .map(|m| m.to_string())
                                    .collect::<Vec<_>>()
                                    .join(",")
                            };

                            let functions =
                                vec![format_function(a), format_function(b), format_function(c)];

                            // Check that all strings are short enough
                            if main.len() <= 20 && functions.iter().all(|f| f.len() <= 20) {
                                return Some((functions, main));
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
fn part2(program: &[i64]) -> i64 {
    // First, get the map
    let state = ComputerState::new(program);
    let initial_output = collect_ascii_output(state);

    let grid: Vec<Vec<char>> = initial_output
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    // Find the path
    let path = find_path(&grid);

    // Compress the path
    let (functions, main_routine) = compress_path(&path).expect("Could not compress path");

    // Run the robot
    let mut state = ComputerState::new(program);
    state.memory[0] = 2;

    // Prepare all inputs
    let mut all_input = String::new();
    all_input.push_str(&main_routine);
    all_input.push('\n');
    for func in &functions {
        all_input.push_str(func);
        all_input.push('\n');
    }
    all_input.push_str("n\n"); // Don't want video feed

    state.inputs.extend(ascii_to_codes(&all_input));

    let (_, outputs) = run_to_completion(state);

    // The last output should be the dust amount
    *outputs.last().unwrap()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn part1_example() {
        let map = r"..#..........
..#..........
#######...###
#.#...#...#.#
#############
..#...#...#..
..#####...^..";

        let _grid: Vec<Vec<char>> = map.lines().map(|line| line.chars().collect()).collect();

        let intersections = vec![(2, 2), (2, 4), (6, 4), (10, 4)];
        let sum: usize = intersections.iter().map(|(x, y)| x * y).sum();
        assert_eq!(sum, 76);
    }

    #[test]
    fn test_path_finding() {
        let map = r"#######...#####
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

        let grid: Vec<Vec<char>> = map.lines().map(|line| line.chars().collect()).collect();

        let path = find_path(&grid);

        // The path should visit all scaffolding tiles
        let mut visited = HashSet::new();
        let ((mut x, mut y), mut dir) = find_robot(&grid);
        visited.insert((x, y));

        for movement in &path {
            match movement {
                Movement::Left => dir = dir.turn_left(),
                Movement::Right => dir = dir.turn_right(),
                Movement::Forward(n) => {
                    let (dx, dy) = dir.delta();
                    for _ in 0..*n {
                        x = (x as i32 + dx) as usize;
                        y = (y as i32 + dy) as usize;
                        visited.insert((x, y));
                    }
                }
            }
        }

        // Count scaffolding in grid
        let scaffold_count = grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|&(_, &ch)| ch == '#' || ch == '^')
                    .map(move |(x, _)| (x, y))
            })
            .count();

        assert_eq!(visited.len(), scaffold_count);
    }
}
