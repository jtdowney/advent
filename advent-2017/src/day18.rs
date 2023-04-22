use std::{
    collections::{HashMap, VecDeque},
    mem,
};

use anyhow::anyhow;
use aoc_runner_derive::{aoc, aoc_generator};
use nom::{Finish, IResult};

#[derive(Clone, Copy, Debug)]
enum Operand {
    Register(char),
    Literal(i64),
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Snd(char),
    Set(char, Operand),
    Add(char, Operand),
    Mul(char, Operand),
    Mod(char, Operand),
    Rcv(char),
    Jgz(Operand, Operand),
}

fn operand(input: &str) -> IResult<&str, Operand> {
    use nom::{
        branch::alt,
        character::complete::{anychar, i64},
        combinator::map,
    };

    let literal = map(i64, Operand::Literal);
    let register = map(anychar, Operand::Register);
    alt((literal, register))(input)
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{anychar, space1},
        combinator::map,
        sequence::{preceded, separated_pair},
    };

    let snd = map(preceded(tag("snd "), anychar), Instruction::Snd);
    let set = map(
        preceded(tag("set "), separated_pair(anychar, space1, operand)),
        |(c, o)| Instruction::Set(c, o),
    );
    let add = map(
        preceded(tag("add "), separated_pair(anychar, space1, operand)),
        |(c, o)| Instruction::Add(c, o),
    );
    let mul = map(
        preceded(tag("mul "), separated_pair(anychar, space1, operand)),
        |(c, o)| Instruction::Mul(c, o),
    );
    let modulo = map(
        preceded(tag("mod "), separated_pair(anychar, space1, operand)),
        |(c, o)| Instruction::Mod(c, o),
    );
    let rcv = map(preceded(tag("rcv "), anychar), Instruction::Rcv);
    let jgz = map(
        preceded(tag("jgz "), separated_pair(operand, space1, operand)),
        |(o1, o2)| Instruction::Jgz(o1, o2),
    );

    alt((snd, set, add, mul, modulo, rcv, jgz))(input)
}

#[aoc_generator(day18)]
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
    outbox: VecDeque<i64>,
    inbox: VecDeque<i64>,
    total_sent: usize,
    blocked: bool,
}

impl State {
    fn run(&mut self, instructions: &[Instruction]) {
        while let Some(instruction) = instructions.get(self.pc) {
            match *instruction {
                Instruction::Snd(register) => {
                    let value = self.registers.get(&register).copied().unwrap_or_default();
                    self.total_sent += 1;
                    self.outbox.push_back(value);
                }
                Instruction::Set(x, y) => {
                    let value = match y {
                        Operand::Literal(n) => n,
                        Operand::Register(register) => {
                            self.registers.get(&register).copied().unwrap_or_default()
                        }
                    };

                    self.registers.insert(x, value);
                }
                Instruction::Add(x, y) => {
                    let value = match y {
                        Operand::Literal(n) => n,
                        Operand::Register(register) => {
                            self.registers.get(&register).copied().unwrap_or_default()
                        }
                    };

                    self.registers
                        .entry(x)
                        .and_modify(|n| *n += value)
                        .or_insert(value);
                }
                Instruction::Mul(x, y) => {
                    let value = match y {
                        Operand::Literal(n) => n,
                        Operand::Register(register) => {
                            self.registers.get(&register).copied().unwrap_or_default()
                        }
                    };

                    self.registers
                        .entry(x)
                        .and_modify(|n| *n *= value)
                        .or_default();
                }
                Instruction::Mod(x, y) => {
                    let value = match y {
                        Operand::Literal(n) => n,
                        Operand::Register(register) => {
                            self.registers.get(&register).copied().unwrap_or_default()
                        }
                    };

                    self.registers
                        .entry(x)
                        .and_modify(|n| *n %= value)
                        .or_default();
                }
                Instruction::Rcv(register) => {
                    if let Some(value) = self.inbox.pop_front() {
                        self.blocked = false;
                        self.registers.insert(register, value);
                    } else {
                        self.blocked = true;
                        return;
                    }
                }
                Instruction::Jgz(x, y) => {
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

                    if value > 0 {
                        self.pc += offset.wrapping_sub(1) as usize;
                    }
                }
            }

            self.pc += 1;
        }
    }
}

#[aoc(day18, part1)]
fn part1(input: &[Instruction]) -> i64 {
    let mut state = State::default();
    state.run(input);
    state.outbox.pop_back().unwrap()
}

#[aoc(day18, part2)]
fn part2(input: &[Instruction]) -> usize {
    let mut state = [State::default(), State::default()];
    for (i, context) in state.iter_mut().enumerate() {
        context.registers.insert('p', i as i64);
    }

    loop {
        for context in state.iter_mut() {
            context.run(input);
        }

        mem::swap(&mut state[0].inbox, &mut state[1].outbox);
        mem::swap(&mut state[1].inbox, &mut state[0].outbox);

        if state[0].inbox.is_empty() && state[1].inbox.is_empty() {
            break;
        }
    }

    state[1].total_sent
}
