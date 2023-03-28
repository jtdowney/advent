use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Clone, Debug)]
struct Room {
    name: String,
    sector_id: u32,
    checksum: String,
}

impl Room {
    fn is_valid(&self) -> bool {
        let mut chars = self.name.chars().filter(|&c| c != '-').collect::<Vec<_>>();
        chars.sort();
        chars.dedup();
        chars.sort_by_key(|&c| (-(self.name.matches(c).count() as i32), c));

        let checksum = chars.into_iter().take(5).collect::<String>();
        checksum == self.checksum
    }

    fn decrypted_name(&self) -> String {
        self.name
            .chars()
            .map(|c| match c {
                '-' => ' ',
                c => {
                    let offset = (c as u8) - 97;
                    let shifted_offset = ((offset as u32 + self.sector_id) % 26) as u8;
                    (shifted_offset + 97) as char
                }
            })
            .collect()
    }
}

#[aoc_generator(day4)]
fn generator(input: &str) -> anyhow::Result<Vec<Room>> {
    let regex = Regex::new(r"^(?P<name>[a-z-]+)-(?P<sector>\d+)\[(?P<checksum>[a-z]{5})\]")?;
    input
        .lines()
        .map(|line| {
            regex
                .captures(line)
                .context("Invalid input")
                .map(|cap| Room {
                    name: cap["name"].to_string(),
                    sector_id: cap["sector"].parse().unwrap(),
                    checksum: cap["checksum"].to_string(),
                })
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(input: &[Room]) -> u32 {
    input
        .iter()
        .filter(|room| room.is_valid())
        .map(|room| room.sector_id)
        .sum()
}

#[aoc(day4, part2)]
fn part2(input: &[Room]) -> Option<u32> {
    input
        .iter()
        .filter(|room| room.is_valid())
        .find(|room| room.decrypted_name() == "northpole object storage")
        .map(|room| room.sector_id)
}
