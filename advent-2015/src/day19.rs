use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

struct Replacement {
    from: String,
    to: String,
}

struct Input {
    replacements: Vec<Replacement>,
    start: String,
}

#[aoc_generator(day19)]
fn generator(input: &str) -> Input {
    let parts = input.split("\n\n").collect::<Vec<_>>();
    let replacements = parts[0]
        .lines()
        .map(|line| {
            let parts = line.split(" => ").collect::<Vec<_>>();
            Replacement {
                from: parts[0].to_string(),
                to: parts[1].to_string(),
            }
        })
        .collect::<Vec<_>>();
    let start = parts[1].to_string();

    Input {
        replacements,
        start,
    }
}

#[aoc(day19, part1)]
fn part1(input: &Input) -> usize {
    let mut molecules = HashSet::new();
    for replacement in &input.replacements {
        for (i, _) in input.start.match_indices(&replacement.from) {
            let mut molecule = input.start.clone();
            molecule.replace_range(i..i + replacement.from.len(), &replacement.to);
            molecules.insert(molecule);
        }
    }

    molecules.len()
}

#[aoc(day19, part2)]
fn part2(input: &Input) -> usize {
    let mut steps = 0;
    let mut molecule = input.start.clone();
    while molecule != "e" {
        for replacement in &input.replacements {
            if let Some(i) = molecule.find(&replacement.to) {
                molecule.replace_range(i..i + replacement.to.len(), &replacement.from);
                steps += 1;
                break;
            }
        }
    }

    steps
}
