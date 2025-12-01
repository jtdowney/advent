use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy, Debug, PartialEq)]
struct Dial(i16);

impl Default for Dial {
    fn default() -> Self {
        Self(50)
    }
}

impl Dial {
    fn rotate_left(self, distance: i16) -> Self {
        let Self(current) = self;
        Self((current - distance).rem_euclid(100))
    }

    fn rotate_right(self, distance: i16) -> Self {
        let Self(current) = self;
        Self((current + distance).rem_euclid(100))
    }

    fn zeros_crossed_left(self, distance: i16) -> i16 {
        let Self(current) = self;
        if current == 0 {
            distance / 100
        } else if distance >= current {
            (distance - current) / 100 + 1
        } else {
            0
        }
    }

    fn zeros_crossed_right(self, distance: i16) -> i16 {
        let Self(current) = self;
        (current + distance) / 100
    }
}

#[aoc_generator(day1)]
fn generator(input: &str) -> anyhow::Result<Vec<(char, i16)>> {
    input
        .lines()
        .map(|line| {
            let (rotation_str, distance_str) = line.split_at(1);
            let rotation = rotation_str.chars().next().context("parsing rotation")?;
            let distance = distance_str.parse()?;
            anyhow::Ok((rotation, distance))
        })
        .collect()
}

fn rotate(dial: Dial, rotation: char, distance: i16) -> Dial {
    match rotation {
        'L' => dial.rotate_left(distance),
        'R' => dial.rotate_right(distance),
        _ => unreachable!(),
    }
}

#[aoc(day1, part1)]
fn part1(input: &[(char, i16)]) -> usize {
    input
        .iter()
        .scan(Dial::default(), |dial, &(rotation, distance)| {
            *dial = rotate(*dial, rotation, distance);
            Some(*dial)
        })
        .filter(|&dial| dial == Dial(0))
        .count()
}

#[aoc(day1, part2)]
fn part2(input: &[(char, i16)]) -> i16 {
    input
        .iter()
        .scan(Dial::default(), |dial, &(rotation, distance)| {
            let zeros = match rotation {
                'L' => dial.zeros_crossed_left(distance),
                'R' => dial.zeros_crossed_right(distance),
                _ => unreachable!(),
            };
            *dial = rotate(*dial, rotation, distance);
            Some(zeros)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dial_starts_at_50() {
        assert_eq!(Dial::default(), Dial(50));
    }

    #[test]
    fn test_rotate_left() {
        let dial = Dial(1);
        assert_eq!(dial.rotate_left(1), Dial(0));
    }

    #[test]
    fn test_rotate_right() {
        let dial = Dial(0);
        assert_eq!(dial.rotate_right(1), Dial(1));
    }

    #[test]
    fn test_rotate_left_from_0() {
        let dial = Dial(0);
        assert_eq!(dial.rotate_left(1), Dial(99));
    }

    #[test]
    fn test_rotate_right_from_99() {
        let dial = Dial(99);
        assert_eq!(dial.rotate_right(1), Dial(0));
    }

    #[test]
    fn test_rotate_keeps_going() {
        let dial = Dial(95);
        assert_eq!(dial.rotate_right(60), Dial(55));
    }
}
