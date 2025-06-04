use std::iter;

use anyhow::{Result, bail, ensure};
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day25)]
fn parse(input: &str) -> Result<Vec<String>> {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| {
            ensure!(
                line.chars()
                    .all(|c| matches!(c, '=' | '-' | '0' | '1' | '2')),
                "invalid SNAFU string: {}",
                line
            );
            Ok(line.to_string())
        })
        .collect()
}

fn snafu_to_decimal(snafu: &str) -> Result<i64> {
    snafu
        .chars()
        .rev()
        .enumerate()
        .map(|(i, ch)| {
            let digit = match ch {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => bail!("invalid SNAFU character: {}", ch),
            };
            Ok(digit * 5_i64.pow(i as u32))
        })
        .sum()
}

fn decimal_to_snafu(decimal: i64) -> String {
    if decimal == 0 {
        return "0".to_string();
    }

    iter::successors(Some(decimal), |&n| {
        let next = (n + 2) / 5;
        (next > 0).then_some(next)
    })
    .map(|n| match n.rem_euclid(5) {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '=',
        4 => '-',
        _ => unreachable!(),
    })
    .collect::<String>()
    .chars()
    .rev()
    .collect()
}

#[aoc(day25, part1)]
fn part1(input: &[String]) -> Result<String> {
    input
        .iter()
        .map(|snafu| snafu_to_decimal(snafu))
        .sum::<anyhow::Result<i64>>()
        .map(decimal_to_snafu)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snafu_to_decimal() {
        assert_eq!(snafu_to_decimal("1=-0-2").unwrap(), 1747);
        assert_eq!(snafu_to_decimal("12111").unwrap(), 906);
        assert_eq!(snafu_to_decimal("2=0=").unwrap(), 198);
        assert_eq!(snafu_to_decimal("21").unwrap(), 11);
        assert_eq!(snafu_to_decimal("2=01").unwrap(), 201);
        assert_eq!(snafu_to_decimal("111").unwrap(), 31);
        assert_eq!(snafu_to_decimal("20012").unwrap(), 1257);
        assert_eq!(snafu_to_decimal("112").unwrap(), 32);
        assert_eq!(snafu_to_decimal("1=-1=").unwrap(), 353);
        assert_eq!(snafu_to_decimal("1-12").unwrap(), 107);
        assert_eq!(snafu_to_decimal("12").unwrap(), 7);
        assert_eq!(snafu_to_decimal("1=").unwrap(), 3);
        assert_eq!(snafu_to_decimal("122").unwrap(), 37);
        assert_eq!(snafu_to_decimal("1").unwrap(), 1);
        assert_eq!(snafu_to_decimal("2").unwrap(), 2);
        assert_eq!(snafu_to_decimal("1=").unwrap(), 3);
        assert_eq!(snafu_to_decimal("1-").unwrap(), 4);
        assert_eq!(snafu_to_decimal("10").unwrap(), 5);
        assert_eq!(snafu_to_decimal("11").unwrap(), 6);
        assert_eq!(snafu_to_decimal("12").unwrap(), 7);
        assert_eq!(snafu_to_decimal("2=").unwrap(), 8);
        assert_eq!(snafu_to_decimal("2-").unwrap(), 9);
        assert_eq!(snafu_to_decimal("20").unwrap(), 10);
        assert_eq!(snafu_to_decimal("1=0").unwrap(), 15);
        assert_eq!(snafu_to_decimal("1-0").unwrap(), 20);
        assert_eq!(snafu_to_decimal("1=11-2").unwrap(), 2022);
        assert_eq!(snafu_to_decimal("1-0---0").unwrap(), 12345);
        assert_eq!(snafu_to_decimal("1121-1110-1=0").unwrap(), 314159265);
    }

    #[test]
    fn test_decimal_to_snafu() {
        assert_eq!(decimal_to_snafu(1), "1");
        assert_eq!(decimal_to_snafu(2), "2");
        assert_eq!(decimal_to_snafu(3), "1=");
        assert_eq!(decimal_to_snafu(4), "1-");
        assert_eq!(decimal_to_snafu(5), "10");
        assert_eq!(decimal_to_snafu(6), "11");
        assert_eq!(decimal_to_snafu(7), "12");
        assert_eq!(decimal_to_snafu(8), "2=");
        assert_eq!(decimal_to_snafu(9), "2-");
        assert_eq!(decimal_to_snafu(10), "20");
        assert_eq!(decimal_to_snafu(15), "1=0");
        assert_eq!(decimal_to_snafu(20), "1-0");
        assert_eq!(decimal_to_snafu(2022), "1=11-2");
        assert_eq!(decimal_to_snafu(12345), "1-0---0");
        assert_eq!(decimal_to_snafu(314159265), "1121-1110-1=0");
        assert_eq!(decimal_to_snafu(1747), "1=-0-2");
        assert_eq!(decimal_to_snafu(906), "12111");
        assert_eq!(decimal_to_snafu(198), "2=0=");
        assert_eq!(decimal_to_snafu(11), "21");
        assert_eq!(decimal_to_snafu(201), "2=01");
        assert_eq!(decimal_to_snafu(31), "111");
        assert_eq!(decimal_to_snafu(1257), "20012");
        assert_eq!(decimal_to_snafu(32), "112");
        assert_eq!(decimal_to_snafu(353), "1=-1=");
        assert_eq!(decimal_to_snafu(107), "1-12");
        assert_eq!(decimal_to_snafu(37), "122");
        assert_eq!(decimal_to_snafu(4890), "2=-1=0");
    }

    #[test]
    fn test_part1_example() {
        let input = vec![
            "1=-0-2".to_string(),
            "12111".to_string(),
            "2=0=".to_string(),
            "21".to_string(),
            "2=01".to_string(),
            "111".to_string(),
            "20012".to_string(),
            "112".to_string(),
            "1=-1=".to_string(),
            "1-12".to_string(),
            "12".to_string(),
            "1=".to_string(),
            "122".to_string(),
        ];

        assert_eq!(part1(&input).unwrap(), "2=-1=0");
    }
}
