use std::{
    collections::{HashMap, HashSet},
    iter,
};

use anyhow::{Context, bail};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::izip;

type Point = (i16, i16);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn next_position(&self, (x, y): Point) -> Point {
        match self {
            Self::North => (x, y - 1),
            Self::West => (x - 1, y),
            Self::South => (x, y + 1),
            Self::East => (x + 1, y),
        }
    }
}

#[derive(Clone, Copy)]
enum Cell {
    ForwardMirror,
    BackwardMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

impl Cell {
    fn next_beam(
        &self,
        Beam {
            direction,
            position,
        }: Beam,
    ) -> Vec<Beam> {
        let (x, y) = position;
        let mut beams = vec![];
        match (self, direction) {
            (Cell::ForwardMirror, Direction::North) | (Cell::BackwardMirror, Direction::South) => {
                beams.push(Beam {
                    direction: Direction::East,
                    position: (x + 1, y),
                })
            }
            (Cell::ForwardMirror, Direction::West) | (Cell::BackwardMirror, Direction::East) => {
                beams.push(Beam {
                    direction: Direction::South,
                    position: (x, y + 1),
                })
            }
            (Cell::ForwardMirror, Direction::South) | (Cell::BackwardMirror, Direction::North) => {
                beams.push(Beam {
                    direction: Direction::West,
                    position: (x - 1, y),
                })
            }
            (Cell::ForwardMirror, Direction::East) | (Cell::BackwardMirror, Direction::West) => {
                beams.push(Beam {
                    direction: Direction::North,
                    position: (x, y - 1),
                })
            }
            (Cell::VerticalSplitter, Direction::North)
            | (Cell::VerticalSplitter, Direction::South)
            | (Cell::HorizontalSplitter, Direction::West)
            | (Cell::HorizontalSplitter, Direction::East) => beams.push(Beam {
                position: direction.next_position(position),
                direction,
            }),
            (Cell::VerticalSplitter, Direction::West)
            | (Cell::VerticalSplitter, Direction::East) => {
                beams.push(Beam {
                    position: (x, y - 1),
                    direction: Direction::North,
                });
                beams.push(Beam {
                    position: (x, y + 1),
                    direction: Direction::South,
                });
            }
            (Cell::HorizontalSplitter, Direction::North)
            | (Cell::HorizontalSplitter, Direction::South) => {
                beams.push(Beam {
                    position: (x - 1, y),
                    direction: Direction::West,
                });
                beams.push(Beam {
                    position: (x + 1, y),
                    direction: Direction::East,
                });
            }
        }

        beams
    }
}

impl TryFrom<char> for Cell {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '/' => Ok(Self::ForwardMirror),
            '\\' => Ok(Self::BackwardMirror),
            '|' => Ok(Self::VerticalSplitter),
            '-' => Ok(Self::HorizontalSplitter),
            _ => bail!("Invalid character: {}", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Beam {
    position: Point,
    direction: Direction,
}

struct Map {
    cells: HashMap<Point, Cell>,
    width: i16,
    height: i16,
}

#[aoc_generator(day16)]
fn generator(input: &str) -> anyhow::Result<Map> {
    let cells = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, c)| c != '.')
                .map(move |(x, c)| anyhow::Ok(((x as i16, y as i16), Cell::try_from(c)?)))
        })
        .collect::<Result<HashMap<_, _>, _>>()?;

    let width = cells
        .keys()
        .map(|&(x, _)| x)
        .max()
        .context("finding width")?;
    let height = cells
        .keys()
        .map(|&(_, y)| y)
        .max()
        .context("finding height")?;

    Ok(Map {
        cells,
        width,
        height,
    })
}

fn simulate(map: &Map, start_beam: Beam) -> usize {
    let mut visited = HashSet::new();
    let mut search = vec![start_beam];

    while let Some(
        beam @ Beam {
            position,
            direction,
        },
    ) = search.pop()
    {
        let (x, y) = position;
        if x > map.width || x < 0 {
            continue;
        }

        if y > map.height || y < 0 {
            continue;
        }

        if !visited.insert(beam) {
            continue;
        }

        if let Some(cell) = map.cells.get(&position) {
            search.extend(cell.next_beam(beam));
        } else {
            let position = direction.next_position(position);
            search.push(Beam {
                direction,
                position,
            })
        }
    }

    let energized = visited
        .into_iter()
        .map(|b| b.position)
        .collect::<HashSet<_>>();
    energized.len()
}

#[aoc(day16, part1)]
fn part1(input: &Map) -> usize {
    simulate(
        input,
        Beam {
            position: (0, 0),
            direction: Direction::East,
        },
    )
}

#[aoc(day16, part2)]
fn part2(input: &Map) -> Option<usize> {
    izip!(
        0..=input.width,
        iter::repeat(0),
        iter::repeat(Direction::South)
    )
    .chain(izip!(
        iter::repeat(0),
        0..=input.height,
        iter::repeat(Direction::East)
    ))
    .chain(izip!(
        0..=input.width,
        iter::repeat(input.height),
        iter::repeat(Direction::North)
    ))
    .chain(izip!(
        iter::repeat(input.width),
        0..=input.height,
        iter::repeat(Direction::West)
    ))
    .map(|(x, y, direction)| {
        simulate(
            input,
            Beam {
                position: (x, y),
                direction,
            },
        )
    })
    .max()
}
