use std::convert::TryFrom;

use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};

use crate::intcode::{ComputerState, StepResult, parse_program, run_network_computer, step};

#[derive(Debug)]
struct Network {
    computers: Vec<ComputerState>,
    is_idle: Vec<bool>,
}

impl Network {
    fn new(program: &[i64]) -> Self {
        let computers = (0..50)
            .map(|i| {
                let mut state = ComputerState::new(program);
                state.inputs.push_back(i64::from(i));
                state
            })
            .collect();
        Self {
            computers,
            is_idle: vec![false; 50],
        }
    }

    fn run_round(&mut self) -> Result<Vec<(usize, i64, i64)>> {
        let mut packets = Vec::new();
        let num_computers = self.computers.len();

        for (i, computer) in self.computers.iter_mut().enumerate() {
            self.is_idle[i] = computer.inputs.is_empty() && {
                let (_, result) = step(computer.clone());
                result == StepResult::NeedInput
            };

            let (new_state, computer_packets) = run_network_computer(computer.clone(), 1000);
            *computer = new_state;

            for (dest, x, y) in computer_packets {
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
                self.computers[dest].inputs.push_back(x);
                self.computers[dest].inputs.push_back(y);
                self.is_idle[dest] = false;
            }
        }
    }

    fn all_idle(&self) -> bool {
        self.is_idle.iter().all(|&idle| idle)
    }
}

#[aoc_generator(day23)]
fn generator(input: &str) -> Result<Vec<i64>> {
    parse_program(input)
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

        if network.all_idle() && !has_activity {
            idle_rounds += 1;
        } else {
            idle_rounds = 0;
        }

        if idle_rounds > 2 {
            if let Some((x, y)) = nat_packet {
                if last_y_sent == Some(y) {
                    return Ok(y);
                }

                network.computers[0].inputs.push_back(x);
                network.computers[0].inputs.push_back(y);
                network.is_idle[0] = false;
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
    fn test_network_creation() {
        let program = vec![99];
        let network = Network::new(&program);

        assert_eq!(network.computers.len(), 50);
        for (i, computer) in network.computers.iter().enumerate() {
            assert_eq!(computer.inputs[0], i64::try_from(i).unwrap());
        }
    }

    #[test]
    fn test_network_packet_routing() {
        let program = vec![104, 2, 104, 100, 104, 200, 99];
        let mut network = Network::new(&program);

        let packets = network.run_round().unwrap();
        assert_eq!(packets.len(), 50);
        assert!(packets.contains(&(2, 100, 200)));
    }

    #[test]
    fn test_network_idle_detection() {
        let program = vec![3, 0, 99];
        let mut network = Network::new(&program);

        network.run_round().unwrap();

        let all_computers_halted = network.computers.iter().all(|computer| {
            let (_, result) = step(computer.clone());
            result == StepResult::Halted
        });
        assert!(all_computers_halted);
    }
}
