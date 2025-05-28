use std::{
    collections::HashMap,
    fmt::{self, Display},
    ops::RangeInclusive,
    str::FromStr,
};

use anyhow::anyhow;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::{IResult, Parser};

type Point = (i32, i32);
type Grid = HashMap<Point, Cell>;

const START: Point = (500, 0);

#[derive(Clone, Copy, Debug, PartialEq)]
enum Cell {
    Clay,
    StaleWater,
    FlowingWater,
}

impl Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Clay => write!(f, "#"),
            Cell::StaleWater => write!(f, "~"),
            Cell::FlowingWater => write!(f, "|"),
        }
    }
}

enum Scan {
    Horizontal(RangeInclusive<i32>, i32),
    Vertical(i32, RangeInclusive<i32>),
}

impl FromStr for Scan {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use nom::{
            branch::alt,
            bytes::complete::tag,
            character::complete::i32,
            combinator::map,
            sequence::{preceded, separated_pair},
        };

        let range = |input| -> IResult<&str, RangeInclusive<i32>> {
            map(separated_pair(i32, tag(".."), i32), |(a, b)| a..=b).parse(input)
        };

        let horizontal = map(
            (
                preceded(tag("y="), i32),
                tag(", "),
                preceded(tag("x="), range),
            ),
            |(y, _, x)| Scan::Horizontal(x, y),
        );
        let vertical = map(
            (
                preceded(tag("x="), i32),
                tag(", "),
                preceded(tag("y="), range),
            ),
            |(x, _, y)| Scan::Vertical(x, y),
        );

        alt((horizontal, vertical)).parse(s)
            .map(|(_, scan)| scan)
            .map_err(|e| anyhow!("unable to parse scan: {}", e))
    }
}

struct Ground {
    cells: HashMap<Point, Cell>,
    min_y: i32,
    max_y: i32,
}

impl Ground {
    // 1. flow down until we hit clay or standing water
    // 2. flow left and right until we hit a wall
    // 3. if we hit a wall then insert standing water
    // 4. if we overflow left or right insert water and flow down
    // 5. if we exceed max_y then exit
    fn flow(&mut self, point @ (x, y): Point) {
        if y >= self.max_y {
            return;
        }

        // println!("# {}, {}", x, y);
        // println!("{}", self);
        // println!();

        let below = (x, y + 1);
        match self.cells.get(&below) {
            Some(Cell::FlowingWater) => return,
            None => {
                self.cells.insert(below, Cell::FlowingWater);
                self.flow(below);
                return;
            }
            _ => {}
        }

        self.cells.insert(point, Cell::FlowingWater);

        // we hit clay or standing water
        let mut left = (x - 1, y);
        let mut left_wall = false;
        loop {
            let left_below = (left.0, left.1 + 1);
            match self.cells.get(&left_below) {
                Some(Cell::FlowingWater) | None => break,
                _ => {}
            }

            if let Some(Cell::Clay) = self.cells.get(&left) {
                left_wall = true;
                break;
            }

            self.cells.insert(left, Cell::FlowingWater);

            left = (left.0 - 1, left.1);
        }

        let mut right = (x + 1, y);
        let mut right_wall = false;
        loop {
            let right_below = (right.0, right.1 + 1);
            match self.cells.get(&right_below) {
                Some(Cell::FlowingWater) | None => break,
                _ => {}
            }

            if let Some(Cell::Clay) = self.cells.get(&right) {
                right_wall = true;
                break;
            }

            self.cells.insert(right, Cell::FlowingWater);

            right = (right.0 + 1, right.1);
        }

        match (left_wall, right_wall) {
            (true, true) => {
                let prev = (x, y - 1);
                let (lx, _) = left;
                let (rx, _) = right;
                for x in lx + 1..rx {
                    self.cells.insert((x, y), Cell::StaleWater);
                }

                self.flow(prev);
            }
            (true, false) => {
                self.cells.insert(right, Cell::FlowingWater);
                self.flow(right);
            }
            (false, true) => {
                self.cells.insert(left, Cell::FlowingWater);
                self.flow(left);
            }
            (false, false) => {
                self.cells.insert(left, Cell::FlowingWater);
                self.cells.insert(right, Cell::FlowingWater);
                self.flow(left);
                self.flow(right);
            }
        }
    }
}

impl FromIterator<(Point, Cell)> for Ground {
    fn from_iter<T: IntoIterator<Item = (Point, Cell)>>(iter: T) -> Self {
        let cells = iter.into_iter().collect::<HashMap<Point, Cell>>();
        let (min_y, max_y) = cells
            .keys()
            .map(|&(_, y)| y)
            .minmax()
            .into_option()
            .unwrap();
        Self {
            cells,
            min_y,
            max_y,
        }
    }
}

impl Display for Ground {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.cells.is_empty() {
            return Ok(());
        }

        let (min_x, max_x) = self
            .cells
            .keys()
            .map(|&(x, _)| x)
            .minmax()
            .into_option()
            .unwrap();

        for y in 0..=self.max_y {
            for x in min_x..=max_x {
                if (x, y) == START {
                    write!(f, "+")?;
                    continue;
                }

                match self.cells.get(&(x, y)) {
                    Some(Cell::Clay) => write!(f, "#")?,
                    Some(Cell::StaleWater) => write!(f, "~")?,
                    Some(Cell::FlowingWater) => write!(f, "|")?,
                    None => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[aoc_generator(day17)]
fn generator(input: &str) -> anyhow::Result<Grid> {
    let scans = input
        .lines()
        .map(|line| line.parse())
        .collect::<anyhow::Result<Vec<Scan>>>()?;

    let cells = scans.into_iter().fold(HashMap::new(), |mut acc, scan| {
        match scan {
            Scan::Horizontal(x, y) => {
                for x in x {
                    acc.insert((x, y), Cell::Clay);
                }
            }
            Scan::Vertical(x, y) => {
                for y in y {
                    acc.insert((x, y), Cell::Clay);
                }
            }
        }

        acc
    });

    Ok(cells)
}

#[aoc(day17, part1)]
fn part1(input: &Grid) -> usize {
    let grid = input.clone();
    let mut ground = Ground::from_iter(grid);
    ground.flow(START);
    ground
        .cells
        .into_iter()
        .filter(|&((_, y), _)| y <= ground.max_y && y >= ground.min_y)
        .map(|(_, cell)| cell)
        .filter(|&cell| cell == Cell::FlowingWater || cell == Cell::StaleWater)
        .count()
}

#[aoc(day17, part2)]
fn part2(input: &Grid) -> usize {
    let grid = input.clone();
    let mut ground = Ground::from_iter(grid);
    ground.flow(START);
    ground
        .cells
        .into_iter()
        .filter(|&((_, y), _)| y <= ground.max_y && y >= ground.min_y)
        .map(|(_, cell)| cell)
        .filter(|&cell| cell == Cell::StaleWater)
        .count()
}
