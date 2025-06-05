use std::collections::{HashMap, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Position = (usize, usize);

#[derive(Debug, Clone)]
struct RaceTrack {
    grid: Vec<Vec<char>>,
    start: Position,
    end: Position,
    width: usize,
    height: usize,
}

#[aoc_generator(day20)]
fn generator(input: &str) -> RaceTrack {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = grid.len();
    let width = grid[0].len();

    let positions = grid
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(col, &ch)| match ch {
                    'S' => Some(('S', (row, col))),
                    'E' => Some(('E', (row, col))),
                    _ => None,
                })
        })
        .collect::<HashMap<_, _>>();

    let start = positions[&'S'];
    let end = positions[&'E'];

    RaceTrack {
        grid,
        start,
        end,
        width,
        height,
    }
}

impl RaceTrack {
    fn is_valid_position(&self, pos: Position) -> bool {
        let (row, col) = pos;
        row < self.height && col < self.width
    }

    fn is_track(&self, pos: Position) -> bool {
        if !self.is_valid_position(pos) {
            return false;
        }
        let (row, col) = pos;
        matches!(self.grid[row][col], '.' | 'S' | 'E')
    }

    fn get_neighbors(&self, (row, col): Position) -> impl Iterator<Item = Position> + '_ {
        [(0, 1), (1, 0), (0, -1), (-1, 0)].into_iter().filter_map(
            move |(dr, dc): (isize, isize)| {
                let new_row = row.checked_add_signed(dr)?;
                let new_col = col.checked_add_signed(dc)?;
                let new_pos = (new_row, new_col);
                self.is_valid_position(new_pos).then_some(new_pos)
            },
        )
    }

    fn find_path(&self) -> HashMap<Position, usize> {
        let mut distances = HashMap::new();
        let mut queue = VecDeque::new();

        queue.push_back((self.start, 0));
        distances.insert(self.start, 0);

        while let Some((pos, dist)) = queue.pop_front() {
            if pos == self.end {
                break;
            }

            for neighbor in self.get_neighbors(pos) {
                if self.is_track(neighbor) && !distances.contains_key(&neighbor) {
                    distances.insert(neighbor, dist + 1);
                    queue.push_back((neighbor, dist + 1));
                }
            }
        }

        distances
    }

    fn find_cheats(&self, min_savings: usize, max_cheat_duration: usize) -> usize {
        let distances = self.find_path();
        let distances_ref = &distances;

        distances
            .iter()
            .flat_map(|(&start_pos, &start_time)| {
                self.find_cheat_destinations(start_pos, max_cheat_duration)
                    .into_iter()
                    .filter(move |&end_pos| {
                        Self::is_valid_cheat(
                            start_pos,
                            start_time,
                            end_pos,
                            distances_ref,
                            min_savings,
                        )
                    })
            })
            .count()
    }

    fn is_valid_cheat(
        start_pos: Position,
        start_time: usize,
        end_pos: Position,
        distances: &HashMap<Position, usize>,
        min_savings: usize,
    ) -> bool {
        distances.get(&end_pos).is_some_and(|&end_time| {
            end_time > start_time && {
                let cheat_distance = manhattan_distance(start_pos, end_pos);
                let normal_path_distance = end_time - start_time;
                normal_path_distance >= cheat_distance + min_savings
            }
        })
    }

    fn find_cheat_destinations(&self, start: Position, max_cheat_duration: usize) -> Vec<Position> {
        let (start_row, start_col) = start;
        let max_dist = isize::try_from(max_cheat_duration).expect("max_cheat_duration too large");

        (-max_dist..=max_dist)
            .cartesian_product(-max_dist..=max_dist)
            .filter(|(dr, dc)| {
                let manhattan_dist = dr.abs() + dc.abs();
                manhattan_dist > 0 && manhattan_dist <= max_dist
            })
            .filter_map(|(dr, dc)| {
                let new_row = start_row.checked_add_signed(dr)?;
                let new_col = start_col.checked_add_signed(dc)?;
                let new_pos = (new_row, new_col);
                self.is_track(new_pos).then_some(new_pos)
            })
            .collect()
    }
}

fn manhattan_distance((a_row, a_col): Position, (b_row, b_col): Position) -> usize {
    a_row.abs_diff(b_row) + a_col.abs_diff(b_col)
}

#[aoc(day20, part1)]
fn part1(track: &RaceTrack) -> usize {
    track.find_cheats(100, 2)
}

#[aoc(day20, part2)]
fn part2(track: &RaceTrack) -> usize {
    track.find_cheats(100, 20)
}

#[cfg(test)]
#[allow(clippy::similar_names)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn test_generator() {
        let track = generator(EXAMPLE_INPUT);
        assert_eq!(track.start, (3, 1));
        assert_eq!(track.end, (7, 5));
        assert_eq!(track.height, 15);
        assert_eq!(track.width, 15);
    }

    #[test]
    fn test_find_path() {
        let track = generator(EXAMPLE_INPUT);
        let distances = track.find_path();

        assert_eq!(distances[&track.end], 84);
        assert_eq!(distances[&track.start], 0);
    }

    #[test]
    fn test_manhattan_distance() {
        assert_eq!(manhattan_distance((0, 0), (2, 1)), 3);
        assert_eq!(manhattan_distance((3, 1), (3, 3)), 2);
        assert_eq!(manhattan_distance((5, 5), (5, 5)), 0);
    }

    #[test]
    fn test_find_cheats_small_savings() {
        let track = generator(EXAMPLE_INPUT);

        let cheats_2_or_more = track.find_cheats(2, 2);
        let cheats_4_or_more = track.find_cheats(4, 2);
        let cheats_6_or_more = track.find_cheats(6, 2);

        assert!(cheats_2_or_more > cheats_4_or_more);
        assert!(cheats_4_or_more > cheats_6_or_more);
    }

    #[test]
    fn test_part1_with_low_threshold() {
        let track = generator(EXAMPLE_INPUT);

        let cheats_saving_at_least_12 = track.find_cheats(12, 2);

        assert!(cheats_saving_at_least_12 > 0);
    }

    #[test]
    fn test_specific_cheat_counts() {
        let track = generator(EXAMPLE_INPUT);

        let cheats_2 = track.find_cheats(2, 2);
        let cheats_4 = track.find_cheats(4, 2);
        let cheats_6 = track.find_cheats(6, 2);
        let cheats_8 = track.find_cheats(8, 2);
        let cheats_10 = track.find_cheats(10, 2);
        let cheats_12 = track.find_cheats(12, 2);
        let cheats_20_savings = track.find_cheats(20, 2);
        let cheats_36 = track.find_cheats(36, 2);
        let cheats_38 = track.find_cheats(38, 2);
        let cheats_40 = track.find_cheats(40, 2);
        let cheats_64 = track.find_cheats(64, 2);

        assert_eq!(cheats_2 - cheats_4, 14);
        assert_eq!(cheats_4 - cheats_6, 14);
        assert_eq!(cheats_6 - cheats_8, 2);
        assert_eq!(cheats_8 - cheats_10, 4);
        assert_eq!(cheats_10 - cheats_12, 2);
        assert_eq!(cheats_12 - cheats_20_savings, 3);
        assert_eq!(cheats_20_savings - cheats_36, 1);
        assert_eq!(cheats_36 - cheats_38, 1);
        assert_eq!(cheats_38 - cheats_40, 1);
        assert_eq!(cheats_40 - cheats_64, 1);
        assert_eq!(cheats_64, 1);
    }

    #[test]
    fn test_part2_example_counts() {
        let track = generator(EXAMPLE_INPUT);

        let cheats_gte_50 = track.find_cheats(50, 20);
        let cheats_gte_52 = track.find_cheats(52, 20);
        let cheats_gte_54 = track.find_cheats(54, 20);
        let cheats_gte_56 = track.find_cheats(56, 20);
        let cheats_gte_58 = track.find_cheats(58, 20);
        let cheats_gte_60 = track.find_cheats(60, 20);
        let cheats_gte_62 = track.find_cheats(62, 20);
        let cheats_gte_64 = track.find_cheats(64, 20);
        let cheats_gte_66 = track.find_cheats(66, 20);
        let cheats_gte_68 = track.find_cheats(68, 20);
        let cheats_gte_70 = track.find_cheats(70, 20);
        let cheats_gte_72 = track.find_cheats(72, 20);
        let cheats_gte_74 = track.find_cheats(74, 20);
        let cheats_gte_76 = track.find_cheats(76, 20);
        let cheats_gte_78 = track.find_cheats(78, 20);

        assert_eq!(cheats_gte_50 - cheats_gte_52, 32);
        assert_eq!(cheats_gte_52 - cheats_gte_54, 31);
        assert_eq!(cheats_gte_54 - cheats_gte_56, 29);
        assert_eq!(cheats_gte_56 - cheats_gte_58, 39);
        assert_eq!(cheats_gte_58 - cheats_gte_60, 25);
        assert_eq!(cheats_gte_60 - cheats_gte_62, 23);
        assert_eq!(cheats_gte_62 - cheats_gte_64, 20);
        assert_eq!(cheats_gte_64 - cheats_gte_66, 19);
        assert_eq!(cheats_gte_66 - cheats_gte_68, 12);
        assert_eq!(cheats_gte_68 - cheats_gte_70, 14);
        assert_eq!(cheats_gte_70 - cheats_gte_72, 12);
        assert_eq!(cheats_gte_72 - cheats_gte_74, 22);
        assert_eq!(cheats_gte_74 - cheats_gte_76, 4);
        assert_eq!(cheats_gte_76 - cheats_gte_78, 3);
    }
}
