use std::{
    collections::{HashMap, VecDeque},
    convert::TryInto,
    ops::{Index, IndexMut},
};

use anyhow::{Context, Result, bail};
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
struct Memory {
    inner: HashMap<usize, i64>,
}

impl From<Vec<i64>> for Memory {
    fn from(program: Vec<i64>) -> Self {
        Self {
            inner: program.into_iter().enumerate().collect(),
        }
    }
}

impl Index<usize> for Memory {
    type Output = i64;

    fn index(&self, index: usize) -> &Self::Output {
        self.inner.get(&index).unwrap_or(&0)
    }
}

impl IndexMut<usize> for Memory {
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
            _ => bail!("unknown parameter mode: {}", value),
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
    waiting_for_input: bool,
}

impl Computer {
    fn new(program: Vec<i64>) -> Self {
        Self {
            memory: program.into(),
            ip: 0,
            rb: 0,
            inputs: VecDeque::new(),
            outputs: VecDeque::new(),
            halted: false,
            waiting_for_input: false,
        }
    }

    fn with_address(program: Vec<i64>, address: i64) -> Self {
        let mut computer = Self::new(program);
        computer.inputs.push_back(address);
        computer
    }

    fn is_idle(&self) -> bool {
        self.waiting_for_input && self.inputs.is_empty()
    }

    fn provide_input(&mut self, value: i64) {
        self.inputs.push_back(value);
    }

    fn collect_packet(&mut self) -> Option<(i64, i64, i64)> {
        if self.outputs.len() >= 3 {
            Some((
                self.outputs.pop_front()?,
                self.outputs.pop_front()?,
                self.outputs.pop_front()?,
            ))
        } else {
            None
        }
    }

    fn read_parameter(&self, offset: usize, mode: ParameterMode) -> Result<i64> {
        let parameter = self.memory[self.ip + offset];
        match mode {
            ParameterMode::Position => {
                let address = parameter
                    .try_into()
                    .context("parameter does not fit into usize")?;
                Ok(self.memory[address])
            }
            ParameterMode::Immediate => Ok(parameter),
            ParameterMode::Relative => {
                let address = (self.rb as i64 + parameter)
                    .try_into()
                    .context("relative address does not fit into usize")?;
                Ok(self.memory[address])
            }
        }
    }

    fn write_destination(&self, offset: usize, mode: ParameterMode) -> Result<usize> {
        let parameter = self.memory[self.ip + offset];
        match mode {
            ParameterMode::Position => parameter
                .try_into()
                .context("parameter does not fit into usize"),
            ParameterMode::Immediate => bail!("destination cannot be in immediate mode"),
            ParameterMode::Relative => (self.rb as i64 + parameter)
                .try_into()
                .context("relative address does not fit into usize"),
        }
    }

    fn decode_instruction(&self) -> Result<(i64, ParameterMode, ParameterMode, ParameterMode)> {
        let instruction = self.memory[self.ip];
        Ok((
            instruction % 100,
            ParameterMode::try_from((instruction / 100) % 10)?,
            ParameterMode::try_from((instruction / 1000) % 10)?,
            ParameterMode::try_from((instruction / 10000) % 10)?,
        ))
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
        self.ip = if condition(test) {
            jump.try_into()
                .context("jump address does not fit into usize")?
        } else {
            self.ip + 3
        };
        Ok(())
    }

    fn step(&mut self) -> Result<()> {
        if self.halted {
            return Ok(());
        }

        let (opcode, mode1, mode2, mode3) = self.decode_instruction()?;

        match opcode {
            1 => self.execute_binary_op(mode1, mode2, mode3, |a, b| a + b)?,
            2 => self.execute_binary_op(mode1, mode2, mode3, |a, b| a * b)?,
            3 => match self.inputs.pop_front() {
                Some(value) => {
                    let dest = self.write_destination(1, mode1)?;
                    self.memory[dest] = value;
                    self.ip += 2;
                    self.waiting_for_input = false;
                }
                None => self.waiting_for_input = true,
            },
            4 => {
                self.outputs.push_back(self.read_parameter(1, mode1)?);
                self.ip += 2;
            }
            5 => self.execute_jump(mode1, mode2, |test| test != 0)?,
            6 => self.execute_jump(mode1, mode2, |test| test == 0)?,
            7 => self.execute_binary_op(mode1, mode2, mode3, |a, b| i64::from(a < b))?,
            8 => self.execute_binary_op(mode1, mode2, mode3, |a, b| i64::from(a == b))?,
            9 => {
                self.rb = (self.rb as i64 + self.read_parameter(1, mode1)?).try_into()?;
                self.ip += 2;
            }
            99 => self.halted = true,
            _ => bail!("unknown opcode: {}", opcode),
        }

        Ok(())
    }

    fn run_steps(&mut self, max_steps: usize) -> Result<()> {
        for _ in 0..max_steps {
            if self.halted || self.is_idle() {
                break;
            }
            self.step()?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Network {
    computers: Vec<Computer>,
}

impl Network {
    fn new(program: &[i64]) -> Self {
        Self {
            computers: (0..50)
                .map(|i| Computer::with_address(program.to_vec(), i64::from(i)))
                .collect(),
        }
    }

    fn run_round(&mut self) -> Result<Vec<(usize, i64, i64)>> {
        let mut packets = Vec::new();
        let num_computers = self.computers.len();

        for computer in &mut self.computers {
            if computer.is_idle() {
                computer.provide_input(-1);
            }

            computer.run_steps(1000)?;

            while let Some((dest, x, y)) = computer.collect_packet() {
                if dest == 255 {
                    packets.push((255, x, y));
                } else if let Ok(dest_usize) = usize::try_from(dest) {
                    if dest_usize < num_computers {
                        packets.push((dest_usize, x, y));
                    }
                }
            }
        }

        Ok(packets)
    }

    fn deliver_packets(&mut self, packets: &[(usize, i64, i64)]) {
        for &(dest, x, y) in packets {
            if dest < self.computers.len() {
                self.computers[dest].provide_input(x);
                self.computers[dest].provide_input(y);
            }
        }
    }

    fn is_idle(&self) -> bool {
        self.computers.iter().all(Computer::is_idle)
    }
}

#[aoc_generator(day23)]
fn generator(input: &str) -> Result<Vec<i64>> {
    input
        .trim()
        .split(',')
        .map(|n| n.parse().context("parsing number"))
        .collect()
}

#[aoc(day23, part1)]
fn part1(input: &[i64]) -> Result<i64> {
    let mut network = Network::new(input);

    loop {
        let packets = network.run_round()?;

        if let Some((_, _, y)) = packets.iter().find(|(dest, _, _)| *dest == 255) {
            return Ok(*y);
        }

        network.deliver_packets(&packets);
    }
}

#[aoc(day23, part2)]
fn part2(input: &[i64]) -> Result<i64> {
    let mut network = Network::new(input);
    let mut nat_packet: Option<(i64, i64)> = None;
    let mut last_y_sent: Option<i64> = None;
    let mut idle_rounds = 0;

    loop {
        let packets = network.run_round()?;
        let has_activity = !packets.is_empty();

        packets
            .iter()
            .filter(|(dest, _, _)| *dest == 255)
            .for_each(|(_, x, y)| nat_packet = Some((*x, *y)));

        network.deliver_packets(&packets);

        if network.is_idle() && !has_activity {
            idle_rounds += 1;
        } else {
            idle_rounds = 0;
        }

        if idle_rounds > 2 {
            if let Some((x, y)) = nat_packet {
                if last_y_sent == Some(y) {
                    return Ok(y);
                }

                network.computers[0].provide_input(x);
                network.computers[0].provide_input(y);
                last_y_sent = Some(y);
                idle_rounds = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameter_mode_parsing() {
        assert_eq!(ParameterMode::try_from(0).unwrap(), ParameterMode::Position);
        assert_eq!(
            ParameterMode::try_from(1).unwrap(),
            ParameterMode::Immediate
        );
        assert_eq!(ParameterMode::try_from(2).unwrap(), ParameterMode::Relative);
        assert!(ParameterMode::try_from(3).is_err());
    }

    #[test]
    fn test_memory_indexing() {
        let program = vec![1, 2, 3, 4, 5];
        let memory = Memory::from(program);

        assert_eq!(memory[0], 1);
        assert_eq!(memory[4], 5);
        assert_eq!(memory[10], 0);
    }

    #[test]
    fn test_computer_initialization() {
        let program = vec![99]; // HALT instruction
        let computer = Computer::with_address(program, 42);

        assert_eq!(computer.inputs.len(), 1);
        assert_eq!(computer.inputs[0], 42);
        assert!(!computer.halted);
        assert!(!computer.waiting_for_input);
    }

    #[test]
    fn test_computer_idle_detection() {
        let program = vec![3, 0, 99]; // INPUT instruction
        let mut computer = Computer::new(program);

        assert!(!computer.is_idle());

        computer.step().unwrap();
        assert!(computer.is_idle());
        assert!(computer.waiting_for_input);

        computer.provide_input(123);
        assert!(!computer.is_idle());
    }

    #[test]
    fn test_packet_collection() {
        let program = vec![
            104, 255, // Output 255 (immediate mode)
            104, 10, // Output 10 (immediate mode)
            104, 20, // Output 20 (immediate mode)
            99, // Halt
        ];
        let mut computer = Computer::new(program);

        computer.run_steps(10).unwrap();

        let packet = computer.collect_packet();
        assert!(packet.is_some());
        assert_eq!(packet.unwrap(), (255, 10, 20));
    }

    #[test]
    fn test_network_creation() {
        let program = vec![99]; // Simple halt program
        let network = Network::new(&program);

        assert_eq!(network.computers.len(), 50);
        for (i, computer) in network.computers.iter().enumerate() {
            assert_eq!(computer.inputs[0], i64::try_from(i).unwrap());
        }
    }

    #[test]
    fn test_network_idle_detection() {
        let program = vec![
            3, 5,  // Read network address to position 5
            99, // Halt
        ];
        let mut network = Network::new(&program);

        for _ in 0..5 {
            network.run_round().unwrap();
        }

        assert!(network.computers.iter().all(|c| c.halted || c.is_idle()));
    }
}
