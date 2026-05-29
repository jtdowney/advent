use std::collections::HashMap;

use anyhow::anyhow;
use aoc_runner_derive::{aoc, aoc_generator};
use nom::{IResult, Parser};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Register(char);

enum Operand {
    Register(Register),
    Literal(i32),
}

enum Instruction {
    Copy(Operand, Register),
    Increment(Register),
    Decrement(Register),
    JumpNotZero(Operand, isize),
}

fn register(input: &str) -> IResult<&str, Register> {
    use nom::{branch::alt, character::complete::char, combinator::map};
    map(alt((char('a'), char('b'), char('c'), char('d'))), |c| {
        Register(c)
    })
    .parse(input)
}

fn operand(input: &str) -> IResult<&str, Operand> {
    use nom::{branch::alt, character::complete::i32, combinator::map};
    alt((map(register, Operand::Register), map(i32, Operand::Literal))).parse(input)
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    use nom::{branch::alt, bytes::complete::tag, character::complete::i32, combinator::map};

    let copy = map(
        (tag("cpy "), operand, tag(" "), register),
        |(_, operand, _, register)| Instruction::Copy(operand, register),
    );
    let increment = map((tag("inc "), register), |(_, register)| {
        Instruction::Increment(register)
    });
    let decrement = map((tag("dec "), register), |(_, register)| {
        Instruction::Decrement(register)
    });
    let jump_not_zero = map(
        (tag("jnz "), operand, tag(" "), i32),
        |(_, operand, _, offset)| Instruction::JumpNotZero(operand, offset as isize),
    );

    alt((copy, increment, decrement, jump_not_zero)).parse(input)
}

#[aoc_generator(day12)]
fn generator(input: &str) -> anyhow::Result<Vec<Instruction>> {
    input
        .lines()
        .map(|line| {
            instruction
                .parse(line)
                .map(|(_, instruction)| instruction)
                .map_err(|_| anyhow!("Invalid input: {}", line))
        })
        .collect()
}

type Registers = HashMap<Register, i32>;

fn default_registers() -> Registers {
    ['a', 'b', 'c', 'd']
        .iter()
        .map(|&c| (Register(c), 0))
        .collect()
}

fn execute(instructions: &[Instruction], mut registers: Registers) -> Registers {
    let mut pc = 0;

    while let Some(instruction) = instructions.get(pc) {
        match instruction {
            Instruction::Copy(operand, register) => {
                let value = match operand {
                    Operand::Register(register) => registers[register],
                    Operand::Literal(value) => *value,
                };

                registers.insert(*register, value);
                pc += 1;
            }
            Instruction::Increment(register) => {
                let value = registers[register];
                registers.insert(*register, value + 1);
                pc += 1;
            }
            Instruction::Decrement(register) => {
                let value = registers[register];
                registers.insert(*register, value - 1);
                pc += 1;
            }
            Instruction::JumpNotZero(operand, offset) => {
                let value = match operand {
                    Operand::Register(register) => registers[register],
                    Operand::Literal(value) => *value,
                };

                if value != 0 {
                    pc = pc.saturating_add_signed(*offset);
                } else {
                    pc += 1;
                }
            }
        }
    }

    registers
}

#[aoc(day12, part1)]
fn part1(input: &[Instruction]) -> i32 {
    let registers = default_registers();
    let registers = execute(input, registers);
    registers[&Register('a')]
}

#[aoc(day12, part2)]
fn part2(input: &[Instruction]) -> i32 {
    let mut registers = default_registers();
    registers.insert(Register('c'), 1);

    let registers = execute(input, registers);
    registers[&Register('a')]
}
