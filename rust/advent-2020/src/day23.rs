use std::iter;

use anyhow::Result;

struct CupCircle {
    next: Vec<usize>,
    current: usize,
    max_label: usize,
}

impl CupCircle {
    fn new(initial_cups: &[usize], total_cups: usize) -> Self {
        let mut next = vec![0; total_cups + 1];

        initial_cups.windows(2).for_each(|w| next[w[0]] = w[1]);

        let last_initial = initial_cups[initial_cups.len() - 1];

        if total_cups > initial_cups.len() {
            next[last_initial] = initial_cups.len() + 1;
            (initial_cups.len() + 1..=total_cups).for_each(|i| next[i] = i + 1);
            next[total_cups] = initial_cups[0];
        } else {
            next[last_initial] = initial_cups[0];
        }

        Self {
            next,
            current: initial_cups[0],
            max_label: total_cups,
        }
    }

    fn play_moves(&mut self, moves: usize) {
        (0..moves).for_each(|_| self.play_round());
    }

    fn play_round(&mut self) {
        let pickup1 = self.next[self.current];
        let pickup2 = self.next[pickup1];
        let pickup3 = self.next[pickup2];

        self.next[self.current] = self.next[pickup3];

        let destination = self.find_destination(pickup1, pickup2, pickup3);

        self.next[pickup3] = self.next[destination];
        self.next[destination] = pickup1;

        self.current = self.next[self.current];
    }

    fn find_destination(&self, pickup1: usize, pickup2: usize, pickup3: usize) -> usize {
        let mut destination = if self.current > 1 {
            self.current - 1
        } else {
            self.max_label
        };

        while destination == pickup1 || destination == pickup2 || destination == pickup3 {
            destination = if destination > 1 {
                destination - 1
            } else {
                self.max_label
            };
        }

        destination
    }

    fn cups_after_one(&self) -> Vec<usize> {
        iter::successors(Some(self.next[1]), |&current| {
            if self.next[current] == 1 {
                None
            } else {
                Some(self.next[current])
            }
        })
        .collect()
    }
}

#[aoc_generator(day23)]
fn generator(input: &str) -> Option<Vec<usize>> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).map(|d| d as usize))
        .collect()
}

#[aoc(day23, part1)]
fn part1(initial_cups: &[usize]) -> Result<String> {
    let mut circle = CupCircle::new(initial_cups, initial_cups.len());
    circle.play_moves(100);

    Ok(circle
        .cups_after_one()
        .into_iter()
        .map(|cup| cup.to_string())
        .collect())
}

#[aoc(day23, part2)]
fn part2(initial_cups: &[usize]) -> Result<usize> {
    let mut circle = CupCircle::new(initial_cups, 1_000_000);
    circle.play_moves(10_000_000);

    let cup1 = circle.next[1];
    let cup2 = circle.next[cup1];

    Ok(cup1 * cup2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_10_moves() {
        let input = "389125467";
        let initial_cups = generator(input).unwrap();
        let mut circle = CupCircle::new(&initial_cups, initial_cups.len());
        circle.play_moves(10);

        let result: String = circle
            .cups_after_one()
            .into_iter()
            .map(|cup| cup.to_string())
            .collect();

        assert_eq!(result, "92658374");
    }

    #[test]
    fn test_part1_100_moves() {
        let input = "389125467";
        let cups = generator(input).unwrap();
        assert_eq!(part1(&cups).unwrap(), "67384529");
    }

    #[test]
    fn test_part2() {
        let input = "389125467";
        let cups = generator(input).unwrap();
        assert_eq!(part2(&cups).unwrap(), 149245887792);
    }
}
