use std::{collections::HashSet, iter};

use anyhow::{Context, Result};
use itertools::{Itertools, iproduct};

const STEPS: usize = 7;

#[derive(Debug, Clone)]
struct Dimension {
    active_cells: HashSet<(i32, i32, i32, i32)>,
}

impl Dimension {
    fn step(&self) -> Result<Dimension> {
        let (min_x, max_x) = self
            .active_cells
            .iter()
            .map(|(x, _, _, _)| x)
            .minmax()
            .into_option()
            .context("No active cells for x dimension")?;
        let (min_y, max_y) = self
            .active_cells
            .iter()
            .map(|(_, y, _, _)| y)
            .minmax()
            .into_option()
            .context("No active cells for y dimension")?;
        let (min_z, max_z) = self
            .active_cells
            .iter()
            .map(|(_, _, z, _)| z)
            .minmax()
            .into_option()
            .context("No active cells for z dimension")?;
        let (min_w, max_w) = self
            .active_cells
            .iter()
            .map(|(_, _, _, w)| w)
            .minmax()
            .into_option()
            .context("No active cells for w dimension")?;

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

        Ok(Dimension { active_cells })
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
fn part1(start: &Dimension) -> Result<usize> {
    let start = start.clone();
    let dimension = iter::successors(Some(Ok(start)), |prev| {
        prev.as_ref().ok().map(|p| {
            p.step().map(|dim| {
                let active_cells = dim
                    .active_cells
                    .iter()
                    .copied()
                    .filter(|&(_, _, _, w)| w == 0)
                    .collect();
                Dimension { active_cells }
            })
        })
    })
    .take(STEPS)
    .last()
    .context("Failed to run simulation")??;

    Ok(dimension.active_cells.len())
}

#[aoc(day17, part2)]
fn part2(start: &Dimension) -> Result<usize> {
    let start = start.clone();
    let dimension = iter::successors(Some(Ok(start)), |prev| prev.as_ref().ok().map(|p| p.step()))
        .take(STEPS)
        .last()
        .context("Failed to run simulation")??;

    Ok(dimension.active_cells.len())
}
