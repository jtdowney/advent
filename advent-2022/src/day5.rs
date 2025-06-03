use std::collections::{BTreeMap, HashMap};

use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};

struct Instruction {
    count: usize,
    source: u32,
    destination: u32,
}

struct Input {
    stacks: BTreeMap<u32, Vec<char>>,
    instructions: Vec<Instruction>,
}

fn parse_stacks(input: &str) -> anyhow::Result<BTreeMap<u32, Vec<char>>> {
    let mut lines = input.lines().rev();
    let labels = lines
        .next()
        .map(|line| {
            line.chars()
                .enumerate()
                .filter_map(|(i, c)| c.to_digit(10).map(|n| (i, n)))
                .collect::<HashMap<usize, u32>>()
        })
        .context("parsing labels")?;

    let stacks = lines.fold(BTreeMap::<u32, Vec<char>>::new(), |mut acc, line| {
        for (i, c) in line.chars().enumerate() {
            if !c.is_ascii_alphabetic() {
                continue;
            }

            match labels.get(&i) {
                Some(&s) => {
                    acc.entry(s).or_default().push(c);
                }
                None => continue,
            }
        }

        acc
    });

    Ok(stacks)
}

fn parse_instructions(input: &str) -> anyhow::Result<Vec<Instruction>> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let _ = parts.next()?;
            let count = parts.next().and_then(|m| m.parse().ok())?;
            let _ = parts.next()?;
            let source = parts.next().and_then(|m| m.parse().ok())?;
            let _ = parts.next()?;
            let destination = parts.next().and_then(|m| m.parse().ok())?;
            Some(Instruction {
                count,
                source,
                destination,
            })
        })
        .collect::<Option<Vec<_>>>()
        .context("parsing instructions")
}

#[aoc_generator(day5)]
fn generator(input: &str) -> anyhow::Result<Input> {
    let mut parts = input.split("\n\n");

    let stacks = parts
        .next()
        .context("reading stacks")
        .and_then(parse_stacks)?;
    let instructions = parts
        .next()
        .context("reading instructions")
        .and_then(parse_instructions)?;

    Ok(Input {
        stacks,
        instructions,
    })
}

#[aoc(day5, part1)]
fn part1(input: &Input) -> anyhow::Result<String> {
    let stacks = input.instructions.iter().try_fold(
        input.stacks.clone(),
        |mut stacks,
         &Instruction {
             count,
             source,
             destination,
         }| {
            let source_stack = stacks.get_mut(&source).context("getting source")?;
            let mut moving = source_stack
                .drain((source_stack.len() - count)..)
                .collect::<Vec<_>>();
            moving.reverse();

            let destination_stack = stacks
                .get_mut(&destination)
                .context("getting destination")?;
            destination_stack.extend_from_slice(&moving);

            Ok::<_, anyhow::Error>(stacks)
        },
    )?;

    let answer = stacks.values().filter_map(|stack| stack.last()).collect();
    Ok(answer)
}

#[aoc(day5, part2)]
fn part2(input: &Input) -> anyhow::Result<String> {
    let stacks = input.instructions.iter().try_fold(
        input.stacks.clone(),
        |mut stacks,
         &Instruction {
             count,
             source,
             destination,
         }| {
            let source_stack = stacks.get_mut(&source).context("getting source")?;
            let moving = source_stack
                .drain((source_stack.len() - count)..)
                .collect::<Vec<_>>();

            let destination_stack = stacks
                .get_mut(&destination)
                .context("getting destination")?;
            destination_stack.extend_from_slice(&moving);

            Ok::<_, anyhow::Error>(stacks)
        },
    )?;

    let answer = stacks.values().filter_map(|stack| stack.last()).collect();
    Ok(answer)
}
