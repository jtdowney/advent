use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    fn from_char(c: char) -> Option<Self> {
        use Amphipod::*;
        match c {
            'A' => Some(Amber),
            'B' => Some(Bronze),
            'C' => Some(Copper),
            'D' => Some(Desert),
            _ => None,
        }
    }

    fn energy_per_step(&self) -> usize {
        use Amphipod::*;
        match self {
            Amber => 1,
            Bronze => 10,
            Copper => 100,
            Desert => 1000,
        }
    }

    fn target_room(&self) -> usize {
        use Amphipod::*;
        match self {
            Amber => 0,
            Bronze => 1,
            Copper => 2,
            Desert => 3,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    hallway: [Option<Amphipod>; 11],
    rooms: Vec<Vec<Option<Amphipod>>>,
}

const VALID_HALLWAY_POSITIONS: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];

impl State {
    fn room_entrance(room_idx: usize) -> usize {
        2 + room_idx * 2
    }

    fn is_complete(&self) -> bool {
        self.hallway.iter().all(|&slot| slot.is_none())
            && self.rooms.iter().enumerate().all(|(room_idx, room)| {
                room.iter().all(|&slot| {
                    slot.map(|amphipod| amphipod.target_room() == room_idx)
                        .unwrap_or(false)
                })
            })
    }

    fn can_move_from_room(&self, room_idx: usize, amphipod: Amphipod, top_idx: usize) -> bool {
        amphipod.target_room() != room_idx
            || self.rooms[room_idx][(top_idx + 1)..]
                .iter()
                .any(|&slot| slot.map(|a| a.target_room() != room_idx).unwrap_or(false))
    }

    fn can_enter_room(&self, target_room: usize) -> bool {
        self.rooms[target_room]
            .iter()
            .all(|&slot| slot.map(|a| a.target_room() == target_room).unwrap_or(true))
    }

    fn is_path_clear(&self, from: usize, to: usize) -> bool {
        let (start, end) = if from < to { (from, to) } else { (to, from) };
        (start..=end).all(|i| self.hallway[i].is_none())
    }

    fn get_movable_amphipod(&self, room_idx: usize) -> Option<(usize, Amphipod)> {
        self.rooms[room_idx]
            .iter()
            .position(|slot| slot.is_some())
            .and_then(|top_idx| {
                self.rooms[room_idx][top_idx]
                    .filter(|&amphipod| self.can_move_from_room(room_idx, amphipod, top_idx))
                    .map(|amphipod| (top_idx, amphipod))
            })
    }

    fn create_room_to_hallway_move(
        &self,
        room_idx: usize,
        top_idx: usize,
        hallway_pos: usize,
        amphipod: Amphipod,
    ) -> (State, usize) {
        let mut new_state = self.clone();
        new_state.rooms[room_idx][top_idx] = None;
        new_state.hallway[hallway_pos] = Some(amphipod);

        let steps = hallway_pos.abs_diff(Self::room_entrance(room_idx)) + top_idx + 1;
        (new_state, steps * amphipod.energy_per_step())
    }

    fn room_to_hallway_moves(&self) -> impl Iterator<Item = (State, usize)> + '_ {
        (0..4).flat_map(move |room_idx| {
            self.get_movable_amphipod(room_idx)
                .into_iter()
                .flat_map(move |(top_idx, amphipod)| {
                    let room_entrance = Self::room_entrance(room_idx);

                    VALID_HALLWAY_POSITIONS
                        .iter()
                        .copied()
                        .filter(move |&hallway_pos| self.is_path_clear(hallway_pos, room_entrance))
                        .map(move |hallway_pos| {
                            self.create_room_to_hallway_move(
                                room_idx,
                                top_idx,
                                hallway_pos,
                                amphipod,
                            )
                        })
                })
        })
    }

    fn get_amphipods_in_hallway(&self) -> impl Iterator<Item = (usize, Amphipod)> + '_ {
        self.hallway
            .iter()
            .enumerate()
            .filter_map(|(pos, &slot)| slot.map(|amphipod| (pos, amphipod)))
    }

    fn find_room_slot(&self, room_idx: usize) -> Option<usize> {
        self.rooms[room_idx]
            .iter()
            .rposition(|&slot| slot.is_none())
    }

    fn calculate_hallway_path(&self, hallway_pos: usize, room_entrance: usize) -> (usize, usize) {
        if hallway_pos < room_entrance {
            (hallway_pos + 1, room_entrance)
        } else {
            (room_entrance, hallway_pos - 1)
        }
    }

    fn create_hallway_to_room_move(
        &self,
        hallway_pos: usize,
        target_room: usize,
        target_slot: usize,
        amphipod: Amphipod,
    ) -> (State, usize) {
        let mut new_state = self.clone();
        new_state.hallway[hallway_pos] = None;
        new_state.rooms[target_room][target_slot] = Some(amphipod);

        let steps = hallway_pos.abs_diff(Self::room_entrance(target_room)) + target_slot + 1;
        (new_state, steps * amphipod.energy_per_step())
    }

    fn try_move_to_room(&self, hallway_pos: usize, amphipod: Amphipod) -> Option<(State, usize)> {
        let target_room = amphipod.target_room();
        let room_entrance = Self::room_entrance(target_room);

        self.can_enter_room(target_room)
            .then(|| self.find_room_slot(target_room))
            .flatten()
            .and_then(|target_slot| {
                let (path_start, path_end) =
                    self.calculate_hallway_path(hallway_pos, room_entrance);

                self.is_path_clear(path_start, path_end).then(|| {
                    self.create_hallway_to_room_move(
                        hallway_pos,
                        target_room,
                        target_slot,
                        amphipod,
                    )
                })
            })
    }

    fn hallway_to_room_moves(&self) -> impl Iterator<Item = (State, usize)> + '_ {
        self.get_amphipods_in_hallway()
            .filter_map(move |(hallway_pos, amphipod)| self.try_move_to_room(hallway_pos, amphipod))
    }

    fn generate_moves(&self) -> Vec<(State, usize)> {
        self.room_to_hallway_moves()
            .chain(self.hallway_to_room_moves())
            .collect()
    }
}

#[aoc_generator(day23)]
fn generator(input: &str) -> State {
    let lines: Vec<&str> = input.lines().collect();

    let rooms = (2..lines.len() - 1).fold(vec![vec![]; 4], |mut rooms, row| {
        [3, 5, 7, 9].iter().enumerate().for_each(|(room_idx, &x)| {
            if let Some(amphipod) = lines[row].chars().nth(x).and_then(Amphipod::from_char) {
                rooms[room_idx].push(Some(amphipod));
            }
        });
        rooms
    });

    State {
        hallway: [None; 11],
        rooms,
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct SearchState {
    cost: usize,
    state: State,
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(state: &State) -> usize {
    let mut heap = BinaryHeap::new();
    let mut costs = HashMap::new();

    heap.push(SearchState {
        cost: 0,
        state: state.clone(),
    });
    costs.insert(state.clone(), 0);

    while let Some(SearchState { cost, state }) = heap.pop() {
        if state.is_complete() {
            return cost;
        }

        if cost > *costs.get(&state).unwrap_or(&usize::MAX) {
            continue;
        }

        for (next_state, move_cost) in state.generate_moves() {
            let next_cost = cost + move_cost;

            if next_cost < *costs.get(&next_state).unwrap_or(&usize::MAX) {
                costs.insert(next_state.clone(), next_cost);
                heap.push(SearchState {
                    cost: next_cost,
                    state: next_state,
                });
            }
        }
    }

    0
}

#[aoc(day23, part1)]
fn part1(input: &State) -> usize {
    solve(input)
}

#[aoc(day23, part2)]
fn part2(input: &State) -> usize {
    use Amphipod::*;

    let extra_amphipods = [
        [Desert, Desert],
        [Copper, Bronze],
        [Bronze, Amber],
        [Amber, Copper],
    ];

    let expanded_state = State {
        hallway: input.hallway,
        rooms: input
            .rooms
            .iter()
            .zip(extra_amphipods)
            .map(|(room, extra)| {
                room.iter()
                    .take(1)
                    .chain(extra.iter().map(|&a| Some(a)).collect::<Vec<_>>().iter())
                    .chain(room.iter().skip(1))
                    .cloned()
                    .collect()
            })
            .collect(),
    };

    solve(&expanded_state)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

    #[test]
    fn test_parse() {
        let state = generator(EXAMPLE);

        assert_eq!(state.hallway, [None; 11]);

        assert_eq!(state.rooms[0].len(), 2);
        assert_eq!(state.rooms[0][0], Some(Amphipod::Bronze));
        assert_eq!(state.rooms[0][1], Some(Amphipod::Amber));
        assert_eq!(state.rooms[1][0], Some(Amphipod::Copper));
        assert_eq!(state.rooms[1][1], Some(Amphipod::Desert));
        assert_eq!(state.rooms[2][0], Some(Amphipod::Bronze));
        assert_eq!(state.rooms[2][1], Some(Amphipod::Copper));
        assert_eq!(state.rooms[3][0], Some(Amphipod::Desert));
        assert_eq!(state.rooms[3][1], Some(Amphipod::Amber));
    }

    #[test]
    fn test_is_complete() {
        let mut state = State {
            hallway: [None; 11],
            rooms: vec![vec![None, None]; 4],
        };

        state.rooms[0][0] = Some(Amphipod::Bronze);
        assert!(!state.is_complete());

        state.rooms[0][0] = Some(Amphipod::Amber);
        state.rooms[0][1] = Some(Amphipod::Amber);
        state.rooms[1][0] = Some(Amphipod::Bronze);
        state.rooms[1][1] = Some(Amphipod::Bronze);
        state.rooms[2][0] = Some(Amphipod::Copper);
        state.rooms[2][1] = Some(Amphipod::Copper);
        state.rooms[3][0] = Some(Amphipod::Desert);
        state.rooms[3][1] = Some(Amphipod::Desert);
        assert!(state.is_complete());
    }

    #[test]
    fn test_amphipod_properties() {
        assert_eq!(Amphipod::Amber.energy_per_step(), 1);
        assert_eq!(Amphipod::Bronze.energy_per_step(), 10);
        assert_eq!(Amphipod::Copper.energy_per_step(), 100);
        assert_eq!(Amphipod::Desert.energy_per_step(), 1000);

        assert_eq!(Amphipod::Amber.target_room(), 0);
        assert_eq!(Amphipod::Bronze.target_room(), 1);
        assert_eq!(Amphipod::Copper.target_room(), 2);
        assert_eq!(Amphipod::Desert.target_room(), 3);
    }

    #[test]
    fn test_generate_moves() {
        let state = generator(EXAMPLE);
        let moves = state.generate_moves();
        assert_eq!(moves.len(), 28);
    }

    #[test]
    fn test_step_counting() {
        let state = State {
            hallway: [None; 11],
            rooms: vec![vec![Some(Amphipod::Bronze)]; 4],
        };

        let moves = state.generate_moves();

        let move_to_0 = moves.iter().find(|(s, _)| s.hallway[0].is_some()).unwrap();
        assert_eq!(move_to_0.1, 30);

        let move_to_3 = moves.iter().find(|(s, _)| s.hallway[3].is_some()).unwrap();
        assert_eq!(move_to_3.1, 20);
    }

    #[test]
    fn test_hallway_to_room_move() {
        let mut state = State {
            hallway: [None; 11],
            rooms: vec![vec![None, None]; 4],
        };

        state.hallway[3] = Some(Amphipod::Amber);

        let moves = state.generate_moves();
        let move_to_room = moves.iter().find(|(s, _)| s.rooms[0][1].is_some()).unwrap();
        assert_eq!(move_to_room.1, 3);
    }

    #[test]
    fn test_simple_scenario() {
        let state = State {
            hallway: [None; 11],
            rooms: vec![
                vec![Some(Amphipod::Amber), Some(Amphipod::Bronze)],
                vec![Some(Amphipod::Bronze), Some(Amphipod::Amber)],
                vec![Some(Amphipod::Copper), Some(Amphipod::Copper)],
                vec![Some(Amphipod::Desert), Some(Amphipod::Desert)],
            ],
        };

        let result = part1(&state);
        assert_eq!(result, 112);
    }

    #[test]
    fn test_real_input() {
        let input = generator(
            "#############
#...........#
###D#D#B#A###
  #B#C#A#C#
  #########",
        );
        let result = part1(&input);
        assert_eq!(result, 16244);
    }

    #[test]
    fn test_part1() {
        let input = generator(EXAMPLE);
        let result = part1(&input);
        println!("Found cost: {}", result);
        assert_eq!(result, 12521);
    }

    #[test]
    fn test_part2() {
        let input = generator(EXAMPLE);
        let result = part2(&input);
        println!("Part 2 cost: {}", result);
        assert_eq!(result, 44169);
    }
}
