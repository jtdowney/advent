use std::{collections::HashMap, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};

const START: &str = ".#./..#/###";

#[derive(Clone, Eq, Hash, PartialEq)]
struct Pattern {
    pixels: Vec<Vec<bool>>,
}

impl FromStr for Pattern {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pixels = s
            .split('/')
            .map(|row| row.chars().map(|c| c == '#').collect())
            .collect();

        Ok(Self { pixels })
    }
}

impl Pattern {
    fn rotate(&self) -> Self {
        let size = self.pixels.len();
        let mut pixels = vec![];
        for i in 0..size {
            let mut row = vec![];
            for j in (0..size).rev() {
                row.push(self.pixels[j][i]);
            }

            pixels.push(row);
        }

        Self { pixels }
    }

    fn flip(&self) -> Self {
        let size = self.pixels.len();
        let start = 0;
        let end = size - 1;
        let mut pixels = vec![];
        for i in 0..size {
            let mut row = self.pixels[i].clone();
            row.swap(start, end);
            pixels.push(row);
        }

        Self { pixels }
    }
}

impl IntoIterator for Pattern {
    type Item = Self;
    type IntoIter = PatternIter;

    fn into_iter(self) -> Self::IntoIter {
        PatternIter {
            pattern: self,
            index: 0,
        }
    }
}

struct PatternIter {
    pattern: Pattern,
    index: usize,
}

impl Iterator for PatternIter {
    type Item = Pattern;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == 8 {
            return None;
        }

        let current = self.pattern.clone();
        let mut next = self.pattern.rotate();
        if self.index == 3 {
            next = next.flip();
        }

        self.pattern = next;
        self.index += 1;

        Some(current)
    }
}

#[derive(Clone)]
struct Rule {
    input: Pattern,
    output: Pattern,
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut patterns = s
            .split(" => ")
            .map(|p| p.parse())
            .collect::<Result<Vec<Pattern>, _>>()?;
        Ok(Self {
            input: patterns.remove(0),
            output: patterns.remove(0),
        })
    }
}

struct RuleBook(HashMap<Pattern, Pattern>);

impl FromIterator<Rule> for RuleBook {
    fn from_iter<I: IntoIterator<Item = Rule>>(iter: I) -> Self {
        let mut map = HashMap::new();
        for rule in iter {
            for pattern in rule.input {
                map.insert(pattern, rule.output.clone());
            }
        }

        Self(map)
    }
}

impl RuleBook {
    fn apply(&self, pattern: &Pattern) -> Pattern {
        self.0.get(pattern).unwrap().clone()
    }
}

struct Image {
    pixels: Vec<Vec<bool>>,
}

impl FromStr for Image {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pixels = s
            .split('/')
            .map(|row| row.chars().map(|c| c == '#').collect())
            .collect();

        Ok(Self { pixels })
    }
}

impl Image {
    fn enhance(&self, rule_book: &RuleBook) -> Self {
        let patterns = self.subdivide();
        let mut new_patterns = vec![];
        for pattern in patterns {
            new_patterns.push(rule_book.apply(&pattern));
        }

        self.combine(&new_patterns)
    }

    fn subdivide(&self) -> Vec<Pattern> {
        let size = self.pixels.len();
        let mut patterns = vec![];
        if size % 2 == 0 {
            for y in (0..size).step_by(2) {
                for x in (0..size).step_by(2) {
                    let mut pixels = vec![];
                    for i in 0..2 {
                        let mut row = vec![];
                        for j in 0..2 {
                            row.push(self.pixels[y + i][x + j]);
                        }

                        pixels.push(row);
                    }

                    patterns.push(Pattern { pixels });
                }
            }
        } else {
            for y in (0..size).step_by(3) {
                for x in (0..size).step_by(3) {
                    let mut pixels = vec![];
                    for i in 0..3 {
                        let mut row = vec![];
                        for j in 0..3 {
                            row.push(self.pixels[y + i][x + j]);
                        }

                        pixels.push(row);
                    }

                    patterns.push(Pattern { pixels });
                }
            }
        }

        patterns
    }

    fn combine(&self, patterns: &[Pattern]) -> Self {
        let size = (patterns.len() as f64).sqrt() as usize;
        let pattern_size = patterns[0].pixels.len();
        let new_size = size * pattern_size;
        let mut pixels = vec![vec![false; new_size]; new_size];
        for y in 0..size {
            for x in 0..size {
                let pattern = &patterns[y * size + x];
                for i in 0..pattern_size {
                    for j in 0..pattern_size {
                        pixels[y * pattern_size + i][x * pattern_size + j] = pattern.pixels[i][j];
                    }
                }
            }
        }

        Image { pixels }
    }

    fn count_ones(&self) -> usize {
        self.pixels
            .iter()
            .map(|row| row.iter().filter(|&&b| b).count())
            .sum()
    }
}

#[aoc_generator(day21)]
fn generator(input: &str) -> anyhow::Result<Vec<Rule>> {
    input.lines().map(|line| line.parse()).collect()
}

#[aoc(day21, part1)]
fn part1(input: &[Rule]) -> usize {
    let rule_book = input.iter().cloned().collect::<RuleBook>();
    let mut image = START.parse::<Image>().unwrap();
    for _ in 0..5 {
        image = image.enhance(&rule_book);
    }

    image.count_ones()
}

#[aoc(day21, part2)]
fn part2(input: &[Rule]) -> usize {
    let rule_book = input.iter().cloned().collect::<RuleBook>();
    let mut image = START.parse::<Image>().unwrap();
    for _ in 0..18 {
        image = image.enhance(&rule_book);
    }

    image.count_ones()
}
