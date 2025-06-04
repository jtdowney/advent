use std::iter;

use anyhow::Result;

const MODULUS: u64 = 20_201_227;
const SUBJECT_NUMBER: u64 = 7;

fn transform(subject_number: u64, loop_size: usize) -> u64 {
    (0..loop_size).fold(1, |value, _| (value * subject_number) % MODULUS)
}

fn find_loop_size(public_key: u64) -> usize {
    iter::successors(Some(1), |&value| Some((value * SUBJECT_NUMBER) % MODULUS))
        .position(|value| value == public_key)
        .unwrap()
}

#[aoc_generator(day25)]
fn generator(input: &str) -> Result<(u64, u64)> {
    let mut lines = input.lines();
    let card_public_key = lines.next().unwrap().parse()?;
    let door_public_key = lines.next().unwrap().parse()?;
    Ok((card_public_key, door_public_key))
}

#[aoc(day25, part1)]
fn part1(&(card_public_key, door_public_key): &(u64, u64)) -> u64 {
    let card_loop_size = find_loop_size(card_public_key);
    transform(door_public_key, card_loop_size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform() {
        assert_eq!(transform(7, 8), 5764801);
        assert_eq!(transform(7, 11), 17807724);
        assert_eq!(transform(17807724, 8), 14897079);
        assert_eq!(transform(5764801, 11), 14897079);
    }

    #[test]
    fn test_find_loop_size() {
        assert_eq!(find_loop_size(5764801), 8);
        assert_eq!(find_loop_size(17807724), 11);
    }

    #[test]
    fn test_part1_example() {
        let input = (5764801, 17807724);
        assert_eq!(part1(&input), 14897079);
    }
}
