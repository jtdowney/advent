use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::{Result, anyhow};
use aoc_runner_derive::{aoc, aoc_generator};
use num::integer::lcm;

type Point = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Blizzard {
    position: Point,
    direction: Direction,
}

#[derive(Debug, Clone)]
pub struct Valley {
    blizzards: Vec<Blizzard>,
    dimensions: Point,
    start: Point,
    end: Point,
    cycle_length: usize,
    blizzard_cache: HashMap<usize, HashSet<Point>>,
}

impl Valley {
    fn new(blizzards: Vec<Blizzard>, dimensions: Point, start: Point, end: Point) -> Self {
        let (width, height) = dimensions;
        let cycle_length = lcm(width - 2, height - 2);
        let mut valley = Self {
            blizzards,
            dimensions,
            start,
            end,
            cycle_length,
            blizzard_cache: HashMap::new(),
        };

        for minute in 0..cycle_length {
            valley
                .blizzard_cache
                .insert(minute, valley.compute_blizzard_positions(minute));
        }

        valley
    }

    fn compute_blizzard_positions(&self, minute: usize) -> HashSet<Point> {
        self.blizzards
            .iter()
            .map(|blizzard| {
                let (width, height) = self.dimensions;
                let (x, y) = blizzard.position;
                match blizzard.direction {
                    Direction::Up => (
                        x,
                        1 + (y - 1 + height - 2 - minute % (height - 2)) % (height - 2),
                    ),
                    Direction::Down => (x, 1 + (y - 1 + minute) % (height - 2)),
                    Direction::Left => (
                        1 + (x - 1 + width - 2 - minute % (width - 2)) % (width - 2),
                        y,
                    ),
                    Direction::Right => (1 + (x - 1 + minute) % (width - 2), y),
                }
            })
            .collect()
    }

    fn blizzard_positions_at(&self, minute: usize) -> &HashSet<Point> {
        &self.blizzard_cache[&(minute % self.cycle_length)]
    }

    fn is_valid_position(&self, pos: Point) -> bool {
        let (x, y) = pos;
        let (width, height) = self.dimensions;
        pos == self.start || pos == self.end || (x > 0 && x < width - 1 && y > 0 && y < height - 1)
    }

    fn neighbors(&self, pos: Point) -> impl Iterator<Item = Point> {
        let (x, y) = pos;
        let (width, height) = self.dimensions;
        [(0, 0), (0, 1), (1, 0), (0, -1), (-1, 0)]
            .into_iter()
            .filter_map(move |(dx, dy)| {
                let nx = x.wrapping_add_signed(dx);
                let ny = y.wrapping_add_signed(dy);
                if nx < width && ny < height {
                    Some((nx, ny))
                } else {
                    None
                }
            })
    }

    fn find_shortest_path_from(
        &self,
        start: Point,
        end: Point,
        start_minute: usize,
    ) -> Option<usize> {
        let mut queue = VecDeque::from([(start, start_minute)]);
        let mut visited = HashSet::from([(start, start_minute % self.cycle_length)]);

        while let Some((pos, minute)) = queue.pop_front() {
            if pos == end {
                return Some(minute);
            }

            let next_minute = minute + 1;
            let blizzards = self.blizzard_positions_at(next_minute);

            self.neighbors(pos)
                .filter(|&next_pos| {
                    self.is_valid_position(next_pos) && !blizzards.contains(&next_pos)
                })
                .filter(|&next_pos| visited.insert((next_pos, next_minute % self.cycle_length)))
                .for_each(|next_pos| queue.push_back((next_pos, next_minute)));
        }

        None
    }

    fn find_shortest_path(&self) -> Option<usize> {
        self.find_shortest_path_from(self.start, self.end, 0)
    }
}

#[aoc_generator(day24)]
pub fn generator(input: &str) -> Result<Valley> {
    let lines: Vec<&str> = input.lines().collect();
    let dimensions = (lines[0].len(), lines.len());
    let (_, height) = dimensions;

    let find_opening = |line: &str, y: usize| {
        line.chars()
            .position(|c| c == '.')
            .map(|x| (x, y))
            .ok_or_else(|| anyhow!("No opening found"))
    };

    let start = find_opening(lines[0], 0)?;
    let end = find_opening(lines[height - 1], height - 1)?;

    let blizzards = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, ch)| {
                match ch {
                    '^' => Some(Direction::Up),
                    'v' => Some(Direction::Down),
                    '<' => Some(Direction::Left),
                    '>' => Some(Direction::Right),
                    _ => None,
                }
                .map(|direction| Blizzard {
                    position: (x, y),
                    direction,
                })
            })
        })
        .collect();

    Ok(Valley::new(blizzards, dimensions, start, end))
}

#[aoc(day24, part1)]
pub fn part1(valley: &Valley) -> Option<usize> {
    valley.find_shortest_path()
}

#[aoc(day24, part2)]
pub fn part2(valley: &Valley) -> Option<usize> {
    let trips = [
        (valley.start, valley.end),
        (valley.end, valley.start),
        (valley.start, valley.end),
    ];

    trips.into_iter().try_fold(0, |time, (from, to)| {
        valley.find_shortest_path_from(from, to, time)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    const SIMPLE_EXAMPLE: &str = "#.#####
#.....#
#>....#
#.....#
#...v.#
#.....#
#####.#";

    #[test]
    fn test_part1() {
        let valley = generator(EXAMPLE).unwrap();
        assert_eq!(part1(&valley), Some(18));
    }

    #[test]
    fn test_simple_example() {
        let valley = generator(SIMPLE_EXAMPLE).unwrap();
        assert!(valley.find_shortest_path().is_some());
    }

    #[test]
    fn test_part2() {
        let valley = generator(EXAMPLE).unwrap();
        assert_eq!(part2(&valley), Some(54));
    }
}
