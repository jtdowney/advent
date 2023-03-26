use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EffectBenefit {
    Armor(u32),
    Damage(u32),
    Mana(u32),
}

#[derive(Clone, Copy, Debug)]
struct Effect {
    benefit: EffectBenefit,
    duration: u32,
}

#[derive(Clone, Copy, Debug)]
struct Spell {
    cost: u32,
    damage: u32,
    heal: u32,
    effect: Option<Effect>,
}

const SPELLS: [Spell; 5] = [
    Spell {
        cost: 53,
        damage: 4,
        heal: 0,
        effect: None,
    },
    Spell {
        cost: 73,
        damage: 2,
        heal: 2,
        effect: None,
    },
    Spell {
        cost: 113,
        damage: 0,
        heal: 0,
        effect: Some(Effect {
            benefit: EffectBenefit::Armor(7),
            duration: 6,
        }),
    },
    Spell {
        cost: 173,
        damage: 0,
        heal: 0,
        effect: Some(Effect {
            benefit: EffectBenefit::Damage(3),
            duration: 6,
        }),
    },
    Spell {
        cost: 229,
        damage: 0,
        heal: 0,
        effect: Some(Effect {
            benefit: EffectBenefit::Mana(101),
            duration: 5,
        }),
    },
];

#[derive(Clone, Copy, Debug)]
struct Player {
    hit_points: u32,
    mana: u32,
    armor: u32,
}

#[derive(Clone, Copy, Debug)]
struct Boss {
    hit_points: u32,
    damage: u32,
}

#[derive(Clone, Debug)]
struct Game {
    player: Player,
    boss: Boss,
    effects: Vec<Effect>,
    turn: u32,
    mana_spent: u32,
}

#[aoc_generator(day22)]
fn generator(input: &str) -> anyhow::Result<Boss> {
    let mut lines = input.lines();
    let hit_points = lines
        .next()
        .context("Boss hit points")?
        .split(": ")
        .last()
        .context("Boss hit points")?
        .parse()?;
    let damage = lines
        .next()
        .context("Boss damage")?
        .split(": ")
        .last()
        .context("Boss damage")?
        .parse()?;
    Ok(Boss { hit_points, damage })
}

const PLAYER: Player = Player {
    hit_points: 50,
    mana: 500,
    armor: 0,
};

fn play(boss: Boss, hard: bool) -> u32 {
    let game = Game {
        player: PLAYER,
        boss,
        effects: Vec::new(),
        turn: 0,
        mana_spent: 0,
    };

    let mut min_mana_spent = u32::MAX;
    let mut queue = vec![game];

    while let Some(mut current) = queue.pop() {
        if current.mana_spent >= min_mana_spent || current.player.hit_points == 0 {
            continue;
        }

        if current.boss.hit_points == 0 {
            min_mana_spent = min_mana_spent.min(current.mana_spent);
            continue;
        }

        if current.turn % 2 == 0 {
            // player's turn
            if hard {
                current.player.hit_points = current.player.hit_points.saturating_sub(1);
                if current.player.hit_points == 0 {
                    continue;
                }
            }

            for effect in current.effects.iter_mut() {
                effect.duration -= 1;

                match effect.benefit {
                    EffectBenefit::Damage(damage) => {
                        current.boss.hit_points = current.boss.hit_points.saturating_sub(damage)
                    }
                    EffectBenefit::Mana(mana) => current.player.mana += mana,
                    EffectBenefit::Armor(armor) => {
                        if effect.duration == 0 {
                            current.player.armor -= armor;
                        }
                    }
                }
            }

            current.effects.retain(|effect| effect.duration > 0);

            let next_spells = SPELLS
                .iter()
                .filter(|spell| spell.cost <= current.player.mana)
                .filter(|spell| {
                    spell
                        .effect
                        .as_ref()
                        .map(|effect| {
                            current
                                .effects
                                .iter()
                                .all(|current_effect| current_effect.benefit != effect.benefit)
                        })
                        .unwrap_or(true)
                });
            for spell in next_spells {
                // println!("Player casts {:?} spell", &spell);
                let mut next_game = current.clone();
                next_game.turn += 1;
                next_game.mana_spent += spell.cost;
                next_game.player.mana -= spell.cost;
                next_game.boss.hit_points = next_game.boss.hit_points.saturating_sub(spell.damage);
                next_game.player.hit_points += spell.heal;

                if let Some(effect) = spell.effect {
                    if let EffectBenefit::Armor(armor) = effect.benefit {
                        next_game.player.armor += armor;
                    }

                    next_game.effects.push(effect);
                }

                queue.push(next_game);
            }
        } else {
            // boss's turn
            for effect in current.effects.iter_mut() {
                effect.duration -= 1;

                match effect.benefit {
                    EffectBenefit::Damage(damage) => {
                        current.boss.hit_points = current.boss.hit_points.saturating_sub(damage)
                    }
                    EffectBenefit::Mana(mana) => current.player.mana += mana,
                    EffectBenefit::Armor(armor) => {
                        if effect.duration == 0 {
                            current.player.armor -= armor;
                        }
                    }
                }
            }

            current.effects.retain(|effect| effect.duration > 0);

            let damage = current.boss.damage.saturating_sub(current.player.armor);
            current.player.hit_points = current.player.hit_points.saturating_sub(damage);
            current.turn += 1;
            queue.push(current);
        }
    }

    min_mana_spent
}

#[aoc(day22, part1)]
fn part1(boss: &Boss) -> u32 {
    play(*boss, false)
}

#[aoc(day22, part2)]
fn part2(boss: &Boss) -> u32 {
    play(*boss, true)
}
