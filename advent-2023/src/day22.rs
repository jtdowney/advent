use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::{anyhow, bail};
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Brick {
    start: (usize, usize, usize),
    end: (usize, usize, usize),
}

impl Brick {
    fn min_z(&self) -> usize {
        let (_, _, z1) = self.start;
        let (_, _, z2) = self.end;
        z1.min(z2)
    }

    fn max_z(&self) -> usize {
        let (_, _, z1) = self.start;
        let (_, _, z2) = self.end;
        z1.max(z2)
    }

    fn occupies(&self) -> Vec<(usize, usize, usize)> {
        let (x1, y1, z1) = self.start;
        let (x2, y2, z2) = self.end;

        (x1.min(x2)..=x1.max(x2))
            .flat_map(|x| {
                (y1.min(y2)..=y1.max(y2))
                    .flat_map(move |y| (z1.min(z2)..=z1.max(z2)).map(move |z| (x, y, z)))
            })
            .collect()
    }

    fn drop_to(&self, new_z: usize) -> Brick {
        let z_diff = self.min_z() - new_z;
        let (x1, y1, z1) = self.start;
        let (x2, y2, z2) = self.end;
        Brick {
            start: (x1, y1, z1 - z_diff),
            end: (x2, y2, z2 - z_diff),
        }
    }

    fn overlaps_xy(&self, other: &Brick) -> bool {
        let (x1, y1, _) = self.start;
        let (x2, y2, _) = self.end;
        let (other_x1, other_y1, _) = other.start;
        let (other_x2, other_y2, _) = other.end;

        let x_overlap =
            x1.min(x2) <= other_x1.max(other_x2) && other_x1.min(other_x2) <= x1.max(x2);
        let y_overlap =
            y1.min(y2) <= other_y1.max(other_y2) && other_y1.min(other_y2) <= y1.max(y2);

        x_overlap && y_overlap
    }
}

impl FromStr for Brick {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (start_str, end_str) = s
            .split_once('~')
            .ok_or_else(|| anyhow!("Invalid brick format"))?;

        let start_coords: Vec<usize> = start_str
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;

        let end_coords: Vec<usize> = end_str
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;

        match (start_coords.as_slice(), end_coords.as_slice()) {
            ([x1, y1, z1], [x2, y2, z2]) => Ok(Brick {
                start: (*x1, *y1, *z1),
                end: (*x2, *y2, *z2),
            }),
            _ => bail!("Invalid coordinate count"),
        }
    }
}

fn build_support_relationships(
    bricks: &[Brick],
) -> (
    HashMap<usize, HashSet<usize>>,
    HashMap<usize, HashSet<usize>>,
) {
    let mut supports: HashMap<usize, HashSet<usize>> =
        (0..bricks.len()).map(|i| (i, HashSet::new())).collect();
    let mut supported_by: HashMap<usize, HashSet<usize>> =
        (0..bricks.len()).map(|i| (i, HashSet::new())).collect();

    for (i, brick) in bricks.iter().enumerate() {
        for (j, other) in bricks.iter().enumerate() {
            if i != j && other.max_z() + 1 == brick.min_z() && brick.overlaps_xy(other) {
                supports.get_mut(&j).unwrap().insert(i);
                supported_by.get_mut(&i).unwrap().insert(j);
            }
        }
    }

    (supports, supported_by)
}

fn settle_bricks(bricks: &[Brick]) -> Vec<Brick> {
    let mut sorted_bricks = bricks.to_vec();
    sorted_bricks.sort_by_key(Brick::min_z);

    let mut settled = Vec::new();
    let mut occupied = HashSet::new();

    for brick in sorted_bricks {
        let mut current = brick.clone();

        while current.min_z() > 1 {
            let dropped = current.drop_to(current.min_z() - 1);
            if dropped.occupies().iter().any(|pos| occupied.contains(pos)) {
                break;
            }
            current = dropped;
        }

        occupied.extend(current.occupies());
        settled.push(current);
    }

    settled
}

fn count_falling_bricks(bricks: &[Brick], removed_index: usize) -> usize {
    let (_, supported_by) = build_support_relationships(bricks);
    let mut falling = HashSet::from([removed_index]);

    loop {
        let new_falling: HashSet<_> = (0..bricks.len())
            .filter(|&i| !falling.contains(&i))
            .filter(|&i| {
                supported_by.get(&i).is_some_and(|supporters| {
                    !supporters.is_empty() && supporters.iter().all(|s| falling.contains(s))
                })
            })
            .collect();

        if new_falling.is_empty() {
            break;
        }

        falling.extend(&new_falling);
    }

    falling.len() - 1
}

fn count_safe_to_disintegrate(bricks: &[Brick]) -> usize {
    let (supports, supported_by) = build_support_relationships(bricks);

    (0..bricks.len())
        .filter(|&i| {
            supports.get(&i).is_none_or(|supported| {
                supported.iter().all(|&brick| {
                    supported_by
                        .get(&brick)
                        .is_none_or(|supporters| supporters.len() > 1)
                })
            })
        })
        .count()
}

#[aoc_generator(day22)]
fn generator(input: &str) -> anyhow::Result<Vec<Brick>> {
    input.lines().map(str::parse).collect()
}

#[aoc(day22, part1)]
fn part1(bricks: &[Brick]) -> usize {
    let settled = settle_bricks(bricks);
    count_safe_to_disintegrate(&settled)
}

#[aoc(day22, part2)]
fn part2(bricks: &[Brick]) -> usize {
    let settled = settle_bricks(bricks);

    (0..settled.len())
        .map(|i| count_falling_bricks(&settled, i))
        .sum()
}
