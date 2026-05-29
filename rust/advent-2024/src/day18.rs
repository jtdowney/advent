use std::collections::{HashSet, VecDeque};

use anyhow::{Result, anyhow};
use aoc_runner_derive::{aoc, aoc_generator};

type Point = (usize, usize);

#[aoc_generator(day18)]
pub fn generator(input: &str) -> Result<Vec<Point>> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line
                .split_once(',')
                .ok_or_else(|| anyhow!("Invalid coordinate format"))?;
            let x = x.parse::<usize>()?;
            let y = y.parse::<usize>()?;
            Ok((x, y))
        })
        .collect()
}

#[aoc(day18, part1)]
pub fn part1(bytes: &[Point]) -> usize {
    find_shortest_path(bytes, 70, 1024).unwrap_or(0)
}

#[aoc(day18, part2)]
pub fn part2(bytes: &[Point]) -> String {
    let (x, y) = find_blocking_byte(bytes, 70).unwrap();
    format!("{x},{y}")
}

fn find_blocking_byte(bytes: &[Point], grid_size: usize) -> Option<Point> {
    let mut left = 0;
    let mut right = bytes.len();

    while left < right {
        let mid = left.midpoint(right);

        if find_shortest_path(bytes, grid_size, mid + 1).is_some() {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    bytes.get(left).copied()
}

fn find_shortest_path(bytes: &[Point], grid_size: usize, byte_count: usize) -> Option<usize> {
    let corrupted: HashSet<Point> = bytes.iter().take(byte_count).copied().collect();

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back(((0, 0), 0));
    visited.insert((0, 0));

    let target = (grid_size, grid_size);

    while let Some(((x, y), steps)) = queue.pop_front() {
        if (x, y) == target {
            return Some(steps);
        }

        for pos in neighbors((x, y), grid_size) {
            if !corrupted.contains(&pos) && !visited.contains(&pos) {
                visited.insert(pos);
                queue.push_back((pos, steps + 1));
            }
        }
    }

    None
}

fn neighbors((x, y): Point, grid_size: usize) -> impl Iterator<Item = Point> {
    [(0, 1), (1, 0), (0, -1), (-1, 0)]
        .into_iter()
        .filter_map(move |(dx, dy)| {
            x.checked_add_signed(dx)
                .zip(y.checked_add_signed(dy))
                .filter(|&(nx, ny)| nx <= grid_size && ny <= grid_size)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn test_generator() {
        let result = generator(EXAMPLE_INPUT).unwrap();
        assert_eq!(result[0], (5, 4));
        assert_eq!(result[1], (4, 2));
        assert_eq!(result.len(), 25);
    }

    #[test]
    fn test_part1_example() {
        let bytes = generator(EXAMPLE_INPUT).unwrap();
        let result = find_shortest_path(&bytes, 6, 12).unwrap();
        assert_eq!(result, 22);
    }

    #[test]
    fn test_part2_example() {
        let bytes = generator(EXAMPLE_INPUT).unwrap();
        let blocking_byte = find_blocking_byte(&bytes, 6).unwrap();
        assert_eq!(blocking_byte, (6, 1));
    }
}
