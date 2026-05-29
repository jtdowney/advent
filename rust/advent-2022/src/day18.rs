use std::collections::{HashSet, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Point = (i32, i32, i32);

const DIRECTIONS: [Point; 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

#[aoc_generator(day18)]
fn generator(input: &str) -> anyhow::Result<HashSet<Point>> {
    input
        .lines()
        .map(|line| {
            let coords: Vec<i32> = line
                .split(',')
                .map(|s| s.parse())
                .collect::<Result<_, _>>()?;

            anyhow::ensure!(coords.len() == 3, "Invalid input line: {}", line);
            Ok((coords[0], coords[1], coords[2]))
        })
        .collect()
}

#[aoc(day18, part1)]
fn part1(cubes: &HashSet<Point>) -> usize {
    cubes
        .iter()
        .flat_map(|&(x, y, z)| {
            DIRECTIONS
                .iter()
                .map(move |(dx, dy, dz)| (x + dx, y + dy, z + dz))
        })
        .filter(|neighbor| !cubes.contains(neighbor))
        .count()
}

#[aoc(day18, part2)]
fn part2(cubes: &HashSet<Point>) -> usize {
    let (min_x, max_x) = cubes
        .iter()
        .map(|&(x, _, _)| x)
        .minmax()
        .into_option()
        .unwrap();
    let (min_y, max_y) = cubes
        .iter()
        .map(|&(_, y, _)| y)
        .minmax()
        .into_option()
        .unwrap();
    let (min_z, max_z) = cubes
        .iter()
        .map(|&(_, _, z)| z)
        .minmax()
        .into_option()
        .unwrap();

    let bounds = (
        (min_x - 1, max_x + 1),
        (min_y - 1, max_y + 1),
        (min_z - 1, max_z + 1),
    );

    let reachable = flood_fill(cubes, bounds);

    cubes
        .iter()
        .flat_map(|&(x, y, z)| {
            DIRECTIONS
                .iter()
                .map(move |(dx, dy, dz)| (x + dx, y + dy, z + dz))
        })
        .filter(|neighbor| reachable.contains(neighbor))
        .count()
}

fn flood_fill(
    cubes: &HashSet<Point>,
    bounds: ((i32, i32), (i32, i32), (i32, i32)),
) -> HashSet<Point> {
    let mut reachable = HashSet::new();
    let mut queue = VecDeque::new();
    let ((min_x, _), (min_y, _), (min_z, _)) = bounds;
    let start = (min_x, min_y, min_z);

    queue.push_back(start);
    reachable.insert(start);

    while let Some((x, y, z)) = queue.pop_front() {
        for (dx, dy, dz) in DIRECTIONS {
            let next = (x + dx, y + dy, z + dz);

            let (x, y, z) = next;
            let ((min_x, max_x), (min_y, max_y), (min_z, max_z)) = bounds;
            if x < min_x || x > max_x || y < min_y || y > max_y || z < min_z || z > max_z {
                continue;
            }

            if !cubes.contains(&next) && reachable.insert(next) {
                queue.push_back(next);
            }
        }
    }

    reachable
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn test_part1() {
        let input = generator(EXAMPLE).unwrap();
        assert_eq!(part1(&input), 64);
    }

    #[test]
    fn test_part2() {
        let input = generator(EXAMPLE).unwrap();
        assert_eq!(part2(&input), 58);
    }
}
