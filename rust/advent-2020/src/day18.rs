use std::sync::LazyLock;

use anyhow::{Context, Result};
use pest::{
    Parser,
    iterators::Pairs,
    pratt_parser::{Op, PrattParser},
};
use pest_derive::Parser;

static PRATT_PARSER: LazyLock<PrattParser<Rule>> = LazyLock::new(|| {
    PrattParser::new()
        .op(Op::infix(Rule::Multiply, pest::pratt_parser::Assoc::Left))
        .op(Op::infix(Rule::Add, pest::pratt_parser::Assoc::Left))
});

#[derive(Parser)]
#[grammar = "day18.pest"]
struct InputParser;

fn calculate(rule: Rule, lhs: i64, rhs: i64) -> i64 {
    match rule {
        Rule::Add => lhs + rhs,
        Rule::Multiply => lhs * rhs,
        _ => unreachable!(),
    }
}

fn part1_evaluate(expression: Pairs<Rule>) -> Result<i64> {
    let mut acc = 0;
    let mut op: Option<Rule> = None;

    for token in expression {
        match token.as_rule() {
            Rule::Number => {
                let value = token
                    .as_str()
                    .parse()
                    .with_context(|| format!("Failed to parse number: '{}'", token.as_str()))?;
                acc = if let Some(o) = op.take() {
                    calculate(o, acc, value)
                } else {
                    value
                };
            }
            Rule::Expression => {
                let value = part1_evaluate(token.clone().into_inner())?;
                acc = if let Some(o) = op.take() {
                    calculate(o, acc, value)
                } else {
                    value
                };
            }
            Rule::Add => op = Some(Rule::Add),
            Rule::Multiply => op = Some(Rule::Multiply),
            Rule::EOI => {}
            _ => unreachable!(),
        }
    }

    Ok(acc)
}

fn part2_evaluate(expression: Pairs<Rule>) -> Result<i64> {
    PRATT_PARSER
        .map_primary(|pair| match pair.as_rule() {
            Rule::Number => pair
                .as_str()
                .parse()
                .with_context(|| format!("Failed to parse number: '{}'", pair.as_str())),
            Rule::Expression => part2_evaluate(pair.into_inner()),
            _ => unreachable!(),
        })
        .map_infix(|lhs, op, rhs| Ok(calculate(op.as_rule(), lhs?, rhs?)))
        .parse(expression)
}

#[aoc(day18, part1)]
fn part1(input: &str) -> Result<i64> {
    input
        .lines()
        .map(|line| {
            let expression = InputParser::parse(Rule::Calculation, line)
                .with_context(|| format!("Failed to parse expression: '{}'", line))?;
            part1_evaluate(expression)
        })
        .sum()
}

#[aoc(day18, part2)]
fn part2(input: &str) -> Result<i64> {
    input
        .lines()
        .map(|line| {
            let mut expression = InputParser::parse(Rule::Calculation, line)
                .with_context(|| format!("Failed to parse expression: '{}'", line))?;
            let expr = expression.next().context("No expression found")?;
            part2_evaluate(expr.into_inner())
        })
        .sum()
}
