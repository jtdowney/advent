use std::{collections::HashSet, str::FromStr};

use anyhow::anyhow;
use aoc_runner_derive::{aoc, aoc_generator};
use nom::{IResult, Parser};

#[derive(Clone, Copy, Debug)]
struct Vector {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug)]
struct Particle {
    position: Vector,
    velocity: Vector,
}

impl FromStr for Particle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use nom::{
            bytes::complete::tag,
            character::complete::{char, i32, space0, space1},
            combinator::map,
            sequence::{delimited, preceded, separated_pair},
        };

        fn vector(input: &str) -> IResult<&str, Vector> {
            map(
                delimited(
                    char('<'),
                    separated_pair(preceded(space0, i32), tag(", "), preceded(space0, i32)),
                    char('>'),
                ),
                |(x, y)| Vector { x, y },
            )
            .parse(input)
        }

        let mut parser = map(
            separated_pair(
                preceded(tag("position="), vector),
                space1,
                preceded(tag("velocity="), vector),
            ),
            |(position, velocity)| Particle { position, velocity },
        );

        parser
            .parse(s)
            .map(|(_, particle)| particle)
            .map_err(|e| anyhow!("Invalid particle: {}", e))
    }
}

#[aoc_generator(day10)]
fn generator(input: &str) -> anyhow::Result<Vec<Particle>> {
    input.lines().map(|line| line.parse()).collect()
}

fn bounds(particles: &[Particle]) -> (i32, i32, i32, i32) {
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for particle in particles {
        min_x = min_x.min(particle.position.x);
        max_x = max_x.max(particle.position.x);
        min_y = min_y.min(particle.position.y);
        max_y = max_y.max(particle.position.y);
    }

    (min_x, max_x, min_y, max_y)
}

fn area(particles: &[Particle]) -> i64 {
    let (min_x, max_x, min_y, max_y) = bounds(particles);
    (max_x - min_x) as i64 * (max_y - min_y) as i64
}

fn draw(particles: &[Particle]) -> String {
    let (min_x, max_x, min_y, max_y) = bounds(particles);
    let grid = particles
        .iter()
        .map(|&Particle { position, .. }| {
            let Vector { x, y } = position;
            (x, y)
        })
        .collect::<HashSet<(i32, i32)>>();
    let mut result = String::new();
    result.push('\n');

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            match grid.get(&(x, y)) {
                Some(_) => result.push('#'),
                None => result.push('.'),
            }
        }

        result.push('\n');
    }

    result
}

fn solve(particles: &[Particle]) -> (usize, String) {
    let mut particles = particles.to_vec();
    let mut last_area = area(&particles);
    let mut time = 0;
    for t in 0.. {
        let next_particles = particles
            .iter()
            .map(|&Particle { position, velocity }| Particle {
                position: Vector {
                    x: position.x + velocity.x,
                    y: position.y + velocity.y,
                },
                velocity,
            })
            .collect::<Vec<_>>();

        let next_area = area(&next_particles);
        if next_area > last_area {
            time = t;
            break;
        }

        particles = next_particles;
        last_area = next_area;
    }

    let screen = draw(&particles);
    (time, screen)
}

#[aoc(day10, part1)]
fn part1(input: &[Particle]) -> String {
    let (_, screen) = solve(input);
    screen
}

#[aoc(day10, part2)]
fn part2(input: &[Particle]) -> usize {
    let (t, _) = solve(input);
    t
}
