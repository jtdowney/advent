use std::collections::VecDeque;

use aoc_runner_derive::aoc;

type Point = (i8, i8);
const START_POSITION: Point = (0, 0);
const END_POSITION: Point = (3, 3);
const WIDTH: i8 = 3;
const HEIGHT: i8 = 3;

#[derive(Clone, Debug)]
struct Step {
    position: Point,
    path: String,
}

impl Step {
    fn next_steps<'a>(&'a self, input: &'a str) -> Vec<Step> {
        let mut context = md5::Context::new();
        context.consume(input);
        context.consume(&self.path);

        let (x, y) = self.position;
        let hash = format!("{:x}", context.compute());
        hash.chars()
            .enumerate()
            .take(4)
            .filter(|(_, c)| ('b'..='f').contains(c))
            .map(move |(i, _)| {
                let position = match i {
                    0 => (x, y - 1),
                    1 => (x, y + 1),
                    2 => (x - 1, y),
                    3 => (x + 1, y),
                    _ => unreachable!(),
                };

                let path = match i {
                    0 => "U",
                    1 => "D",
                    2 => "L",
                    3 => "R",
                    _ => unreachable!(),
                };

                let mut step = self.clone();
                step.position = position;
                step.path.push_str(path);
                step
            })
            .filter(|step| {
                let (x, y) = step.position;
                x >= 0 && y >= 0 && x <= WIDTH && y <= HEIGHT
            })
            .collect()
    }
}

#[aoc(day17, part1)]
fn part1(input: &str) -> String {
    let mut next_steps = VecDeque::from_iter([Step {
        position: START_POSITION,
        path: String::new(),
    }]);

    while let Some(step) = next_steps.pop_front() {
        if step.position == END_POSITION {
            return step.path;
        }

        for step in step.next_steps(input) {
            next_steps.push_back(step);
        }
    }

    unreachable!()
}

#[aoc(day17, part2)]
fn part2(input: &str) -> Option<usize> {
    let mut next_steps = VecDeque::from_iter([Step {
        position: START_POSITION,
        path: String::new(),
    }]);

    let mut solutions = vec![];

    while let Some(step) = next_steps.pop_front() {
        if step.position == END_POSITION {
            solutions.push(step.path);
            continue;
        }

        for step in step.next_steps(input) {
            next_steps.push_back(step);
        }
    }

    solutions.iter().map(|s| s.len()).max()
}
