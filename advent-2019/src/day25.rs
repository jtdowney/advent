use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};

use crate::intcode::{ComputerState, ascii_to_codes, parse_program, run_to_completion};

type Path = Vec<String>;
type ItemLocations = HashMap<String, Path>;

#[aoc_generator(day25)]
fn generator(input: &str) -> Result<Vec<i64>> {
    parse_program(input)
}

fn send_command(state: ComputerState, cmd: &str) -> (ComputerState, String) {
    let mut state = state;
    if !cmd.is_empty() {
        state.inputs.extend(ascii_to_codes(&format!("{}\n", cmd)));
    }

    let (new_state, outputs) = run_to_completion(state);
    let output_text = outputs
        .into_iter()
        .filter(|&c| (0..=127).contains(&c))
        .map(|c| c as u8 as char)
        .collect();

    (new_state, output_text)
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

fn run_computer_with_path(program: &[i64], path: &Path) -> String {
    let mut state = ComputerState::new(program);
    let (new_state, initial_output) = send_command(state, "");
    state = new_state;

    if path.is_empty() {
        initial_output
    } else {
        let mut last_output = String::new();
        for cmd in path {
            let (new_state, output) = send_command(state, cmd);
            state = new_state;
            last_output = output;
        }
        last_output
    }
}

fn explore_ship(program: &[i64]) -> Result<(Path, ItemLocations)> {
    let dangerous_items: HashSet<_> = [
        "escape pod", "giant electromagnet", "infinite loop", "molten lava", "photons",
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

        let output = run_computer_with_path(program, &path);
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
    mut state: ComputerState,
    path: &Path,
    action: impl FnOnce(ComputerState) -> (ComputerState, String),
) -> (ComputerState, String) {
    for cmd in path {
        let (new_state, _) = send_command(state, cmd);
        state = new_state;
    }

    let (mut state, result) = action(state);

    for cmd in path.iter().rev() {
        let (new_state, _) = send_command(state, opposite_dir(cmd));
        state = new_state;
    }

    (state, result)
}

fn find_password(
    program: &[i64],
    checkpoint_path: Path,
    item_locations: ItemLocations,
) -> Result<String> {
    let mut state = ComputerState::new(program);
    let (new_state, _) = send_command(state, "");
    state = new_state;

    let items_collected: Vec<_> = item_locations
        .into_iter()
        .filter_map(|(item, path)| {
            let (new_state, output) = navigate_and_execute(state.clone(), &path, |s| {
                send_command(s, &format!("take {}", item))
            });
            if output.contains("You take") {
                state = new_state;
                Some(item)
            } else {
                None
            }
        })
        .collect();

    for cmd in &checkpoint_path {
        let (new_state, _) = send_command(state, cmd);
        state = new_state;
    }

    let (new_state, last_output) = send_command(state.clone(), "");
    state = new_state;
    let (_, doors, _) = parse_room_info(&last_output);

    let pressure_plate_dir = doors
        .iter()
        .find(|&dir| {
            let (new_state, output) = send_command(state.clone(), dir);
            let found = output.contains("Pressure-Sensitive Floor");
            if output.contains("==") && !output.contains("Security Checkpoint") {
                let (_, _) = send_command(new_state, opposite_dir(dir));
            }
            found
        })
        .ok_or_else(|| anyhow::anyhow!("Could not find pressure plate direction"))?;

    (0..(1 << items_collected.len()))
        .find_map(|mask| {
            let mut test_state = state.clone();

            for item in &items_collected {
                let (new_state, _) = send_command(test_state, &format!("drop {}", item));
                test_state = new_state;
            }

            for (i, item) in items_collected.iter().enumerate() {
                if mask & (1 << i) != 0 {
                    let (new_state, _) = send_command(test_state, &format!("take {}", item));
                    test_state = new_state;
                }
            }

            let (_, output) = send_command(test_state, pressure_plate_dir);
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
        .ok_or_else(|| anyhow::anyhow!("Could not find the correct item combination"))
}

#[aoc(day25, part1)]
fn part1(program: &[i64]) -> Result<String> {
    let (checkpoint_path, item_locations) = explore_ship(program)?;
    find_password(program, checkpoint_path, item_locations)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let program = generator("1,2,3,4,5").unwrap();
        assert_eq!(program, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_ascii_conversion() {
        let output = vec![72, 101, 108, 108, 111, 10];
        let text: String = output.iter().map(|&c| c as u8 as char).collect();
        assert_eq!(text, "Hello\n");
    }

    #[test]
    fn test_send_command() {
        let program = vec![104, 79, 104, 75, 104, 10, 99];
        let state = ComputerState::new(&program);
        let (_, output) = send_command(state, "");
        assert_eq!(output, "OK\n");
    }

    #[test]
    fn test_parse_room_info() {
        let output = "== Hull Breach ==\nYou got in through a hole in the floor here. To keep your ship from also freezing, the hole has been sealed.\n\nDoors here lead:\n- north\n- south\n- west\n\nItems here:\n- fixed point\n\nCommand?";
        let (room_name, doors, items) = parse_room_info(output);
        assert_eq!(room_name, Some("== Hull Breach ==".to_string()));
        assert_eq!(doors, vec!["north", "south", "west"]);
        assert_eq!(items, vec!["fixed point"]);
    }

    #[test]
    fn test_opposite_dir() {
        assert_eq!(opposite_dir("north"), "south");
        assert_eq!(opposite_dir("south"), "north");
        assert_eq!(opposite_dir("east"), "west");
        assert_eq!(opposite_dir("west"), "east");
    }

    #[test]
    fn test_day25_exploration() {
        let input = std::fs::read_to_string("input/2019/day25.txt").unwrap();
        let program = generator(&input).unwrap();
        let state = ComputerState::new(&program);

        let (state, output) = send_command(state, "");
        assert!(output.contains("Hull Breach"));

        let (state, output) = send_command(state, "north");
        assert!(output.contains("Navigation"));

        let (state, output) = send_command(state, "south");
        assert!(output.contains("Hull Breach"));

        let (_, output) = send_command(state, "west");
        assert!(output.contains("Science Lab"));
    }
}