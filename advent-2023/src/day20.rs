use std::collections::{HashMap, VecDeque};

use anyhow::{Context, bail};
use aoc_runner_derive::{aoc, aoc_generator};
use num::integer::lcm;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone)]
enum ModuleType {
    FlipFlop { on: bool },
    Conjunction { memory: HashMap<String, Pulse> },
    Broadcast,
}

#[derive(Debug, Clone)]
struct Module {
    module_type: ModuleType,
    destinations: Vec<String>,
}

type Configuration = HashMap<String, Module>;

impl Module {
    fn parse_with_name(s: &str) -> anyhow::Result<(String, Self)> {
        let (module_info, destinations) = s.split_once(" -> ").context("invalid line format")?;
        let destinations: Vec<String> = destinations.split(", ").map(str::to_string).collect();

        let (name, module_type) = match module_info.chars().next() {
            Some('%') => (&module_info[1..], ModuleType::FlipFlop { on: false }),
            Some('&') => (
                &module_info[1..],
                ModuleType::Conjunction {
                    memory: HashMap::new(),
                },
            ),
            _ if module_info == "broadcaster" => (module_info, ModuleType::Broadcast),
            _ => bail!("Unknown module type: {}", module_info),
        };

        Ok((
            name.to_string(),
            Module {
                module_type,
                destinations,
            },
        ))
    }

    fn process_pulse(&mut self, from: String, pulse: Pulse) -> Option<Pulse> {
        match &mut self.module_type {
            ModuleType::Broadcast => Some(pulse),
            ModuleType::FlipFlop { on } => match pulse {
                Pulse::High => None,
                Pulse::Low => {
                    *on = !*on;
                    Some(if *on { Pulse::High } else { Pulse::Low })
                }
            },
            ModuleType::Conjunction { memory } => {
                memory.insert(from, pulse);
                Some(if memory.values().all(|&p| p == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                })
            }
        }
    }
}

#[aoc_generator(day20)]
fn generator(input: &str) -> anyhow::Result<Configuration> {
    let mut configuration: Configuration = input
        .lines()
        .filter_map(|line| Module::parse_with_name(line).ok())
        .collect();

    let conjunctions: Vec<String> = configuration
        .iter()
        .filter(|(_, module)| matches!(module.module_type, ModuleType::Conjunction { .. }))
        .map(|(name, _)| name.clone())
        .collect();

    for conj_name in conjunctions {
        let inputs: HashMap<String, Pulse> = configuration
            .iter()
            .filter(|(_, module)| module.destinations.contains(&conj_name))
            .map(|(name, _)| (name.clone(), Pulse::Low))
            .collect();

        if let Some(module) = configuration.get_mut(&conj_name) {
            if let ModuleType::Conjunction { memory } = &mut module.module_type {
                *memory = inputs;
            }
        }
    }

    Ok(configuration)
}

fn simulate_pulses<F>(configuration: &mut Configuration, mut callback: F)
where
    F: FnMut(&str, &str, Pulse),
{
    let mut queue = VecDeque::new();
    queue.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));

    while let Some((from, to, pulse)) = queue.pop_front() {
        callback(&from, &to, pulse);

        if let Some(module) = configuration.get_mut(&to) {
            if let Some(output) = module.process_pulse(from, pulse) {
                for dest in &module.destinations {
                    queue.push_back((to.clone(), dest.clone(), output));
                }
            }
        }
    }
}

#[aoc(day20, part1)]
fn part1(configuration: &Configuration) -> u64 {
    let mut configuration = configuration.clone();
    let mut low_pulses = 0;
    let mut high_pulses = 0;

    for _ in 0..1000 {
        simulate_pulses(&mut configuration, |_, _, pulse| match pulse {
            Pulse::Low => low_pulses += 1,
            Pulse::High => high_pulses += 1,
        });
    }

    low_pulses * high_pulses
}

#[aoc(day20, part2)]
fn part2(configuration: &Configuration) -> Option<u64> {
    let rx_input = configuration.iter().find_map(|(name, module)| {
        module
            .destinations
            .contains(&"rx".to_string())
            .then(|| name.clone())
    })?;

    let conjunction_inputs = match &configuration[&rx_input].module_type {
        ModuleType::Conjunction { memory } => memory.keys().cloned().collect::<Vec<_>>(),
        _ => panic!("rx input must be a conjunction"),
    };

    let mut cycles = HashMap::new();
    let mut configuration = configuration.clone();
    let mut button_presses = 0;

    while cycles.len() < conjunction_inputs.len() {
        button_presses += 1;
        simulate_pulses(&mut configuration, |from, to, pulse| {
            if to == rx_input
                && pulse == Pulse::High
                && conjunction_inputs.contains(&from.to_string())
            {
                cycles.entry(from.to_string()).or_insert(button_presses);
            }
        });
    }

    cycles.values().copied().reduce(lcm)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
        let configuration = generator(input).unwrap();
        assert_eq!(part1(&configuration), 32000000);
    }

    #[test]
    fn test_part1_example2() {
        let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
        let configuration = generator(input).unwrap();
        assert_eq!(part1(&configuration), 11687500);
    }
}
