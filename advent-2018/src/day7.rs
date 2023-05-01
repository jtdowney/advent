use std::collections::{HashMap, HashSet};

use anyhow::anyhow;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy)]
struct Work {
    step: char,
    completion_time: usize,
}

fn completion_time(step: char) -> usize {
    (step as usize) - 64 + 60
}

fn parse_step(source: &str) -> Option<(char, char)> {
    let mut parts = source.split_whitespace();
    let id = parts.nth(1).and_then(|p| p.chars().next())?;
    let block = parts.nth(5).and_then(|p| p.chars().next())?;

    Some((id, block))
}

struct Input {
    step_blocks: HashMap<char, Vec<char>>,
    step_blocked_by: HashMap<char, Vec<char>>,
    ready: HashSet<char>,
}

#[aoc_generator(day7)]
fn generator(input: &str) -> anyhow::Result<Input> {
    let input = input
        .lines()
        .map(parse_step)
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| anyhow!("Invalid input"))?;

    let mut step_blocks = HashMap::new();
    let mut step_blocked_by = HashMap::new();
    for &(step, block) in &input {
        step_blocks.entry(step).or_insert_with(Vec::new).push(block);
        step_blocked_by
            .entry(block)
            .or_insert_with(Vec::new)
            .push(step);
    }

    let ready = input
        .iter()
        .filter(|(step, _)| !step_blocked_by.contains_key(step))
        .map(|&(step, _)| step)
        .collect::<HashSet<char>>();

    Ok(Input {
        step_blocks,
        step_blocked_by,
        ready,
    })
}

#[aoc(day7, part1)]
fn part1(input: &Input) -> String {
    let mut completed = HashSet::new();
    let mut ready = input.ready.clone();
    let mut result = String::new();

    while let Some(&step) = ready.iter().min() {
        result.push(step);
        ready.remove(&step);
        completed.insert(step);

        let children = input
            .step_blocks
            .get(&step)
            .map(|c| c.as_slice())
            .unwrap_or_default();
        for &child in children {
            if let Some(parents) = input.step_blocked_by.get(&child) {
                if parents.iter().all(|p| completed.contains(p)) {
                    ready.insert(child);
                }
            } else {
                ready.insert(child);
            }
        }
    }

    result
}

#[aoc(day7, part2)]
fn part2(input: &Input) -> Option<usize> {
    let mut completed = HashSet::new();
    let mut ready = input.ready.clone();
    let mut workers: [Option<Work>; 5] = Default::default();

    let max_steps = input
        .step_blocks
        .keys()
        .chain(input.step_blocked_by.keys())
        .collect::<HashSet<&char>>()
        .len();

    (0..).find(|&t| {
        for worker in workers.iter_mut() {
            match *worker {
                Some(Work {
                    step,
                    completion_time,
                }) if t == completion_time => {
                    completed.insert(step);

                    let children = input
                        .step_blocks
                        .get(&step)
                        .map(|c| c.as_slice())
                        .unwrap_or_default();
                    for &child in children {
                        if let Some(parents) = input.step_blocked_by.get(&child) {
                            if parents.iter().all(|p| completed.contains(p)) {
                                ready.insert(child);
                            }
                        } else {
                            ready.insert(child);
                        }
                    }
                }
                None => {}
                _ => continue,
            }

            if let Some(&step) = ready.iter().min() {
                ready.remove(&step);
                let completion_time = t + completion_time(step);
                *worker = Some(Work {
                    step,
                    completion_time,
                });
            } else {
                *worker = None;
            }
        }

        completed.len() == max_steps
    })
}
