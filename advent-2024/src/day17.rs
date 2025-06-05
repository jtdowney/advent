use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
struct Computer {
    register_a: u64,
    register_b: u64,
    register_c: u64,
    instruction_pointer: usize,
    program: Vec<u8>,
    output: Vec<u8>,
}

impl Computer {
    fn new(register_a: u64, register_b: u64, register_c: u64, program: Vec<u8>) -> Self {
        Self {
            register_a,
            register_b,
            register_c,
            instruction_pointer: 0,
            program,
            output: Vec::new(),
        }
    }

    fn get_combo_operand(&self, operand: u8) -> u64 {
        match operand {
            0..=3 => u64::from(operand),
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => panic!("Invalid combo operand: {operand}"),
        }
    }

    fn divide_a_by_power_of_2(&self, operand: u8) -> u64 {
        self.register_a >> self.get_combo_operand(operand)
    }

    fn execute_instruction(&mut self, opcode: u8, operand: u8) {
        match opcode {
            0 => self.register_a = self.divide_a_by_power_of_2(operand),
            1 => self.register_b ^= u64::from(operand),
            2 => self.register_b = self.get_combo_operand(operand) % 8,
            3 => {
                if self.register_a != 0 {
                    self.instruction_pointer = operand as usize;
                    return;
                }
            }
            4 => self.register_b ^= self.register_c,
            5 => self
                .output
                .push((self.get_combo_operand(operand) % 8) as u8),
            6 => self.register_b = self.divide_a_by_power_of_2(operand),
            7 => self.register_c = self.divide_a_by_power_of_2(operand),
            _ => panic!("Invalid opcode: {opcode}"),
        }
        self.instruction_pointer += 2;
    }

    fn run(&mut self) {
        while self.instruction_pointer + 1 < self.program.len() {
            let (opcode, operand) = (
                self.program[self.instruction_pointer],
                self.program[self.instruction_pointer + 1],
            );
            self.execute_instruction(opcode, operand);
        }
    }

    fn get_output_string(&self) -> String {
        self.output
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(",")
    }
}

fn parse_register_line(line: &str, prefix: &str) -> Option<u64> {
    line.strip_prefix(prefix)?.parse().ok()
}

fn parse_program_line(line: &str) -> Option<Vec<u8>> {
    line.strip_prefix("Program: ")?
        .split(',')
        .map(|s| s.parse().ok())
        .collect()
}

#[aoc_generator(day17)]
fn generator(input: &str) -> Option<Computer> {
    let lines: Vec<&str> = input.lines().collect();

    let register_a = parse_register_line(lines.get(0)?, "Register A: ")?;
    let register_b = parse_register_line(lines.get(1)?, "Register B: ")?;
    let register_c = parse_register_line(lines.get(2)?, "Register C: ")?;
    let program = parse_program_line(lines.get(4)?)?;

    Some(Computer::new(register_a, register_b, register_c, program))
}

fn test_candidate_for_digit(candidate: u64, program: &[u8], target_digit: u8) -> bool {
    let mut computer = Computer::new(candidate, 0, 0, program.to_vec());
    computer.run();
    computer.output.first() == Some(&target_digit)
}

fn find_quine_register_a(program: &[u8]) -> Option<u64> {
    program
        .iter()
        .rev()
        .try_fold(vec![0u64], |candidates, &target_digit| {
            let next_candidates: Vec<u64> = candidates
                .iter()
                .flat_map(|&candidate| (0..8).map(move |bits| (candidate << 3) | bits))
                .filter(|&test_a| test_candidate_for_digit(test_a, program, target_digit))
                .collect();

            if next_candidates.is_empty() {
                None
            } else {
                Some(next_candidates)
            }
        })
        .and_then(|candidates| {
            candidates.into_iter().find(|&candidate| {
                let mut computer = Computer::new(candidate, 0, 0, program.to_vec());
                computer.run();
                computer.output == program
            })
        })
}

#[aoc(day17, part1)]
fn part1(computer: &Computer) -> String {
    let mut computer = computer.clone();
    computer.run();
    computer.get_output_string()
}

#[aoc(day17, part2)]
fn part2(computer: &Computer) -> u64 {
    find_quine_register_a(&computer.program).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut computer = Computer::new(0, 0, 9, vec![2, 6]);
        computer.run();
        assert_eq!(computer.register_b, 1);
    }

    #[test]
    fn test_example_2() {
        let mut computer = Computer::new(10, 0, 0, vec![5, 0, 5, 1, 5, 4]);
        computer.run();
        assert_eq!(computer.get_output_string(), "0,1,2");
    }

    #[test]
    fn test_example_3() {
        let mut computer = Computer::new(2024, 0, 0, vec![0, 1, 5, 4, 3, 0]);
        computer.run();
        assert_eq!(computer.get_output_string(), "4,2,5,6,7,7,7,7,3,1,0");
        assert_eq!(computer.register_a, 0);
    }

    #[test]
    fn test_example_4() {
        let mut computer = Computer::new(0, 29, 0, vec![1, 7]);
        computer.run();
        assert_eq!(computer.register_b, 26);
    }

    #[test]
    fn test_example_5() {
        let mut computer = Computer::new(0, 2024, 43690, vec![4, 0]);
        computer.run();
        assert_eq!(computer.register_b, 44354);
    }

    #[test]
    fn test_part1_example() {
        let input = r"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

        let computer = generator(input).unwrap();
        assert_eq!(part1(&computer), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part2_example() {
        let input = r"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

        let computer = generator(input).unwrap();
        assert_eq!(part2(&computer), 117_440);
    }
}
