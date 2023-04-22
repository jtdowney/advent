use std::ops::{Add, AddAssign};

use anyhow::anyhow;
use aoc_runner_derive::{aoc, aoc_generator};
use nom::{Finish, IResult};

const ITERATIONS: usize = 500;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Vector {
    x: i64,
    y: i64,
    z: i64,
}

impl Vector {
    fn distance(&self, other: Vector) -> i64 {
        (self.x + other.x).abs() + (self.y + other.y).abs() + (self.z + other.z).abs()
    }

    fn distance_from_origin(&self) -> i64 {
        self.distance(Vector::default())
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

#[derive(Clone, Copy, Debug)]
struct Particle {
    position: Vector,
    velocity: Vector,
    acceleration: Vector,
}

impl Particle {
    fn is_coliding(&self, other: Particle) -> bool {
        self.position == other.position
    }
}

fn vector(input: &str) -> IResult<&str, Vector> {
    use nom::{
        bytes::complete::tag,
        character::complete::i64,
        combinator::map,
        sequence::{delimited, tuple},
    };

    map(
        delimited(
            tag("<"),
            tuple((i64, tag(","), i64, tag(","), i64)),
            tag(">"),
        ),
        |(x, _, y, _, z)| Vector { x, y, z },
    )(input)
}

fn particle(input: &str) -> IResult<&str, Particle> {
    use nom::{
        bytes::complete::tag,
        combinator::map,
        sequence::{preceded, tuple},
    };

    map(
        tuple((
            preceded(tag("p="), vector),
            tag(", "),
            preceded(tag("v="), vector),
            tag(", "),
            preceded(tag("a="), vector),
        )),
        |(position, _, velocity, _, acceleration)| Particle {
            position,
            velocity,
            acceleration,
        },
    )(input)
}

#[aoc_generator(day20)]
fn generator(input: &str) -> anyhow::Result<Vec<Particle>> {
    input
        .lines()
        .map(|line| {
            particle(line)
                .finish()
                .map(|(_, p)| p)
                .map_err(|e| anyhow!("unable to parse line {:?}: {}", line, e))
        })
        .collect()
}

#[aoc(day20, part1)]
fn part1(input: &[Particle]) -> Option<usize> {
    let mut particles = input.to_vec();
    for _ in 0..ITERATIONS {
        for particle in particles.iter_mut() {
            particle.velocity += particle.acceleration;
        }

        for particle in particles.iter_mut() {
            particle.position += particle.velocity;
        }
    }

    particles
        .iter()
        .enumerate()
        .min_by_key(|&(_, particle)| particle.position.distance_from_origin())
        .map(|(id, _)| id)
}

#[aoc(day20, part2)]
fn part2(input: &[Particle]) -> usize {
    let mut particles = input.to_vec();
    for _ in 0..ITERATIONS {
        for particle in particles.iter_mut() {
            particle.velocity += particle.acceleration;
        }

        for particle in particles.iter_mut() {
            particle.position += particle.velocity;
        }

        let all_particles = particles.clone();
        particles.retain(|p1| {
            all_particles
                .iter()
                .filter(|&p2| p1.is_coliding(*p2))
                .count()
                == 1
        })
    }

    particles.len()
}
