use std::{cmp::Ordering, collections::HashSet};

use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};
use num::integer::lcm;

type Vector = (i64, i64, i64);

#[derive(Copy, Clone)]
struct Moon {
    position: Vector,
    velocity: Vector,
}

impl Moon {
    fn potenial_energy(self) -> i64 {
        self.position.0.abs() + self.position.1.abs() + self.position.2.abs()
    }

    fn kinetic_energy(self) -> i64 {
        self.velocity.0.abs() + self.velocity.1.abs() + self.velocity.2.abs()
    }

    fn total_energy(self) -> i64 {
        self.potenial_energy() * self.kinetic_energy()
    }

    fn accelerate(&mut self, delta: Vector) {
        self.velocity.0 += delta.0;
        self.velocity.1 += delta.1;
        self.velocity.2 += delta.2;
    }

    fn update_position(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        self.position.2 += self.velocity.2;
    }
}

fn delta(a: i64, b: i64) -> i64 {
    match a.cmp(&b) {
        Ordering::Less => 1,
        Ordering::Greater => -1,
        Ordering::Equal => 0,
    }
}

fn calculate_accleration(moons: &mut [Moon], index: usize) {
    let length = moons.len();
    let delta = (0..length)
        .filter(|&j| j != index)
        .map(|j| {
            let (ax, ay, az) = moons[index].position;
            let (bx, by, bz) = moons[j].position;

            (delta(ax, bx), delta(ay, by), delta(az, bz))
        })
        .fold(Vector::default(), |(x, y, z), (dx, dy, dz)| {
            (x + dx, y + dy, z + dz)
        });

    moons[index].accelerate(delta);
}

#[aoc_generator(day12)]
fn generate(input: &str) -> anyhow::Result<Vec<Moon>> {
    input
        .lines()
        .map(|line| {
            let parts = line
                .trim_matches(['<', '>'].as_ref())
                .split(", ")
                .map(|part| {
                    part.split('=')
                        .next_back()
                        .context("no value after =")?
                        .parse()
                        .context("parse number")
                })
                .collect::<anyhow::Result<Vec<i64>>>()?;
            let position = (parts[0], parts[1], parts[2]);
            Ok(Moon {
                position,
                velocity: Vector::default(),
            })
        })
        .collect()
}

#[aoc(day12, part1)]
fn part1(input: &[Moon]) -> i64 {
    let mut moons = input.to_vec();
    let length = moons.len();

    for _ in 0..1000 {
        for i in 0..length {
            calculate_accleration(&mut moons, i);
        }

        for moon in moons.iter_mut() {
            moon.update_position();
        }
    }

    moons.iter().map(|m| m.total_energy()).sum()
}

#[aoc(day12, part2)]
fn part2(input: &[Moon]) -> u64 {
    let length = input.len();
    let mut times = (0u64, 0, 0);

    let mut moons = input.to_vec();
    let mut x_positions = HashSet::new();
    for t in 0.. {
        for i in 0..length {
            calculate_accleration(&mut moons, i);
        }

        for moon in moons.iter_mut() {
            moon.update_position();
        }

        let positions = moons
            .iter()
            .map(|moon| (moon.position.0, moon.velocity.0))
            .collect::<Vec<(i64, i64)>>();
        if x_positions.contains(&positions) {
            times.0 = t;
            break;
        } else {
            x_positions.insert(positions);
        }
    }

    let mut moons = input.to_vec();
    let mut y_positions = HashSet::new();
    for t in 0.. {
        for i in 0..length {
            calculate_accleration(&mut moons, i);
        }

        for moon in moons.iter_mut() {
            moon.update_position();
        }

        let positions = moons
            .iter()
            .map(|moon| (moon.position.1, moon.velocity.1))
            .collect::<Vec<(i64, i64)>>();
        if y_positions.contains(&positions) {
            times.1 = t;
            break;
        } else {
            y_positions.insert(positions);
        }
    }

    let mut moons = input.to_vec();
    let mut z_positions = HashSet::new();
    for t in 0.. {
        for i in 0..length {
            calculate_accleration(&mut moons, i);
        }

        for moon in moons.iter_mut() {
            moon.update_position();
        }

        let positions = moons
            .iter()
            .map(|moon| (moon.position.2, moon.velocity.2))
            .collect::<Vec<(i64, i64)>>();
        if z_positions.contains(&positions) {
            times.2 = t;
            break;
        } else {
            z_positions.insert(positions);
        }
    }

    let a = lcm(times.0, times.1);
    lcm(a, times.2)
}
