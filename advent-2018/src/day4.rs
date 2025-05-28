use std::{collections::HashMap, str::FromStr};

use anyhow::anyhow;
use aoc_runner_derive::{aoc, aoc_generator};
use nom::{Finish, Parser};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Action {
    StartShift(u16),
    FallAsleep,
    WakeUp,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Event {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    action: Action,
}

impl FromStr for Event {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use nom::{
            branch::alt,
            bytes::complete::tag,
            character::complete::{char, space1, u16, u8},
            combinator::map,
            error::Error,
            sequence::{delimited, preceded},
        };

        let mut parser = map(
            (
                delimited(
                    char::<_, Error<_>>('['),
                    (
                        u16,
                        char('-'),
                        u8,
                        char('-'),
                        u8,
                        char(' '),
                        u8,
                        char(':'),
                        u8,
                    ),
                    char(']'),
                ),
                space1,
                alt((
                    map(preceded(tag("Guard #"), u16), Action::StartShift),
                    map(tag("falls asleep"), |_| Action::FallAsleep),
                    map(tag("wakes up"), |_| Action::WakeUp),
                )),
            ),
            |((year, _, month, _, day, _, hour, _, minute), _, action)| Event {
                year,
                month,
                day,
                hour,
                minute,
                action,
            },
        );

        let (_, event) = parser.parse(s)
            .finish()
            .map_err(|e| anyhow!("unable to parse event: {}", e))?;
        Ok(event)
    }
}

type Schedule = HashMap<u16, HashMap<u8, usize>>;

#[aoc_generator(day4)]
fn generator(input: &str) -> anyhow::Result<Schedule> {
    let mut events = input
        .lines()
        .map(|line| line.parse())
        .collect::<anyhow::Result<Vec<Event>>>()?;
    events.sort();

    let (schedule, _, _) = events.iter().fold(
        (HashMap::new(), 0, 0),
        |(mut acc, current_guard, start_sleep), event| match event.action {
            Action::StartShift(guard) => (acc, guard, 0),
            Action::FallAsleep => (acc, current_guard, event.minute),
            Action::WakeUp => {
                for minute in start_sleep..event.minute {
                    *acc.entry(current_guard)
                        .or_insert_with(HashMap::new)
                        .entry(minute)
                        .or_insert(0) += 1;
                }

                (acc, current_guard, 0)
            }
        },
    );

    Ok(schedule)
}

#[aoc(day4, part1)]
fn part1(input: &Schedule) -> Option<u32> {
    input
        .iter()
        .max_by_key(|(_, counts)| counts.values().sum::<usize>())
        .and_then(|(guard, counts)| {
            counts
                .iter()
                .max_by_key(|&(_, count)| count)
                .map(|(minute, _)| (guard, minute))
        })
        .map(|(&guard, &minute)| u32::from(guard) * u32::from(minute))
}

#[aoc(day4, part2)]
fn part2(input: &Schedule) -> Option<u32> {
    input
        .iter()
        .max_by_key(|(_, counts)| counts.values().max())
        .and_then(|(guard, counts)| {
            counts
                .iter()
                .max_by_key(|&(_, count)| count)
                .map(|(minute, _)| (guard, minute))
        })
        .map(|(&guard, &minute)| u32::from(guard) * u32::from(minute))
}
