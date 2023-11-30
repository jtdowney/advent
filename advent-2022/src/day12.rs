use std::{
    collections::{HashMap, HashSet, VecDeque},
    iter,
};

use aoc_runner_derive::{aoc, aoc_generator};

type Point = (i32, i32);

#[derive(Clone, Debug, Default)]
struct Grid {
    start: Point,
    end: Point,
    map: HashMap<Point, u8>,
}

#[aoc_generator(day12)]
fn generator(input: &str) -> Grid {
    input
        .lines()
        .enumerate()
        .fold(Grid::default(), |mut grid, (y, line)| {
            for (x, b) in line.bytes().enumerate() {
                let point = (x as i32, y as i32);

                if b == b'S' {
                    grid.start = point;
                } else if b == b'E' {
                    grid.end = point;
                }

                let b = match b {
                    b'S' => b'a',
                    b'E' => b'z',
                    b => b,
                };

                grid.map.insert(point, b);
            }

            grid
        })
}

fn find_shortest_path(grid: &Grid, start: Point) -> Option<usize> {
    let mut path = HashMap::<Point, Point>::new();
    let mut visited = HashSet::new();
    visited.insert(start);

    let mut search = VecDeque::new();
    search.push_back(start);

    while let Some(current @ (x, y)) = search.pop_front() {
        if current == grid.end {
            let steps = iter::successors(Some(&current), |point| path.get(point)).count() - 1;
            return Some(steps);
        }

        let current_height = grid.map[&current];
        let neighbors = [(-1, 0), (0, 1), (1, 0), (0, -1)]
            .iter()
            .map(move |&(dx, dy)| (x + dx, y + dy))
            .filter(|n| !visited.contains(n))
            .filter(|n| grid.map.contains_key(n))
            .filter(|n| {
                let neighbor_height = grid.map[n];
                neighbor_height <= current_height + 1
            })
            .collect::<Vec<_>>();
        for neighbor in neighbors {
            visited.insert(neighbor);
            path.insert(neighbor, current);
            search.push_back(neighbor);
        }
    }

    None
}

#[aoc(day12, part1)]
fn part1(input: &Grid) -> Option<usize> {
    find_shortest_path(input, input.start)
}

#[aoc(day12, part2)]
fn part2(input: &Grid) -> Option<usize> {
    let mut steps = input
        .map
        .iter()
        .filter(|(_, &height)| height == b'a')
        .filter_map(|(&start, _)| find_shortest_path(input, start))
        .collect::<Vec<_>>();
    steps.sort_unstable();
    steps.first().copied()
}
