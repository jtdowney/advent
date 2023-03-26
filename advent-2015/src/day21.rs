use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{iproduct, Itertools};

#[derive(Debug, Clone, Copy)]
enum Item {
    Weapon { cost: u16, damage: u16 },
    Armor { cost: u16, armor: u16 },
    DamageRing { cost: u16, damage: u16 },
    DefenseRing { cost: u16, armor: u16 },
}

impl Item {
    fn cost(&self) -> u16 {
        match self {
            Item::Weapon { cost, .. } => *cost,
            Item::Armor { cost, .. } => *cost,
            Item::DamageRing { cost, .. } => *cost,
            Item::DefenseRing { cost, .. } => *cost,
        }
    }

    fn damage(&self) -> u16 {
        match self {
            Item::Weapon { damage, .. } => *damage,
            Item::DamageRing { damage, .. } => *damage,
            _ => 0,
        }
    }

    fn armor(&self) -> u16 {
        match self {
            Item::Armor { armor, .. } => *armor,
            Item::DefenseRing { armor, .. } => *armor,
            _ => 0,
        }
    }
}

const WEAPONS: [Item; 5] = [
    Item::Weapon { cost: 8, damage: 4 },
    Item::Weapon {
        cost: 10,
        damage: 5,
    },
    Item::Weapon {
        cost: 25,
        damage: 6,
    },
    Item::Weapon {
        cost: 40,
        damage: 7,
    },
    Item::Weapon {
        cost: 74,
        damage: 8,
    },
];

const ARMOR: [Item; 5] = [
    Item::Armor { cost: 13, armor: 1 },
    Item::Armor { cost: 31, armor: 2 },
    Item::Armor { cost: 53, armor: 3 },
    Item::Armor { cost: 75, armor: 4 },
    Item::Armor {
        cost: 102,
        armor: 5,
    },
];

const RINGS: [Item; 6] = [
    Item::DamageRing {
        cost: 25,
        damage: 1,
    },
    Item::DamageRing {
        cost: 50,
        damage: 2,
    },
    Item::DamageRing {
        cost: 100,
        damage: 3,
    },
    Item::DefenseRing { cost: 20, armor: 1 },
    Item::DefenseRing { cost: 40, armor: 2 },
    Item::DefenseRing { cost: 80, armor: 3 },
];

struct Character {
    hit_points: u16,
    damage: u16,
    armor: u16,
}

struct Loadout {
    weapon: Item,
    armor: Option<Item>,
    rings: Vec<Item>,
}

impl Loadout {
    fn cost(&self) -> u16 {
        self.weapon.cost()
            + self.armor.map(|a| a.cost()).unwrap_or(0)
            + self.rings.iter().map(|r| r.cost()).sum::<u16>()
    }
}

#[aoc_generator(day21)]
fn generator(input: &str) -> anyhow::Result<(Character, Character)> {
    let mut lines = input.lines();

    let hit_points = lines
        .next()
        .context("Missing boss hit points")?
        .split(' ')
        .last()
        .context("Missing boss hit points")?
        .parse()?;
    let damage = lines
        .next()
        .context("Missing boss damage")?
        .split(' ')
        .last()
        .context("Missing boss damage")?
        .parse()?;
    let armor = lines
        .next()
        .context("Missing boss armor")?
        .split(' ')
        .last()
        .context("Missing boss armor")?
        .parse()?;

    let boss = Character {
        hit_points,
        damage,
        armor,
    };

    let player = Character {
        hit_points: 100,
        damage: 0,
        armor: 0,
    };

    Ok((player, boss))
}

fn fight(loadout: &Loadout, player: &Character, boss: &Character) -> bool {
    let player_damage = player.damage
        + loadout.weapon.damage()
        + loadout.rings.iter().map(|r| r.damage()).sum::<u16>();
    let player_armor = player.armor
        + loadout.armor.map(|a| a.armor()).unwrap_or(0)
        + loadout.rings.iter().map(|r| r.armor()).sum::<u16>();

    let boss_damage = boss.damage;
    let boss_armor = boss.armor;

    let player_damage = if player_damage > boss_armor {
        player_damage - boss_armor
    } else {
        1
    };

    let boss_damage = if boss_damage > player_armor {
        boss_damage - player_armor
    } else {
        1
    };

    let player_turns = (boss.hit_points as f32 / player_damage as f32).ceil() as u16;
    let boss_turns = (player.hit_points as f32 / boss_damage as f32).ceil() as u16;

    player_turns <= boss_turns
}

#[aoc(day21, part1)]
fn part1((player, boss): &(Character, Character)) -> u16 {
    let armor_combinations = (0..=1)
        .flat_map(|i| ARMOR.iter().copied().combinations(i))
        .collect_vec();
    let ring_combinations = (0..=2)
        .flat_map(|i| RINGS.iter().copied().combinations(i))
        .collect_vec();

    iproduct!(WEAPONS, armor_combinations, ring_combinations)
        .map(|(weapon, armor, rings)| {
            let armor = armor.get(0).copied();
            let rings = rings.iter().copied().collect_vec();

            Loadout {
                weapon,
                armor,
                rings,
            }
        })
        .filter(|loadout| fight(loadout, player, boss))
        .map(|loadout| loadout.cost())
        .min()
        .unwrap()
}

#[aoc(day21, part2)]
fn part2((player, boss): &(Character, Character)) -> u16 {
    let armor_combinations = (0..=1)
        .flat_map(|i| ARMOR.iter().copied().combinations(i))
        .collect_vec();
    let ring_combinations = (0..=2)
        .flat_map(|i| RINGS.iter().copied().combinations(i))
        .collect_vec();

    iproduct!(WEAPONS, armor_combinations, ring_combinations)
        .map(|(weapon, armor, rings)| {
            let armor = armor.get(0).copied();
            let rings = rings.iter().copied().collect_vec();

            Loadout {
                weapon,
                armor,
                rings,
            }
        })
        .filter(|loadout| !fight(loadout, player, boss))
        .map(|loadout| loadout.cost())
        .max()
        .unwrap()
}
