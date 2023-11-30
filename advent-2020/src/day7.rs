use std::collections::{HashMap, VecDeque};

use regex::Regex;

const SEARCH: &str = "shiny gold";

struct Rule {
    count: usize,
    name: String,
}

#[aoc_generator(day7)]
fn generator(input: &str) -> HashMap<String, Vec<Rule>> {
    let name_re = Regex::new(r"^(.+) bags contain").unwrap();
    let rule_re = Regex::new(r"(\d+) (.+?) bags?").unwrap();
    input
        .lines()
        .map(|line| {
            let name_match = name_re.captures(line).unwrap();
            let name = name_match.get(1).map(|m| m.as_str().to_string()).unwrap();
            let rules = rule_re
                .captures_iter(line)
                .map(|rule_capture| {
                    let count = rule_capture
                        .get(1)
                        .and_then(|m| m.as_str().parse().ok())
                        .unwrap();
                    let name = rule_capture.get(2).map(|m| m.as_str().to_string()).unwrap();
                    Rule { count, name }
                })
                .collect();
            (name, rules)
        })
        .collect()
}

#[aoc(day7, part1)]
fn part1(rules: &HashMap<String, Vec<Rule>>) -> usize {
    rules
        .keys()
        .map(|k| k.as_str())
        .filter(|&name| name != SEARCH)
        .filter(|&name| {
            let mut search_queue = VecDeque::new();
            search_queue.push_back(name);

            while let Some(entry) = search_queue.pop_front() {
                let children = rules[entry]
                    .iter()
                    .map(|r| r.name.as_str())
                    .collect::<Vec<_>>();
                if children.contains(&SEARCH) {
                    return true;
                } else {
                    search_queue.extend(children);
                }
            }

            false
        })
        .count()
}

#[aoc(day7, part2)]
fn part2(rules: &HashMap<String, Vec<Rule>>) -> usize {
    let mut count = 0;
    let mut search_queue = VecDeque::new();
    search_queue.push_back((1, SEARCH));

    while let Some((n, entry)) = search_queue.pop_front() {
        for rule in &rules[entry] {
            let weight = n * rule.count;
            count += weight;
            search_queue.push_back((weight, rule.name.as_str()));
        }
    }

    count
}
