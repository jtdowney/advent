use aoc_runner_derive::{aoc, aoc_generator};

use crate::vm::{Instruction, Machine, Opcode, Program};

#[aoc_generator(day21)]
fn generator(input: &str) -> anyhow::Result<Program> {
    Program::parse(input)
}

#[aoc(day21, part1)]
fn part1(input: &Program) -> usize {
    let mut comparisons = Vec::new();

    for (ip, instruction) in input.instructions.iter().enumerate() {
        if let Instruction(Opcode::Eqrr, a, b, _) = instruction {
            if *a == 0 {
                comparisons.push((ip, *b));
            } else if *b == 0 {
                comparisons.push((ip, *a));
            }
        }
    }

    let (comparison_ip, comparison_reg) = if comparisons.len() == 1 {
        comparisons[0]
    } else {
        comparisons
            .iter()
            .min_by_key(|(ip, _)| (*ip as i32 - 28).abs())
            .copied()
            .expect("No comparison with register 0 found")
    };

    let mut machine = Machine::new(input.ip_register);

    loop {
        if machine.ip == comparison_ip {
            return machine.registers[comparison_reg];
        }

        if !machine.step(&input.instructions) {
            panic!("Program terminated without finding comparison");
        }
    }
}

#[aoc(day21, part2)]
fn part2(input: &Program) -> usize {
    use std::collections::HashSet;

    let mut comparisons = Vec::new();

    for (ip, instruction) in input.instructions.iter().enumerate() {
        if let Instruction(Opcode::Eqrr, a, b, _) = instruction {
            if *a == 0 {
                comparisons.push((ip, *b));
            } else if *b == 0 {
                comparisons.push((ip, *a));
            }
        }
    }

    let (comparison_ip, comparison_reg) = if comparisons.len() == 1 {
        comparisons[0]
    } else {
        comparisons
            .iter()
            .min_by_key(|(ip, _)| (*ip as i32 - 28).abs())
            .copied()
            .expect("No comparison with register 0 found")
    };

    let mut machine = Machine::new(input.ip_register);
    let mut seen_values = HashSet::new();
    let mut last_new_value = 0;

    loop {
        if machine.ip == comparison_ip {
            let value = machine.registers[comparison_reg];

            if !seen_values.insert(value) {
                return last_new_value;
            }
            last_new_value = value;
        }

        if !machine.step(&input.instructions) {
            panic!("Program terminated without finding a cycle");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instruction() -> anyhow::Result<()> {
        let instruction: Instruction = "seti 5 0 1".parse()?;
        match instruction {
            Instruction(Opcode::Seti, 5, 0, 1) => {}
            _ => panic!("Incorrect parsing"),
        }
        Ok(())
    }

    #[test]
    fn test_machine_halts_when_register_0_matches() -> anyhow::Result<()> {
        let program = Program {
            ip_register: 3,
            instructions: vec![
                "seti 123 0 1".parse()?, // r1 = 123
                "eqrr 0 1 2".parse()?,   // r2 = (r0 == r1) ? 1 : 0
                "addr 2 3 3".parse()?,   // r3 = r2 + r3 (skip next if equal)
                "seti 0 0 3".parse()?,   // r3 = 0 (jump to beginning)
                "seti 999 0 0".parse()?, // r0 = 999 (halt by going past end)
            ],
        };

        let mut machine = Machine::new(program.ip_register);
        machine.registers[0] = 123;

        let mut steps = 0;
        while machine.step(&program.instructions) && steps < 100 {
            steps += 1;
        }

        assert!(steps < 100);
        assert_eq!(machine.ip, 5);
        Ok(())
    }

    #[test]
    fn test_find_halting_value() -> anyhow::Result<()> {
        let program = Program {
            ip_register: 3,
            instructions: vec![
                "seti 42 0 1".parse()?, // r1 = 42
                "eqrr 0 1 2".parse()?,  // r2 = (r0 == r1) ? 1 : 0
                "addr 2 3 3".parse()?,  // r3 = r2 + r3 (skip next if equal)
                "seti 0 0 3".parse()?,  // r3 = 0 (jump to beginning)
            ],
        };

        assert_eq!(part1(&program), 42);
        Ok(())
    }

    #[test]
    fn test_find_last_unique_halting_value() -> anyhow::Result<()> {
        let program = Program {
            ip_register: 5,
            instructions: vec![
                "seti 10 0 1".parse()?, // 0: r1 = 10
                "eqrr 0 1 2".parse()?,  // 1: r2 = (r0 == r1) ? 1 : 0
                "addr 2 5 5".parse()?,  // 2: r5 = r2 + r5 (skip if equal)
                "seti 4 0 5".parse()?,  // 3: r5 = 4 (jump to next value)
                "seti 99 0 0".parse()?, // 4: r0 = 99 (would halt)
                "seti 20 0 1".parse()?, // 5: r1 = 20
                "eqrr 0 1 2".parse()?,  // 6: r2 = (r0 == r1) ? 1 : 0
                "addr 2 5 5".parse()?,  // 7: r5 = r2 + r5 (skip if equal)
                "seti 9 0 5".parse()?,  // 8: r5 = 9 (jump to next value)
                "seti 99 0 0".parse()?, // 9: r0 = 99 (would halt)
                "seti 30 0 1".parse()?, // 10: r1 = 30
                "eqrr 0 1 2".parse()?,  // 11: r2 = (r0 == r1) ? 1 : 0
                "addr 2 5 5".parse()?,  // 12: r5 = r2 + r5 (skip if equal)
                "seti 0 0 5".parse()?,  // 13: r5 = 0 (jump back to start)
                "seti 99 0 0".parse()?, // 14: r0 = 99 (would halt)
            ],
        };

        assert_eq!(part2(&program), 30);
        Ok(())
    }
}
