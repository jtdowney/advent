use std::collections::{HashMap, HashSet, VecDeque};

use aoc_runner_derive::aoc;

type Point = (i32, i32);

#[derive(Debug)]
struct Map {
    doors: HashSet<(Point, Point)>,
}

impl Map {
    fn from_regex(regex: &str) -> Self {
        let mut doors = HashSet::new();
        let mut positions = vec![(0, 0)];
        let mut stack = Vec::new();

        for ch in regex.chars() {
            match ch {
                '^' | '$' => {}
                'N' => {
                    let mut new_positions = Vec::new();
                    for &(x, y) in &positions {
                        let next = (x, y - 1);
                        let mut door = ((x, y), next);
                        let (from, to) = door;
                        if from > to {
                            door = (to, from);
                        }
                        doors.insert(door);
                        new_positions.push(next);
                    }
                    positions = new_positions;
                }
                'S' => {
                    let mut new_positions = Vec::new();
                    for &(x, y) in &positions {
                        let next = (x, y + 1);
                        let mut door = ((x, y), next);
                        let (from, to) = door;
                        if from > to {
                            door = (to, from);
                        }
                        doors.insert(door);
                        new_positions.push(next);
                    }
                    positions = new_positions;
                }
                'E' => {
                    let mut new_positions = Vec::new();
                    for &(x, y) in &positions {
                        let next = (x + 1, y);
                        let mut door = ((x, y), next);
                        let (from, to) = door;
                        if from > to {
                            door = (to, from);
                        }
                        doors.insert(door);
                        new_positions.push(next);
                    }
                    positions = new_positions;
                }
                'W' => {
                    let mut new_positions = Vec::new();
                    for &(x, y) in &positions {
                        let next = (x - 1, y);
                        let mut door = ((x, y), next);
                        let (from, to) = door;
                        if from > to {
                            door = (to, from);
                        }
                        doors.insert(door);
                        new_positions.push(next);
                    }
                    positions = new_positions;
                }
                '(' => {
                    stack.push((positions.clone(), HashSet::new()));
                }
                '|' => {
                    let (branch_start, branch_ends) = stack.last_mut().unwrap();
                    for pos in &positions {
                        branch_ends.insert(*pos);
                    }
                    positions = branch_start.clone();
                }
                ')' => {
                    let (_, mut branch_ends) = stack.pop().unwrap();
                    for pos in &positions {
                        branch_ends.insert(*pos);
                    }
                    positions = branch_ends.into_iter().collect();
                }
                _ => panic!("unexpected character: {}", ch),
            }
        }

        Map { doors }
    }

    fn find_distances(&self) -> HashMap<Point, usize> {
        let mut distances = HashMap::new();
        let mut queue = VecDeque::new();

        distances.insert((0, 0), 0);
        queue.push_back((0, 0));

        while let Some(pos) = queue.pop_front() {
            let dist = distances[&pos];

            let (x, y) = pos;
            let neighbors = [(x, y - 1), (x, y + 1), (x + 1, y), (x - 1, y)];

            for next in &neighbors {
                let mut door = (pos, *next);
                let (from, to) = door;
                if from > to {
                    door = (to, from);
                }

                if self.doors.contains(&door) && !distances.contains_key(next) {
                    distances.insert(*next, dist + 1);
                    queue.push_back(*next);
                }
            }
        }

        distances
    }
}

#[aoc(day20, part1)]
fn part1(input: &str) -> Option<usize> {
    let map = Map::from_regex(input);
    let distances = map.find_distances();
    distances.values().max().cloned()
}

#[aoc(day20, part2)]
fn part2(input: &str) -> usize {
    let map = Map::from_regex(input);
    let distances = map.find_distances();
    distances.values().filter(|&&d| d >= 1000).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = "^WNE$";
        assert_eq!(part1(input).unwrap(), 3);
    }

    #[test]
    fn test_example2() {
        let input = "^ENWWW(NEEE|SSE(EE|N))$";
        assert_eq!(part1(input).unwrap(), 10);
    }

    #[test]
    fn test_example3() {
        let input = "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$";
        assert_eq!(part1(input).unwrap(), 18);
    }

    #[test]
    fn test_example4() {
        let input = "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$";
        assert_eq!(part1(input).unwrap(), 23);
    }

    #[test]
    fn test_example5() {
        let input = "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$";
        assert_eq!(part1(input).unwrap(), 31);
    }
}
