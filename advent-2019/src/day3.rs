use std::collections::HashMap;

use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};

type Point = (i32, i32);
type Wire = HashMap<Point, usize>;
const ORIGIN: Point = (0, 0);

fn parse_wire(input: &str) -> Wire {
    let mut wire = Wire::new();
    let mut position = ORIGIN;
    let mut steps = 0;

    for instruction in input.split(',') {
        let direction = instruction.chars().next().unwrap();
        let distance = instruction[1..].parse::<usize>().unwrap();

        for _ in 0..distance {
            steps += 1;
            match direction {
                'U' => position.1 += 1,
                'D' => position.1 -= 1,
                'L' => position.0 -= 1,
                'R' => position.0 += 1,
                _ => unreachable!(),
            }

            wire.entry(position).or_insert(steps);
        }
    }

    wire
}

#[aoc_generator(day3)]
fn generate(input: &str) -> anyhow::Result<(Wire, Wire)> {
    let mut wires = input.lines().map(parse_wire);
    let wire1 = wires.next().context("missing wire 1")?;
    let wire2 = wires.next().context("missing wire 2")?;
    Ok((wire1, wire2))
}

#[aoc(day3, part1)]
fn part1((wire1, wire2): &(Wire, Wire)) -> Option<i32> {
    wire1
        .keys()
        .filter(|&position| wire2.contains_key(position))
        .map(|&(x, y)| x.abs() + y.abs())
        .min()
}

#[aoc(day3, part2)]
fn part2((wire1, wire2): &(Wire, Wire)) -> Option<usize> {
    wire1
        .keys()
        .filter(|&position| wire2.contains_key(position))
        .map(|position| wire1[position] + wire2[position])
        .min()
}
