use std::collections::HashMap;

use anyhow::anyhow;
use aoc_runner_derive::{aoc, aoc_generator};
use nom::IResult;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Operand {
    Register(char),
    Literal(i32),
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Copy(Operand, Operand),
    Increment(Operand),
    Decrement(Operand),
    JumpNotZero(Operand, Operand),
    Toggle(Operand),
}

fn register(input: &str) -> IResult<&str, char> {
    use nom::{branch::alt, character::complete::char};
    alt((char('a'), char('b'), char('c'), char('d')))(input)
}

fn operand(input: &str) -> IResult<&str, Operand> {
    use nom::{branch::alt, character::complete::i32, combinator::map};
    alt((map(register, Operand::Register), map(i32, Operand::Literal)))(input)
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    use nom::{branch::alt, bytes::complete::tag, combinator::map, sequence::tuple};

    let copy = map(
        tuple((tag("cpy "), operand, tag(" "), operand)),
        |(_, op1, _, op2)| Instruction::Copy(op1, op2),
    );
    let increment = map(tuple((tag("inc "), operand)), |(_, op)| {
        Instruction::Increment(op)
    });
    let decrement = map(tuple((tag("dec "), operand)), |(_, op)| {
        Instruction::Decrement(op)
    });
    let jump_not_zero = map(
        tuple((tag("jnz "), operand, tag(" "), operand)),
        |(_, op1, _, op2)| Instruction::JumpNotZero(op1, op2),
    );
    let toggle = map(tuple((tag("tgl "), operand)), |(_, op)| {
        Instruction::Toggle(op)
    });

    alt((copy, increment, decrement, jump_not_zero, toggle))(input)
}

#[aoc_generator(day23)]
fn generator(input: &str) -> anyhow::Result<Vec<Instruction>> {
    input
        .lines()
        .map(|line| {
            instruction(line)
                .map(|(_, instruction)| instruction)
                .map_err(|_| anyhow!("Invalid input: {}", line))
        })
        .collect()
}

type Registers = HashMap<char, i32>;

fn default_registers() -> Registers {
    ['a', 'b', 'c', 'd'].iter().map(|&c| (c, 0)).collect()
}

fn execute(instructions: &[Instruction], mut registers: Registers) -> Registers {
    let mut instructions = instructions.to_vec();
    let mut pc = 0;

    while let Some(instruction) = instructions.get(pc).cloned() {
        match instruction {
            Instruction::Copy(op1, op2) => {
                let value = match op1 {
                    Operand::Register(register) => registers[&register],
                    Operand::Literal(value) => value,
                };

                if let Operand::Register(register) = op2 {
                    registers.insert(register, value);
                }

                pc += 1;
            }
            Instruction::Increment(op) => {
                if let Operand::Register(register) = op {
                    let value = registers[&register];
                    registers.insert(register, value + 1);
                }

                pc += 1;
            }
            Instruction::Decrement(op) => {
                if let Operand::Register(register) = op {
                    let value = registers[&register];
                    registers.insert(register, value - 1);
                }

                pc += 1;
            }
            Instruction::JumpNotZero(op1, op2) => {
                let value = match op1 {
                    Operand::Register(register) => registers[&register],
                    Operand::Literal(value) => value,
                };

                let offset = match op2 {
                    Operand::Register(register) => registers[&register],
                    Operand::Literal(value) => value,
                };

                if value != 0 {
                    pc = pc.saturating_add_signed(offset as isize);
                } else {
                    pc += 1;
                }
            }
            Instruction::Toggle(op) => {
                let value = match op {
                    Operand::Register(register) => registers[&register],
                    Operand::Literal(value) => value,
                };
                let offset = value as isize;
                let location = pc.saturating_add_signed(offset);

                if let Some(instruction) = instructions.get_mut(location) {
                    *instruction = match *instruction {
                        Instruction::Copy(op1, op2) => Instruction::JumpNotZero(op1, op2),
                        Instruction::JumpNotZero(op1, op2) => Instruction::Copy(op1, op2),
                        Instruction::Increment(op) => Instruction::Decrement(op),
                        Instruction::Decrement(op) => Instruction::Increment(op),
                        Instruction::Toggle(op) => Instruction::Increment(op),
                    };
                }

                pc += 1;
            }
        }
    }

    registers
}

#[aoc(day23, part1)]
fn part1(input: &[Instruction]) -> i32 {
    let mut registers = default_registers();
    registers.insert('a', 7);

    let registers = execute(input, registers);
    registers[&'a']
}

#[aoc(day23, part2)]
fn part2(input: &[Instruction]) -> i32 {
    let mut registers = default_registers();
    registers.insert('a', 12);

    let registers = execute(input, registers);
    registers[&'a']
}
