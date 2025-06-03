use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};

type Memory = HashMap<usize, i64>;
type Path = Vec<String>;
type ItemLocations = HashMap<String, Path>;

#[derive(Debug, Clone)]
struct IntcodeComputer {
    memory: Memory,
    ip: usize,
    relative_base: i64,
    inputs: Vec<i64>,
    outputs: Vec<i64>,
}

impl IntcodeComputer {
    fn new(memory: Memory) -> Self {
        Self {
            memory,
            ip: 0,
            relative_base: 0,
            inputs: vec![],
            outputs: vec![],
        }
    }

    fn run(&mut self) -> Result<()> {
        loop {
            let opcode = self.memory.get(&self.ip).copied().unwrap_or(0) % 100;
            let modes = self.memory.get(&self.ip).copied().unwrap_or(0) / 100;

            match opcode {
                1 => {
                    let a = self.get_value(1, modes % 10)?;
                    let b = self.get_value(2, (modes / 10) % 10)?;
                    let c = self.get_address(3, (modes / 100) % 10)?;
                    self.memory.insert(c, a + b);
                    self.ip += 4;
                }
                2 => {
                    let a = self.get_value(1, modes % 10)?;
                    let b = self.get_value(2, (modes / 10) % 10)?;
                    let c = self.get_address(3, (modes / 100) % 10)?;
                    self.memory.insert(c, a * b);
                    self.ip += 4;
                }
                3 => {
                    let addr = self.get_address(1, modes % 10)?;
                    if self.inputs.is_empty() {
                        return Ok(());
                    }
                    let value = self.inputs.remove(0);
                    self.memory.insert(addr, value);
                    self.ip += 2;
                }
                4 => {
                    let value = self.get_value(1, modes % 10)?;
                    self.outputs.push(value);
                    self.ip += 2;
                }
                5 => {
                    let test = self.get_value(1, modes % 10)?;
                    let dest = self.get_value(2, (modes / 10) % 10)?;
                    self.ip = if test != 0 {
                        dest as usize
                    } else {
                        self.ip + 3
                    };
                }
                6 => {
                    let test = self.get_value(1, modes % 10)?;
                    let dest = self.get_value(2, (modes / 10) % 10)?;
                    self.ip = if test == 0 {
                        dest as usize
                    } else {
                        self.ip + 3
                    };
                }
                7 => {
                    let a = self.get_value(1, modes % 10)?;
                    let b = self.get_value(2, (modes / 10) % 10)?;
                    let c = self.get_address(3, (modes / 100) % 10)?;
                    self.memory.insert(c, if a < b { 1 } else { 0 });
                    self.ip += 4;
                }
                8 => {
                    let a = self.get_value(1, modes % 10)?;
                    let b = self.get_value(2, (modes / 10) % 10)?;
                    let c = self.get_address(3, (modes / 100) % 10)?;
                    self.memory.insert(c, if a == b { 1 } else { 0 });
                    self.ip += 4;
                }
                9 => {
                    let value = self.get_value(1, modes % 10)?;
                    self.relative_base += value;
                    self.ip += 2;
                }
                99 => return Ok(()),
                _ => anyhow::bail!("Unknown opcode: {}", opcode),
            }
        }
    }

    fn get_value(&self, offset: usize, mode: i64) -> Result<i64> {
        let param = self.memory.get(&(self.ip + offset)).copied().unwrap_or(0);
        match mode {
            0 => Ok(self.memory.get(&(param as usize)).copied().unwrap_or(0)),
            1 => Ok(param),
            2 => {
                let addr = (self.relative_base + param) as usize;
                Ok(self.memory.get(&addr).copied().unwrap_or(0))
            }
            _ => anyhow::bail!("Unknown mode: {}", mode),
        }
    }

    fn get_address(&self, offset: usize, mode: i64) -> Result<usize> {
        let param = self.memory.get(&(self.ip + offset)).copied().unwrap_or(0);
        match mode {
            0 => Ok(param as usize),
            2 => Ok((self.relative_base + param) as usize),
            _ => anyhow::bail!("Invalid mode for address: {}", mode),
        }
    }
}

#[aoc_generator(day25)]
fn generator(input: &str) -> Result<Memory> {
    input
        .trim()
        .split(',')
        .enumerate()
        .map(|(i, v)| Ok((i, v.parse()?)))
        .collect()
}

fn send_command(computer: &mut IntcodeComputer, cmd: &str) -> Result<String> {
    computer
        .inputs
        .extend(format!("{}\n", cmd).chars().map(|c| c as i64));
    computer.run()?;

    Ok(computer
        .outputs
        .drain(..)
        .filter(|&c| (0..=127).contains(&c))
        .map(|c| c as u8 as char)
        .collect())
}

fn parse_room_info(output: &str) -> (Option<String>, Vec<String>, Vec<String>) {
    let lines: Vec<_> = output.lines().collect();

    let room_name = lines
        .iter()
        .find(|line| line.starts_with("==") && line.ends_with("=="))
        .map(|s| s.to_string());

    let extract_list = |target: &str| {
        lines
            .iter()
            .skip_while(|line| !line.contains(target))
            .skip(1)
            .take_while(|line| line.starts_with("- "))
            .map(|line| line[2..].to_string())
            .collect()
    };

    (
        room_name,
        extract_list("Doors here lead:"),
        extract_list("Items here:"),
    )
}

fn opposite_dir(dir: &str) -> &str {
    match dir {
        "north" => "south",
        "south" => "north",
        "east" => "west",
        "west" => "east",
        _ => panic!("Unknown direction: {}", dir),
    }
}

fn run_computer_with_path(memory: &Memory, path: &Path) -> Result<String> {
    let mut computer = IntcodeComputer::new(memory.clone());
    let initial_output = send_command(&mut computer, "")?;

    if path.is_empty() {
        Ok(initial_output)
    } else {
        path.iter()
            .try_fold(String::new(), |_, cmd| send_command(&mut computer, cmd))
    }
}

fn explore_ship(memory: &Memory) -> Result<(Path, ItemLocations)> {
    let dangerous_items: HashSet<_> = [
        "escape pod",
        "giant electromagnet",
        "infinite loop",
        "molten lava",
        "photons",
    ]
    .iter()
    .map(|&s| s.to_string())
    .collect();

    let mut visited_rooms = HashSet::new();
    let mut queue = VecDeque::new();
    let mut checkpoint_path = None;
    let mut item_locations = HashMap::new();

    queue.push_back(vec![]);

    while let Some(path) = queue.pop_front() {
        if path.len() > 20 {
            continue;
        }

        let output = run_computer_with_path(memory, &path)?;
        let (room_name, doors, items) = parse_room_info(&output);

        if let Some(room) = room_name {
            if visited_rooms.contains(&room) {
                continue;
            }
            visited_rooms.insert(room.clone());

            if room.contains("Security Checkpoint") && checkpoint_path.is_none() {
                checkpoint_path = Some(path.clone());
            }

            items
                .into_iter()
                .filter(|item| !dangerous_items.contains(item))
                .for_each(|item| {
                    item_locations.insert(item, path.clone());
                });

            doors.into_iter().for_each(|dir| {
                let mut new_path = path.clone();
                new_path.push(dir);
                queue.push_back(new_path);
            });
        }
    }

    checkpoint_path
        .ok_or_else(|| anyhow::anyhow!("Could not find Security Checkpoint!"))
        .map(|path| (path, item_locations))
}

fn navigate_and_execute(
    computer: &mut IntcodeComputer,
    path: &Path,
    action: impl FnOnce(&mut IntcodeComputer) -> Result<String>,
) -> Result<String> {
    path.iter()
        .try_for_each(|cmd| send_command(computer, cmd).map(|_| ()))?;
    let result = action(computer)?;
    path.iter()
        .rev()
        .try_for_each(|cmd| send_command(computer, opposite_dir(cmd)).map(|_| ()))?;
    Ok(result)
}

fn find_password(
    memory: &Memory,
    checkpoint_path: Path,
    item_locations: ItemLocations,
) -> Result<String> {
    let mut computer = IntcodeComputer::new(memory.clone());
    send_command(&mut computer, "")?;

    let items_collected: Vec<String> = item_locations
        .into_iter()
        .filter_map(|(item, path)| {
            navigate_and_execute(&mut computer, &path, |comp| {
                send_command(comp, &format!("take {}", item))
            })
            .ok()
            .filter(|output| output.contains("You take"))
            .map(|_| item)
        })
        .collect();

    let mut last_output = String::new();
    for cmd in checkpoint_path {
        last_output = send_command(&mut computer, &cmd)?;
    }

    let (_, doors, _) = parse_room_info(&last_output);

    let pressure_plate_dir = doors
        .iter()
        .find(|&dir| {
            let output = send_command(&mut computer, dir).unwrap_or_default();
            let found = output.contains("Pressure-Sensitive Floor");
            if output.contains("==") && !output.contains("Security Checkpoint") {
                let _ = send_command(&mut computer, opposite_dir(dir));
            }
            found
        })
        .ok_or_else(|| anyhow::anyhow!("Could not find pressure plate direction"))?;

    (0..(1 << items_collected.len()))
        .find_map(|mask| {
            items_collected.iter().for_each(|item| {
                let _ = send_command(&mut computer, &format!("drop {}", item));
            });

            items_collected
                .iter()
                .enumerate()
                .filter(|(i, _)| mask & (1 << i) != 0)
                .for_each(|(_, item)| {
                    let _ = send_command(&mut computer, &format!("take {}", item));
                });

            send_command(&mut computer, pressure_plate_dir)
                .ok()
                .and_then(|output| {
                    output
                        .lines()
                        .find(|line| line.contains("typing") && line.contains("on the keypad"))
                        .and_then(|line| {
                            line.find("typing ").and_then(|start| {
                                let rest = &line[start + 7..];
                                rest.find(" on the keypad")
                                    .map(|end| rest[..end].to_string())
                            })
                        })
                })
        })
        .ok_or_else(|| anyhow::anyhow!("Could not find the correct item combination"))
}

#[aoc(day25, part1)]
fn part1(initial_memory: &Memory) -> Result<String> {
    let (checkpoint_path, item_locations) = explore_ship(initial_memory)?;
    find_password(initial_memory, checkpoint_path, item_locations)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intcode_computer_initialization() {
        let memory = HashMap::from([(0, 1), (1, 2), (2, 3)]);
        let computer = IntcodeComputer::new(memory.clone());

        assert_eq!(computer.memory[&0], 1);
        assert_eq!(computer.memory[&1], 2);
        assert_eq!(computer.memory[&2], 3);
        assert_eq!(computer.ip, 0);
        assert_eq!(computer.relative_base, 0);
    }

    #[test]
    fn test_parse_input() {
        let input = "1,2,3,4,5";
        let memory = generator(input).unwrap();

        assert_eq!(memory[&0], 1);
        assert_eq!(memory[&1], 2);
        assert_eq!(memory[&2], 3);
        assert_eq!(memory[&3], 4);
        assert_eq!(memory[&4], 5);
    }

    #[test]
    fn test_ascii_conversion() {
        let output = vec![72, 101, 108, 108, 111, 10];
        let text: String = output.iter().map(|&c| c as u8 as char).collect();
        assert_eq!(text, "Hello\n");
    }

    #[test]
    fn test_intcode_run_simple_output() {
        let memory = HashMap::from([(0, 104), (1, 65), (2, 99)]);
        let mut computer = IntcodeComputer::new(memory);

        computer.run().unwrap();
        assert_eq!(computer.outputs, vec![65]);
    }

    #[test]
    fn test_intcode_run_with_input() {
        let memory = HashMap::from([(0, 3), (1, 0), (2, 4), (3, 0), (4, 99)]);
        let mut computer = IntcodeComputer::new(memory);
        computer.inputs = vec![42];

        computer.run().unwrap();
        assert_eq!(computer.outputs, vec![42]);
    }

    #[test]
    fn test_send_ascii_command() {
        let command = "north\n";
        let ascii_codes: Vec<i64> = command.chars().map(|c| c as i64).collect();
        assert_eq!(ascii_codes, vec![110, 111, 114, 116, 104, 10]);
    }

    #[test]
    fn test_day25_exploration() {
        let input = std::fs::read_to_string("input/2019/day25.txt").unwrap();
        let memory = generator(&input).unwrap();
        let mut computer = IntcodeComputer::new(memory);

        let output = send_command(&mut computer, "").unwrap();
        assert!(output.contains("Hull Breach"));

        let output = send_command(&mut computer, "north").unwrap();
        assert!(output.contains("Navigation"));

        let output = send_command(&mut computer, "south").unwrap();
        assert!(output.contains("Hull Breach"));

        let output = send_command(&mut computer, "west").unwrap();
        assert!(output.contains("Science Lab"));
    }
}
