use std::collections::HashMap;

use anyhow::bail;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::iproduct;
use nom::{
    Finish, IResult, Parser,
    bytes::complete::tag,
    character::complete::{alpha1, i16},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
};

type Ingrediants = HashMap<String, HashMap<String, isize>>;

fn properties(input: &str) -> IResult<&str, HashMap<String, isize>> {
    let (input, list) = separated_list1(
        tag(", "),
        separated_pair(map(alpha1, str::to_string), tag(" "), map(i16, isize::from)),
    )
    .parse(input)?;

    Ok((input, list.into_iter().collect()))
}

fn ingrediant(input: &str) -> IResult<&str, (String, HashMap<String, isize>)> {
    let (input, name) = map(alpha1, str::to_string).parse(input)?;
    let (input, _) = tag(": ").parse(input)?;
    let (input, properties) = properties(input)?;
    Ok((input, (name, properties)))
}

#[aoc_generator(day15)]
fn generator(input: &str) -> anyhow::Result<Ingrediants> {
    input.lines().try_fold(Ingrediants::new(), |mut acc, line| {
        match ingrediant(line).finish() {
            Ok((_, (key, value))) => acc.insert(key, value),
            Err(e) => bail!("failed to parse {line}: {e}"),
        };

        Ok(acc)
    })
}

#[aoc(day15, part1)]
fn part1(input: &Ingrediants) -> isize {
    iproduct!(0..=100, 0..=100, 0..=100, 0..=100)
        .filter(|(a, b, c, d)| a + b + c + d == 100)
        .map(|(sprinkles, butterscotch, chocolate, candy)| {
            let mut lookup = HashMap::new();
            lookup.insert("Sprinkles", sprinkles);
            lookup.insert("Butterscotch", butterscotch);
            lookup.insert("Chocolate", chocolate);
            lookup.insert("Candy", candy);

            ["capacity", "durability", "flavor", "texture"]
                .iter()
                .map(|prop| {
                    lookup
                        .iter()
                        .map(|(name, value)| input[*name][*prop] * value)
                        .sum::<isize>()
                })
                .map(|prop| if prop.is_negative() { 0 } else { prop })
                .product::<isize>()
        })
        .max()
        .unwrap()
}

#[aoc(day15, part2)]
fn part2(input: &Ingrediants) -> isize {
    iproduct!(0..=100, 0..=100, 0..=100, 0..=100)
        .filter(|(a, b, c, d)| a + b + c + d == 100)
        .filter_map(|(sprinkles, butterscotch, chocolate, candy)| {
            let mut lookup = HashMap::new();
            lookup.insert("Sprinkles", sprinkles);
            lookup.insert("Butterscotch", butterscotch);
            lookup.insert("Chocolate", chocolate);
            lookup.insert("Candy", candy);

            let calories = lookup
                .iter()
                .map(|(name, value)| input[*name]["calories"] * value)
                .sum::<isize>();
            if calories != 500 {
                return None;
            }

            let value = ["capacity", "durability", "flavor", "texture"]
                .iter()
                .map(|prop| {
                    lookup
                        .iter()
                        .map(|(name, value)| input[*name][*prop] * value)
                        .sum::<isize>()
                })
                .map(|prop| if prop.is_negative() { 0 } else { prop })
                .product::<isize>();
            Some(value)
        })
        .max()
        .unwrap()
}
