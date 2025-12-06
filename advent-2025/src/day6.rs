use anyhow::Context;
use aoc_runner_derive::aoc;

#[aoc(day6, part1)]
fn part1(input: &str) -> u64 {
    let input: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_ascii_whitespace().collect())
        .collect();

    let width = input[0].len();
    (0..width)
        .filter_map(|col| {
            input
                .iter()
                .scan(vec![], |acc, row| match row[col].parse::<u64>() {
                    Ok(value) => {
                        acc.push(value);
                        Some(value)
                    }
                    _ => match row[col] {
                        "+" => Some(acc.iter().sum()),
                        "*" => Some(acc.iter().product()),
                        _ => unreachable!(),
                    },
                })
                .last()
        })
        .sum()
}

fn read_column_value(lines: &[&str], col: usize) -> Option<u64> {
    lines
        .iter()
        .filter_map(|line| {
            line.as_bytes()
                .get(col)
                .filter(|b| b.is_ascii_digit())
                .map(|b| u64::from(b - b'0'))
        })
        .fold(None, |acc, digit| {
            Some(acc.unwrap_or_default() * 10 + digit)
        })
}

#[aoc(day6, part2)]
fn part2(input: &str) -> anyhow::Result<u64> {
    let lines: Vec<&str> = input.lines().collect();
    let operator_line = lines.last().context("reading last line")?;
    let value_lines = &lines[..lines.len() - 1];

    let operators: Vec<(usize, char)> = operator_line
        .char_indices()
        .filter(|(_, c)| !c.is_ascii_whitespace())
        .collect();

    let max_length = lines
        .iter()
        .map(|line| line.len())
        .max()
        .unwrap_or_default();

    operators
        .iter()
        .enumerate()
        .map(|(i, &(start, op))| {
            let end = operators.get(i + 1).map_or(max_length, |(pos, _)| *pos);

            let values = (start..end).filter_map(|col| read_column_value(value_lines, col));
            let result: u64 = match op {
                '+' => values.sum(),
                '*' => values.product(),
                _ => unreachable!(),
            };

            Ok(result)
        })
        .sum()
}
