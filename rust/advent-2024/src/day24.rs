use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::{Context, anyhow, bail, ensure};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GateType {
    And,
    Or,
    Xor,
}

impl FromStr for GateType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "AND" => Ok(GateType::And),
            "OR" => Ok(GateType::Or),
            "XOR" => Ok(GateType::Xor),
            _ => bail!("Unknown gate type: {s}"),
        }
    }
}

#[derive(Debug, Clone)]
struct Gate {
    input1: String,
    input2: String,
    op: GateType,
    output: String,
}

impl FromStr for Gate {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let tokens = s.split_whitespace().collect_vec();
        ensure!(
            tokens.len() == 5 && tokens[3] == "->",
            "Invalid gate format: {s}",
        );

        Ok(Gate {
            input1: tokens[0].to_string(),
            op: tokens[1].parse()?,
            input2: tokens[2].to_string(),
            output: tokens[4].to_string(),
        })
    }
}

#[derive(Debug)]
struct Circuit {
    initial_values: HashMap<String, bool>,
    gates: Vec<Gate>,
}

#[aoc_generator(day24)]
fn generator(input: &str) -> anyhow::Result<Circuit> {
    let (initial_part, gates_part) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("Invalid input format"))?;

    let initial_values = initial_part
        .lines()
        .map(|line| {
            let (wire, value) = line
                .split_once(": ")
                .ok_or_else(|| anyhow!("Invalid initial value format"))?;
            anyhow::Ok((wire.to_string(), value == "1"))
        })
        .collect::<Result<HashMap<_, _>, _>>()?;

    let gates = gates_part
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<Gate>, _>>()?;

    Ok(Circuit {
        initial_values,
        gates,
    })
}

#[aoc(day24, part1)]
fn part1(circuit: &Circuit) -> anyhow::Result<u64> {
    let mut wire_values = circuit.initial_values.clone();
    let mut remaining_gates = circuit.gates.clone();

    while !remaining_gates.is_empty() {
        let (processable, deferred): (Vec<_>, Vec<_>) =
            remaining_gates.into_iter().partition(|gate| {
                wire_values.contains_key(&gate.input1) && wire_values.contains_key(&gate.input2)
            });

        ensure!(
            !processable.is_empty(),
            "Circuit has cyclic dependencies or missing inputs"
        );

        for gate in processable {
            let input1_val = wire_values[&gate.input1];
            let input2_val = wire_values[&gate.input2];

            let output_val = match gate.op {
                GateType::And => input1_val && input2_val,
                GateType::Or => input1_val || input2_val,
                GateType::Xor => input1_val ^ input2_val,
            };

            wire_values.insert(gate.output, output_val);
        }

        remaining_gates = deferred;
    }

    Ok(wire_values
        .into_iter()
        .filter(|(wire, _)| wire.starts_with('z'))
        .sorted_by_key(|(wire, _)| wire.clone())
        .enumerate()
        .filter(|(_, (_, value))| *value)
        .fold(0u64, |acc, (i, _)| acc | (1u64 << i)))
}

impl Gate {
    fn has_xy_inputs(&self) -> bool {
        let has_x = self.input1.starts_with('x') || self.input2.starts_with('x');
        let has_y = self.input1.starts_with('y') || self.input2.starts_with('y');
        has_x && has_y
    }

    fn is_bit0_and(&self) -> bool {
        matches!(
            (&*self.input1, &*self.input2),
            ("x00", "y00") | ("y00", "x00")
        )
    }

    fn feeds_to(&self, gates: &[Gate], gate_type: GateType) -> bool {
        gates
            .iter()
            .any(|g| g.op == gate_type && (g.input1 == self.output || g.input2 == self.output))
    }

    fn feeds_to_any(&self, gates: &[Gate], types: &[GateType]) -> Vec<bool> {
        types.iter().map(|&t| self.feeds_to(gates, t)).collect()
    }
}

#[aoc(day24, part2)]
fn part2(circuit: &Circuit) -> anyhow::Result<String> {
    let max_z = circuit
        .gates
        .iter()
        .filter(|g| g.output.starts_with('z'))
        .filter_map(|g| g.output[1..].parse::<usize>().ok())
        .max()
        .context("No z-gates found")?;

    let final_carry = format!("z{max_z:02}");

    let check_z_output_type = |gate: &Gate| {
        gate.output.starts_with('z')
            && gate.output[1..].parse::<usize>().is_ok_and(|n| n < max_z)
            && gate.op != GateType::Xor
    };

    let check_xor_non_z = |gate: &Gate| {
        gate.op == GateType::Xor
            && !gate.output.starts_with('z')
            && if gate.has_xy_inputs() {
                let feeds = gate.feeds_to_any(&circuit.gates, &[GateType::Xor, GateType::And]);
                !feeds[0] || !feeds[1]
            } else {
                true
            }
    };

    let check_xy_xor_to_z = |gate: &Gate| {
        gate.op == GateType::Xor
            && gate.output.starts_with('z')
            && gate.output != "z00"
            && gate.has_xy_inputs()
    };

    let check_and_to_z = |gate: &Gate| gate.op == GateType::And && gate.output.starts_with('z');

    let check_or_to_z = |gate: &Gate| {
        gate.op == GateType::Or && gate.output.starts_with('z') && gate.output != final_carry
    };

    let check_and_missing_or = |gate: &Gate| {
        gate.op == GateType::And
            && !gate.feeds_to(&circuit.gates, GateType::Or)
            && (!gate.has_xy_inputs() || (gate.has_xy_inputs() && !gate.is_bit0_and()))
    };

    let check_or_missing_outputs = |gate: &Gate| {
        gate.op == GateType::Or && gate.output != final_carry && {
            let feeds = gate.feeds_to_any(&circuit.gates, &[GateType::Xor, GateType::And]);
            feeds.iter().any(|&f| !f)
        }
    };

    let check_xy_xor_misdirected = |bit: usize| -> Option<String> {
        if bit == 0 {
            return None;
        }

        let (x_wire, y_wire, z_wire) = (
            format!("x{bit:02}"),
            format!("y{bit:02}"),
            format!("z{bit:02}"),
        );

        circuit
            .gates
            .iter()
            .find(|g| {
                g.op == GateType::Xor
                    && g.output == z_wire
                    && ((g.input1 == x_wire && g.input2 == y_wire)
                        || (g.input1 == y_wire && g.input2 == x_wire))
            })
            .map(|g| g.output.clone())
    };

    let swapped_wires: HashSet<String> = circuit
        .gates
        .iter()
        .filter_map(|gate| {
            if check_z_output_type(gate)
                || check_xor_non_z(gate)
                || check_xy_xor_to_z(gate)
                || check_and_to_z(gate)
                || check_or_to_z(gate)
                || check_and_missing_or(gate)
                || check_or_missing_outputs(gate)
            {
                Some(gate.output.clone())
            } else {
                None
            }
        })
        .chain((0..45).filter_map(check_xy_xor_misdirected))
        .collect();

    Ok(swapped_wires.into_iter().sorted().join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";

    const EXAMPLE_2: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    #[test]
    fn test_parser() {
        let circuit = generator(EXAMPLE_1).unwrap();
        assert_eq!(circuit.initial_values.len(), 6);
        assert_eq!(circuit.gates.len(), 3);
        assert!(*circuit.initial_values.get("x00").unwrap());
        assert!(!*circuit.initial_values.get("y00").unwrap());
    }

    #[test]
    fn test_part1_example1() {
        let circuit = generator(EXAMPLE_1).unwrap();
        assert_eq!(part1(&circuit).unwrap(), 4);
    }

    #[test]
    fn test_part1_example2() {
        let circuit = generator(EXAMPLE_2).unwrap();
        assert_eq!(part1(&circuit).unwrap(), 2024);
    }
}
