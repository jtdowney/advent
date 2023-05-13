use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point(isize, isize);

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        let Point(sx, sy) = self;
        let Point(ox, oy) = other;

        (sy, sx).cmp(&(oy, ox))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Point {
    fn adjacent(self) -> impl Iterator<Item = Point> {
        let Point(x, y) = self;
        vec![
            Point(x, y - 1),
            Point(x - 1, y),
            Point(x + 1, y),
            Point(x, y + 1),
        ]
        .into_iter()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
enum Race {
    Elf,
    Goblin,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
struct Character {
    position: Point,
    race: Race,
    hit_points: i16,
    attack_power: i16,
    dead: bool,
}

impl Character {
    fn new(race: Race, position: Point) -> Character {
        Character {
            position,
            race,
            hit_points: 200,
            attack_power: 3,
            dead: false,
        }
    }

    fn is_alive(&self) -> bool {
        !self.dead
    }

    fn take_hit(&mut self, attack: i16) {
        self.hit_points -= attack;
        if self.hit_points <= 0 {
            self.dead = true;
        }
    }
}

struct Simulation {
    characters: Vec<Character>,
    grid: HashMap<Point, bool>,
}

impl Simulation {
    fn is_complete(&self) -> bool {
        if let Some(&Character { race, .. }) = self.characters.first() {
            self.characters
                .iter()
                .filter(|c| c.is_alive())
                .all(|c| c.race == race)
        } else {
            true
        }
    }

    fn remove_dead(&mut self) {
        self.characters.retain(|c| c.is_alive());
    }

    fn find_enemy_to_attack(&self, character: &Character) -> Option<usize> {
        let character_map = self
            .characters
            .iter()
            .enumerate()
            .filter(|(_, c)| c.is_alive() && character.race != c.race)
            .map(|(i, &character)| (character.position, i))
            .collect::<HashMap<Point, usize>>();

        character
            .position
            .adjacent()
            .filter_map(|point| character_map.get(&point).map(|&i| (i, self.characters[i])))
            .min_by_key(|(_, c)| (c.hit_points, c.position))
            .map(|(i, _)| i)
    }

    fn find_move(&self, Character { position, race, .. }: &Character) -> Option<Point> {
        let targets = self
            .characters
            .iter()
            .filter(|c| c.is_alive() && c.race != *race)
            .map(|c| c.position)
            .collect::<HashSet<Point>>();

        let blocked = self
            .characters
            .iter()
            .filter(|c| c.is_alive())
            .map(|c| c.position)
            .collect::<HashSet<Point>>();

        let mut came_from = HashMap::new();
        let mut search = BinaryHeap::new();
        search.push(Reverse((0, *position)));

        while let Some(Reverse((distance, next))) = search.pop() {
            let neighbors = next
                .adjacent()
                .filter(|p| !came_from.contains_key(p))
                .filter(|p| self.grid[p])
                .collect::<Vec<Point>>();
            for neighbor in neighbors {
                if targets.contains(&neighbor) {
                    return came_from.remove(&next);
                } else if blocked.contains(&neighbor) {
                    continue;
                } else {
                    let prev = came_from.get(&next).unwrap_or(&neighbor);
                    came_from.insert(neighbor, *prev);
                    search.push(Reverse((distance + 1, neighbor)));
                }
            }
        }

        None
    }

    fn total_health(&self) -> i64 {
        self.characters
            .iter()
            .filter(|c| c.is_alive())
            .map(|c| i64::from(c.hit_points))
            .sum()
    }

    fn tick(&mut self) -> bool {
        self.characters
            .sort_by_key(|&Character { position, .. }| position);
        for i in 0..self.characters.len() {
            if !self.characters[i].is_alive() {
                continue;
            }

            if self.is_complete() {
                self.remove_dead();
                return true;
            }

            let next_move = self.find_move(&self.characters[i]);
            if let Some(point) = next_move {
                self.characters[i].position = point;
            }

            let enemy = self.find_enemy_to_attack(&self.characters[i]);
            if let Some(enemy_index) = enemy {
                let attack = self.characters[i].attack_power;
                self.characters[enemy_index].take_hit(attack);
            }
        }

        self.remove_dead();

        false
    }
}

struct Input {
    grid: HashMap<Point, bool>,
    characters: Vec<Character>,
}

#[aoc_generator(day15)]
fn generator(input: &str) -> Input {
    let (grid, characters) =
        input
            .lines()
            .enumerate()
            .fold((HashMap::new(), vec![]), |(map, characters), (y, line)| {
                line.chars().enumerate().fold(
                    (map, characters),
                    |(mut map, mut characters), (x, c)| {
                        let point = Point(x as isize, y as isize);

                        match c {
                            'E' | 'G' => {
                                map.insert(point, true);
                                let race = match c {
                                    'E' => Race::Elf,
                                    'G' => Race::Goblin,
                                    _ => unreachable!(),
                                };

                                characters.push(Character::new(race, point));
                            }
                            '.' => {
                                map.insert(point, true);
                            }
                            '#' => {
                                map.insert(point, false);
                            }
                            _ => unimplemented!(),
                        }

                        (map, characters)
                    },
                )
            });

    Input { grid, characters }
}

#[aoc(day15, part1)]
fn part1(input: &Input) -> i64 {
    let characters = input.characters.to_vec();
    let mut simulation = Simulation {
        grid: input.grid.clone(),
        characters,
    };
    for t in 0.. {
        if simulation.tick() {
            return t * simulation.total_health();
        }
    }

    unreachable!()
}

#[aoc(day15, part2)]
fn part2(input: &Input) -> i64 {
    let total_elf_count = input
        .characters
        .iter()
        .filter(|c| c.race == Race::Elf)
        .count();

    for attack in 4.. {
        let characters = input
            .characters
            .iter()
            .map(|&c| {
                if c.race == Race::Elf {
                    Character {
                        attack_power: attack,
                        ..c
                    }
                } else {
                    c
                }
            })
            .collect();

        let mut simulation = Simulation {
            grid: input.grid.clone(),
            characters,
        };
        for t in 0.. {
            if simulation.tick() {
                let elf_count = simulation
                    .characters
                    .iter()
                    .filter(|c| c.race == Race::Elf)
                    .count();
                if elf_count == total_elf_count {
                    return t * simulation.total_health();
                } else {
                    break;
                }
            }
        }
    }

    unreachable!()
}
