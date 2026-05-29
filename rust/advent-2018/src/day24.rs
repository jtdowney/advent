use std::{
    cmp::{Ordering, Reverse},
    collections::HashSet,
    str::FromStr,
    sync::LazyLock,
};

use anyhow::{Result, anyhow};
use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum AttackType {
    Fire,
    Cold,
    Radiation,
    Slashing,
    Bludgeoning,
}

impl FromStr for AttackType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "fire" => Ok(AttackType::Fire),
            "cold" => Ok(AttackType::Cold),
            "radiation" => Ok(AttackType::Radiation),
            "slashing" => Ok(AttackType::Slashing),
            "bludgeoning" => Ok(AttackType::Bludgeoning),
            _ => anyhow::bail!("Unknown attack type: {}", s),
        }
    }
}

#[derive(Debug, Clone)]
struct Group {
    units: usize,
    hit_points: usize,
    attack_damage: usize,
    attack_type: AttackType,
    initiative: usize,
    weaknesses: Vec<AttackType>,
    immunities: Vec<AttackType>,
}

impl Group {
    fn effective_power(&self) -> usize {
        self.units * self.attack_damage
    }

    fn damage_to(&self, defender: &Group) -> usize {
        if defender.immunities.contains(&self.attack_type) {
            0
        } else if defender.weaknesses.contains(&self.attack_type) {
            self.effective_power() * 2
        } else {
            self.effective_power()
        }
    }

    fn take_damage(&mut self, damage: usize) {
        let units_killed = damage / self.hit_points;
        self.units = self.units.saturating_sub(units_killed);
    }
}

#[derive(Debug, Clone)]
struct Armies {
    immune_system: Vec<Group>,
    infection: Vec<Group>,
}

static GROUP_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(\d+) units each with (\d+) hit points (?:\(([^)]+)\) )?with an attack that does (\d+) (\w+) damage at initiative (\d+)$").unwrap()
});

impl FromStr for Group {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let caps = GROUP_RE
            .captures(s)
            .ok_or_else(|| anyhow!("Failed to parse group: {}", s))?;

        let units = caps[1].parse()?;
        let hit_points = caps[2].parse()?;
        let attack_damage = caps[4].parse()?;
        let attack_type = AttackType::from_str(&caps[5])?;
        let initiative = caps[6].parse()?;

        let mut weaknesses = Vec::new();
        let mut immunities = Vec::new();

        if let Some(modifiers) = caps.get(3) {
            for part in modifiers.as_str().split("; ") {
                if let Some(types) = part.strip_prefix("weak to ") {
                    for t in types.split(", ") {
                        weaknesses.push(AttackType::from_str(t)?);
                    }
                } else if let Some(types) = part.strip_prefix("immune to ") {
                    for t in types.split(", ") {
                        immunities.push(AttackType::from_str(t)?);
                    }
                }
            }
        }

        Ok(Group {
            units,
            hit_points,
            attack_damage,
            attack_type,
            initiative,
            weaknesses,
            immunities,
        })
    }
}

#[aoc_generator(day24)]
fn generator(input: &str) -> Result<Armies> {
    let mut sections = input.split("\n\n");

    let immune_section = sections
        .next()
        .ok_or_else(|| anyhow!("Missing immune system section"))?;

    let infection_section = sections
        .next()
        .ok_or_else(|| anyhow!("Missing infection section"))?;

    let immune_system = immune_section
        .lines()
        .skip(1)
        .map(str::parse)
        .collect::<Result<Vec<_>>>()?;

    let infection = infection_section
        .lines()
        .skip(1)
        .map(str::parse)
        .collect::<Result<Vec<_>>>()?;

    Ok(Armies {
        immune_system,
        infection,
    })
}

fn simulate_combat(armies: Armies) -> usize {
    let (_, count) = simulate_combat_with_boost(armies, 0);
    count
}

fn simulate_combat_with_boost(mut armies: Armies, boost: usize) -> (bool, usize) {
    for group in &mut armies.immune_system {
        group.attack_damage += boost;
    }

    loop {
        if armies.immune_system.is_empty() {
            return (false, armies.infection.iter().map(|g| g.units).sum());
        }

        if armies.infection.is_empty() {
            return (true, armies.immune_system.iter().map(|g| g.units).sum());
        }

        let mut groups: Vec<(bool, usize)> = Vec::new();

        for (i, _) in armies.immune_system.iter().enumerate() {
            groups.push((true, i));
        }

        for (i, _) in armies.infection.iter().enumerate() {
            groups.push((false, i));
        }

        groups.sort_by(|&(a_is_immune, a_idx), &(b_is_immune, b_idx)| {
            let a_group = if a_is_immune {
                &armies.immune_system[a_idx]
            } else {
                &armies.infection[a_idx]
            };

            let b_group = if b_is_immune {
                &armies.immune_system[b_idx]
            } else {
                &armies.infection[b_idx]
            };

            let power_cmp = b_group.effective_power().cmp(&a_group.effective_power());
            if power_cmp != Ordering::Equal {
                power_cmp
            } else {
                b_group.initiative.cmp(&a_group.initiative)
            }
        });

        let mut targets: Vec<Option<(bool, usize)>> = vec![None; groups.len()];
        let mut targeted = HashSet::new();

        for (attacker_idx, &(attacker_is_immune, attacker_group_idx)) in groups.iter().enumerate() {
            let attacker = if attacker_is_immune {
                &armies.immune_system[attacker_group_idx]
            } else {
                &armies.infection[attacker_group_idx]
            };

            let mut best_target = None;
            let mut best_damage = 0;
            let mut best_power = 0;
            let mut best_initiative = 0;

            for &(defender_is_immune, defender_group_idx) in &groups {
                if attacker_is_immune == defender_is_immune
                    || targeted.contains(&(defender_is_immune, defender_group_idx))
                {
                    continue;
                }

                let defender = if defender_is_immune {
                    &armies.immune_system[defender_group_idx]
                } else {
                    &armies.infection[defender_group_idx]
                };

                let damage = attacker.damage_to(defender);
                if damage == 0 {
                    continue;
                }

                if damage > best_damage
                    || (damage == best_damage && defender.effective_power() > best_power)
                    || (damage == best_damage
                        && defender.effective_power() == best_power
                        && defender.initiative > best_initiative)
                {
                    best_target = Some((defender_is_immune, defender_group_idx));
                    best_damage = damage;
                    best_power = defender.effective_power();
                    best_initiative = defender.initiative;
                }
            }

            if let Some(target) = best_target {
                targets[attacker_idx] = Some(target);
                targeted.insert(target);
            }
        }

        let mut attack_order: Vec<usize> = (0..groups.len()).collect();
        attack_order.sort_by_key(|&idx| {
            let (is_immune, group_idx) = groups[idx];
            let init = if is_immune {
                armies.immune_system[group_idx].initiative
            } else {
                armies.infection[group_idx].initiative
            };

            Reverse(init)
        });

        let mut any_killed = false;

        for &attacker_idx in &attack_order {
            if let Some((target_is_immune, target_group_idx)) = targets[attacker_idx] {
                let (attacker_is_immune, attacker_group_idx) = groups[attacker_idx];

                let attacker_units = if attacker_is_immune {
                    armies.immune_system[attacker_group_idx].units
                } else {
                    armies.infection[attacker_group_idx].units
                };

                if attacker_units == 0 {
                    continue;
                }

                let damage = {
                    let attacker = if attacker_is_immune {
                        &armies.immune_system[attacker_group_idx]
                    } else {
                        &armies.infection[attacker_group_idx]
                    };

                    let defender = if target_is_immune {
                        &armies.immune_system[target_group_idx]
                    } else {
                        &armies.infection[target_group_idx]
                    };

                    attacker.damage_to(defender)
                };

                let units_before = if target_is_immune {
                    armies.immune_system[target_group_idx].units
                } else {
                    armies.infection[target_group_idx].units
                };

                if target_is_immune {
                    armies.immune_system[target_group_idx].take_damage(damage);
                } else {
                    armies.infection[target_group_idx].take_damage(damage);
                }

                let units_after = if target_is_immune {
                    armies.immune_system[target_group_idx].units
                } else {
                    armies.infection[target_group_idx].units
                };

                if units_after < units_before {
                    any_killed = true;
                }
            }
        }

        armies.immune_system.retain(|g| g.units > 0);
        armies.infection.retain(|g| g.units > 0);

        if !any_killed {
            return (false, 0);
        }
    }
}

#[aoc(day24, part1)]
fn part1(input: &Armies) -> usize {
    simulate_combat(input.clone())
}

#[aoc(day24, part2)]
fn part2(input: &Armies) -> usize {
    let mut low = 0;
    let mut high = 10000;

    while high < 100000 {
        let (immune_won, _) = simulate_combat_with_boost(input.clone(), high);
        if immune_won {
            break;
        }
        high *= 2;
    }

    while low < high {
        let mid = (low + high) / 2;
        let (immune_won, units) = simulate_combat_with_boost(input.clone(), mid);

        if immune_won && units > 0 {
            high = mid;
        } else {
            low = mid + 1;
        }
    }

    let (_, units) = simulate_combat_with_boost(input.clone(), low);
    units
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";

    #[test]
    fn test_parse_groups() {
        let armies = generator(EXAMPLE_INPUT).unwrap();

        assert_eq!(armies.immune_system.len(), 2);
        assert_eq!(armies.infection.len(), 2);

        let group = &armies.immune_system[0];
        assert_eq!(group.units, 17);
        assert_eq!(group.hit_points, 5390);
        assert_eq!(group.attack_damage, 4507);
        assert_eq!(group.attack_type, AttackType::Fire);
        assert_eq!(group.initiative, 2);
        assert_eq!(
            group.weaknesses,
            vec![AttackType::Radiation, AttackType::Bludgeoning]
        );
        assert!(group.immunities.is_empty());

        let group = &armies.immune_system[1];
        assert_eq!(group.units, 989);
        assert_eq!(group.hit_points, 1274);
        assert_eq!(group.attack_damage, 25);
        assert_eq!(group.attack_type, AttackType::Slashing);
        assert_eq!(group.initiative, 3);
        assert_eq!(
            group.weaknesses,
            vec![AttackType::Bludgeoning, AttackType::Slashing]
        );
        assert_eq!(group.immunities, vec![AttackType::Fire]);

        let group = &armies.infection[0];
        assert_eq!(group.units, 801);
        assert_eq!(group.hit_points, 4706);
        assert_eq!(group.attack_damage, 116);
        assert_eq!(group.attack_type, AttackType::Bludgeoning);
        assert_eq!(group.initiative, 1);
        assert_eq!(group.weaknesses, vec![AttackType::Radiation]);
        assert!(group.immunities.is_empty());

        let group = &armies.infection[1];
        assert_eq!(group.units, 4485);
        assert_eq!(group.hit_points, 2961);
        assert_eq!(group.attack_damage, 12);
        assert_eq!(group.attack_type, AttackType::Slashing);
        assert_eq!(group.initiative, 4);
        assert_eq!(group.weaknesses, vec![AttackType::Fire, AttackType::Cold]);
        assert_eq!(group.immunities, vec![AttackType::Radiation]);
    }

    #[test]
    fn test_effective_power() {
        let group = Group {
            units: 18,
            hit_points: 729,
            attack_damage: 8,
            attack_type: AttackType::Radiation,
            initiative: 10,
            weaknesses: vec![AttackType::Fire],
            immunities: vec![AttackType::Cold, AttackType::Slashing],
        };
        assert_eq!(group.effective_power(), 144);
    }

    #[test]
    fn test_damage_calculation() {
        let attacker = Group {
            units: 10,
            hit_points: 100,
            attack_damage: 20,
            attack_type: AttackType::Fire,
            initiative: 1,
            weaknesses: vec![],
            immunities: vec![],
        };

        let defender_normal = Group {
            units: 5,
            hit_points: 50,
            attack_damage: 10,
            attack_type: AttackType::Cold,
            initiative: 2,
            weaknesses: vec![],
            immunities: vec![],
        };

        let defender_weak = Group {
            units: 5,
            hit_points: 50,
            attack_damage: 10,
            attack_type: AttackType::Cold,
            initiative: 2,
            weaknesses: vec![AttackType::Fire],
            immunities: vec![],
        };

        let defender_immune = Group {
            units: 5,
            hit_points: 50,
            attack_damage: 10,
            attack_type: AttackType::Cold,
            initiative: 2,
            weaknesses: vec![],
            immunities: vec![AttackType::Fire],
        };

        assert_eq!(attacker.damage_to(&defender_normal), 200);
        assert_eq!(attacker.damage_to(&defender_weak), 400);
        assert_eq!(attacker.damage_to(&defender_immune), 0);
    }

    #[test]
    fn test_specific_damage_calc() {
        let immune_group = Group {
            units: 191,
            hit_points: 1274,
            attack_damage: 25,
            attack_type: AttackType::Slashing,
            initiative: 3,
            weaknesses: vec![AttackType::Bludgeoning, AttackType::Slashing],
            immunities: vec![AttackType::Fire],
        };

        let infection_group = Group {
            units: 783,
            hit_points: 4706,
            attack_damage: 116,
            attack_type: AttackType::Bludgeoning,
            initiative: 1,
            weaknesses: vec![AttackType::Radiation],
            immunities: vec![],
        };

        let damage = immune_group.damage_to(&infection_group);
        assert_eq!(damage, 191 * 25);

        let units_killed = damage / infection_group.hit_points;
        assert_eq!(units_killed, 1);
    }

    #[test]
    fn test_parse_edge_cases() {
        let group: Group = "18 units each with 729 hit points with an attack that does 8 radiation damage at initiative 10".parse().unwrap();
        assert_eq!(group.units, 18);
        assert_eq!(group.hit_points, 729);
        assert_eq!(group.attack_damage, 8);
        assert_eq!(group.attack_type, AttackType::Radiation);
        assert_eq!(group.initiative, 10);
        assert!(group.weaknesses.is_empty());
        assert!(group.immunities.is_empty());

        let group: Group = "100 units each with 1000 hit points (immune to fire) with an attack that does 10 cold damage at initiative 5".parse().unwrap();
        assert_eq!(group.immunities, vec![AttackType::Fire]);
        assert!(group.weaknesses.is_empty());

        let group: Group = "100 units each with 1000 hit points (weak to fire) with an attack that does 10 cold damage at initiative 5".parse().unwrap();
        assert_eq!(group.weaknesses, vec![AttackType::Fire]);
        assert!(group.immunities.is_empty());

        let group: Group = "100 units each with 1000 hit points (immune to fire, cold) with an attack that does 10 slashing damage at initiative 5".parse().unwrap();
        assert_eq!(group.immunities, vec![AttackType::Fire, AttackType::Cold]);
        assert!(group.weaknesses.is_empty());
    }

    #[test]
    fn test_combat_example() {
        let armies = generator(EXAMPLE_INPUT).unwrap();
        let result = simulate_combat(armies);
        assert_eq!(result, 5216);
    }

    #[test]
    fn test_first_round() {
        let armies = generator(EXAMPLE_INPUT).unwrap();

        let inf2 = &armies.infection[1];
        let imm2 = &armies.immune_system[1];
        assert_eq!(inf2.damage_to(imm2), 4485 * 12 * 2);
        assert_eq!(107640 / 1274, 84);
    }

    #[test]
    fn test_detailed_combat_simulation() {
        let armies = generator(EXAMPLE_INPUT).unwrap();
        let result = simulate_combat(armies);
        assert_eq!(result, 5216);
    }

    #[test]
    fn test_boost_combat() {
        let armies = generator(EXAMPLE_INPUT).unwrap();

        let (immune_won, units) = simulate_combat_with_boost(armies, 1570);
        assert!(immune_won);
        assert_eq!(units, 51);
    }

    #[test]
    fn test_part2() {
        let armies = generator(EXAMPLE_INPUT).unwrap();

        let result = part2(&armies);
        assert_eq!(result, 51);
    }
}
