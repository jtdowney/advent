use std::sync::LazyLock;

use pest::{
    iterators::Pairs,
    pratt_parser::{Op, PrattParser},
    Parser,
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

fn part1_evaluate(expression: Pairs<Rule>) -> i64 {
    let (value, _) = expression.fold((0, None), |(acc, mut op), token| match token.as_rule() {
        Rule::Number => {
            let value = token.as_str().parse().unwrap();
            let value = if let Some(o) = op.take() {
                calculate(o, acc, value)
            } else {
                value
            };

            (value, op)
        }
        Rule::Expression => {
            let value = part1_evaluate(token.clone().into_inner());
            let value = if let Some(o) = op.take() {
                calculate(o, acc, value)
            } else {
                value
            };

            (value, op)
        }
        Rule::Add => (acc, Some(Rule::Add)),
        Rule::Multiply => (acc, Some(Rule::Multiply)),
        Rule::EOI => (acc, op),
        _ => unreachable!(),
    });

    value
}

fn part2_evaluate(expression: Pairs<Rule>) -> i64 {
    PRATT_PARSER
        .map_primary(|pair| match pair.as_rule() {
            Rule::Number => pair.as_str().parse().unwrap(),
            Rule::Expression => part2_evaluate(pair.into_inner()),
            _ => unreachable!(),
        })
        .map_infix(|lhs, op, rhs| calculate(op.as_rule(), lhs, rhs))
        .parse(expression)
}

#[aoc(day18, part1)]
fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let expression = InputParser::parse(Rule::Calculation, line).unwrap();
            part1_evaluate(expression)
        })
        .sum()
}

#[aoc(day18, part2)]
fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let mut expression = InputParser::parse(Rule::Calculation, line).unwrap();
            let expr = expression.next().unwrap();
            part2_evaluate(expr.into_inner())
        })
        .sum()
}
