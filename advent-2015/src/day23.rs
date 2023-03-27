use std::ops::{Index, IndexMut};

use anyhow::anyhow;
use aoc_runner_derive::{aoc, aoc_generator};
use nom::{Finish, IResult};

type Register = char;

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(i32),
    JumpIfEven(Register, i32),
    JumpIfOne(Register, i32),
}

fn register(input: &str) -> IResult<&str, Register> {
    use nom::character::complete::alpha1;
    use nom::combinator::map_opt;

    map_opt(alpha1, |s: &str| s.chars().next())(input)
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::combinator::map;
    use nom::sequence::{preceded, tuple};

    let offset = nom::character::complete::i32;
    let half = map(preceded(tag("hlf "), register), Instruction::Half);
    let triple = map(preceded(tag("tpl "), register), Instruction::Triple);
    let increment = map(preceded(tag("inc "), register), Instruction::Increment);
    let jump = map(preceded(tag("jmp "), offset), Instruction::Jump);
    let jump_if_even = map(
        tuple((tag("jie "), register, tag(", "), offset)),
        |(_, r, _, o)| Instruction::JumpIfEven(r, o),
    );
    let jump_if_one = map(
        tuple((tag("jio "), register, tag(", "), offset)),
        |(_, r, _, o)| Instruction::JumpIfOne(r, o),
    );

    alt((half, triple, increment, jump, jump_if_even, jump_if_one))(input)
}

#[aoc_generator(day23)]
fn generator(input: &str) -> anyhow::Result<Vec<Instruction>> {
    input
        .lines()
        .map(|line| {
            instruction(line)
                .finish()
                .map(|(_, i)| i)
                .map_err(|_| anyhow!("Invalid instruction: {}", line))
        })
        .collect()
}

struct Registers {
    data: [u32; 2],
}

fn register_to_index(register: Register) -> usize {
    (register as u8 - b'a') as usize
}

impl Index<Register> for Registers {
    type Output = u32;

    fn index(&self, index: Register) -> &Self::Output {
        let index = register_to_index(index);
        &self.data[index]
    }
}

impl IndexMut<Register> for Registers {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        let index = register_to_index(index);
        &mut self.data[index]
    }
}

fn execute(instructions: &[Instruction], mut registers: Registers) -> u32 {
    let mut pc = 0;

    while let Some(&instruction) = instructions.get(pc) {
        match instruction {
            Instruction::Half(r) => {
                registers[r] /= 2;
                pc += 1;
            }
            Instruction::Triple(r) => {
                registers[r] *= 3;
                pc += 1;
            }
            Instruction::Increment(r) => {
                registers[r] += 1;
                pc += 1;
            }
            Instruction::Jump(offset) => {
                pc = (pc as i32 + offset) as usize;
            }
            Instruction::JumpIfEven(r, offset) => {
                if registers[r] % 2 == 0 {
                    pc = (pc as i32 + offset) as usize;
                } else {
                    pc += 1;
                }
            }
            Instruction::JumpIfOne(r, offset) => {
                if registers[r] == 1 {
                    pc = (pc as i32 + offset) as usize;
                } else {
                    pc += 1;
                }
            }
        }
    }

    registers['b']
}

#[aoc(day23, part1)]
fn part1(input: &[Instruction]) -> u32 {
    let registers = Registers { data: [0, 0] };
    execute(input, registers)
}

#[aoc(day23, part2)]
fn part2(input: &[Instruction]) -> u32 {
    let registers = Registers { data: [1, 0] };
    execute(input, registers)
}
