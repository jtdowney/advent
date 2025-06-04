use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy)]
enum Rock {
    Horizontal,
    Plus,
    ReverseL,
    Vertical,
    Square,
}

impl Rock {
    const SHAPES: [Rock; 5] = [
        Rock::Horizontal,
        Rock::Plus,
        Rock::ReverseL,
        Rock::Vertical,
        Rock::Square,
    ];

    fn get_shape(&self, x: i64, y: i64) -> Vec<(i64, i64)> {
        match self {
            Rock::Horizontal => vec![(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],
            Rock::Plus => vec![
                (x + 1, y),
                (x, y + 1),
                (x + 1, y + 1),
                (x + 2, y + 1),
                (x + 1, y + 2),
            ],
            Rock::ReverseL => vec![
                (x, y),
                (x + 1, y),
                (x + 2, y),
                (x + 2, y + 1),
                (x + 2, y + 2),
            ],
            Rock::Vertical => vec![(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],
            Rock::Square => vec![(x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1)],
        }
    }
}

struct Chamber {
    occupied: HashSet<(i64, i64)>,
    max_height: i64,
    width: i64,
}

impl Chamber {
    fn new() -> Self {
        Chamber {
            occupied: HashSet::new(),
            max_height: 0,
            width: 7,
        }
    }

    fn can_place(&self, positions: &[(i64, i64)]) -> bool {
        positions
            .iter()
            .all(|&(x, y)| x >= 0 && x < self.width && y > 0 && !self.occupied.contains(&(x, y)))
    }

    fn place_rock(&mut self, positions: &[(i64, i64)]) {
        for &(x, y) in positions {
            self.occupied.insert((x, y));
            self.max_height = self.max_height.max(y);
        }
    }
}

#[aoc_generator(day17)]
fn generator(input: &str) -> anyhow::Result<Vec<char>> {
    Ok(input.trim().chars().collect())
}

#[aoc(day17, part1)]
fn part1(jets: &[char]) -> usize {
    simulate_rocks(jets, 2022)
}

#[aoc(day17, part2)]
fn part2(jets: &[char]) -> usize {
    simulate_rocks(jets, 1_000_000_000_000)
}

fn simulate_rocks(jets: &[char], num_rocks: usize) -> usize {
    let mut chamber = Chamber::new();
    let mut jet_index = 0;
    let mut seen: HashMap<(usize, usize, Vec<i64>), (usize, i64)> = HashMap::new();

    for rock_count in 0..num_rocks {
        let rock_index = rock_count % Rock::SHAPES.len();
        let state_key = (
            rock_index,
            jet_index % jets.len(),
            get_top_profile(&chamber),
        );

        if let Some(&(prev_rock_count, prev_height)) = seen.get(&state_key) {
            let cycle_length = rock_count - prev_rock_count;
            let height_per_cycle = chamber.max_height - prev_height;
            let remaining_rocks = num_rocks - rock_count;
            let full_cycles = remaining_rocks / cycle_length;
            let partial_cycle = remaining_rocks % cycle_length;

            (0..partial_cycle).for_each(|i| {
                drop_rock(&mut chamber, jets, &mut jet_index, rock_count + i);
            });

            return (chamber.max_height + full_cycles as i64 * height_per_cycle) as usize;
        }

        seen.insert(state_key, (rock_count, chamber.max_height));
        drop_rock(&mut chamber, jets, &mut jet_index, rock_count);
    }

    chamber.max_height as usize
}

fn get_top_profile(chamber: &Chamber) -> Vec<i64> {
    (0..chamber.width)
        .map(|x| {
            ((chamber.max_height - 100).max(0)..=chamber.max_height)
                .rev()
                .find(|&y| chamber.occupied.contains(&(x, y)))
                .map(|y| chamber.max_height - y)
                .unwrap_or(0)
        })
        .collect()
}

fn drop_rock(chamber: &mut Chamber, jets: &[char], jet_index: &mut usize, rock_count: usize) {
    let rock = Rock::SHAPES[rock_count % Rock::SHAPES.len()];
    let (mut x, mut y) = (2, chamber.max_height + 4);

    loop {
        let jet = jets[*jet_index % jets.len()];
        *jet_index += 1;

        let new_x = x + if jet == '<' { -1 } else { 1 };
        let horizontal_positions = rock.get_shape(new_x, y);
        if chamber.can_place(&horizontal_positions) {
            x = new_x;
        }

        let fall_positions = rock.get_shape(x, y - 1);
        if chamber.can_place(&fall_positions) {
            y -= 1;
        } else {
            chamber.place_rock(&rock.get_shape(x, y));
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_part1() {
        let jets = generator(EXAMPLE).unwrap();
        assert_eq!(part1(&jets), 3068);
    }

    #[test]
    fn test_part2() {
        let jets = generator(EXAMPLE).unwrap();
        assert_eq!(part2(&jets), 1514285714288);
    }
}
