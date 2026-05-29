use aoc_runner_derive::aoc;

const NUMBERS: [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
const NUMBER_STRINGS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn number_string_to_value(number: &str) -> Option<u32> {
    if let Ok(n) = number.parse::<u32>() {
        return Some(n);
    }

    if let Some(index) = NUMBER_STRINGS.iter().position(|&item| item == number) {
        return Some((index + 1) as u32);
    }

    None
}

fn find_number<'a, S>(line: &str, search: S, reverse: bool) -> Option<u32>
where
    S: Iterator<Item = &'a str>,
{
    let matches = search.filter_map(|p| {
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
    let search = NUMBERS.iter().copied();
    input
        .lines()
        .map(|line| {
            let tens = find_number(line, search.clone(), false)?;
            let ones = find_number(line, search.clone(), true)?;
            Some(tens * 10 + ones)
        })
        .sum()
}

#[aoc(day1, part2)]
fn part2(input: &str) -> Option<u32> {
    let search = NUMBERS
        .iter()
        .copied()
        .chain(NUMBER_STRINGS.iter().copied());
    input
        .lines()
        .map(|line| {
            let tens = find_number(line, search.clone(), false)?;
            let ones = find_number(line, search.clone(), true)?;
            Some(tens * 10 + ones)
        })
        .sum()
}
