use std::{
    collections::{HashMap, VecDeque},
    ops::{Index, IndexMut},
};

use anyhow::{Context, Result};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

pub fn parse_program(input: &str) -> Result<Vec<i64>> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse::<i64>().context("Failed to parse number"))
        .collect()
}

#[derive(Debug, Clone)]
pub struct Memory {
    inner: HashMap<usize, i64>,
}

impl From<&[i64]> for Memory {
    fn from(source: &[i64]) -> Memory {
        Memory {
            inner: source.iter().cloned().enumerate().collect(),
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

pub fn parse_opcode(instruction: i64) -> i64 {
    instruction % 100
}

pub fn parse_mode(instruction: i64, position: usize) -> ParameterMode {
    let divisor = [100, 1000, 10000][position];
    match (instruction / divisor) % 10 {
        0 => ParameterMode::Position,
        1 => ParameterMode::Immediate,
        2 => ParameterMode::Relative,
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone)]
pub struct ComputerState {
    pub memory: Memory,
    pub ip: usize,
    pub rb: usize,
    pub inputs: VecDeque<i64>,
}

impl ComputerState {
    pub fn new(program: &[i64]) -> Self {
        ComputerState {
            memory: Memory::from(program),
            ip: 0,
            rb: 0,
            inputs: VecDeque::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StepResult {
    Continue,
    Output(i64),
    NeedInput,
    Halted,
}

fn read_parameter(state: &ComputerState, position: usize) -> i64 {
    let instruction = state.memory[state.ip];
    let source = state.ip + position + 1;
    match parse_mode(instruction, position) {
        ParameterMode::Position => state.memory[state.memory[source] as usize],
        ParameterMode::Immediate => state.memory[source],
        ParameterMode::Relative => {
            state.memory[(state.rb as i64 + state.memory[source]) as usize]
        }
    }
}

fn read_destination(state: &ComputerState, position: usize) -> usize {
    let instruction = state.memory[state.ip];
    match parse_mode(instruction, position) {
        ParameterMode::Position => state.memory[state.ip + 1 + position] as usize,
        ParameterMode::Immediate => unreachable!(),
        ParameterMode::Relative => {
            (state.rb as i64 + state.memory[state.ip + position + 1]) as usize
        }
    }
}

pub fn step(mut state: ComputerState) -> (ComputerState, StepResult) {
    let instruction = state.memory[state.ip];
    let opcode = parse_opcode(instruction);

    match opcode {
        1 => {
            let (left, right, dest) = (
                read_parameter(&state, 0),
                read_parameter(&state, 1),
                read_destination(&state, 2),
            );
            state.memory[dest] = left + right;
            state.ip += 4;
            (state, StepResult::Continue)
        }
        2 => {
            let (left, right, dest) = (
                read_parameter(&state, 0),
                read_parameter(&state, 1),
                read_destination(&state, 2),
            );
            state.memory[dest] = left * right;
            state.ip += 4;
            (state, StepResult::Continue)
        }
        3 => {
            if let Some(value) = state.inputs.pop_front() {
                let dest = read_destination(&state, 0);
                state.memory[dest] = value;
                state.ip += 2;
                (state, StepResult::Continue)
            } else {
                (state, StepResult::NeedInput)
            }
        }
        4 => {
            let value = read_parameter(&state, 0);
            state.ip += 2;
            (state, StepResult::Output(value))
        }
        5 => {
            let (cond, target) = (read_parameter(&state, 0), read_parameter(&state, 1));
            state.ip = if cond != 0 { target as usize } else { state.ip + 3 };
            (state, StepResult::Continue)
        }
        6 => {
            let (cond, target) = (read_parameter(&state, 0), read_parameter(&state, 1));
            state.ip = if cond == 0 { target as usize } else { state.ip + 3 };
            (state, StepResult::Continue)
        }
        7 => {
            let (left, right, dest) = (
                read_parameter(&state, 0),
                read_parameter(&state, 1),
                read_destination(&state, 2),
            );
            state.memory[dest] = (left < right) as i64;
            state.ip += 4;
            (state, StepResult::Continue)
        }
        8 => {
            let (left, right, dest) = (
                read_parameter(&state, 0),
                read_parameter(&state, 1),
                read_destination(&state, 2),
            );
            state.memory[dest] = (left == right) as i64;
            state.ip += 4;
            (state, StepResult::Continue)
        }
        9 => {
            state.rb = (state.rb as i64 + read_parameter(&state, 0)) as usize;
            state.ip += 2;
            (state, StepResult::Continue)
        }
        99 => (state, StepResult::Halted),
        _ => unreachable!(),
    }
}

fn run_until<F>(mut state: ComputerState, mut pred: F) -> (ComputerState, Vec<i64>)
where
    F: FnMut(&StepResult) -> bool,
{
    let mut outputs = Vec::new();
    loop {
        let (new_state, result) = step(state);
        state = new_state;
        match result {
            StepResult::Output(value) => outputs.push(value),
            _ if pred(&result) => break,
            StepResult::Continue => continue,
            _ => break,
        }
    }
    (state, outputs)
}

pub fn run_to_completion(state: ComputerState) -> (ComputerState, Vec<i64>) {
    run_until(state, |r| matches!(r, StepResult::NeedInput | StepResult::Halted))
}

pub fn run_until_output(state: ComputerState) -> (ComputerState, Option<i64>) {
    let mut state = state;
    loop {
        let (new_state, result) = step(state);
        state = new_state;
        match result {
            StepResult::Continue => continue,
            StepResult::Output(value) => return (state, Some(value)),
            _ => return (state, None),
        }
    }
}

pub fn run_with_inputs(program: &[i64], inputs: &[i64]) -> Vec<i64> {
    let mut state = ComputerState::new(program);
    state.inputs.extend(inputs);
    run_to_completion(state).1
}

pub fn run_interactive(mut state: ComputerState, input: i64) -> (ComputerState, Option<i64>) {
    state.inputs.push_back(input);
    run_until_output(state)
}

pub fn collect_ascii_output(state: ComputerState) -> String {
    run_to_completion(state)
        .1
        .into_iter()
        .map(|code| code as u8 as char)
        .collect()
}

pub fn ascii_to_codes(input: &str) -> Vec<i64> {
    input.chars().map(|ch| ch as i64).collect()
}

pub fn run_ascii_program(mut state: ComputerState, lines: &[&str]) -> Result<i64> {
    state.inputs.extend(
        lines
            .iter()
            .flat_map(|line| ascii_to_codes(&format!("{}\n", line))),
    );

    let outputs = run_to_completion(state).1;
    outputs
        .last()
        .filter(|&&last| last > 127)
        .copied()
        .ok_or_else(|| {
            let message: String = outputs.into_iter().map(|code| code as u8 as char).collect();
            anyhow::anyhow!("Program failed: {}", message.trim())
        })
}

pub fn run_network_computer(
    mut state: ComputerState,
    max_steps: usize,
) -> (ComputerState, Vec<(i64, i64, i64)>) {
    let mut packets = Vec::new();
    let mut buffer = Vec::new();
    
    for _ in 0..max_steps {
        if state.inputs.is_empty() {
            let (_, result) = step(state.clone());
            if result == StepResult::NeedInput {
                state.inputs.push_back(-1);
            }
        }

        let (new_state, result) = step(state);
        state = new_state;

        match result {
            StepResult::Output(value) => {
                buffer.push(value);
                if buffer.len() == 3 {
                    packets.push((buffer[0], buffer[1], buffer[2]));
                    buffer.clear();
                }
            }
            StepResult::Continue => continue,
            _ => break,
        }
    }
    
    (state, packets)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_program() {
        let program = parse_program("1,2,3,4,5").unwrap();
        assert_eq!(program, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_memory_from_slice() {
        let memory = Memory::from([1, 2, 3].as_slice());
        assert_eq!(memory[0], 1);
        assert_eq!(memory[1], 2);
        assert_eq!(memory[2], 3);
        assert_eq!(memory[100], 0);
    }

    #[test]
    fn test_memory_write() {
        let mut memory = Memory::from([1, 2, 3].as_slice());
        memory[5] = 42;
        assert_eq!(memory[5], 42);
    }

    #[test]
    fn test_parse_opcode() {
        assert_eq!(parse_opcode(1002), 2);
        assert_eq!(parse_opcode(99), 99);
        assert_eq!(parse_opcode(10101), 1);
    }

    #[test]
    fn test_parse_mode() {
        assert_eq!(parse_mode(1002, 0), ParameterMode::Position);
        assert_eq!(parse_mode(1002, 1), ParameterMode::Immediate);
        assert_eq!(parse_mode(20101, 2), ParameterMode::Relative);
    }

    #[test]
    fn test_computer_state_new() {
        let state = ComputerState::new(&[1, 2, 3]);
        assert_eq!(state.ip, 0);
        assert_eq!(state.rb, 0);
        assert!(state.inputs.is_empty());
    }

    #[test]
    fn test_run_simple_add() {
        let state = ComputerState::new(&[1, 5, 6, 7, 99, 10, 20, 0]);
        let (final_state, _) = run_to_completion(state);
        assert_eq!(final_state.memory[7], 30);
    }

    #[test]
    fn test_run_with_output() {
        let state = ComputerState::new(&[4, 3, 99, 42]);
        let (_, outputs) = run_to_completion(state);
        assert_eq!(outputs, vec![42]);
    }

    #[test]
    fn test_run_with_input() {
        let mut state = ComputerState::new(&[3, 5, 4, 5, 99, 0]);
        state.inputs.push_back(123);
        let (final_state, outputs) = run_to_completion(state);
        assert_eq!(final_state.memory[5], 123);
        assert_eq!(outputs, vec![123]);
    }

    #[test]
    fn test_relative_mode() {
        let mut state = ComputerState::new(&[109, 10, 203, -1, 204, -1, 99]);
        state.inputs.push_back(42);
        let (final_state, outputs) = run_to_completion(state);
        assert_eq!(final_state.memory[9], 42);
        assert_eq!(outputs, vec![42]);
    }

    #[test]
    fn test_large_numbers() {
        let state = ComputerState::new(&[104, 1125899906842624, 99]);
        let (_, outputs) = run_to_completion(state);
        assert_eq!(outputs, vec![1125899906842624]);
    }

    #[test]
    fn test_run_until_output() {
        let state = ComputerState::new(&[4, 7, 4, 8, 99, 0, 0, 42, 123]);
        let (state, output) = run_until_output(state);
        assert_eq!(output, Some(42));
        let (_, output) = run_until_output(state);
        assert_eq!(output, Some(123));
    }

    #[test]
    fn test_run_with_inputs() {
        let outputs = run_with_inputs(&[3, 9, 3, 10, 4, 9, 4, 10, 99, 0, 0], &[5, 7]);
        assert_eq!(outputs, vec![5, 7]);
    }

    #[test]
    fn test_run_interactive() {
        let state = ComputerState::new(&[3, 9, 101, 10, 9, 9, 4, 9, 99, 0]);
        let (state, output) = run_interactive(state, 5);
        assert_eq!(output, Some(15));
        let (_, result) = step(state);
        assert_eq!(result, StepResult::Halted);
    }

    #[test]
    fn test_collect_ascii_output() {
        let state = ComputerState::new(&[104, 72, 104, 73, 104, 10, 99]);
        let output = collect_ascii_output(state);
        assert_eq!(output, "HI\n");
    }

    #[test]
    fn test_ascii_to_codes() {
        let ascii_codes = ascii_to_codes("ABC\n");
        assert_eq!(ascii_codes, vec![65, 66, 67, 10]);
    }

    #[test]
    fn test_run_ascii_program() {
        let state = ComputerState::new(&[3, 12, 3, 13, 1, 12, 13, 14, 4, 14, 99, 0, 0, 0, 0]);
        let result = run_ascii_program(state, &["AB"]);
        assert_eq!(result.unwrap(), 131);
    }

    #[test]
    fn test_run_network_computer() {
        let state = ComputerState::new(&[104, 255, 104, 10, 104, 20, 99]);
        let (_, packets) = run_network_computer(state, 10);
        assert_eq!(packets, vec![(255, 10, 20)]);
    }

    #[test]
    fn test_run_network_computer_with_idle() {
        let state = ComputerState::new(&[3, 11, 104, 100, 4, 11, 104, 200, 99, 0, 0, 0]);
        let (final_state, packets) = run_network_computer(state, 20);
        assert_eq!(packets, vec![(100, -1, 200)]);
        assert_eq!(final_state.memory[11], -1);
    }
}