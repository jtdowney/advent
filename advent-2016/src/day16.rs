use aoc_runner_derive::aoc;

fn expand(a: &str) -> String {
    let b = a.chars().rev().map(|c| match c {
        '1' => '0',
        '0' => '1',
        _ => unreachable!(),
    });

    let mut result = String::with_capacity(a.len() * 2 + 1);
    result.push_str(a);
    result.push('0');
    result.extend(b);
    result
}

fn calculate_checksum(input: &str) -> String {
    input
        .as_bytes()
        .chunks(2)
        .map(|parts| if parts[0] == parts[1] { '1' } else { '0' })
        .collect()
}

fn solve(input: &str, length: usize) -> String {
    let mut data = input.to_string();
    while data.len() < length {
        data = expand(&data);
    }

    data.truncate(length);

    let mut checksum = calculate_checksum(&data);
    while checksum.len() % 2 == 0 {
        checksum = calculate_checksum(&checksum);
    }

    checksum
}

#[aoc(day16, part1)]
fn part1(input: &str) -> String {
    solve(input, 272)
}

#[aoc(day16, part2)]
fn part2(input: &str) -> String {
    const DISK_SIZE: usize = 35_651_584;
    solve(input, DISK_SIZE)
}
