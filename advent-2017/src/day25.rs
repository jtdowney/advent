use std::collections::HashMap;

use anyhow::anyhow;
use aoc_runner_derive::{aoc, aoc_generator};
use nom::{Finish, IResult};

type Instruction = ((char, bool), Rule);

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Rule {
    write_value: bool,
    next_direction: Direction,
    next_state: char,
}

#[derive(Clone, Debug)]
struct Program {
    initial_state: char,
    rules: HashMap<(char, bool), Rule>,
    diagnostic_steps: usize,
}

#[derive(Clone, Debug, Default)]
struct State {
    tape: HashMap<i32, bool>,
    cursor: i32,
}

fn value(input: &str) -> IResult<&str, bool> {
    use nom::{branch::alt, bytes::complete::tag, combinator::map};
    map(alt((tag("0"), tag("1"))), |s| s == "1")(input)
}

fn rule(input: &str) -> IResult<&str, Rule> {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{anychar, newline, space1},
        combinator::map,
        sequence::{preceded, terminated, tuple},
    };

    let direction = map(alt((tag("left"), tag("right"))), |s| {
        if s == "left" {
            Direction::Left
        } else {
            Direction::Right
        }
    });

    map(
        tuple((
            terminated(
                preceded(tuple((space1, tag("- Write the value "))), value),
                tuple((tag("."), newline)),
            ),
            terminated(
                preceded(tuple((space1, tag("- Move one slot to the "))), direction),
                tuple((tag("."), newline)),
            ),
            terminated(
                preceded(tuple((space1, tag("- Continue with state "))), anychar),
                tag("."),
            ),
        )),
        |(write_value, next_direction, next_state)| Rule {
            write_value,
            next_direction,
            next_state,
        },
    )(input)
}

fn instructions(input: &str) -> IResult<&str, [Instruction; 2]> {
    use nom::{
        bytes::complete::tag,
        character::complete::{anychar, newline, space1},
        combinator::map,
        sequence::{preceded, terminated, tuple},
    };

    map(
        tuple((
            terminated(
                preceded(tag("In state "), anychar),
                tuple((tag(":"), newline)),
            ),
            terminated(
                preceded(tuple((space1, tag("If the current value is "))), value),
                tuple((tag(":"), newline)),
            ),
            terminated(rule, newline),
            terminated(
                preceded(tuple((space1, tag("If the current value is "))), value),
                tuple((tag(":"), newline)),
            ),
            rule,
        )),
        |(state, value1, rule1, value2, rule2)| {
            [((state, value1), rule1), ((state, value2), rule2)]
        },
    )(input)
}

fn program(input: &str) -> IResult<&str, Program> {
    use nom::{
        bytes::complete::tag,
        character::complete::{anychar, newline, u32},
        combinator::map,
        multi::separated_list1,
        sequence::{preceded, terminated, tuple},
    };

    map(
        tuple((
            terminated(
                preceded(tag("Begin in state "), anychar),
                tuple((tag("."), newline)),
            ),
            terminated(
                preceded(
                    tag("Perform a diagnostic checksum after "),
                    map(u32, |n| n as usize),
                ),
                tuple((tag(" steps."), newline)),
            ),
            newline,
            map(
                separated_list1(tuple((newline, newline)), instructions),
                |rules| rules.into_iter().flatten().collect(),
            ),
        )),
        |(initial_state, diagnostic_steps, _, rules)| Program {
            initial_state,
            diagnostic_steps,
            rules,
        },
    )(input)
}

#[aoc_generator(day25)]
fn generator(input: &str) -> anyhow::Result<Program> {
    program(input)
        .finish()
        .map(|(_, program)| program)
        .map_err(|e| anyhow!("unable to parse: {}", e))
}

#[aoc(day25, part1)]
fn part1(input: &Program) -> usize {
    let (state, _) = (0..input.diagnostic_steps).fold(
        (State::default(), input.initial_state),
        |(mut state, current_state), _| {
            let current_value = *state.tape.entry(state.cursor).or_default();
            let rule = input
                .rules
                .get(&(current_state, current_value))
                .unwrap_or_else(|| {
                    panic!(
                        "no rule for state {} and value {}",
                        current_state, current_value
                    )
                });

            state.tape.insert(state.cursor, rule.write_value);
            state.cursor += match rule.next_direction {
                Direction::Left => -1,
                Direction::Right => 1,
            };

            (state, rule.next_state)
        },
    );

    state.tape.values().filter(|&v| *v).count()
}
