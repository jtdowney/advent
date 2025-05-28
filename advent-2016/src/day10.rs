use std::collections::{BTreeSet, HashMap, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{IResult, Parser};

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
struct Microchip(u32);

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Bot(u32);

#[derive(Clone, Copy, Debug)]
enum Destination {
    Bot(Bot),
    Output(u32),
}

#[derive(Clone, Copy, Debug)]
enum Command {
    Pickup(Microchip),
    Transfer(Destination, Destination),
}

#[derive(Clone, Copy, Debug)]
struct Instruction(Bot, Command);

fn bot(input: &str) -> IResult<&str, Bot> {
    use nom::{character::complete::u32, combinator::map};
    map(u32, Bot).parse(input)
}

fn destination(input: &str) -> IResult<&str, Destination> {
    use nom::{
        branch::alt, bytes::complete::tag, character::complete::u32, combinator::map,
        sequence::preceded,
    };

    alt((
        map(preceded(tag("bot "), bot), Destination::Bot),
        map(preceded(tag("output "), u32), Destination::Output),
    ))
    .parse(input)
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    use nom::{branch::alt, bytes::complete::tag, character::complete::u32, combinator::map};

    let mut microchip = map(u32, Microchip);
    let pickup = map(
        |input| {
            let (input, _) = tag("value ")(input)?;
            let (input, m) = microchip.parse(input)?;
            let (input, _) = tag(" goes to bot ")(input)?;
            let (input, b) = bot(input)?;
            Ok((input, (m, b)))
        },
        |(m, b)| Instruction(b, Command::Pickup(m)),
    );
    let transfer = map(
        |input| {
            let (input, _) = tag("bot ")(input)?;
            let (input, b) = bot(input)?;
            let (input, _) = tag(" gives low to ")(input)?;
            let (input, low) = destination(input)?;
            let (input, _) = tag(" and high to ")(input)?;
            let (input, high) = destination(input)?;
            Ok((input, (b, low, high)))
        },
        |(b, low, high)| Instruction(b, Command::Transfer(low, high)),
    );

    alt((pickup, transfer)).parse(input)
}

#[aoc_generator(day10)]
fn generate(input: &str) -> anyhow::Result<Vec<Instruction>> {
    input
        .lines()
        .map(|line| {
            instruction
                .parse(line)
                .map(|(_, i)| i)
                .map_err(|_| anyhow::anyhow!("Error parsing instruction: {}", line))
        })
        .collect()
}

#[derive(Debug, Default)]
struct State {
    bots: HashMap<Bot, BTreeSet<Microchip>>,
    outputs: HashMap<u32, Vec<Microchip>>,
}

fn expand(instructions: &[Instruction], halt_early: bool) -> State {
    let mut state = State::default();
    let mut instructions = instructions.iter().copied().collect::<VecDeque<_>>();

    while let Some(Instruction(bot, command)) = instructions.pop_front() {
        let slots_used = state.bots.entry(bot).or_insert_with(BTreeSet::new).len();
        match command {
            Command::Pickup(microchip) => {
                state.bots.entry(bot).or_default().insert(microchip);
            }
            Command::Transfer(low_dest, high_dest) if slots_used == 2 => {
                let microchips = state.bots.get(&bot).unwrap();
                let low = *microchips.iter().next().unwrap();
                let high = *microchips.iter().next_back().unwrap();

                if halt_early && low == Microchip(17) && high == Microchip(61) {
                    return state;
                }

                state.bots.remove(&bot);

                match low_dest {
                    Destination::Bot(b) => {
                        state.bots.entry(b).or_default().insert(low);
                    }
                    Destination::Output(o) => {
                        state.outputs.entry(o).or_default().push(low);
                    }
                };

                match high_dest {
                    Destination::Bot(b) => {
                        state.bots.entry(b).or_default().insert(high);
                    }
                    Destination::Output(o) => {
                        state.outputs.entry(o).or_default().push(high);
                    }
                };
            }
            _ => {
                instructions.push_back(Instruction(bot, command));
            }
        }
    }

    state
}

#[aoc(day10, part1)]
fn part1(input: &[Instruction]) -> Option<u32> {
    expand(input, true)
        .bots
        .iter()
        .find_map(|(&Bot(id), chips)| {
            if chips.contains(&Microchip(17)) && chips.contains(&Microchip(61)) {
                Some(id)
            } else {
                None
            }
        })
}

#[aoc(day10, part2)]
fn part2(input: &[Instruction]) -> Option<u32> {
    let state = expand(input, false);
    (0..=2)
        .map(|i| {
            state
                .outputs
                .get(&i)
                .and_then(|o| o.first())
                .map(|Microchip(m)| m)
        })
        .product()
}
