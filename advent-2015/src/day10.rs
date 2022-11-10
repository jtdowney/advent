use aoc_runner_derive::aoc;

fn expand(value: &str) -> String {
    let (mut acc, prev, count) = value.chars().fold(
        (String::new(), None, 0),
        |(mut acc, prev, count), c| match prev {
            Some(p) if p == c => (acc, prev, count + 1),
            Some(p) => {
                acc.push_str(&format!("{count}{p}"));
                (acc, Some(c), 1)
            }
            None => (acc, Some(c), 1),
        },
    );

    if let Some(p) = prev {
        acc.push_str(&format!("{count}{p}"));
    }

    acc
}

#[aoc(day10, part1)]
fn part1(input: &str) -> usize {
    (0..40).fold(input.to_string(), |acc, _| expand(&acc)).len()
}

#[aoc(day10, part2)]
fn part2(input: &str) -> usize {
    (0..50).fold(input.to_string(), |acc, _| expand(&acc)).len()
}
