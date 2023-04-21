use std::collections::{HashMap, HashSet, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};

use crate::knot_hasher::{Digest, KnotHasher};

type Point = (i32, i32);

#[aoc_generator(day14)]
fn generator(input: &str) -> Vec<Digest> {
    (0..128)
        .map(|i| {
            let mut hasher = KnotHasher::default();
            hasher.hash(format!("{}-{}", input, i).as_bytes())
        })
        .collect()
}

#[aoc(day14, part1)]
fn part1(input: &[Digest]) -> u32 {
    input
        .iter()
        .map(|Digest(data)| data.iter().map(|b| b.count_ones()).sum::<u32>())
        .sum()
}

#[aoc(day14, part2)]
fn part2(input: &[Digest]) -> u32 {
    let used = input
        .iter()
        .enumerate()
        .flat_map(|(y, Digest(data))| {
            data.iter()
                .flat_map(|&n| {
                    let mut n = n;
                    let mut bits = [false; 8];
                    for i in (0..u8::BITS).rev() {
                        bits[i as usize] = n & 1 == 1;
                        n >>= 1;
                    }
                    bits.into_iter()
                })
                .enumerate()
                .filter(|&(_, cell)| cell)
                .map(move |(x, _)| (x as i32, y as i32))
        })
        .collect::<HashSet<Point>>();

    let mut regions = HashMap::new();
    let mut next_region = 0;
    for y in 0..128 {
        for x in 0..128 {
            let point = (x, y);
            if !used.contains(&point) || regions.contains_key(&point) {
                continue;
            }

            next_region += 1;

            let mut queue = VecDeque::from_iter([point]);
            while let Some(current @ (cx, cy)) = queue.pop_front() {
                if used.contains(&current) && !regions.contains_key(&current) {
                    regions.insert(current, next_region);
                } else {
                    continue;
                }

                for (dx, dy) in [(0, -1), (-1, 0), (0, 1), (1, 0)] {
                    let nx = cx + dx;
                    let ny = cy + dy;

                    if (0..=127).contains(&nx) && (0..=127).contains(&ny) {
                        queue.push_back((nx, ny));
                    }
                }
            }
        }
    }

    next_region
}
