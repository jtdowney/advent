use std::collections::HashSet;

use anyhow::{Result, bail};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Point = (i32, i32, i32, i32);

#[aoc_generator(day25)]
fn parse(input: &str) -> Result<Vec<Point>> {
    input
        .lines()
        .map(|line| {
            let coords: Vec<i32> = line
                .split(',')
                .map(|s| s.trim().parse())
                .collect::<Result<Vec<_>, _>>()?;

            if coords.len() != 4 {
                bail!("Expected 4 coordinates, got {}", coords.len());
            }

            Ok((coords[0], coords[1], coords[2], coords[3]))
        })
        .collect()
}

fn manhattan_distance(&(x1, y1, z1, w1): &Point, &(x2, y2, z2, w2): &Point) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs() + (w1 - w2).abs()
}

fn find(parent: &mut [usize], x: usize) -> usize {
    if parent[x] != x {
        parent[x] = find(parent, parent[x]);
    }

    parent[x]
}

fn union(parent: &mut [usize], x: usize, y: usize) {
    let root_x = find(parent, x);
    let root_y = find(parent, y);

    if root_x != root_y {
        parent[root_x] = root_y;
    }
}

#[aoc(day25, part1)]
fn part1(points: &[Point]) -> usize {
    let n = points.len();
    let mut parent = (0..n).collect::<Vec<_>>();
    for (i, j) in (0..n).tuple_combinations() {
        if manhattan_distance(&points[i], &points[j]) <= 3 {
            union(&mut parent, i, j);
        }
    }

    let constellations = (0..n).map(|i| find(&mut parent, i)).collect::<HashSet<_>>();
    constellations.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = " 0,0,0,0
 3,0,0,0";
        let points = parse(input).unwrap();
        assert_eq!(points.len(), 2);
        assert_eq!(points[0], (0, 0, 0, 0));
        assert_eq!(points[1], (3, 0, 0, 0));
    }

    #[test]
    fn test_part1_example1() {
        let input = " 0,0,0,0
 3,0,0,0
 0,3,0,0
 0,0,3,0
 0,0,0,3
 0,0,0,6
 9,0,0,0
12,0,0,0";
        let points = parse(input).unwrap();
        assert_eq!(part1(&points), 2);
    }

    #[test]
    fn test_part1_example2() {
        let input = "-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0";
        let points = parse(input).unwrap();
        assert_eq!(part1(&points), 4);
    }

    #[test]
    fn test_part1_example3() {
        let input = "1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2";
        let points = parse(input).unwrap();
        assert_eq!(part1(&points), 3);
    }

    #[test]
    fn test_part1_example4() {
        let input = "1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2";
        let points = parse(input).unwrap();
        assert_eq!(part1(&points), 8);
    }
}
