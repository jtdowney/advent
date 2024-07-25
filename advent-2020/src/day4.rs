use std::{
    collections::{HashMap, HashSet},
    sync::LazyLock,
};

type Passport = HashMap<String, String>;

static REQUIRED_KEYS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .copied()
        .collect::<HashSet<&'static str>>()
});
static EYE_COLORS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .iter()
        .copied()
        .collect::<HashSet<&'static str>>()
});

fn validate_year(year: &str, minimum: u16, maximum: u16) -> bool {
    year.parse::<u16>()
        .map(|n| n >= minimum && n <= maximum)
        .unwrap_or_default()
}

fn part1_validator(passport: &Passport) -> bool {
    let keys = passport
        .keys()
        .map(String::as_str)
        .collect::<HashSet<&str>>();
    REQUIRED_KEYS.is_subset(&keys)
}

fn part2_validator(passport: &Passport) -> bool {
    REQUIRED_KEYS.iter().all(|&key| {
        passport
            .get(key)
            .map(|value| match key {
                "byr" => validate_year(value, 1920, 2002),
                "iyr" => validate_year(value, 2010, 2020),
                "eyr" => validate_year(value, 2020, 2030),
                "hgt" => {
                    let position = value.bytes().position(|b| !b.is_ascii_digit());
                    if let Some(index) = position {
                        let (amount, unit) = value.split_at(index);
                        let amount = amount.parse::<u16>();
                        match (unit, amount) {
                            ("cm", Ok(v)) => (150..=193).contains(&v),
                            ("in", Ok(v)) => (59..=76).contains(&v),
                            _ => false,
                        }
                    } else {
                        false
                    }
                }
                "hcl" => {
                    let (start, value) = value.split_at(1);
                    if start == "#" && value.len() == 6 {
                        u32::from_str_radix(value, 16).is_ok()
                    } else {
                        false
                    }
                }
                "ecl" => EYE_COLORS.contains(value.as_str()),
                "pid" => {
                    if value.len() == 9 {
                        value.parse::<u32>().is_ok()
                    } else {
                        false
                    }
                }
                _ => unreachable!(),
            })
            .unwrap_or_default()
    })
}

fn solve<F>(passports: &[Passport], validator: F) -> usize
where
    F: Fn(&Passport) -> bool,
{
    passports
        .iter()
        .filter(|passport| validator(passport))
        .count()
}

#[aoc_generator(day4)]
fn generator(input: &str) -> Vec<Passport> {
    input
        .split("\n\n")
        .map(|row| {
            row.split_whitespace()
                .fold(Passport::new(), |mut acc, part| {
                    let mut values = part.split(':');
                    let key = values.next().unwrap().to_string();
                    let value = values.next().unwrap().to_string();
                    acc.insert(key, value);
                    acc
                })
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(passports: &[Passport]) -> usize {
    solve(passports, part1_validator)
}

#[aoc(day4, part2)]
fn part2(passports: &[Passport]) -> usize {
    solve(passports, part2_validator)
}
