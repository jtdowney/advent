use std::{cmp::max, str::FromStr};

use anyhow::anyhow;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Default, Debug)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    draws: Vec<Draw>,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use nom::{
            branch::alt,
            bytes::complete::tag,
            character::complete::u32,
            combinator::map,
            error::Error,
            multi::separated_list1,
            sequence::{pair, preceded, terminated},
            Finish,
        };

        map(
            pair(
                terminated(preceded(tag("Game "), u32::<_, Error<_>>), tag(": ")),
                separated_list1(
                    tag("; "),
                    map(
                        separated_list1(
                            tag(", "),
                            pair(
                                terminated(u32, tag(" ")),
                                alt((tag("red"), tag("green"), tag("blue"))),
                            ),
                        ),
                        |colors| {
                            colors
                                .iter()
                                .fold(Draw::default(), |mut draw, &(count, color)| {
                                    match color {
                                        "red" => draw.red += count,
                                        "green" => draw.green += count,
                                        "blue" => draw.blue += count,
                                        _ => unreachable!(),
                                    }
                                    draw
                                })
                        },
                    ),
                ),
            ),
            |(id, draws)| Game { id, draws },
        )(input)
        .finish()
        .map(|(_, game)| game)
        .map_err(|e| anyhow!("error parsing game: {:?}", e))
    }
}

#[aoc_generator(day2)]
fn generator(input: &str) -> anyhow::Result<Vec<Game>> {
    input.lines().map(|line| line.parse::<Game>()).collect()
}

#[aoc(day2, part1)]
fn part1(input: &[Game]) -> u32 {
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;

    input
        .iter()
        .filter(|game| {
            game.draws
                .iter()
                .all(|draw| draw.red <= MAX_RED && draw.green <= MAX_GREEN && draw.blue <= MAX_BLUE)
        })
        .map(|game| game.id)
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &[Game]) -> u32 {
    input
        .iter()
        .map(|game| {
            let max = game.draws.iter().fold(Draw::default(), |mut acc, draw| {
                acc.red = max(acc.red, draw.red);
                acc.green = max(acc.green, draw.green);
                acc.blue = max(acc.blue, draw.blue);
                acc
            });

            max.red * max.green * max.blue
        })
        .sum()
}
