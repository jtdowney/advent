use std::{collections::HashSet, iter};

use itertools::{iproduct, Itertools};

const STEPS: usize = 7;

#[derive(Debug, Clone)]
struct Dimension {
    active_cells: HashSet<(i32, i32, i32, i32)>,
}

impl Dimension {
    fn step(&self) -> Dimension {
        let (min_x, max_x) = self
            .active_cells
            .iter()
            .map(|(x, _, _, _)| x)
            .minmax()
            .into_option()
            .unwrap();
        let (min_y, max_y) = self
            .active_cells
            .iter()
            .map(|(_, y, _, _)| y)
            .minmax()
            .into_option()
            .unwrap();
        let (min_z, max_z) = self
            .active_cells
            .iter()
            .map(|(_, _, z, _)| z)
            .minmax()
            .into_option()
            .unwrap();
        let (min_w, max_w) = self
            .active_cells
            .iter()
            .map(|(_, _, _, w)| w)
            .minmax()
            .into_option()
            .unwrap();

        let neighbors = iproduct!(-1..=1, -1..=1, -1..=1, -1..=1)
            .filter(|&point| point != (0, 0, 0, 0))
            .collect::<Vec<_>>();
        let active_cells = iproduct!(
            min_x - 1..=max_x + 1,
            min_y - 1..=max_y + 1,
            min_z - 1..=max_z + 1,
            min_w - 1..=max_w + 1
        )
        .filter_map(|point| {
            let (x, y, z, w) = point;
            let active = self.active_cells.contains(&point);
            let active_neighbors = neighbors
                .iter()
                .map(|(dx, dy, dz, dw)| (x + dx, y + dy, z + dz, w + dw))
                .filter(|neighbor| self.active_cells.contains(neighbor))
                .count();
            match (active, active_neighbors) {
                (true, 2) => Some(point),
                (_, 3) => Some(point),
                _ => None,
            }
        })
        .collect();

        Dimension { active_cells }
    }
}

#[aoc_generator(day17)]
fn generator(input: &str) -> Dimension {
    let active_cells = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, ch)| match ch {
                    '#' => Some((x as i32, y as i32, 0, 0)),
                    _ => None,
                })
        })
        .collect();

    Dimension { active_cells }
}

#[aoc(day17, part1)]
fn part1(start: &Dimension) -> usize {
    let start = start.clone();
    let dimension = iter::successors(Some(start), |prev| {
        let active_cells = prev
            .step()
            .active_cells
            .iter()
            .copied()
            .filter(|&(_, _, _, w)| w == 0)
            .collect();
        Some(Dimension { active_cells })
    })
    .take(STEPS)
    .last()
    .unwrap();

    dimension.active_cells.len()
}

#[aoc(day17, part2)]
fn part2(start: &Dimension) -> usize {
    let start = start.clone();
    let dimension = iter::successors(Some(start), |prev| Some(prev.step()))
        .take(STEPS)
        .last()
        .unwrap();

    dimension.active_cells.len()
}
