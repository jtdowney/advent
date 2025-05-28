use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, Context};
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy)]
enum Condition {
    Immediate,
    LessThan(char, u32),
    GreaterThan(char, u32),
}

impl Condition {
    fn apply(&self, part: &Part) -> bool {
        match self {
            Condition::Immediate => true,
            Condition::LessThan(property, value) => part
                .properties
                .get(property)
                .map(|n| n < value)
                .unwrap_or_default(),
            Condition::GreaterThan(property, value) => part
                .properties
                .get(property)
                .map(|n| n > value)
                .unwrap_or_default(),
        }
    }
}

enum Action {
    Accept,
    Reject,
    Continue(String),
}

impl FromStr for Action {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "A" => Ok(Action::Accept),
            "R" => Ok(Action::Reject),
            name => Ok(Action::Continue(name.to_owned())),
        }
    }
}

struct Rule {
    name: String,
    workflow: Vec<(Condition, Action)>,
}

impl Rule {
    fn apply(&self, part: &Part) -> Option<&Action> {
        self.workflow.iter().find_map(|(condition, action)| {
            if condition.apply(part) {
                Some(action)
            } else {
                None
            }
        })
    }
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use nom::{
            branch::alt,
            bytes::complete::take_till1,
            character::complete::{alpha1, anychar, char, u32},
            combinator::{map, map_res},
            multi::separated_list1,
            sequence::{delimited, pair, separated_pair},
            Finish, IResult, Parser,
        };

        fn condition(input: &str) -> IResult<&str, Condition> {
            map(
                (anychar, alt((char('<'), char('>'))), u32),
                |(property, operator, value)| match operator {
                    '<' => Condition::LessThan(property, value),
                    '>' => Condition::GreaterThan(property, value),
                    _ => unreachable!(),
                },
            ).parse(input)
        }

        fn conditional_action(input: &str) -> IResult<&str, (Condition, Action)> {
            separated_pair(condition, char(':'), map_res(alpha1, str::parse)).parse(input)
        }

        fn workflow_step(input: &str) -> IResult<&str, (Condition, Action)> {
            alt((
                conditional_action,
                map(map_res(alpha1, str::parse), |action| {
                    (Condition::Immediate, action)
                }),
            )).parse(input)
        }

        map(
            pair(
                map(take_till1(|c| c == '{'), str::to_owned),
                delimited(
                    char('{'),
                    separated_list1(char(','), workflow_step),
                    char('}'),
                ),
            ),
            |(name, workflow)| Rule { name, workflow },
        ).parse(input)
        .finish()
        .map(|(_, o)| o)
        .map_err(|e| anyhow!("parsing rule: {:?}", e))
    }
}

struct Part {
    properties: HashMap<char, u32>,
}

impl FromStr for Part {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use nom::{
            character::complete::{anychar, char, u32},
            combinator::map,
            error::Error,
            multi::separated_list1,
            sequence::{delimited, separated_pair},
            Finish, Parser,
        };

        map(
            delimited(
                char::<_, Error<_>>('{'),
                separated_list1(char(','), separated_pair(anychar, char('='), u32)),
                char('}'),
            ),
            |pairs| Part {
                properties: pairs.into_iter().collect(),
            },
        ).parse(input)
        .finish()
        .map(|(_, o)| o)
        .map_err(|e| anyhow!("parsing part: {:?}", e))
    }
}

#[aoc_generator(day19)]
fn generator(input: &str) -> anyhow::Result<(HashMap<String, Rule>, Vec<Part>)> {
    let (rules, parts) = input.split_once("\n\n").context("splitting input")?;
    let rules = rules
        .lines()
        .map(|line| {
            let rule = Rule::from_str(line)?;
            anyhow::Ok((rule.name.clone(), rule))
        })
        .collect::<Result<_, _>>()?;
    let parts = parts.lines().map(str::parse).collect::<Result<_, _>>()?;

    Ok((rules, parts))
}

#[aoc(day19, part1)]
fn part1((rules, parts): &(HashMap<String, Rule>, Vec<Part>)) -> u32 {
    let mut accepted = vec![];
    for part in parts {
        let mut name = "in";
        while let Some(rule) = rules.get(name) {
            match rule.apply(part) {
                Some(Action::Continue(next)) => name = next.as_str(),
                Some(Action::Accept) => {
                    accepted.push(part);
                    break;
                }
                _ => break,
            }
        }
    }

    accepted
        .iter()
        .flat_map(|part| part.properties.values())
        .sum()
}

#[aoc(day19, part2)]
fn part2((rules, _): &(HashMap<String, Rule>, Vec<Part>)) -> u64 {
    let mut properties = HashMap::new();
    for name in "xmas".chars() {
        properties.insert(name, 1..4000);
    }

    let mut ranges = HashMap::new();
    ranges.insert("in", properties);

    let mut paths = vec![];
    let mut search = vec!["in"];
    while let Some(name) = search.pop() {
        let rule = &rules[name];
        let mut rule_range = ranges[name].clone();

        for (condition, action) in &rule.workflow {
            let mut workflow_range = rule_range.clone();

            match *condition {
                Condition::Immediate => {}
                Condition::LessThan(property, n) => {
                    rule_range.entry(property).or_default().start = n;
                    workflow_range.entry(property).or_default().end = n - 1;
                }
                Condition::GreaterThan(property, n) => {
                    workflow_range.entry(property).or_default().start = n + 1;
                    rule_range.entry(property).or_default().end = n;
                }
            }

            match action {
                Action::Accept => paths.push(workflow_range),
                Action::Reject => continue,
                Action::Continue(destination) => {
                    ranges.insert(destination, workflow_range);
                    search.push(destination);
                }
            }
        }
    }

    paths
        .into_iter()
        .map(|ranges| {
            ranges
                .into_values()
                .map(|range| range.len() as u64 + 1)
                .product::<u64>()
        })
        .sum()
}
