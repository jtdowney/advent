use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy, Debug)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

type Coord = (i8, i8);

#[aoc_generator(day2)]
fn generator(input: &str) -> Vec<Vec<Move>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'U' => Move::Up,
                    'D' => Move::Down,
                    'L' => Move::Left,
                    'R' => Move::Right,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

type Keypad = fn(Coord) -> Option<char>;

fn solve(moves: &[Vec<Move>], keypad: Keypad) -> String {
    moves
        .iter()
        .map(|moves| {
            moves.iter().fold((1, 1), |coord, m| {
                let new_coord = match m {
                    Move::Up => (coord.0, coord.1 - 1),
                    Move::Down => (coord.0, coord.1 + 1),
                    Move::Left => (coord.0 - 1, coord.1),
                    Move::Right => (coord.0 + 1, coord.1),
                };

                if keypad(new_coord).is_some() {
                    new_coord
                } else {
                    coord
                }
            })
        })
        .map(|coord| keypad(coord).unwrap())
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[Vec<Move>]) -> String {
    fn keypad(coord: Coord) -> Option<char> {
        match coord {
            (0, 0) => Some('1'),
            (1, 0) => Some('2'),
            (2, 0) => Some('3'),
            (0, 1) => Some('4'),
            (1, 1) => Some('5'),
            (2, 1) => Some('6'),
            (0, 2) => Some('7'),
            (1, 2) => Some('8'),
            (2, 2) => Some('9'),
            _ => None,
        }
    }

    solve(input, keypad)
}

#[aoc(day2, part2)]
fn part2(input: &[Vec<Move>]) -> String {
    fn keypad(coord: Coord) -> Option<char> {
        match coord {
            (2, 0) => Some('1'),
            (1, 1) => Some('2'),
            (2, 1) => Some('3'),
            (3, 1) => Some('4'),
            (0, 2) => Some('5'),
            (1, 2) => Some('6'),
            (2, 2) => Some('7'),
            (3, 2) => Some('8'),
            (4, 2) => Some('9'),
            (1, 3) => Some('A'),
            (2, 3) => Some('B'),
            (3, 3) => Some('C'),
            (2, 4) => Some('D'),
            _ => None,
        }
    }

    solve(input, keypad)
}
