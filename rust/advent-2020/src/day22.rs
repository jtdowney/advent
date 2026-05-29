use std::{
    collections::{HashSet, VecDeque},
    iter,
};

use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Game {
    player1: VecDeque<usize>,
    player2: VecDeque<usize>,
}

impl Game {
    fn is_finished(&self) -> bool {
        self.player1.is_empty() || self.player2.is_empty()
    }

    fn winner_deck(&self) -> &VecDeque<usize> {
        if self.player1.is_empty() {
            &self.player2
        } else {
            &self.player1
        }
    }

    fn play_round<F>(&self, determine_winner: F) -> Self
    where
        F: FnOnce(usize, usize, &Self) -> bool,
    {
        let mut game = self.clone();
        let card1 = game.player1.pop_front().unwrap();
        let card2 = game.player2.pop_front().unwrap();

        if determine_winner(card1, card2, &game) {
            game.player1.push_back(card1);
            game.player1.push_back(card2);
        } else {
            game.player2.push_back(card2);
            game.player2.push_back(card1);
        }

        game
    }
}

fn calculate_score(deck: &VecDeque<usize>) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, &card)| card * (i + 1))
        .sum()
}

#[aoc_generator(day22)]
fn generator(input: &str) -> Result<Game> {
    let (player1_section, player2_section) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow::anyhow!("Invalid input format"))?;

    let parse_deck = |section: &str| {
        section
            .lines()
            .skip(1)
            .map(|line| line.parse())
            .collect::<Result<VecDeque<_>, _>>()
    };

    Ok(Game {
        player1: parse_deck(player1_section)?,
        player2: parse_deck(player2_section)?,
    })
}

#[aoc(day22, part1)]
fn part1(game: &Game) -> Result<usize> {
    let final_game = iter::successors(Some(game.clone()), |g| {
        (!g.is_finished()).then(|| g.play_round(|card1, card2, _| card1 > card2))
    })
    .last()
    .unwrap();

    Ok(calculate_score(final_game.winner_deck()))
}

fn play_recursive_combat(initial_game: Game) -> (bool, VecDeque<usize>) {
    let mut seen_states = HashSet::new();
    let mut game = initial_game;

    loop {
        if game.player1.is_empty() {
            return (false, game.player2);
        }
        if game.player2.is_empty() {
            return (true, game.player1);
        }
        if !seen_states.insert(game.clone()) {
            return (true, game.player1);
        }

        game = game.play_round(|card1, card2, current_game| {
            if current_game.player1.len() >= card1 && current_game.player2.len() >= card2 {
                let sub_deck1 = current_game.player1.iter().take(card1).cloned().collect();
                let sub_deck2 = current_game.player2.iter().take(card2).cloned().collect();
                let (player1_wins, _) = play_recursive_combat(Game {
                    player1: sub_deck1,
                    player2: sub_deck2,
                });
                player1_wins
            } else {
                card1 > card2
            }
        });
    }
}

#[aoc(day22, part2)]
fn part2(game: &Game) -> Result<usize> {
    let (_, winner_deck) = play_recursive_combat(game.clone());
    Ok(calculate_score(&winner_deck))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

    #[test]
    fn test_part1() {
        let game = generator(EXAMPLE).unwrap();
        assert_eq!(part1(&game).unwrap(), 306);
    }

    #[test]
    fn test_part2() {
        let game = generator(EXAMPLE).unwrap();
        assert_eq!(part2(&game).unwrap(), 291);
    }
}
