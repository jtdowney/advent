use aoc_runner_derive::{aoc, aoc_generator};

fn rating(bank: &str, activate: usize) -> Option<u64> {
    let digits = bank
        .chars()
        .map(|c| c.to_digit(10))
        .collect::<Option<Vec<_>>>()?;
    let len = digits.len();

    if len < activate {
        return None;
    }

    (1..=activate)
        .rev()
        .try_fold((0, 0), |(result, pos), remaining| {
            let end = len - remaining;
            digits[pos..=end]
                .iter()
                .enumerate()
                .max_by(|(i1, d1), (i2, d2)| d1.cmp(d2).then(i2.cmp(i1)))
                .map(|(max_idx, max_digit)| {
                    (result * 10 + u64::from(*max_digit), pos + max_idx + 1)
                })
        })
        .map(|(result, _)| result)
}

#[aoc_generator(day3)]
fn generator(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect()
}

#[aoc(day3, part1)]
fn part1(input: &[String]) -> u64 {
    input.iter().filter_map(|bank| rating(bank, 2)).sum()
}

#[aoc(day3, part2)]
fn part2(input: &[String]) -> u64 {
    input.iter().filter_map(|bank| rating(bank, 12)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ratings_part1() {
        assert_eq!(rating("987654321111111", 2), Some(98));
        assert_eq!(rating("811111111111119", 2), Some(89));
        assert_eq!(rating("234234234234278", 2), Some(78));
        assert_eq!(rating("818181911112111", 2), Some(92));
    }

    #[test]
    fn test_ratings_part2() {
        assert_eq!(rating("987654321111111", 12), Some(987_654_321_111));
        assert_eq!(rating("811111111111119", 12), Some(811_111_111_119));
        assert_eq!(rating("234234234234278", 12), Some(434_234_234_278));
        assert_eq!(rating("818181911112111", 12), Some(888_911_112_111));
    }
}
