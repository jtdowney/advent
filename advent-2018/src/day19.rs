use std::str::FromStr;

use anyhow::{Context, anyhow};
use aoc_runner_derive::{aoc, aoc_generator};
use nom::Parser;

#[derive(Copy, Clone, Debug)]
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

struct Program {
    ip_register: usize,
    instructions: Vec<Instruction>,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use nom::{
            Finish,
            branch::alt,
            bytes::complete::tag,
            character::complete::{space1, u32},
            combinator::{map, value},
            error::Error,
        };

        let mut instruction = map(
            (
                alt((
                    value(Opcode::Addr, tag::<_, _, Error<&str>>("addr")),
                    value(Opcode::Addi, tag("addi")),
                    value(Opcode::Mulr, tag("mulr")),
                    value(Opcode::Muli, tag("muli")),
                    value(Opcode::Banr, tag("banr")),
                    value(Opcode::Bani, tag("bani")),
                    value(Opcode::Borr, tag("borr")),
                    value(Opcode::Bori, tag("bori")),
                    value(Opcode::Setr, tag("setr")),
                    value(Opcode::Seti, tag("seti")),
                    value(Opcode::Gtir, tag("gtir")),
                    value(Opcode::Gtri, tag("gtri")),
                    value(Opcode::Gtrr, tag("gtrr")),
                    value(Opcode::Eqir, tag("eqir")),
                    value(Opcode::Eqri, tag("eqri")),
                    value(Opcode::Eqrr, tag("eqrr")),
                )),
                space1,
                u32,
                space1,
                u32,
                space1,
                u32,
            ),
            |(opcode, _, a, _, b, _, c)| Instruction(opcode, a as usize, b as usize, c as usize),
        );

        instruction
            .parse(s)
            .finish()
            .map(|(_, instruction)| instruction)
            .map_err(|e| anyhow!("unable to parse instruction: {:?}", e))
    }
}

struct Machine {
    registers: [usize; 6],
    ip: usize,
    ip_register: usize,
}

impl Machine {
    fn new(ip_register: usize) -> Self {
        Self {
            ip_register,
            ip: 0,
            registers: [0; 6],
        }
    }

    fn step(&mut self, instructions: &[Instruction]) -> bool {
        self.ip = self.registers[self.ip_register];
        if let Some(&Instruction(opcode, a, b, c)) = instructions.get(self.ip) {
            match opcode {
                Opcode::Addr => self.registers[c] = self.registers[a] + self.registers[b],
                Opcode::Addi => self.registers[c] = self.registers[a] + b,
                Opcode::Mulr => self.registers[c] = self.registers[a] * self.registers[b],
                Opcode::Muli => self.registers[c] = self.registers[a] * b,
                Opcode::Banr => self.registers[c] = self.registers[a] & self.registers[b],
                Opcode::Bani => self.registers[c] = self.registers[a] & b,
                Opcode::Borr => self.registers[c] = self.registers[a] | self.registers[b],
                Opcode::Bori => self.registers[c] = self.registers[a] | b,
                Opcode::Setr => self.registers[c] = self.registers[a],
                Opcode::Seti => self.registers[c] = a,
                Opcode::Gtir => self.registers[c] = if a > self.registers[b] { 1 } else { 0 },
                Opcode::Gtri => self.registers[c] = if self.registers[a] > b { 1 } else { 0 },
                Opcode::Gtrr => {
                    self.registers[c] = if self.registers[a] > self.registers[b] {
                        1
                    } else {
                        0
                    }
                }
                Opcode::Eqir => self.registers[c] = if a == self.registers[b] { 1 } else { 0 },
                Opcode::Eqri => self.registers[c] = if self.registers[a] == b { 1 } else { 0 },
                Opcode::Eqrr => {
                    self.registers[c] = if self.registers[a] == self.registers[b] {
                        1
                    } else {
                        0
                    }
                }
            }

            self.registers[self.ip_register] += 1;

            true
        } else {
            false
        }
    }
}

#[aoc_generator(day19)]
fn generator(input: &str) -> anyhow::Result<Program> {
    let mut lines = input.lines();
    let first = lines.next().context("missing first line")?;
    let ip_register = first
        .strip_prefix("#ip ")
        .context("missing #ip prefix")?
        .parse()
        .context("unable to parse ip register")?;

    let instructions = lines
        .map(|line| line.parse())
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Program {
        ip_register,
        instructions,
    })
}

#[aoc(day19, part1)]
fn part1(input: &Program) -> usize {
    let mut machine = Machine::new(input.ip_register);
    while machine.step(&input.instructions) {}
    machine.registers[0]
}

#[aoc(day19, part2)]
fn part2(input: &Program) -> usize {
    let mut machine = Machine::new(input.ip_register);
    machine.registers[0] = 1;

    let mut count = 0;
    while machine.step(&input.instructions) {
        count += 1;
        if count > 25 {
            break;
        }
    }

    let n = machine.registers[4];
    n + (1..=n / 2).filter(|x| n % x == 0).sum::<usize>()
}
