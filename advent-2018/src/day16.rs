use std::{
    collections::{HashMap, HashSet},
    num::ParseIntError,
};

use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

type Registers = Vec<usize>;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

#[derive(Copy, Clone, Debug)]
struct Instruction(Opcode, usize, usize, usize);

impl Instruction {
    fn apply(&self, before: &[usize]) -> Registers {
        let mut after = before.to_vec();
        match *self {
            Instruction(Opcode::Addr, a, b, c) => after[c] = before[a] + before[b],
            Instruction(Opcode::Addi, a, b, c) => after[c] = before[a] + b,
            Instruction(Opcode::Mulr, a, b, c) => after[c] = before[a] * before[b],
            Instruction(Opcode::Muli, a, b, c) => after[c] = before[a] * b,
            Instruction(Opcode::Banr, a, b, c) => after[c] = before[a] & before[b],
            Instruction(Opcode::Bani, a, b, c) => after[c] = before[a] & b,
            Instruction(Opcode::Borr, a, b, c) => after[c] = before[a] | before[b],
            Instruction(Opcode::Bori, a, b, c) => after[c] = before[a] | b,
            Instruction(Opcode::Setr, a, _, c) => after[c] = before[a],
            Instruction(Opcode::Seti, a, _, c) => after[c] = a,
            Instruction(Opcode::Gtir, a, b, c) => after[c] = if a > before[b] { 1 } else { 0 },
            Instruction(Opcode::Gtri, a, b, c) => after[c] = if before[a] > b { 1 } else { 0 },
            Instruction(Opcode::Gtrr, a, b, c) => {
                after[c] = if before[a] > before[b] { 1 } else { 0 }
            }
            Instruction(Opcode::Eqir, a, b, c) => after[c] = if a == before[b] { 1 } else { 0 },
            Instruction(Opcode::Eqri, a, b, c) => after[c] = if before[a] == b { 1 } else { 0 },
            Instruction(Opcode::Eqrr, a, b, c) => {
                after[c] = if before[a] == before[b] { 1 } else { 0 }
            }
        };

        after
    }
}

const ALL_OPCODES: [Opcode; 16] = [
    Opcode::Addr,
    Opcode::Addi,
    Opcode::Mulr,
    Opcode::Muli,
    Opcode::Banr,
    Opcode::Bani,
    Opcode::Borr,
    Opcode::Bori,
    Opcode::Setr,
    Opcode::Seti,
    Opcode::Gtir,
    Opcode::Gtri,
    Opcode::Gtrr,
    Opcode::Eqir,
    Opcode::Eqri,
    Opcode::Eqrr,
];

#[derive(Debug)]
struct Example {
    before: Registers,
    after: Registers,
    instruction: (u8, usize, usize, usize),
}

struct Input {
    examples: Vec<Example>,
    program: Vec<Vec<u8>>,
}

#[aoc_generator(day16)]
fn generator(input: &str) -> anyhow::Result<Input> {
    let parts = input
        .split("\n\n\n")
        .map(|part| part.into())
        .collect::<Vec<String>>();
    let input = parts[0]
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let re = Regex::new(r"(\d)").unwrap();

    let examples = input
        .chunks(3)
        .map(|lines| {
            let before = re
                .captures_iter(&lines[0])
                .map(|c| c[1].parse().unwrap())
                .collect();

            let mut instruction_parts = lines[1].split_whitespace();
            let instruction1 = instruction_parts
                .next()
                .and_then(|s| s.parse().ok())
                .context("unable to parse instruction")?;
            let instruction2 = instruction_parts
                .next()
                .and_then(|s| s.parse().ok())
                .context("unable to parse instruction")?;
            let instruction3 = instruction_parts
                .next()
                .and_then(|s| s.parse().ok())
                .context("unable to parse instruction")?;
            let instruction4 = instruction_parts
                .next()
                .and_then(|s| s.parse().ok())
                .context("unable to parse instruction")?;
            let instruction = (instruction1, instruction2, instruction3, instruction4);

            let after = re
                .captures_iter(&lines[2])
                .map(|c| c[1].parse())
                .collect::<Result<_, ParseIntError>>()?;

            Ok(Example {
                before,
                after,
                instruction,
            })
        })
        .collect::<anyhow::Result<Vec<Example>>>()?;

    let program = parts[1]
        .lines()
        .filter(|s| !s.is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|item| item.parse().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<u8>>>();

    Ok(Input { examples, program })
}

#[aoc(day16, part1)]
fn part1(input: &Input) -> usize {
    input
        .examples
        .iter()
        .map(
            |Example {
                 before,
                 after,
                 instruction: (_, a, b, c),
             }| {
                ALL_OPCODES
                    .iter()
                    .map(|opcode| Instruction(*opcode, *a, *b, *c))
                    .filter(|instruction| &instruction.apply(before) == after)
                    .count()
            },
        )
        .filter(|&count| count >= 3)
        .count()
}

#[aoc(day16, part2)]
fn part2(input: &Input) -> usize {
    let mut candidates = input
        .examples
        .iter()
        .map(
            |Example {
                 before,
                 after,
                 instruction: (code, a, b, c),
             }| {
                (
                    *code,
                    ALL_OPCODES
                        .iter()
                        .map(|opcode| (opcode, Instruction(*opcode, *a, *b, *c)))
                        .filter(|(_, instruction)| &instruction.apply(before) == after)
                        .map(|(&opcode, _)| opcode)
                        .collect(),
                )
            },
        )
        .collect::<HashMap<u8, HashSet<Opcode>>>();

    let mut opcode_mapping = HashMap::new();
    loop {
        for code in opcode_mapping.keys() {
            candidates.remove(code);
        }

        if candidates.is_empty() {
            break;
        }

        let used_opcodes = opcode_mapping
            .values()
            .copied()
            .collect::<HashSet<Opcode>>();
        let filtered_candidates = candidates
            .iter()
            .map(|(code, opcodes)| {
                (
                    code,
                    opcodes
                        .iter()
                        .filter(|opcode| !used_opcodes.contains(opcode))
                        .copied()
                        .collect::<Vec<Opcode>>(),
                )
            })
            .filter(|(_, opcodes)| opcodes.len() == 1)
            .map(|(code, opcodes)| (*code, opcodes[0]))
            .collect::<HashMap<u8, Opcode>>();

        opcode_mapping.extend(filtered_candidates);
    }

    let instructions = input.program.iter().map(|args| {
        Instruction(
            opcode_mapping[&args[0]],
            args[1] as usize,
            args[2] as usize,
            args[3] as usize,
        )
    });

    let mut registers = vec![0, 0, 0, 0];
    for instruction in instructions {
        registers = instruction.apply(&registers);
    }

    registers[0]
}
