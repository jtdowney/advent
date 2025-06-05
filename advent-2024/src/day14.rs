use std::collections::HashSet;

use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::Regex;

type Point = (i32, i32);

#[derive(Debug, Clone)]
struct Robot {
    position: Point,
    velocity: Point,
}

impl Robot {
    fn move_robot(&self, steps: i32, width: i32, height: i32) -> Point {
        let (px, py) = self.position;
        let (vx, vy) = self.velocity;

        let new_x = ((px + vx * steps) % width + width) % width;
        let new_y = ((py + vy * steps) % height + height) % height;

        (new_x, new_y)
    }
}

fn calculate_safety_factor(robots: &[Robot], steps: i32, width: i32, height: i32) -> i32 {
    let (mid_x, mid_y) = (width / 2, height / 2);

    robots
        .iter()
        .map(|robot| robot.move_robot(steps, width, height))
        .filter(|&(x, y)| x != mid_x && y != mid_y)
        .map(|(x, y)| match (x < mid_x, y < mid_y) {
            (true, true) => 0,
            (false, true) => 1,
            (true, false) => 2,
            (false, false) => 3,
        })
        .counts()
        .values()
        .product::<usize>()
        .try_into()
        .expect("product overflow")
}

fn has_dense_cluster(robots: &[Robot], steps: i32, width: i32, height: i32) -> bool {
    let positions: HashSet<Point> = robots
        .iter()
        .map(|robot| robot.move_robot(steps, width, height))
        .collect();

    (0..height).any(|y| {
        (0..width)
            .map(|x| positions.contains(&(x, y)))
            .chunk_by(|&v| v)
            .into_iter()
            .filter(|(k, _)| *k)
            .map(|(_, group)| group.count())
            .max()
            .unwrap_or(0)
            >= 10
    })
}

#[aoc_generator(day14)]
fn generator(input: &str) -> Result<Vec<Robot>> {
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)")?;

    input
        .lines()
        .map(|line| {
            let caps = re
                .captures(line)
                .ok_or_else(|| anyhow::anyhow!("Invalid line: {}", line))?;
            let px = caps[1].parse::<i32>()?;
            let py = caps[2].parse::<i32>()?;
            let vx = caps[3].parse::<i32>()?;
            let vy = caps[4].parse::<i32>()?;

            Ok(Robot {
                position: (px, py),
                velocity: (vx, vy),
            })
        })
        .collect()
}

#[aoc(day14, part1)]
fn part1(robots: &[Robot]) -> i32 {
    calculate_safety_factor(robots, 100, 101, 103)
}

#[aoc(day14, part2)]
fn part2(robots: &[Robot]) -> i32 {
    let (width, height) = (101, 103);

    (0..=(width * height))
        .find(|&seconds| has_dense_cluster(robots, seconds, width, height))
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;

    #[test]
    fn test_parse_robots() {
        let robots = generator(EXAMPLE).unwrap();
        assert_eq!(robots.len(), 12);
        assert_eq!(robots[0].position, (0, 4));
        assert_eq!(robots[0].velocity, (3, -3));
        assert_eq!(robots[1].position, (6, 3));
        assert_eq!(robots[1].velocity, (-1, -3));
    }

    #[test]
    fn test_robot_movement() {
        let robot = Robot {
            position: (2, 4),
            velocity: (2, -3),
        };

        assert_eq!(robot.move_robot(1, 11, 7), (4, 1));
        assert_eq!(robot.move_robot(2, 11, 7), (6, 5));
        assert_eq!(robot.move_robot(3, 11, 7), (8, 2));
        assert_eq!(robot.move_robot(4, 11, 7), (10, 6));
        assert_eq!(robot.move_robot(5, 11, 7), (1, 3));
    }

    #[test]
    fn test_safety_factor() {
        let robots = generator(EXAMPLE).unwrap();
        assert_eq!(calculate_safety_factor(&robots, 100, 11, 7), 12);
    }

    #[test]
    fn test_part1_example() {
        let robots = generator(EXAMPLE).unwrap();
        assert_eq!(calculate_safety_factor(&robots, 100, 11, 7), 12);
    }

    #[test]
    fn test_dense_cluster_detection() {
        let robots: Vec<Robot> = (45..56)
            .map(|x| Robot {
                position: (x, 50),
                velocity: (0, 0),
            })
            .collect();

        assert!(has_dense_cluster(&robots, 0, 101, 103));
    }
}
