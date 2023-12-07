use std::{cmp::Ordering, collections::HashSet, str::FromStr};

use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jack,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            c => panic!("Invalid card value {c}"),
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        (*self as isize).cmp(&(*other as isize)).reverse()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        (*self as isize).cmp(&(*other as isize)).reverse()
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Hand {
    cards: (Card, Card, Card, Card, Card),
    jokers: bool,
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let Hand {
            cards: (a, b, c, d, e),
            ..
        } = *self;

        let mut variants = HashSet::new();
        if self.jokers && [a, b, c, d, e].iter().any(|&c| c == Card::Jack) {
            let stable_cards = [a, b, c, d, e]
                .into_iter()
                .filter(|&c| c != Card::Jack)
                .collect_vec();
            if stable_cards.is_empty() {
                variants.insert([Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ace]);
            } else {
                let need = 5 - stable_cards.len();
                let candidates = stable_cards
                    .iter()
                    .copied()
                    .cycle()
                    .take(need * stable_cards.len())
                    .collect_vec();
                let wildcards = candidates.iter().copied().permutations(need);
                variants.extend(wildcards.map(|wildcard| {
                    let mut next = stable_cards.clone();
                    next.extend(wildcard);

                    let mut next_cards: [Card; 5] = next.try_into().expect("5 cards");
                    next_cards.sort_unstable();
                    next_cards
                }));
            }
        } else {
            let mut hand = [a, b, c, d, e];
            hand.sort_unstable();
            variants.insert(hand);
        }

        variants
            .into_iter()
            .map(|cards| match cards {
                [a, b, c, d, e] if a == b && b == c && c == d && d == e => HandType::FiveOfKind,
                [a, b, c, d, _] if a == b && b == c && c == d => HandType::FourOfKind,
                [_, b, c, d, e] if b == c && c == d && d == e => HandType::FourOfKind,
                [a, b, c, d, e] if a == b && b == c && d == e => HandType::FullHouse,
                [a, b, c, d, e] if a == b && c == d && d == e => HandType::FullHouse,
                [a, b, c, _, _] if a == b && b == c => HandType::ThreeOfKind,
                [_, b, c, d, _] if b == c && c == d => HandType::ThreeOfKind,
                [_, _, c, d, e] if c == d && d == e => HandType::ThreeOfKind,
                [a, b, c, d, _] if a == b && c == d => HandType::TwoPair,
                [_, b, c, d, e] if b == c && d == e => HandType::TwoPair,
                [a, b, _, d, e] if a == b && d == e => HandType::TwoPair,
                [a, b, _, _, _] if a == b => HandType::OnePair,
                [_, b, c, _, _] if b == c => HandType::OnePair,
                [_, _, c, d, _] if c == d => HandType::OnePair,
                [_, _, _, d, e] if d == e => HandType::OnePair,
                _ => HandType::HighCard,
            })
            .max()
            .unwrap()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            Ordering::Equal => {
                let left: [Card; 5] = self.cards.into();
                let right: [Card; 5] = other.cards.into();
                left.iter()
                    .zip(right.iter())
                    .find_map(|(&l, &r)| match (l, r) {
                        (l, r) if self.jokers && l == Card::Jack && r != Card::Jack => {
                            Some(Ordering::Less)
                        }
                        (l, r) if self.jokers && l != Card::Jack && r == Card::Jack => {
                            Some(Ordering::Greater)
                        }
                        (l, r) => match l.cmp(&r) {
                            Ordering::Equal => None,
                            ord => Some(ord),
                        },
                    })
                    .unwrap_or(Ordering::Equal)
            }
            o => o,
        }
    }
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s
            .chars()
            .map(Card::from)
            .collect_tuple()
            .context("parsing hand")?;
        Ok(Self {
            cards,
            jokers: false,
        })
    }
}

#[aoc_generator(day7)]
fn generator(input: &str) -> anyhow::Result<Vec<(Hand, u32)>> {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').context("splitting line")?;
            Ok((hand.parse()?, bid.parse()?))
        })
        .collect()
}

#[aoc(day7, part1)]
fn part1(input: &[(Hand, u32)]) -> u32 {
    let mut hands = input.to_vec();
    hands.sort_by_key(|(hand, _)| *hand);
    hands
        .into_iter()
        .zip(1..)
        .map(|((_, bid), rank)| rank * bid)
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &[(Hand, u32)]) -> u32 {
    let mut hands = input.to_vec();
    for (hand, _) in hands.iter_mut() {
        hand.jokers = true;
    }

    hands.sort_by_key(|(hand, _)| *hand);
    hands
        .into_iter()
        .zip(1..)
        .map(|((_, bid), rank)| rank * bid)
        .sum()
}
