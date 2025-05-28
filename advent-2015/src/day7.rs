use std::{collections::HashMap, str::FromStr};

use anyhow::bail;
use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, space1, u16},
};

fn variable(input: &str) -> IResult<&str, Expression> {
    let (input, name) = alpha1.parse(input)?;
    Ok((input, Expression::Variable(name.to_string())))
}

fn literal(input: &str) -> IResult<&str, Expression> {
    let (input, value) = u16.parse(input)?;
    Ok((input, Expression::Literal(value)))
}

fn term(input: &str) -> IResult<&str, Expression> {
    alt((literal, variable)).parse(input)
}

fn and(input: &str) -> IResult<&str, Expression> {
    let (input, lhs) = term(input)?;
    let (input, _) = space1.parse(input)?;
    let (input, _) = tag("AND").parse(input)?;
    let (input, _) = space1.parse(input)?;
    let (input, rhs) = term(input)?;
    Ok((input, Expression::And(Box::new(lhs), Box::new(rhs))))
}

fn or(input: &str) -> IResult<&str, Expression> {
    let (input, lhs) = term(input)?;
    let (input, _) = space1.parse(input)?;
    let (input, _) = tag("OR").parse(input)?;
    let (input, _) = space1.parse(input)?;
    let (input, rhs) = term(input)?;
    Ok((input, Expression::Or(Box::new(lhs), Box::new(rhs))))
}

fn not(input: &str) -> IResult<&str, Expression> {
    let (input, _) = tag("NOT").parse(input)?;
    let (input, _) = space1.parse(input)?;
    let (input, term) = term(input)?;
    Ok((input, Expression::Not(Box::new(term))))
}

fn left_shift(input: &str) -> IResult<&str, Expression> {
    let (input, lhs) = term(input)?;
    let (input, _) = space1.parse(input)?;
    let (input, _) = tag("LSHIFT").parse(input)?;
    let (input, _) = space1.parse(input)?;
    let (input, rhs) = literal(input)?;
    Ok((input, Expression::LeftShift(Box::new(lhs), Box::new(rhs))))
}

fn right_shift(input: &str) -> IResult<&str, Expression> {
    let (input, lhs) = term(input)?;
    let (input, _) = space1.parse(input)?;
    let (input, _) = tag("RSHIFT").parse(input)?;
    let (input, _) = space1.parse(input)?;
    let (input, rhs) = literal(input)?;
    Ok((input, Expression::RightShift(Box::new(lhs), Box::new(rhs))))
}

fn expression(input: &str) -> IResult<&str, Expression> {
    alt((and, or, not, left_shift, right_shift, term)).parse(input)
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, expression) = expression(input)?;
    let (input, _) = space1.parse(input)?;
    let (input, _) = tag("->").parse(input)?;
    let (input, _) = space1.parse(input)?;
    let (input, destination) = alpha1.parse(input)?;
    let destination = destination.to_string();
    Ok((
        input,
        Instruction {
            expression,
            destination,
        },
    ))
}

#[derive(Clone, Debug)]
enum Expression {
    Literal(u16),
    Variable(String),
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    LeftShift(Box<Expression>, Box<Expression>),
    RightShift(Box<Expression>, Box<Expression>),
    Not(Box<Expression>),
}

#[derive(Debug)]
struct Instruction {
    expression: Expression,
    destination: String,
}

type Environment = HashMap<String, Expression>;

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match instruction(s) {
            Ok(("", instruction)) => Ok(instruction),
            Ok((input, _)) => bail!("extra input at end: {input}"),
            Err(e) => bail!("failed to parse \"{s}\": {e}"),
        }
    }
}

fn execute(
    expression: &Expression,
    environment: &Environment,
    cache: &mut HashMap<String, u16>,
) -> u16 {
    match expression {
        Expression::Literal(value) => *value,
        Expression::Variable(name) => match cache.get(name) {
            Some(v) => *v,
            None => {
                let value = execute(&environment[name], environment, cache);
                cache.insert(name.to_string(), value);
                value
            }
        },
        Expression::And(lhs, rhs) => {
            let lhs = execute(lhs, environment, cache);
            let rhs = execute(rhs, environment, cache);
            lhs & rhs
        }
        Expression::Or(lhs, rhs) => {
            let lhs = execute(lhs, environment, cache);
            let rhs = execute(rhs, environment, cache);
            lhs | rhs
        }
        Expression::LeftShift(lhs, rhs) => {
            let lhs = execute(lhs, environment, cache);
            let rhs = execute(rhs, environment, cache);
            lhs << rhs
        }
        Expression::RightShift(lhs, rhs) => {
            let lhs = execute(lhs, environment, cache);
            let rhs = execute(rhs, environment, cache);
            lhs >> rhs
        }
        Expression::Not(expr) => {
            let value = execute(expr, environment, cache);
            !value
        }
    }
}

#[aoc_generator(day7)]
fn generator(input: &str) -> anyhow::Result<Environment> {
    input
        .lines()
        .map(str::parse::<Instruction>)
        .map(|instruction| instruction.map(|i| (i.destination, i.expression)))
        .collect()
}

#[aoc(day7, part1)]
fn part1(environment: &Environment) -> u16 {
    let mut cache = HashMap::new();
    execute(&environment["a"], environment, &mut cache)
}

#[aoc(day7, part2)]
fn part2(environment: &Environment) -> u16 {
    let mut cache = HashMap::new();
    let a = execute(&environment["a"], environment, &mut cache);

    let mut cache = HashMap::new();
    cache.insert("b".to_string(), a);
    execute(&environment["a"], environment, &mut cache)
}
