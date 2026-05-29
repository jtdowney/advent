use std::{collections::HashMap, str::FromStr};

use anyhow::{Result, bail};
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, PartialEq)]
struct Reaction {
    inputs: Vec<(usize, String)>,
    output_amount: usize,
    output_chemical: String,
}

impl FromStr for Reaction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts = s.split(" => ").collect::<Vec<&str>>();
        if parts.len() != 2 {
            bail!("Invalid reaction format");
        }

        let inputs = parts[0]
            .split(", ")
            .map(|input| {
                let input_parts = input.split(' ').collect::<Vec<&str>>();
                if input_parts.len() != 2 {
                    bail!("Invalid input format");
                }

                let amount = input_parts[0].parse::<usize>()?;
                Ok((amount, input_parts[1].to_string()))
            })
            .collect::<Result<Vec<(usize, String)>>>()?;

        let output_parts = parts[1].split(' ').collect::<Vec<&str>>();
        if output_parts.len() != 2 {
            bail!("Invalid output format");
        }

        let output_amount = output_parts[0].parse::<usize>()?;
        let output_chemical = output_parts[1].to_string();

        Ok(Reaction {
            inputs,
            output_amount,
            output_chemical,
        })
    }
}

#[aoc_generator(day14)]
fn generator(input: &str) -> Result<Vec<Reaction>> {
    input.lines().map(str::parse).collect()
}

#[aoc(day14, part1)]
fn part1(reactions: &[Reaction]) -> usize {
    calculate_ore(reactions, 1)
}

#[aoc(day14, part2)]
fn part2(reactions: &[Reaction]) -> usize {
    let ore_available = 1_000_000_000_000_usize;

    let mut low = 1;
    let mut high = ore_available;

    while low < high {
        let mid = (low + high).div_ceil(2);
        let ore_needed = calculate_ore(reactions, mid);

        if ore_needed <= ore_available {
            low = mid;
        } else {
            high = mid - 1;
        }
    }

    low
}

fn calculate_ore(reactions: &[Reaction], fuel_amount: usize) -> usize {
    let mut reaction_map = HashMap::<&str, &Reaction>::new();
    for reaction in reactions {
        reaction_map.insert(&reaction.output_chemical, reaction);
    }

    let mut needed = HashMap::new();
    needed.insert("FUEL".to_string(), fuel_amount);

    let mut leftovers = HashMap::new();

    loop {
        let next_chemical = needed
            .iter()
            .find(|(chemical, amount)| chemical.as_str() != "ORE" && **amount > 0)
            .map(|(chemical, _)| chemical.clone());

        match next_chemical {
            None => break,
            Some(chemical) => {
                let amount_needed = needed[&chemical];
                needed.insert(chemical.clone(), 0);

                if let Some(&leftover) = leftovers.get(&chemical) {
                    if leftover >= amount_needed {
                        leftovers.insert(chemical.clone(), leftover - amount_needed);
                        continue;
                    } else {
                        let remaining = amount_needed - leftover;
                        leftovers.insert(chemical.clone(), 0);

                        let reaction = reaction_map[chemical.as_str()];
                        let times = remaining.div_ceil(reaction.output_amount);
                        let produced = times * reaction.output_amount;
                        let leftover = produced - remaining;

                        if leftover > 0 {
                            *leftovers.entry(chemical).or_insert(0) += leftover;
                        }

                        for (input_amount, input_chemical) in &reaction.inputs {
                            *needed.entry(input_chemical.clone()).or_insert(0) +=
                                times * input_amount;
                        }
                    }
                } else {
                    let reaction = reaction_map[chemical.as_str()];
                    let times = amount_needed.div_ceil(reaction.output_amount);
                    let produced = times * reaction.output_amount;
                    let leftover = produced - amount_needed;

                    if leftover > 0 {
                        leftovers.insert(chemical, leftover);
                    }

                    for (input_amount, input_chemical) in &reaction.inputs {
                        *needed.entry(input_chemical.clone()).or_insert(0) += times * input_amount;
                    }
                }
            }
        }
    }

    needed.get("ORE").copied().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_reaction() {
        let input = "10 ORE => 10 A";
        let reactions = generator(input).unwrap();
        assert_eq!(reactions.len(), 1);
        assert_eq!(
            reactions[0],
            Reaction {
                inputs: vec![(10, "ORE".to_string())],
                output_amount: 10,
                output_chemical: "A".to_string(),
            }
        );
    }

    #[test]
    fn test_parse_multiple_inputs() {
        let input = "7 A, 1 B => 1 C";
        let reactions = generator(input).unwrap();
        assert_eq!(reactions.len(), 1);
        assert_eq!(
            reactions[0],
            Reaction {
                inputs: vec![(7, "A".to_string()), (1, "B".to_string())],
                output_amount: 1,
                output_chemical: "C".to_string(),
            }
        );
    }

    #[test]
    fn test_simple_ore_calculation() {
        let input = "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";
        let reactions = generator(input).unwrap();
        assert_eq!(part1(&reactions), 31);
    }

    #[test]
    fn test_second_example() {
        let input = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";
        let reactions = generator(input).unwrap();
        assert_eq!(part1(&reactions), 165);
    }

    #[test]
    fn test_third_example() {
        let input = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
        let reactions = generator(input).unwrap();
        assert_eq!(part1(&reactions), 13312);
    }

    #[test]
    fn test_fourth_example() {
        let input = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF";
        let reactions = generator(input).unwrap();
        assert_eq!(part1(&reactions), 180697);
    }

    #[test]
    fn test_fifth_example() {
        let input = "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";
        let reactions = generator(input).unwrap();
        assert_eq!(part1(&reactions), 2210736);
    }

    #[test]
    fn test_part2_third_example() {
        let input = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
        let reactions = generator(input).unwrap();
        assert_eq!(part2(&reactions), 82892753);
    }

    #[test]
    fn test_part2_fourth_example() {
        let input = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF";
        let reactions = generator(input).unwrap();
        assert_eq!(part2(&reactions), 5586022);
    }

    #[test]
    fn test_part2_fifth_example() {
        let input = "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";
        let reactions = generator(input).unwrap();
        assert_eq!(part2(&reactions), 460664);
    }
}
