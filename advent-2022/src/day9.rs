use std::{collections::HashSet, iter};

use aoc_runner_derive::{aoc, aoc_generator};
use eyre::bail;
use nom::{
    branch::alt,
    character::complete::{char, space1, u32},
    combinator::{map, value},
    sequence::separated_pair,
    IResult, Parser,
};

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug)]
struct Instruction {
    direction: Direction,
    count: usize,
}

type Point = (i32, i32);

struct State<const N: usize> {
    knots: [Point; N],
    visited: HashSet<Point>,
}

impl<const N: usize> Default for State<N> {
    fn default() -> Self {
        let knots = [(0, 0); N];
        let visited = HashSet::new();
        Self { knots, visited }
    }
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    alt((
        value(Direction::Up, char('U')),
        value(Direction::Down, char('D')),
        value(Direction::Left, char('L')),
        value(Direction::Right, char('R')),
    )).parse(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    map(
        separated_pair(parse_direction, space1, u32),
        |(direction, count)| Instruction {
            direction,
            count: count as usize,
        },
    ).parse(input)
}

#[aoc_generator(day9)]
fn generator(input: &str) -> eyre::Result<Vec<Instruction>> {
    input
        .lines()
        .map(|line| match parse_instruction(line) {
            Ok((_, i)) => Ok(i),
            Err(e) => bail!("error parsing{:?}: {}", line, e),
        })
        .collect()
}

fn neighbors((x, y): Point) -> impl Iterator<Item = Point> {
    [
        (-1, 1),
        (0, 1),
        (1, 1),
        (-1, 0),
        (1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ]
    .iter()
    .map(move |&(dx, dy)| (x + dx, y + dy))
}

fn is_touching(left: Point, right: Point) -> bool {
    if left == right {
        return true;
    }

    neighbors(left).any(|n| n == right)
}

fn move_knot<const N: usize>(mut state: State<N>, direction: Direction) -> State<N> {
    let (hx, hy) = state.knots[0];
    state.knots[0] = match direction {
        Direction::Up => (hx, hy + 1),
        Direction::Down => (hx, hy - 1),
        Direction::Left => (hx - 1, hy),
        Direction::Right => (hx + 1, hy),
    };

    for i in 1..N {
        let head @ (hx, hy) = state.knots[i - 1];
        let tail = state.knots[i];

        if is_touching(head, tail) {
            continue;
        }

        let candidates = neighbors(tail)
            .filter(|&nt| is_touching(nt, head))
            .collect::<Vec<_>>();
        let best = candidates.iter().find(|&(nx, ny)| *nx == hx || *ny == hy);
        state.knots[i] = match best {
            Some(&p) => p,
            None => candidates[0],
        };
    }

    state.visited.insert(state.knots[N - 1]);
    state
}

fn solve<const N: usize>(input: &[Instruction]) -> usize {
    let state: State<N> = input.iter().fold(
        State::default(),
        |state, &Instruction { direction, count }| {
            iter::repeat_n(direction, count).fold(state, move_knot)
        },
    );

    state.visited.len()
}

#[aoc(day9, part1)]
fn part1(input: &[Instruction]) -> usize {
    solve::<2>(input)
}

#[aoc(day9, part2)]
fn part2(input: &[Instruction]) -> usize {
    solve::<10>(input)
}
