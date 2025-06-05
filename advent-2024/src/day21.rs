use std::collections::HashMap;

use anyhow::anyhow;
use aoc_runner_derive::{aoc, aoc_generator};

type Position = (i32, i32);

#[derive(Debug)]
struct Keypad {
    buttons: HashMap<char, Position>,
    gap: Position,
}

impl Keypad {
    fn numeric() -> Self {
        let buttons = [
            ('7', (0, 0)),
            ('8', (0, 1)),
            ('9', (0, 2)),
            ('4', (1, 0)),
            ('5', (1, 1)),
            ('6', (1, 2)),
            ('1', (2, 0)),
            ('2', (2, 1)),
            ('3', (2, 2)),
            ('0', (3, 1)),
            ('A', (3, 2)),
        ]
        .into_iter()
        .collect();

        Self {
            buttons,
            gap: (3, 0),
        }
    }

    fn directional() -> Self {
        let buttons = [
            ('^', (0, 1)),
            ('A', (0, 2)),
            ('<', (1, 0)),
            ('v', (1, 1)),
            ('>', (1, 2)),
        ]
        .into_iter()
        .collect();

        Self {
            buttons,
            gap: (0, 0),
        }
    }

    fn get_all_paths(&self, from: char, to: char) -> Vec<String> {
        let start = self.buttons[&from];
        let end = self.buttons[&to];

        if start == end {
            return vec!["A".to_string()];
        }

        let (sr, sc) = start;
        let (er, ec) = end;

        let dr = er - sr;
        let dc = ec - sc;

        let vert_moves = match dr.signum() {
            1 => "v".repeat(dr.unsigned_abs() as usize),
            -1 => "^".repeat(dr.unsigned_abs() as usize),
            _ => String::new(),
        };

        let horiz_moves = match dc.signum() {
            1 => ">".repeat(dc.unsigned_abs() as usize),
            -1 => "<".repeat(dc.unsigned_abs() as usize),
            _ => String::new(),
        };

        let horiz_first_valid = self.is_path_valid(start, dc, 0, dr);
        let vert_first_valid = self.is_path_valid(start, 0, dr, dc);

        match (horiz_moves.is_empty(), vert_moves.is_empty()) {
            (false, false) => {
                let mut paths = Vec::new();
                if horiz_first_valid {
                    paths.push(format!("{horiz_moves}{vert_moves}A"));
                }
                if vert_first_valid {
                    let path = format!("{vert_moves}{horiz_moves}A");
                    if !paths.contains(&path) {
                        paths.push(path);
                    }
                }
                paths
            }
            (false, true) => vec![format!("{horiz_moves}A")],
            (true, false) => vec![format!("{vert_moves}A")],
            (true, true) => vec!["A".to_string()],
        }
    }

    #[allow(clippy::similar_names)]
    fn is_path_valid(
        &self,
        start: Position,
        dc_first: i32,
        dr_first: i32,
        second_move: i32,
    ) -> bool {
        let (mut row, mut col) = start;

        for _ in 0..dc_first.abs() {
            col += dc_first.signum();
            if (row, col) == self.gap {
                return false;
            }
        }

        for _ in 0..dr_first.abs() {
            row += dr_first.signum();
            if (row, col) == self.gap {
                return false;
            }
        }

        let is_second_horizontal = dr_first != 0;
        for _ in 0..second_move.abs() {
            if is_second_horizontal {
                col += second_move.signum();
            } else {
                row += second_move.signum();
            }
            if (row, col) == self.gap {
                return false;
            }
        }

        true
    }
}

fn compute_min_length(
    sequence: &str,
    depth: usize,
    max_depth: usize,
    directional: &Keypad,
    cache: &mut HashMap<(String, usize), usize>,
) -> usize {
    if depth == max_depth {
        return sequence.len();
    }

    let key = (sequence.to_string(), depth);
    if let Some(&cached) = cache.get(&key) {
        return cached;
    }

    let total = sequence
        .chars()
        .scan('A', |current, target| {
            let paths = directional.get_all_paths(*current, target);
            *current = target;
            Some(paths)
        })
        .map(|paths| {
            paths
                .iter()
                .map(|path| compute_min_length(path, depth + 1, max_depth, directional, cache))
                .min()
                .unwrap()
        })
        .sum();

    cache.insert(key, total);
    total
}

fn find_all_sequences(code: &str, numeric: &Keypad) -> Option<Vec<String>> {
    code.chars()
        .scan(
            ('A', vec![String::new()]),
            |(current, sequences), target| {
                let paths = numeric.get_all_paths(*current, target);
                *current = target;

                *sequences = sequences
                    .iter()
                    .flat_map(|seq| paths.iter().map(move |path| format!("{seq}{path}")))
                    .collect();

                Some(sequences.clone())
            },
        )
        .last()
}

fn find_shortest_sequence(code: &str, num_robots: usize) -> Option<usize> {
    let numeric = Keypad::numeric();
    let directional = Keypad::directional();
    let mut cache = HashMap::new();

    find_all_sequences(code, &numeric)?
        .iter()
        .map(|seq| compute_min_length(seq, 0, num_robots, &directional, &mut cache))
        .min()
}

#[aoc_generator(day21)]
fn generator(input: &str) -> Vec<String> {
    input.lines().map(str::to_string).collect()
}

#[aoc(day21, part1)]
fn part1(codes: &[String]) -> anyhow::Result<usize> {
    codes
        .iter()
        .map(|code| {
            let length =
                find_shortest_sequence(code, 2).ok_or_else(|| anyhow!("No sequence found"))?;
            let numeric_part: usize = code[..code.len() - 1].parse()?;
            Ok(length * numeric_part)
        })
        .sum()
}

#[aoc(day21, part2)]
fn part2(codes: &[String]) -> anyhow::Result<usize> {
    codes
        .iter()
        .map(|code| {
            let length =
                find_shortest_sequence(code, 25).ok_or_else(|| anyhow!("No sequence found"))?;
            let numeric_part: usize = code[..code.len() - 1].parse()?;
            Ok(length * numeric_part)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numeric_keypad_paths() {
        let keypad = Keypad::numeric();

        let paths = keypad.get_all_paths('A', '0');
        assert!(paths.contains(&"<A".to_string()));

        let paths = keypad.get_all_paths('0', '2');
        assert!(paths.contains(&"^A".to_string()));

        let paths = keypad.get_all_paths('2', '9');
        assert!(
            paths
                .iter()
                .any(|p| p == ">^^A" || p == "^>^A" || p == "^^>A")
        );
    }

    #[test]
    fn test_directional_keypad_paths() {
        let keypad = Keypad::directional();

        let paths = keypad.get_all_paths('A', '<');
        assert!(paths.iter().any(|p| p == "v<<A" || p == "<v<A"));

        let paths = keypad.get_all_paths('<', 'A');
        assert!(paths.iter().any(|p| p == ">>^A" || p == ">^>A"));
    }

    #[test]
    fn test_find_shortest_sequence() {
        let length = find_shortest_sequence("029A", 2);
        assert_eq!(length, Some(68));
    }

    #[test]
    fn test_part1_example() {
        let codes = vec![
            "029A".to_string(),
            "980A".to_string(),
            "179A".to_string(),
            "456A".to_string(),
            "379A".to_string(),
        ];

        let result = part1(&codes).unwrap();
        assert_eq!(result, 126384);
    }
}
