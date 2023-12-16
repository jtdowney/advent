use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day15)]
fn generator(input: &str) -> Vec<String> {
    input.split(',').map(String::from).collect()
}

fn hash(input: &str) -> u8 {
    input.bytes().fold(0, |acc, value| {
        let next = (acc as u32 + value as u32) * 17;
        (next % 256) as u8
    })
}

#[aoc(day15, part1)]
fn part1(input: &[String]) -> u32 {
    input.iter().map(|s| hash(s) as u32).sum()
}

#[aoc(day15, part2)]
fn part2(input: &[String]) -> u32 {
    let mut boxes = HashMap::<u8, Vec<(&str, u32)>>::new();
    for item in input {
        if let Some((label, focal)) = item.split_once('=') {
            let focal = focal.parse::<u32>().expect("number");
            let key = hash(label);

            let lenses = boxes.entry(key).or_default();
            if let Some(i) = lenses.iter().position(|&(l, _)| l == label) {
                lenses[i] = (label, focal);
            } else {
                lenses.push((label, focal));
            }
        } else {
            let label = item.trim_end_matches('-');
            let key = hash(label);

            let lenses = boxes.entry(key).or_default();
            if let Some(i) = lenses.iter().position(|&(l, _)| l == label) {
                lenses.remove(i);
            }
        }
    }

    boxes
        .iter()
        .map(|(id, slots)| {
            slots
                .iter()
                .enumerate()
                .map(|(slot, &(_, focal))| (*id as u32 + 1) * (slot as u32 + 1) * focal)
                .sum::<u32>()
        })
        .sum()
}
