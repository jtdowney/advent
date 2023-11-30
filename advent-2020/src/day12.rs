use std::{num::ParseIntError, ops::Add, str::FromStr};

#[derive(Copy, Clone)]
enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instruction = match s {
            "N" => Action::North,
            "S" => Action::South,
            "E" => Action::East,
            "W" => Action::West,
            "L" => Action::Left,
            "R" => Action::Right,
            "F" => Action::Forward,
            _ => unreachable!(),
        };

        Ok(instruction)
    }
}

#[derive(Copy, Clone)]
struct Instruction(Action, i32);

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_at(1);
        let action = a.parse().unwrap();
        let argument = b.parse()?;
        let operation = Instruction(action, argument);
        Ok(operation)
    }
}

#[derive(Copy, Clone)]
struct Point(i32, i32);

impl Add<Instruction> for Point {
    type Output = Point;

    fn add(self, rhs: Instruction) -> Self::Output {
        let Point(x, y) = self;
        match rhs {
            Instruction(Action::North, arg) => Point(x, y - arg),
            Instruction(Action::South, arg) => Point(x, y + arg),
            Instruction(Action::East, arg) => Point(x + arg, y),
            Instruction(Action::West, arg) => Point(x - arg, y),
            _ => unimplemented!(),
        }
    }
}

impl Point {
    fn rotate(&self, degrees: i32) -> Point {
        let Point(x, y) = *self;
        match degrees {
            0 => Point(x, y),
            90 | -270 => Point(-y, x),
            180 | -180 => Point(-x, -y),
            270 | -90 => Point(y, -x),
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone)]
struct Ship {
    position: Point,
    direction: Point,
}

impl Add<Instruction> for Ship {
    type Output = Ship;

    fn add(self, rhs: Instruction) -> Self::Output {
        match rhs {
            Instruction(Action::Left, arg) => Ship {
                direction: self.direction.rotate(-arg),
                ..self
            },
            Instruction(Action::Right, arg) => Ship {
                direction: self.direction.rotate(arg),
                ..self
            },
            Instruction(Action::Forward, arg) => {
                let Point(px, py) = self.position;
                let Point(dx, dy) = self.direction;
                Ship {
                    position: Point(px + dx * arg, py + dy * arg),
                    ..self
                }
            }
            _ => unimplemented!(),
        }
    }
}

#[aoc_generator(day12)]
fn generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap()
}

#[aoc(day12, part1)]
fn part1(input: &[Instruction]) -> i32 {
    let state = Ship {
        position: Point(0, 0),
        direction: Point(1, 0),
    };

    let ship = input
        .iter()
        .fold(state, |ship, &instruction| match instruction {
            Instruction(Action::Left, _)
            | Instruction(Action::Right, _)
            | Instruction(Action::Forward, _) => ship + instruction,
            _ => Ship {
                position: ship.position + instruction,
                ..ship
            },
        });

    let Point(x, y) = ship.position;
    x.abs() + y.abs()
}

#[aoc(day12, part2)]
fn part2(input: &[Instruction]) -> i32 {
    let state = Ship {
        position: Point(0, 0),
        direction: Point(10, -1),
    };

    let ship = input
        .iter()
        .fold(state, |ship, &instruction| match instruction {
            Instruction(Action::Left, _)
            | Instruction(Action::Right, _)
            | Instruction(Action::Forward, _) => ship + instruction,
            _ => Ship {
                direction: ship.direction + instruction,
                ..ship
            },
        });

    let Point(x, y) = ship.position;
    x.abs() + y.abs()
}
