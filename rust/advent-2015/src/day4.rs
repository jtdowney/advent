use aoc_runner_derive::aoc;
use md5::{Digest, Md5};

#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
    (1..)
        .find(|n| {
            let data = format!("{input}{n}");
            let digest = Md5::digest(data);
            let hash = hex::encode(digest);
            hash.starts_with("00000")
        })
        .unwrap()
}

#[aoc(day4, part2)]
fn part2(input: &str) -> usize {
    (1..)
        .find(|n| {
            let data = format!("{input}{n}");
            let digest = Md5::digest(data);
            let hash = hex::encode(digest);
            hash.starts_with("000000")
        })
        .unwrap()
}
