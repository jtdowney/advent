use std::{collections::HashMap, sync::LazyLock};

use anyhow::anyhow;
use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    Finish, IResult, Parser,
    bytes::complete::tag,
    character::complete::{alpha1, space1, u16},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
};

static ATTRIBUTES: LazyLock<HashMap<&'static str, usize>> = LazyLock::new(|| {
    let mut attributes = HashMap::new();
    attributes.insert("children", 3);
    attributes.insert("cats", 7);
    attributes.insert("samoyeds", 2);
    attributes.insert("pomeranians", 3);
    attributes.insert("akitas", 0);
    attributes.insert("vizslas", 0);
    attributes.insert("goldfish", 5);
    attributes.insert("trees", 3);
    attributes.insert("cars", 2);
    attributes.insert("perfumes", 1);
    attributes
});

fn properties(input: &str) -> IResult<&str, HashMap<String, usize>> {
    let (input, list) = separated_list1(
        tag(", "),
        separated_pair(
            map(alpha1, str::to_string),
            tag(": "),
            map(u16, usize::from),
        ),
    )
    .parse(input)?;

    Ok((input, list.into_iter().collect()))
}

fn sue(input: &str) -> IResult<&str, (usize, HashMap<String, usize>)> {
    let (input, _) = tag("Sue").parse(input)?;
    let (input, _) = space1.parse(input)?;
    let (input, index) = map(u16, usize::from).parse(input)?;
    let (input, _) = tag(":").parse(input)?;
    let (input, _) = space1.parse(input)?;
    let (input, props) = properties(input)?;

    Ok((input, (index, props)))
}

#[aoc_generator(day16)]
fn generator(input: &str) -> anyhow::Result<HashMap<usize, HashMap<String, usize>>> {
    input
        .lines()
        .map(|line| {
            sue(line)
                .finish()
                .map(|(_, v)| v)
                .map_err(|e| anyhow!("unable to parse {line}: {e}"))
        })
        .collect()
}

#[aoc(day16, part1)]
fn part1(input: &HashMap<usize, HashMap<String, usize>>) -> usize {
    let (&index, _) = input
        .iter()
        .find(|(_, properties)| {
            properties
                .iter()
                .all(|(key, &value)| ATTRIBUTES[key.as_str()] == value)
        })
        .unwrap();

    index
}

#[aoc(day16, part2)]
fn part2(input: &HashMap<usize, HashMap<String, usize>>) -> usize {
    let (&index, _) = input
        .iter()
        .find(|(_, properties)| {
            properties.iter().all(|(key, &value)| match key.as_str() {
                "cats" | "trees" => ATTRIBUTES[key.as_str()] < value,
                "pomeranians" | "goldfish" => ATTRIBUTES[key.as_str()] > value,
                _ => ATTRIBUTES[key.as_str()] == value,
            })
        })
        .unwrap();

    index
}
