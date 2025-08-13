use std::collections::HashMap;

use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy, Debug)]
struct Layer {
    depth: u16,
    range: u16,
    position: u16,
    reverse: bool,
}

impl Layer {
    fn step(&mut self) {
        if self.reverse {
            self.position -= 1;
        } else {
            self.position += 1;
        }

        if self.position == 0 {
            self.reverse = false;
        } else if self.position == self.range - 1 {
            self.reverse = true;
        }
    }
}

#[aoc_generator(day13)]
fn generator(input: &str) -> anyhow::Result<Vec<Layer>> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let depth = parts.next().context("unable to find depth")?.parse()?;
            let range = parts.next().context("unable to find range")?.parse()?;
            Ok(Layer {
                depth,
                range,
                position: 0,
                reverse: false,
            })
        })
        .collect()
}

fn trace_packet(layers: &[Layer]) -> u16 {
    let last = layers.last().unwrap().depth;
    let mut layers = layers
        .iter()
        .map(|&l| (l.depth, l))
        .collect::<HashMap<u16, Layer>>();

    (0..=last).fold(0, |mut acc, t| {
        if let Some(layer) = layers.get(&t)
            && layer.position == 0
        {
            acc += layer.depth * layer.range;
        }

        for layer in layers.values_mut() {
            layer.step();
        }

        acc
    })
}

#[aoc(day13, part1)]
fn part1(input: &[Layer]) -> u16 {
    trace_packet(input)
}

#[aoc(day13, part2)]
fn part2(input: &[Layer]) -> Option<u16> {
    let start = input.to_vec();
    (0..)
        .scan(start, |layers, t| {
            for layer in layers.iter_mut() {
                layer.step();
            }

            let score = trace_packet(layers);
            Some((t, score))
        })
        .find_map(|(t, score)| if score == 0 { Some(t) } else { None })
}
