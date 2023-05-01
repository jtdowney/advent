use aoc_runner_derive::aoc;

trait PolymerUnit {
    fn is_reacting(&self, other: Self) -> bool;
}

impl PolymerUnit for char {
    fn is_reacting(&self, other: Self) -> bool {
        if self.is_ascii_uppercase() {
            self.to_ascii_lowercase() == other
        } else {
            self.to_ascii_uppercase() == other
        }
    }
}

fn fully_react(input: &[char]) -> usize {
    let mut polymers = input.to_vec();
    'outer: loop {
        for i in 0..polymers.len() {
            let current = polymers[i];
            if let Some(&next) = polymers.get(i + 1) {
                if current.is_reacting(next) {
                    let _ = polymers.drain(i..=i + 1);
                    continue 'outer;
                }
            }
        }

        break;
    }

    polymers.len()
}

#[aoc(day5, part1)]
fn part1(input: &str) -> usize {
    let polymers = input.chars().collect::<Vec<_>>();
    fully_react(&polymers)
}

#[aoc(day5, part2)]
fn part2(input: &str) -> Option<usize> {
    ('a'..='z')
        .map(|c| {
            input
                .chars()
                .filter(|u| !c.eq_ignore_ascii_case(u))
                .collect::<Vec<_>>()
        })
        .map(|polymers| fully_react(&polymers))
        .min()
}
