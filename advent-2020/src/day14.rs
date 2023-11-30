use std::{
    collections::HashMap,
    str::{self, FromStr},
};

#[derive(Debug)]
enum Instruction {
    Mask(String),
    Memory(u64, u64),
}

#[derive(Debug, Default)]
struct State {
    mask: String,
    memory: HashMap<u64, u64>,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parts = value.split('=').map(str::trim).collect::<Vec<&str>>();
        let instruction = if parts[0].starts_with("mem[") {
            let end = parts[0].bytes().position(|b| b == b']').unwrap();
            let address = parts[0][4..end].parse().unwrap();
            let value = parts[1].parse().unwrap();
            Instruction::Memory(address, value)
        } else {
            let value = parts[1].to_string();
            Instruction::Mask(value)
        };

        Ok(instruction)
    }
}

#[aoc_generator(day14)]
fn generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap()
}

#[aoc(day14, part1)]
fn part1(instructions: &[Instruction]) -> u64 {
    let state =
        instructions.iter().fold(
            State::default(),
            |mut state, instruction| match instruction {
                Instruction::Mask(m) => {
                    state.mask = m.clone();
                    state
                }
                Instruction::Memory(address, value) => {
                    let value_bits = format!("{:036b}", value);
                    let result = state
                        .mask
                        .chars()
                        .zip(value_bits.chars())
                        .map(|(m, v)| if m == 'X' { v } else { m })
                        .collect::<String>();

                    let masked_value = u64::from_str_radix(&result, 2).unwrap();
                    *state.memory.entry(*address).or_default() = masked_value;
                    state
                }
            },
        );

    state.memory.values().sum()
}

#[aoc(day14, part2)]
#[allow(clippy::needless_collect)]
fn part2(instructions: &[Instruction]) -> u64 {
    let state =
        instructions.iter().fold(
            State::default(),
            |mut state, instruction| match instruction {
                Instruction::Mask(m) => {
                    state.mask = m.clone();
                    state
                }
                Instruction::Memory(address, value) => {
                    let address_bits = format!("{:036b}", address);
                    let template = state
                        .mask
                        .chars()
                        .zip(address_bits.chars())
                        .map(|(m, v)| if m == '0' { v } else { m })
                        .collect::<String>();
                    let floating_positions = template
                        .chars()
                        .enumerate()
                        .filter(|&(_, ch)| ch == 'X')
                        .map(|(i, _)| i)
                        .collect::<Vec<usize>>();
                    let max = 2u32.pow(floating_positions.len() as u32);
                    (0..max)
                        .filter_map(|n| {
                            let bits = format!("{:032b}", n)
                                .chars()
                                .rev()
                                .take(floating_positions.len())
                                .enumerate()
                                .map(|(i, c)| (floating_positions[i], c))
                                .collect::<HashMap<usize, char>>();
                            let address = template
                                .chars()
                                .enumerate()
                                .map(|(p, ch)| if ch == 'X' { bits[&p] } else { ch })
                                .collect::<String>();
                            u64::from_str_radix(&address, 2).ok()
                        })
                        .for_each(|address| {
                            *state.memory.entry(address).or_default() = *value;
                        });
                    state
                }
            },
        );

    state.memory.values().sum()
}
