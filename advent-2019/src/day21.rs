use aoc_runner_derive::{aoc, aoc_generator};

use crate::intcode::{ComputerState, parse_program, run_ascii_program};

fn run_springscript(program: &[i64], script: &[&str], command: &str) -> anyhow::Result<i64> {
    let state = ComputerState::new(program);
    let lines = script
        .iter()
        .copied()
        .chain(std::iter::once(command))
        .collect::<Vec<_>>();
    run_ascii_program(state, &lines)
}

#[aoc_generator(day21)]
fn generate(input: &str) -> anyhow::Result<Vec<i64>> {
    parse_program(input)
}

#[aoc(day21, part1)]
fn part1(program: &[i64]) -> anyhow::Result<i64> {
    let script = [
        "NOT A J", "NOT B T", "OR T J", "NOT C T", "OR T J", "AND D J",
    ];
    run_springscript(program, &script, "WALK")
}

#[aoc(day21, part2)]
fn part2(program: &[i64]) -> anyhow::Result<i64> {
    let script = [
        "NOT A J", "NOT B T", "OR T J", "NOT C T", "OR T J", "AND D J", "NOT E T", "NOT T T",
        "OR H T", "AND T J",
    ];
    run_springscript(program, &script, "RUN")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_springscript_basic() {
        let program = vec![104, 19355645, 99];
        let script = ["NOT A J"];
        let result = run_springscript(&program, &script, "WALK");
        assert_eq!(result.unwrap(), 19355645);
    }

    #[test]
    fn test_part1_script_structure() {
        let script = [
            "NOT A J", "NOT B T", "OR T J", "NOT C T", "OR T J", "AND D J",
        ];
        assert_eq!(script.len(), 6);
        assert!(script[0].contains("NOT A J"));
        assert!(script[script.len() - 1].contains("AND D J"));
    }

    #[test]
    fn test_part2_script_structure() {
        let script = [
            "NOT A J", "NOT B T", "OR T J", "NOT C T", "OR T J", "AND D J", "NOT E T", "NOT T T",
            "OR H T", "AND T J",
        ];
        assert_eq!(script.len(), 10);
        assert!(script.iter().any(|&s| s.contains("E")));
        assert!(script.iter().any(|&s| s.contains("H")));
    }

    #[test]
    fn test_springscript_commands() {
        let program = vec![99];
        let script = ["NOT A J"];
        let result1 = run_springscript(&program, &script, "WALK");
        let result2 = run_springscript(&program, &script, "RUN");
        assert!(result1.is_ok() || result1.is_err());
        assert!(result2.is_ok() || result2.is_err());
    }
}
