use std::collections::HashMap;

use anyhow::anyhow;
use aoc_runner_derive::{aoc, aoc_generator};
use nom::{Finish, IResult, Parser};

#[derive(Clone, Copy, Debug)]
enum Operand {
    Register(char),
    Literal(i64),
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Set(char, Operand),
    Sub(char, Operand),
    Mul(char, Operand),
    Jnz(Operand, Operand),
}

fn operand(input: &str) -> IResult<&str, Operand> {
    use nom::{
        branch::alt,
        character::complete::{anychar, i64},
        combinator::map,
    };

    let literal = map(i64, Operand::Literal);
    let register = map(anychar, Operand::Register);
    alt((literal, register)).parse(input)
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{anychar, space1},
        combinator::map,
        sequence::{preceded, separated_pair},
    };

    let set = map(
        preceded(tag("set "), separated_pair(anychar, space1, operand)),
        |(c, o)| Instruction::Set(c, o),
    );
    let sub = map(
        preceded(tag("sub "), separated_pair(anychar, space1, operand)),
        |(c, o)| Instruction::Sub(c, o),
    );
    let mul = map(
        preceded(tag("mul "), separated_pair(anychar, space1, operand)),
        |(c, o)| Instruction::Mul(c, o),
    );
    let jnz = map(
        preceded(tag("jnz "), separated_pair(operand, space1, operand)),
        |(o1, o2)| Instruction::Jnz(o1, o2),
    );

    alt((set, sub, mul, jnz)).parse(input)
}

#[aoc_generator(day23)]
fn generator(input: &str) -> anyhow::Result<Vec<Instruction>> {
    input
        .lines()
        .map(|line| {
            instruction(line)
                .finish()
                .map(|(_, i)| i)
                .map_err(|e| anyhow!("unable to parse instruction {:?}: {}", line, e))
        })
        .collect()
}

#[derive(Default)]
struct State {
    pc: usize,
    registers: HashMap<char, i64>,
    multiplications: usize,
}

impl State {
    fn run(&mut self, instructions: &[Instruction]) {
        while let Some(instruction) = instructions.get(self.pc) {
            match *instruction {
                Instruction::Set(x, y) => {
                    let value = match y {
                        Operand::Literal(n) => n,
                        Operand::Register(register) => {
                            self.registers.get(&register).copied().unwrap_or_default()
                        }
                    };

                    self.registers.insert(x, value);
                }
                Instruction::Sub(x, y) => {
                    let value = match y {
                        Operand::Literal(n) => n,
                        Operand::Register(register) => {
                            self.registers.get(&register).copied().unwrap_or_default()
                        }
                    };

                    self.registers
                        .entry(x)
                        .and_modify(|n| *n -= value)
                        .or_insert(value);
                }
                Instruction::Mul(x, y) => {
                    let value = match y {
                        Operand::Literal(n) => n,
                        Operand::Register(register) => {
                            self.registers.get(&register).copied().unwrap_or_default()
                        }
                    };

                    self.multiplications += 1;
                    self.registers
                        .entry(x)
                        .and_modify(|n| *n *= value)
                        .or_default();
                }
                Instruction::Jnz(x, y) => {
                    let value = match x {
                        Operand::Literal(n) => n,
                        Operand::Register(register) => {
                            self.registers.get(&register).copied().unwrap_or_default()
                        }
                    };

                    let offset = match y {
                        Operand::Literal(n) => n,
                        Operand::Register(register) => {
                            self.registers.get(&register).copied().unwrap_or_default()
                        }
                    };

                    if value != 0 {
                        self.pc += offset.wrapping_sub(1) as usize;
                    }
                }
            }

            self.pc += 1;
        }
    }
}

#[aoc(day23, part1)]
fn part1(input: &[Instruction]) -> usize {
    let mut state = State::default();
    state.run(input);
    state.multiplications
}

#[aoc(day23, part2)]
fn part2(_: &[Instruction]) -> usize {
    let mut b = (84 * 100) + 100_000;
    let c = b + 17_000;
    let mut d;
    let mut f;
    let mut h = 0;

    while b <= c {
        f = 1;
        d = 2;

        while d != b {
            if b % d == 0 {
                f = 0;
                break;
            }

            d += 1;
        }

        if f == 0 {
            h += 1;
        }

        b += 17;
    }

    h
}

// #[aoc(day23, part2)]
// fn part2(input: &[Instruction]) -> usize {
//     let mut state = [State::default(), State::default()];
//     for (i, context) in state.iter_mut().enumerate() {
//         context.registers.insert('p', i as i64);
//     }

//     loop {
//         for context in state.iter_mut() {
//             context.run(input);
//         }

//         mem::swap(&mut state[0].inbox, &mut state[1].outbox);
//         mem::swap(&mut state[1].inbox, &mut state[0].outbox);

//         if state[0].inbox.is_empty() && state[1].inbox.is_empty() {
//             break;
//         }
//     }

//     state[1].total_sent
// }
