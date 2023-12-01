use aoc_runner_derive::aoc;

const NUMBER_STRINGS: [&str; 18] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
    "5", "6", "7", "8", "9",
];

fn is_digits(number: &str) -> bool {
    number.chars().all(|c| c.is_ascii_digit())
}

fn number_string_to_value(number: &str) -> Option<u32> {
    match number {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        s if is_digits(s) => s.parse().ok(),
        _ => None,
    }
}

fn find_number(line: &str, reverse: bool) -> Option<u32> {
    let matches = NUMBER_STRINGS.iter().filter_map(|p| {
        if reverse {
            line.rmatch_indices(p).next()
        } else {
            line.match_indices(p).next()
        }
    });

    let best_match = if reverse {
        matches.max_by_key(|&(i, _)| i)
    } else {
        matches.min_by_key(|&(i, _)| i)
    };

    best_match.and_then(|(_, number)| number_string_to_value(number))
}

#[aoc(day1, part1)]
fn part1(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            let bstr = line.as_bytes();
            let tens = *bstr.iter().find(|c| c.is_ascii_digit())?;
            let ones = *bstr.iter().rfind(|c| c.is_ascii_digit())?;

            Some(((tens - b'0') * 10 + (ones - b'0')) as u32)
        })
        .sum()
}

#[aoc(day1, part2)]
fn part2(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            let tens = find_number(line, false)?;
            let ones = find_number(line, true)?;
            Some(tens * 10 + ones)
        })
        .sum()
}
