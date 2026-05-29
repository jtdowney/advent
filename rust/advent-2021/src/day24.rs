use std::str::FromStr;

use anyhow::{Result, ensure};
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, PartialEq)]
enum Instruction {
    Inp(char),
    Add(char, Operand),
    Mul(char, Operand),
    Div(char, Operand),
    Mod(char, Operand),
    Eql(char, Operand),
}

#[derive(Debug, Clone, PartialEq)]
enum Operand {
    Variable(char),
    Literal(i64),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split_whitespace().collect();

        if parts.is_empty() {
            return Err(anyhow::anyhow!("Empty instruction"));
        }

        fn parse_two_arg(
            parts: Vec<&str>,
            op: fn(char, Operand) -> Instruction,
        ) -> Result<Instruction> {
            ensure!(
                parts.len() == 3,
                "{} requires exactly 2 arguments",
                parts[0]
            );

            let var = parts[1].chars().next().unwrap();
            let operand = if let Ok(num) = parts[2].parse::<i64>() {
                Operand::Literal(num)
            } else {
                Operand::Variable(parts[2].chars().next().unwrap())
            };
            Ok(op(var, operand))
        }

        match parts[0] {
            "inp" => {
                ensure!(parts.len() == 2, "inp requires exactly 1 argument");
                Ok(Instruction::Inp(parts[1].chars().next().unwrap()))
            }
            "add" => parse_two_arg(parts, Instruction::Add),
            "mul" => parse_two_arg(parts, Instruction::Mul),
            "div" => parse_two_arg(parts, Instruction::Div),
            "mod" => parse_two_arg(parts, Instruction::Mod),
            "eql" => parse_two_arg(parts, Instruction::Eql),
            _ => Err(anyhow::anyhow!("Unknown instruction: {}", parts[0])),
        }
    }
}

#[aoc_generator(day24)]
fn generator(input: &str) -> Result<Vec<Instruction>> {
    input.lines().map(str::parse).collect()
}

#[derive(Debug, Default)]
struct Alu {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

impl Alu {
    fn execute(&mut self, instructions: &[Instruction], inputs: &[i64]) -> Result<()> {
        let mut input_index = 0;

        for instruction in instructions {
            match instruction {
                Instruction::Inp(var) => {
                    self.set_var(*var, inputs[input_index]);
                    input_index += 1;
                }
                Instruction::Add(var, operand) => {
                    let result = self.get_var(*var) + self.get_operand_value(operand);
                    self.set_var(*var, result);
                }
                Instruction::Mul(var, operand) => {
                    let result = self.get_var(*var) * self.get_operand_value(operand);
                    self.set_var(*var, result);
                }
                Instruction::Div(var, operand) => {
                    let result = self.get_var(*var) / self.get_operand_value(operand);
                    self.set_var(*var, result);
                }
                Instruction::Mod(var, operand) => {
                    let result = self.get_var(*var) % self.get_operand_value(operand);
                    self.set_var(*var, result);
                }
                Instruction::Eql(var, operand) => {
                    let result = if self.get_var(*var) == self.get_operand_value(operand) {
                        1
                    } else {
                        0
                    };
                    self.set_var(*var, result);
                }
            }
        }

        Ok(())
    }

    fn get_var(&self, var: char) -> i64 {
        match var {
            'w' => self.w,
            'x' => self.x,
            'y' => self.y,
            'z' => self.z,
            _ => panic!("Unknown variable: {}", var),
        }
    }

    fn set_var(&mut self, var: char, value: i64) {
        match var {
            'w' => self.w = value,
            'x' => self.x = value,
            'y' => self.y = value,
            'z' => self.z = value,
            _ => panic!("Unknown variable: {}", var),
        }
    }

    fn get_operand_value(&self, operand: &Operand) -> i64 {
        match operand {
            Operand::Variable(var) => self.get_var(*var),
            Operand::Literal(value) => *value,
        }
    }
}

fn validate_model_number(instructions: &[Instruction], digits: &[i64]) -> Result<bool> {
    let mut alu = Alu::default();
    alu.execute(instructions, digits)?;
    Ok(alu.z == 0)
}

fn extract_literal(instruction: &Instruction) -> Result<i64> {
    match instruction {
        Instruction::Div(_, Operand::Literal(val)) => Ok(*val),
        Instruction::Add(_, Operand::Literal(val)) => Ok(*val),
        _ => Err(anyhow::anyhow!("Expected literal operand")),
    }
}

fn extract_monad_blocks(instructions: &[Instruction]) -> Result<Vec<(i64, i64, i64)>> {
    (0..14)
        .map(|i| {
            let start = i * 18;
            Ok((
                extract_literal(&instructions[start + 4])?,
                extract_literal(&instructions[start + 5])?,
                extract_literal(&instructions[start + 15])?,
            ))
        })
        .collect()
}

fn solve_monad_constraints(instructions: &[Instruction], largest: bool) -> Result<i64> {
    let blocks = extract_monad_blocks(instructions)?;
    let mut digits = [0; 14];
    let mut stack = Vec::new();

    for (i, (div_z, add_x, add_y)) in blocks.iter().enumerate() {
        if *div_z == 1 {
            stack.push((i, *add_y));
        } else if let Some((j, prev_add_y)) = stack.pop() {
            let diff = prev_add_y + add_x;
            let range: Box<dyn Iterator<Item = i64>> = if largest {
                Box::new((1..=9).rev())
            } else {
                Box::new(1..=9)
            };

            for digit_j in range {
                let digit_i = digit_j + diff;
                if (1..=9).contains(&digit_i) {
                    digits[j] = digit_j;
                    digits[i] = digit_i;
                    break;
                }
            }
        }
    }

    Ok(digits.iter().fold(0, |acc, &digit| acc * 10 + digit))
}

#[aoc(day24, part1)]
fn part1(instructions: &[Instruction]) -> Result<i64> {
    let num_inputs = instructions
        .iter()
        .filter(|i| matches!(i, Instruction::Inp(_)))
        .count();

    match num_inputs {
        0 => Ok(0),
        1..=2 => Ok((1..=9)
            .rev()
            .map(|d| vec![d; num_inputs])
            .find(|digits| validate_model_number(instructions, digits).unwrap_or(false))
            .map(|digits| digits.iter().fold(0, |acc, &digit| acc * 10 + digit))
            .unwrap_or(0)),
        14 => solve_monad_constraints(instructions, true),
        _ => Ok(0),
    }
}

#[aoc(day24, part2)]
fn part2(instructions: &[Instruction]) -> Result<i64> {
    let num_inputs = instructions
        .iter()
        .filter(|i| matches!(i, Instruction::Inp(_)))
        .count();

    match num_inputs {
        0 => Ok(0),
        1..=2 => Ok((1..=9)
            .map(|d| vec![d; num_inputs])
            .find(|digits| validate_model_number(instructions, digits).unwrap_or(false))
            .map(|digits| digits.iter().fold(0, |acc, &digit| acc * 10 + digit))
            .unwrap_or(0)),
        14 => solve_monad_constraints(instructions, false),
        _ => Ok(0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn model_number_to_digits(model_number: i64) -> Vec<i64> {
        let mut digits = Vec::new();
        let mut num = model_number;

        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }

        digits.reverse();
        digits
    }

    #[test]
    fn test_parse_instruction() {
        let input = "inp x";
        let instructions = generator(input).unwrap();
        assert_eq!(instructions.len(), 1);
        assert_eq!(instructions[0], Instruction::Inp('x'));
    }

    #[test]
    fn test_parse_add_with_variable() {
        let input = "add x y";
        let instructions = generator(input).unwrap();
        assert_eq!(instructions.len(), 1);
        assert_eq!(
            instructions[0],
            Instruction::Add('x', Operand::Variable('y'))
        );
    }

    #[test]
    fn test_parse_add_with_literal() {
        let input = "add x -1";
        let instructions = generator(input).unwrap();
        assert_eq!(instructions.len(), 1);
        assert_eq!(instructions[0], Instruction::Add('x', Operand::Literal(-1)));
    }

    #[test]
    fn test_parse_multiple_instructions() {
        let input = "inp x\nmul x -1";
        let instructions = generator(input).unwrap();
        assert_eq!(instructions.len(), 2);
        assert_eq!(instructions[0], Instruction::Inp('x'));
        assert_eq!(instructions[1], Instruction::Mul('x', Operand::Literal(-1)));
    }

    #[test]
    fn test_parse_all_instruction_types() {
        let input = "inp w\nadd x y\nmul y 2\ndiv z 5\nmod w 3\neql x z";
        let instructions = generator(input).unwrap();
        assert_eq!(instructions.len(), 6);
        assert_eq!(instructions[0], Instruction::Inp('w'));
        assert_eq!(
            instructions[1],
            Instruction::Add('x', Operand::Variable('y'))
        );
        assert_eq!(instructions[2], Instruction::Mul('y', Operand::Literal(2)));
        assert_eq!(instructions[3], Instruction::Div('z', Operand::Literal(5)));
        assert_eq!(instructions[4], Instruction::Mod('w', Operand::Literal(3)));
        assert_eq!(
            instructions[5],
            Instruction::Eql('x', Operand::Variable('z'))
        );
    }

    #[test]
    fn test_alu_simple_input() {
        let input = "inp x\nmul x -1";
        let instructions = generator(input).unwrap();
        let mut alu = Alu::default();
        alu.execute(&instructions, &[5]).unwrap();
        assert_eq!(alu.x, -5);
    }

    #[test]
    fn test_alu_example_program() {
        let input = "inp z\ninp x\nmul z 3\neql z x";
        let instructions = generator(input).unwrap();
        let mut alu = Alu::default();
        alu.execute(&instructions, &[3, 9]).unwrap();
        assert_eq!(alu.z, 1);

        let mut alu2 = Alu::default();
        alu2.execute(&instructions, &[3, 8]).unwrap();
        assert_eq!(alu2.z, 0);
    }

    #[test]
    fn test_alu_binary_conversion() {
        let input = "inp w\nadd z w\nmod z 2\ndiv w 2\nadd y w\nmod y 2\ndiv w 2\nadd x w\nmod x 2\ndiv w 2\nmod w 2";
        let instructions = generator(input).unwrap();
        let mut alu = Alu::default();
        alu.execute(&instructions, &[11]).unwrap();
        assert_eq!(alu.w, 1);
        assert_eq!(alu.x, 0);
        assert_eq!(alu.y, 1);
        assert_eq!(alu.z, 1);
    }

    #[test]
    fn test_is_valid_model_number() {
        let model_number = 13579246899999_i64;
        let digits = model_number_to_digits(model_number);
        assert_eq!(digits.len(), 14);
        assert_eq!(digits[0], 1);
        assert_eq!(digits[13], 9);
        assert!(digits.iter().all(|&d| (1..=9).contains(&d)));
    }

    #[test]
    fn test_validate_model_number_basic() {
        let input = "inp w\nmul w 0";
        let instructions = generator(input).unwrap();
        assert!(validate_model_number(&instructions, &[1]).unwrap());
    }

    #[test]
    fn test_validate_model_number_invalid() {
        let input = "inp w\nadd z w";
        let instructions = generator(input).unwrap();
        assert!(!validate_model_number(&instructions, &[1]).unwrap());
    }

    #[test]
    fn test_find_largest_simple() {
        let input = "inp w\nadd z w\nadd z -1";
        let instructions = generator(input).unwrap();

        assert!(validate_model_number(&instructions, &[1]).unwrap());
        assert!(!validate_model_number(&instructions, &[2]).unwrap());

        let largest = part1(&instructions).unwrap();
        assert_eq!(largest, 1);
    }

    #[test]
    fn test_verify_solution() {
        let candidate = 45989929946199_i64;
        let digits = model_number_to_digits(candidate);

        assert_eq!(digits.len(), 14);
        assert!(digits.iter().all(|&d| (1..=9).contains(&d)));

        let input = std::fs::read_to_string("input/2021/day24.txt").unwrap();
        let instructions = generator(&input).unwrap();
        let is_valid = validate_model_number(&instructions, &digits).unwrap();
        assert!(is_valid);
    }

    #[test]
    fn test_find_smallest_simple() {
        let input = "inp w\nadd z w\nadd z -1";
        let instructions = generator(input).unwrap();

        let smallest = part2(&instructions).unwrap();
        assert_eq!(smallest, 1);
    }

    #[test]
    fn test_verify_smallest_solution() {
        let candidate = 11912814611156_i64;
        let digits = model_number_to_digits(candidate);

        assert_eq!(digits.len(), 14);
        assert!(digits.iter().all(|&d| (1..=9).contains(&d)));

        let input = std::fs::read_to_string("input/2021/day24.txt").unwrap();
        let instructions = generator(&input).unwrap();
        let is_valid = validate_model_number(&instructions, &digits).unwrap();
        assert!(is_valid);
    }

    #[test]
    fn test_instruction_from_str() {
        use std::str::FromStr;

        assert_eq!(
            Instruction::from_str("inp x").unwrap(),
            Instruction::Inp('x')
        );
        assert_eq!(
            Instruction::from_str("add x y").unwrap(),
            Instruction::Add('x', Operand::Variable('y'))
        );
        assert_eq!(
            Instruction::from_str("add x -1").unwrap(),
            Instruction::Add('x', Operand::Literal(-1))
        );
        assert_eq!(
            Instruction::from_str("mul y 2").unwrap(),
            Instruction::Mul('y', Operand::Literal(2))
        );
        assert_eq!(
            Instruction::from_str("div z 5").unwrap(),
            Instruction::Div('z', Operand::Literal(5))
        );
        assert_eq!(
            Instruction::from_str("mod w 3").unwrap(),
            Instruction::Mod('w', Operand::Literal(3))
        );
        assert_eq!(
            Instruction::from_str("eql x z").unwrap(),
            Instruction::Eql('x', Operand::Variable('z'))
        );
    }

    #[test]
    fn test_instruction_from_str_invalid() {
        use std::str::FromStr;

        assert!(Instruction::from_str("invalid x").is_err());
        assert!(Instruction::from_str("inp").is_err());
        assert!(Instruction::from_str("add x").is_err());
    }
}
