use std::num::ParseIntError;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day20)]
fn generator(input: &str) -> Result<usize, ParseIntError> {
    input.parse()
}

fn elves_visited(house: usize) -> Vec<usize> {
    let mut elves = vec![];
    let mut i = 1;
    while i * i <= house {
        if house.is_multiple_of(i) {
            elves.push(i);

            if i * i != house {
                elves.push(house / i);
            }
        }

        i += 1;
    }

    elves
}

#[aoc(day20, part1)]
fn part1(input: &usize) -> usize {
    (1..)
        .map(|house| {
            let presents = elves_visited(house)
                .iter()
                .map(|elf| elf * 10)
                .sum::<usize>();
            (house, presents)
        })
        .find(|&(_, presents)| presents >= *input)
        .map(|(house, _)| house)
        .unwrap()
}

#[aoc(day20, part2)]
fn part2(input: &usize) -> usize {
    (1..)
        .map(|house| {
            let presents = elves_visited(house)
                .iter()
                .filter(|&elf| house / elf <= 50)
                .map(|elf| elf * 11)
                .sum::<usize>();
            (house, presents)
        })
        .find(|&(_, presents)| presents >= *input)
        .map(|(house, _)| house)
        .unwrap()
}
