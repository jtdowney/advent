use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Point = (i32, i32);
type Map = HashMap<Point, Tile>;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Open,
    Wall,
}

#[derive(Debug, Clone, Copy)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug, Clone)]
enum Instruction {
    Move(usize),
    Turn(Turn),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Direction {
    fn turn(&self, turn: Turn) -> Self {
        let dirs = [
            Direction::Right,
            Direction::Down,
            Direction::Left,
            Direction::Up,
        ];
        let idx = *self as usize;
        let new_idx = match turn {
            Turn::Right => (idx + 1) % 4,
            Turn::Left => (idx + 3) % 4,
        };
        dirs[new_idx]
    }

    fn delta(&self) -> Point {
        match self {
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Up => (-1, 0),
        }
    }
}

#[aoc_generator(day22)]
fn generator(input: &str) -> anyhow::Result<(Map, Vec<Instruction>)> {
    let (map_str, path_str) = input.split_once("\n\n").unwrap();

    let map = map_str
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(col, ch)| match ch {
                    '.' => Some(((row as i32, col as i32), Tile::Open)),
                    '#' => Some(((row as i32, col as i32), Tile::Wall)),
                    _ => None,
                })
        })
        .collect();

    let mut instructions = Vec::new();
    let mut num_str = String::new();

    let flush_num = |num_str: &mut String, instructions: &mut Vec<Instruction>| {
        if !num_str.is_empty() {
            instructions.push(Instruction::Move(num_str.parse().unwrap()));
            num_str.clear();
        }
    };

    for ch in path_str.trim().chars() {
        match ch {
            '0'..='9' => num_str.push(ch),
            'L' => {
                flush_num(&mut num_str, &mut instructions);
                instructions.push(Instruction::Turn(Turn::Left));
            }
            'R' => {
                flush_num(&mut num_str, &mut instructions);
                instructions.push(Instruction::Turn(Turn::Right));
            }
            _ => unreachable!(),
        }
    }

    flush_num(&mut num_str, &mut instructions);
    Ok((map, instructions))
}

fn find_start(map: &Map) -> Point {
    let min_row = map.keys().map(|&(r, _)| r).min().unwrap();
    map.keys()
        .filter(|&&(r, _)| r == min_row)
        .filter(|&&pos| map[&pos] == Tile::Open)
        .map(|&(_, c)| (min_row, c))
        .min_by_key(|&(_, c)| c)
        .unwrap()
}

fn wrap_position(map: &Map, pos: Point, dir: Direction) -> Point {
    let (r, c) = pos;

    let filter_and_extract = |row_match: bool| {
        map.keys()
            .filter(move |&&(row, col)| if row_match { row == r } else { col == c })
            .map(move |&(row, col)| if row_match { col } else { row })
    };

    match dir {
        Direction::Right => (r, filter_and_extract(true).min().unwrap()),
        Direction::Left => (r, filter_and_extract(true).max().unwrap()),
        Direction::Down => (filter_and_extract(false).min().unwrap(), c),
        Direction::Up => (filter_and_extract(false).max().unwrap(), c),
    }
}

fn step(map: &Map, pos: Point, dir: Direction) -> Option<Point> {
    let (dr, dc) = dir.delta();
    let new_pos = (pos.0 + dr, pos.1 + dc);

    match map.get(&new_pos) {
        Some(Tile::Open) => Some(new_pos),
        Some(Tile::Wall) => None,
        None => {
            let wrapped = wrap_position(map, pos, dir);
            match map.get(&wrapped) {
                Some(Tile::Open) => Some(wrapped),
                Some(Tile::Wall) => None,
                None => unreachable!(),
            }
        }
    }
}

fn simulate<F>(map: &Map, instructions: &[Instruction], mut step_fn: F) -> i32
where
    F: FnMut(Point, Direction) -> Option<(Point, Direction)>,
{
    let (mut pos, mut dir) = (find_start(map), Direction::Right);

    for instruction in instructions {
        match instruction {
            Instruction::Move(dist) => {
                for _ in 0..*dist {
                    if let Some((new_pos, new_dir)) = step_fn(pos, dir) {
                        pos = new_pos;
                        dir = new_dir;
                    } else {
                        break;
                    }
                }
            }
            Instruction::Turn(turn) => dir = dir.turn(*turn),
        }
    }

    1000 * (pos.0 + 1) + 4 * (pos.1 + 1) + dir as i32
}

#[aoc(day22, part1)]
fn part1((map, instructions): &(Map, Vec<Instruction>)) -> i32 {
    simulate(map, instructions, |pos, dir| {
        step(map, pos, dir).map(|p| (p, dir))
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Face {
    id: usize,
    top_left: Point,
}

fn detect_faces(map: &Map) -> (Vec<Face>, i32) {
    let (min_row, max_row) = map.keys().map(|&(r, _)| r).minmax().into_option().unwrap();
    let (min_col, max_col) = map.keys().map(|&(_, c)| c).minmax().into_option().unwrap();

    let height = max_row - min_row + 1;
    let width = max_col - min_col + 1;
    let face_size = (height / 4).max(width / 4).min((height / 3).max(width / 3));

    let faces = (0..=max_row)
        .step_by(face_size as usize)
        .flat_map(|r| {
            (0..=max_col)
                .step_by(face_size as usize)
                .filter(move |&c| map.contains_key(&(r, c)))
                .map(move |c| (r, c))
        })
        .enumerate()
        .map(|(i, (r, c))| Face {
            id: i + 1,
            top_left: (r, c),
        })
        .collect();

    (faces, face_size)
}

fn get_face(faces: &[Face], face_size: i32, pos: Point) -> Option<&Face> {
    faces.iter().find(|face| {
        let (fr, fc) = face.top_left;
        pos.0 >= fr && pos.0 < fr + face_size && pos.1 >= fc && pos.1 < fc + face_size
    })
}

fn cube_wrap(
    map: &Map,
    faces: &[Face],
    face_size: i32,
    pos: Point,
    dir: Direction,
) -> (Point, Direction) {
    let current_face = get_face(faces, face_size, pos).unwrap();
    let (fr, fc) = current_face.top_left;
    let local_r = pos.0 - fr;
    let local_c = pos.1 - fc;

    #[derive(Clone, Copy)]
    enum EdgeMapping {
        Same(usize),
        Flip(usize),
    }

    use EdgeMapping::*;

    let transform_coord = |coord: i32, mapping: EdgeMapping| match mapping {
        Same(idx) => match idx {
            0 => coord,
            1 => face_size - 1 - coord,
            _ => unreachable!(),
        },
        Flip(idx) => match idx {
            0 => face_size - 1 - coord,
            1 => coord,
            _ => unreachable!(),
        },
    };

    let edges = if face_size == 50 {
        vec![
            ((1, Direction::Up), (6, Direction::Right, Same(0))),
            ((1, Direction::Left), (4, Direction::Right, Flip(0))),
            ((1, Direction::Right), (2, Direction::Right, Same(0))),
            ((1, Direction::Down), (3, Direction::Down, Same(0))),
            ((2, Direction::Up), (6, Direction::Up, Same(0))),
            ((2, Direction::Left), (1, Direction::Left, Same(0))),
            ((2, Direction::Right), (5, Direction::Left, Flip(0))),
            ((2, Direction::Down), (3, Direction::Left, Same(0))),
            ((3, Direction::Up), (1, Direction::Up, Same(0))),
            ((3, Direction::Left), (4, Direction::Down, Same(0))),
            ((3, Direction::Right), (2, Direction::Up, Same(0))),
            ((3, Direction::Down), (5, Direction::Down, Same(0))),
            ((4, Direction::Up), (3, Direction::Right, Same(0))),
            ((4, Direction::Left), (1, Direction::Right, Flip(0))),
            ((4, Direction::Right), (5, Direction::Right, Same(0))),
            ((4, Direction::Down), (6, Direction::Down, Same(0))),
            ((5, Direction::Up), (3, Direction::Up, Same(0))),
            ((5, Direction::Left), (4, Direction::Left, Same(0))),
            ((5, Direction::Right), (2, Direction::Left, Flip(0))),
            ((5, Direction::Down), (6, Direction::Left, Same(0))),
            ((6, Direction::Up), (4, Direction::Up, Same(0))),
            ((6, Direction::Left), (1, Direction::Down, Same(0))),
            ((6, Direction::Down), (2, Direction::Down, Same(0))),
            ((6, Direction::Right), (5, Direction::Up, Same(0))),
        ]
    } else if face_size == 4 {
        vec![
            ((1, Direction::Up), (2, Direction::Down, Flip(0))),
            ((1, Direction::Left), (3, Direction::Down, Same(0))),
            ((1, Direction::Right), (6, Direction::Left, Flip(0))),
            ((1, Direction::Down), (4, Direction::Down, Same(0))),
            ((2, Direction::Up), (1, Direction::Down, Flip(0))),
            ((2, Direction::Left), (6, Direction::Up, Flip(0))),
            ((2, Direction::Right), (3, Direction::Right, Same(0))),
            ((2, Direction::Down), (5, Direction::Up, Flip(0))),
            ((3, Direction::Up), (1, Direction::Right, Same(0))),
            ((3, Direction::Left), (2, Direction::Left, Same(0))),
            ((3, Direction::Right), (4, Direction::Right, Same(0))),
            ((3, Direction::Down), (5, Direction::Right, Flip(0))),
            ((4, Direction::Up), (1, Direction::Up, Same(0))),
            ((4, Direction::Left), (3, Direction::Left, Same(0))),
            ((4, Direction::Right), (6, Direction::Down, Flip(0))),
            ((4, Direction::Down), (5, Direction::Down, Same(0))),
            ((5, Direction::Up), (4, Direction::Up, Same(0))),
            ((5, Direction::Left), (3, Direction::Up, Flip(0))),
            ((5, Direction::Right), (6, Direction::Right, Same(0))),
            ((5, Direction::Down), (2, Direction::Up, Flip(0))),
            ((6, Direction::Up), (4, Direction::Left, Flip(0))),
            ((6, Direction::Left), (5, Direction::Left, Same(0))),
            ((6, Direction::Down), (2, Direction::Right, Flip(0))),
            ((6, Direction::Right), (1, Direction::Left, Flip(0))),
        ]
    } else {
        vec![]
    };

    let mapping = edges
        .iter()
        .find(|&&((id, d), _)| id == current_face.id && d == dir)
        .map(|&(_, target)| target);

    if let Some((target_id, new_dir, edge_map)) = mapping {
        let target = &faces[target_id - 1];
        let (tr, tc) = target.top_left;

        let new_pos = match (dir, new_dir) {
            (Direction::Up | Direction::Down, Direction::Up | Direction::Down) => (
                if new_dir == Direction::Up {
                    tr + face_size - 1
                } else {
                    tr
                },
                tc + transform_coord(local_c, edge_map),
            ),
            (Direction::Up | Direction::Down, Direction::Left | Direction::Right) => (
                tr + transform_coord(local_c, edge_map),
                if new_dir == Direction::Left {
                    tc + face_size - 1
                } else {
                    tc
                },
            ),
            (Direction::Left | Direction::Right, Direction::Up | Direction::Down) => (
                if new_dir == Direction::Up {
                    tr + face_size - 1
                } else {
                    tr
                },
                tc + transform_coord(local_r, edge_map),
            ),
            (Direction::Left | Direction::Right, Direction::Left | Direction::Right) => (
                tr + transform_coord(local_r, edge_map),
                if new_dir == Direction::Left {
                    tc + face_size - 1
                } else {
                    tc
                },
            ),
        };

        (new_pos, new_dir)
    } else {
        let wrapped = wrap_position(map, pos, dir);
        (wrapped, dir)
    }
}

fn step_cube(
    map: &Map,
    faces: &[Face],
    face_size: i32,
    pos: Point,
    dir: Direction,
) -> Option<(Point, Direction)> {
    let current_face = get_face(faces, face_size, pos).unwrap();
    let (fr, fc) = current_face.top_left;
    let local_r = pos.0 - fr;
    let local_c = pos.1 - fc;

    let at_edge = match dir {
        Direction::Up => local_r == 0,
        Direction::Down => local_r == face_size - 1,
        Direction::Left => local_c == 0,
        Direction::Right => local_c == face_size - 1,
    };

    let (dr, dc) = dir.delta();
    let new_pos = (pos.0 + dr, pos.1 + dc);

    if at_edge && get_face(faces, face_size, new_pos).is_none_or(|f| f.id != current_face.id) {
        let (wrapped_pos, new_dir) = cube_wrap(map, faces, face_size, pos, dir);
        map.get(&wrapped_pos)
            .filter(|&&tile| tile == Tile::Open)
            .map(|_| (wrapped_pos, new_dir))
    } else {
        map.get(&new_pos)
            .filter(|&&tile| tile == Tile::Open)
            .map(|_| (new_pos, dir))
    }
}

#[aoc(day22, part2)]
fn part2((map, instructions): &(Map, Vec<Instruction>)) -> i32 {
    let (faces, face_size) = detect_faces(map);
    simulate(map, instructions, |pos, dir| {
        step_cube(map, &faces, face_size, pos, dir)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";
        let parsed = generator(input).unwrap();
        assert_eq!(part1(&parsed), 6032);
    }

    #[test]
    fn test_part2() {
        let input = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";
        let parsed = generator(input).unwrap();
        assert_eq!(part2(&parsed), 5031);
    }
}
