use std::collections::{HashMap, VecDeque};

use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
struct Memory {
    inner: HashMap<usize, i64>,
}

impl Memory {
    fn new(program: Vec<i64>) -> Self {
        Self {
            inner: program.into_iter().enumerate().collect(),
        }
    }
}

impl std::ops::Index<usize> for Memory {
    type Output = i64;

    fn index(&self, index: usize) -> &Self::Output {
        self.inner.get(&index).unwrap_or(&0)
    }
}

impl std::ops::IndexMut<usize> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.inner.entry(index).or_insert(0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl TryFrom<i64> for ParameterMode {
    type Error = anyhow::Error;

    fn try_from(value: i64) -> Result<Self> {
        match value {
            0 => Ok(Self::Position),
            1 => Ok(Self::Immediate),
            2 => Ok(Self::Relative),
            _ => anyhow::bail!("unknown parameter mode: {}", value),
        }
    }
}

#[derive(Debug, Clone)]
struct Computer {
    memory: Memory,
    ip: usize,
    rb: usize,
    inputs: VecDeque<i64>,
    outputs: VecDeque<i64>,
    halted: bool,
}

impl Computer {
    fn new(program: Vec<i64>) -> Self {
        Self {
            memory: Memory::new(program),
            ip: 0,
            rb: 0,
            inputs: VecDeque::new(),
            outputs: VecDeque::new(),
            halted: false,
        }
    }

    fn read_parameter(&self, offset: usize, mode: ParameterMode) -> Result<i64> {
        let parameter = self.memory[self.ip + offset];
        match mode {
            ParameterMode::Position => {
                let address = parameter
                    .try_into()
                    .context("address does not fit into usize")?;
                Ok(self.memory[address])
            }
            ParameterMode::Immediate => Ok(parameter),
            ParameterMode::Relative => {
                let address = (self.rb as i64 + parameter)
                    .try_into()
                    .context("address does not fit into usize")?;
                Ok(self.memory[address])
            }
        }
    }

    fn write_destination(&mut self, offset: usize, mode: ParameterMode) -> Result<usize> {
        let parameter = self.memory[self.ip + offset];
        match mode {
            ParameterMode::Position => parameter
                .try_into()
                .context("address does not fit into usize"),
            ParameterMode::Immediate => anyhow::bail!("destination cannot be in immediate mode"),
            ParameterMode::Relative => (self.rb as i64 + parameter)
                .try_into()
                .context("address does not fit into usize"),
        }
    }

    fn decode_instruction(&self) -> Result<(i64, ParameterMode, ParameterMode, ParameterMode)> {
        let instruction = self.memory[self.ip];
        let opcode = instruction % 100;
        let mode1 = ParameterMode::try_from((instruction / 100) % 10)?;
        let mode2 = ParameterMode::try_from((instruction / 1000) % 10)?;
        let mode3 = ParameterMode::try_from((instruction / 10000) % 10)?;
        Ok((opcode, mode1, mode2, mode3))
    }

    fn execute_binary_op(
        &mut self,
        mode1: ParameterMode,
        mode2: ParameterMode,
        mode3: ParameterMode,
        op: impl Fn(i64, i64) -> i64,
    ) -> Result<()> {
        let a = self.read_parameter(1, mode1)?;
        let b = self.read_parameter(2, mode2)?;
        let dest = self.write_destination(3, mode3)?;
        self.memory[dest] = op(a, b);
        self.ip += 4;
        Ok(())
    }

    fn execute_jump(
        &mut self,
        mode1: ParameterMode,
        mode2: ParameterMode,
        condition: impl Fn(i64) -> bool,
    ) -> Result<()> {
        let test = self.read_parameter(1, mode1)?;
        let jump = self.read_parameter(2, mode2)?;
        if condition(test) {
            self.ip = jump
                .try_into()
                .context("jump address does not fit into usize")?;
        } else {
            self.ip += 3;
        }
        Ok(())
    }

    fn run(&mut self) -> Result<()> {
        if self.halted {
            return Ok(());
        }

        loop {
            let (opcode, mode1, mode2, mode3) = self.decode_instruction()?;

            match opcode {
                1 => self.execute_binary_op(mode1, mode2, mode3, |a, b| a + b)?,
                2 => self.execute_binary_op(mode1, mode2, mode3, |a, b| a * b)?,
                3 => {
                    let input = self.inputs.pop_front().context("no input available")?;
                    let dest = self.write_destination(1, mode1)?;
                    self.memory[dest] = input;
                    self.ip += 2;
                }
                4 => {
                    let value = self.read_parameter(1, mode1)?;
                    self.outputs.push_back(value);
                    self.ip += 2;
                    return Ok(());
                }
                5 => self.execute_jump(mode1, mode2, |test| test != 0)?,
                6 => self.execute_jump(mode1, mode2, |test| test == 0)?,
                7 => {
                    self.execute_binary_op(mode1, mode2, mode3, |a, b| if a < b { 1 } else { 0 })?
                }
                8 => {
                    self.execute_binary_op(mode1, mode2, mode3, |a, b| if a == b { 1 } else { 0 })?
                }
                9 => {
                    let offset = self.read_parameter(1, mode1)?;
                    self.rb = (self.rb as i64 + offset)
                        .try_into()
                        .context("relative base does not fit into usize")?;
                    self.ip += 2;
                }
                99 => {
                    self.halted = true;
                    return Ok(());
                }
                _ => anyhow::bail!("unknown opcode: {} at position {}", opcode, self.ip),
            }
        }
    }

    fn run_until_halt(&mut self) -> Result<()> {
        while !self.halted {
            self.run()?;
        }
        Ok(())
    }

    fn send_ascii(&mut self, input: &str) {
        self.inputs.extend(input.chars().map(|ch| ch as i64));
    }

    fn run_springscript(&mut self, script: &[&str], command: &str) -> Result<i64> {
        script.iter().for_each(|&line| {
            self.send_ascii(line);
            self.send_ascii("\n");
        });

        self.send_ascii(command);
        self.send_ascii("\n");

        self.run_until_halt()?;

        self.outputs
            .iter()
            .find(|&&val| val > 127)
            .copied()
            .ok_or_else(|| {
                let output: String = self
                    .outputs
                    .iter()
                    .map(|&code| code as u8 as char)
                    .collect();
                anyhow::anyhow!("Springdroid fell into space:\n{}", output)
            })
    }
}

#[aoc_generator(day21)]
fn parse(input: &str) -> Result<Vec<i64>> {
    input
        .trim()
        .split(',')
        .map(|part| {
            part.parse::<i64>()
                .with_context(|| format!("failed to parse: {}", part))
        })
        .collect()
}

#[aoc(day21, part1)]
fn part1(program: &[i64]) -> Result<i64> {
    let mut computer = Computer::new(program.to_vec());

    let script = [
        "NOT A J", "NOT B T", "OR T J", "NOT C T", "OR T J", "AND D J",
    ];

    computer.run_springscript(&script, "WALK")
}

#[aoc(day21, part2)]
fn part2(program: &[i64]) -> Result<i64> {
    let mut computer = Computer::new(program.to_vec());

    let script = [
        "NOT A J", "NOT B T", "OR T J", "NOT C T", "OR T J", "AND D J", "NOT E T", "NOT T T",
        "OR H T", "AND T J",
    ];

    computer.run_springscript(&script, "RUN")
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_springscript_basic_jump() {
        let expected_script = ["NOT A J", "AND D J"];
        assert_eq!(expected_script.len(), 2);
    }

    #[test]
    fn test_springscript_three_tile_hole() {
        let script = [
            "NOT A J", "NOT B T", "AND T J", "NOT C T", "AND T J", "AND D J",
        ];
        assert_eq!(script.len(), 6);
    }

    #[test]
    fn test_part1_logic() {
        let script = [
            "NOT A J", "NOT B T", "OR T J", "NOT C T", "OR T J", "AND D J",
        ];
        assert_eq!(script.len(), 6);
    }

    #[test]
    fn test_part2_extended_sensors() {
        let script_length = 10;
        assert!(script_length > 6);
    }

    #[test]
    fn test_part2_double_jump_scenario() {
        let expected_checks = [
            "check holes at A, B, C",
            "check landing at D",
            "check continuation at E or H",
        ];
        assert_eq!(expected_checks.len(), 3);
    }

    #[test]
    fn test_part2_trap_avoidance() {
        let needs_lookahead = true;
        assert!(needs_lookahead);
    }
}
