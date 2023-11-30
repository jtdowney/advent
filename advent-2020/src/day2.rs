use regex::Regex;

type PasswordRule = (usize, usize, char);
struct Entry {
    rule: PasswordRule,
    password: String,
}

#[aoc_generator(day2)]
fn generator(input: &str) -> Vec<Entry> {
    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    input
        .lines()
        .map(|line| {
            let captures = re.captures(&line).unwrap();

            let a = captures[1].parse().unwrap();
            let b = captures[2].parse().unwrap();
            let letter = captures[3].chars().next().unwrap();
            let password = captures[4].to_string();
            let rule = (a, b, letter);
            Entry { rule, password }
        })
        .collect()
}

fn part1_policy((minimum, maximum, letter): PasswordRule, password: &str) -> bool {
    let count = password.chars().filter(|&c| c == letter).count();
    count >= minimum && count <= maximum
}

fn part2_policy((first, last, letter): PasswordRule, password: &str) -> bool {
    let chars = password.chars().collect::<Vec<char>>();
    let a = chars[first - 1];
    let b = chars[last - 1];

    (a == letter) ^ (b == letter)
}

fn solve<F>(input: &[Entry], policy: F) -> usize
where
    F: Fn(PasswordRule, &str) -> bool,
{
    input
        .iter()
        .filter(|entry| policy(entry.rule, &entry.password))
        .count()
}

#[aoc(day2, part1)]
fn part1(input: &[Entry]) -> usize {
    solve(input, part1_policy)
}

#[aoc(day2, part2)]
fn part2(input: &[Entry]) -> usize {
    solve(input, part2_policy)
}
