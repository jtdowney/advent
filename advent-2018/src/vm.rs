use std::str::FromStr;

use anyhow::{Context, anyhow};
use nom::Parser;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Opcode {
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

pub const ALL_OPCODES: [Opcode; 16] = [
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

#[derive(Copy, Clone, Debug)]
pub struct Instruction(pub Opcode, pub usize, pub usize, pub usize);

impl Instruction {
    pub fn apply(&self, registers: &mut [usize]) {
        let Instruction(opcode, a, b, c) = *self;
        match opcode {
            Opcode::Addr => registers[c] = registers[a] + registers[b],
            Opcode::Addi => registers[c] = registers[a] + b,
            Opcode::Mulr => registers[c] = registers[a] * registers[b],
            Opcode::Muli => registers[c] = registers[a] * b,
            Opcode::Banr => registers[c] = registers[a] & registers[b],
            Opcode::Bani => registers[c] = registers[a] & b,
            Opcode::Borr => registers[c] = registers[a] | registers[b],
            Opcode::Bori => registers[c] = registers[a] | b,
            Opcode::Setr => registers[c] = registers[a],
            Opcode::Seti => registers[c] = a,
            Opcode::Gtir => registers[c] = if a > registers[b] { 1 } else { 0 },
            Opcode::Gtri => registers[c] = if registers[a] > b { 1 } else { 0 },
            Opcode::Gtrr => registers[c] = if registers[a] > registers[b] { 1 } else { 0 },
            Opcode::Eqir => registers[c] = if a == registers[b] { 1 } else { 0 },
            Opcode::Eqri => registers[c] = if registers[a] == b { 1 } else { 0 },
            Opcode::Eqrr => registers[c] = if registers[a] == registers[b] { 1 } else { 0 },
        }
    }

    pub fn apply_to(&self, before: &[usize]) -> Vec<usize> {
        let mut after = before.to_vec();
        self.apply(&mut after);
        after
    }
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

pub struct Machine {
    pub registers: [usize; 6],
    pub ip: usize,
    pub ip_register: usize,
}

impl Machine {
    pub fn new(ip_register: usize) -> Self {
        Self {
            ip_register,
            ip: 0,
            registers: [0; 6],
        }
    }

    pub fn step(&mut self, instructions: &[Instruction]) -> bool {
        self.registers[self.ip_register] = self.ip;
        if let Some(&instruction) = instructions.get(self.ip) {
            instruction.apply(&mut self.registers);
            self.ip = self.registers[self.ip_register] + 1;
            true
        } else {
            false
        }
    }

    pub fn step_alt(&mut self, instructions: &[Instruction]) -> bool {
        self.ip = self.registers[self.ip_register];
        if let Some(&instruction) = instructions.get(self.ip) {
            instruction.apply(&mut self.registers);
            self.registers[self.ip_register] += 1;
            true
        } else {
            false
        }
    }
}

pub struct Program {
    pub ip_register: usize,
    pub instructions: Vec<Instruction>,
}

impl Program {
    pub fn parse(input: &str) -> anyhow::Result<Self> {
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
}
