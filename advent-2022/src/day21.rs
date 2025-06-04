use std::collections::HashMap;

use anyhow::{Result, bail};
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
enum Job {
    Number(i64),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

#[aoc_generator(day21)]
fn generator(input: &str) -> anyhow::Result<HashMap<String, Job>> {
    input
        .lines()
        .map(|line| {
            let (name, job_str) = line.split_once(": ").unwrap();
            let job = job_str.parse::<i64>().map(Job::Number).unwrap_or_else(|_| {
                let mut parts = job_str.split_whitespace();
                let left = parts.next().unwrap().to_string();
                let op = parts.next().unwrap();
                let right = parts.next().unwrap().to_string();

                match op {
                    "+" => Job::Add(left, right),
                    "-" => Job::Sub(left, right),
                    "*" => Job::Mul(left, right),
                    "/" => Job::Div(left, right),
                    _ => unreachable!(),
                }
            });
            Ok((name.to_string(), job))
        })
        .collect()
}

fn evaluate(monkeys: &HashMap<String, Job>, name: &str) -> i64 {
    let eval = |n: &str| evaluate(monkeys, n);

    match &monkeys[name] {
        Job::Number(n) => *n,
        Job::Add(a, b) => eval(a) + eval(b),
        Job::Sub(a, b) => eval(a) - eval(b),
        Job::Mul(a, b) => eval(a) * eval(b),
        Job::Div(a, b) => eval(a) / eval(b),
    }
}

#[aoc(day21, part1)]
fn part1(monkeys: &HashMap<String, Job>) -> i64 {
    evaluate(monkeys, "root")
}

#[derive(Debug, Clone, PartialEq)]
enum Expr {
    Num(i64),
    Human,
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

fn build_expr(monkeys: &HashMap<String, Job>, name: &str) -> Expr {
    if name == "humn" {
        Expr::Human
    } else {
        match &monkeys[name] {
            Job::Number(n) => Expr::Num(*n),
            Job::Add(a, b) => Expr::Add(
                Box::new(build_expr(monkeys, a)),
                Box::new(build_expr(monkeys, b)),
            ),
            Job::Sub(a, b) => Expr::Sub(
                Box::new(build_expr(monkeys, a)),
                Box::new(build_expr(monkeys, b)),
            ),
            Job::Mul(a, b) => Expr::Mul(
                Box::new(build_expr(monkeys, a)),
                Box::new(build_expr(monkeys, b)),
            ),
            Job::Div(a, b) => Expr::Div(
                Box::new(build_expr(monkeys, a)),
                Box::new(build_expr(monkeys, b)),
            ),
        }
    }
}

fn eval_expr(expr: &Expr) -> Option<i64> {
    match expr {
        Expr::Num(n) => Some(*n),
        Expr::Human => None,
        Expr::Add(a, b) => Some(eval_expr(a)? + eval_expr(b)?),
        Expr::Sub(a, b) => Some(eval_expr(a)? - eval_expr(b)?),
        Expr::Mul(a, b) => Some(eval_expr(a)? * eval_expr(b)?),
        Expr::Div(a, b) => Some(eval_expr(a)? / eval_expr(b)?),
    }
}

fn solve_for_human(expr: &Expr, target: i64) -> Result<i64> {
    match expr {
        Expr::Human => Ok(target),
        Expr::Add(a, b) => match (eval_expr(a), eval_expr(b)) {
            (Some(a_val), None) => solve_for_human(b, target - a_val),
            (None, Some(b_val)) => solve_for_human(a, target - b_val),
            _ => bail!("Invalid expression state in Add"),
        },
        Expr::Sub(a, b) => match (eval_expr(a), eval_expr(b)) {
            (Some(a_val), None) => solve_for_human(b, a_val - target),
            (None, Some(b_val)) => solve_for_human(a, target + b_val),
            _ => bail!("Invalid expression state in Sub"),
        },
        Expr::Mul(a, b) => match (eval_expr(a), eval_expr(b)) {
            (Some(a_val), None) => solve_for_human(b, target / a_val),
            (None, Some(b_val)) => solve_for_human(a, target / b_val),
            _ => bail!("Invalid expression state in Mul"),
        },
        Expr::Div(a, b) => match (eval_expr(a), eval_expr(b)) {
            (Some(a_val), None) => solve_for_human(b, a_val / target),
            (None, Some(b_val)) => solve_for_human(a, target * b_val),
            _ => bail!("Invalid expression state in Div"),
        },
        _ => bail!("Cannot solve for non-human expression"),
    }
}

#[aoc(day21, part2)]
fn part2(monkeys: &HashMap<String, Job>) -> Result<i64> {
    let (left, right) = match &monkeys["root"] {
        Job::Add(a, b) | Job::Sub(a, b) | Job::Mul(a, b) | Job::Div(a, b) => (a, b),
        _ => bail!("Root must be an operation"),
    };

    let left_expr = build_expr(monkeys, left);
    let right_expr = build_expr(monkeys, right);

    match (eval_expr(&left_expr), eval_expr(&right_expr)) {
        (None, Some(val)) => solve_for_human(&left_expr, val),
        (Some(val), None) => solve_for_human(&right_expr, val),
        _ => bail!("Invalid root expression state"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";
        let parsed = generator(input).unwrap();
        assert_eq!(part1(&parsed), 152);
    }

    #[test]
    fn test_part2() {
        let input = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";
        let parsed = generator(input).unwrap();
        assert_eq!(part2(&parsed).unwrap(), 301);
    }
}
