use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Maze {
    grid: Vec<Vec<char>>,
    start: Point,
    keys: HashMap<char, Point>,
}

#[derive(Debug)]
struct MultiMaze {
    grid: Vec<Vec<char>>,
    starts: [Point; 4],
    keys: HashMap<char, Point>,
}

trait Grid {
    fn grid(&self) -> &Vec<Vec<char>>;

    fn neighbors(&self, point: Point) -> impl Iterator<Item = Point> + '_ {
        [(0, 1), (1, 0), (0, -1), (-1, 0)]
            .iter()
            .filter_map(move |&(dx, dy)| {
                let new_x = point.x.wrapping_add_signed(dx);
                let new_y = point.y.wrapping_add_signed(dy);

                self.grid()
                    .get(new_y)?
                    .get(new_x)
                    .map(|_| Point { x: new_x, y: new_y })
            })
    }
}

impl Grid for Maze {
    fn grid(&self) -> &Vec<Vec<char>> {
        &self.grid
    }
}

impl Grid for MultiMaze {
    fn grid(&self) -> &Vec<Vec<char>> {
        &self.grid
    }
}

#[derive(Debug)]
struct PathInfo {
    distance: usize,
    required_keys: u32,
}

#[aoc_generator(day18)]
fn parse(input: &str) -> Result<Maze> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let (start, keys) = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, &cell)| (Point { x, y }, cell))
        })
        .fold(
            (Point { x: 0, y: 0 }, HashMap::new()),
            |(mut start, mut keys), (point, cell)| {
                match cell {
                    '@' => start = point,
                    'a'..='z' => {
                        keys.insert(cell, point);
                    }
                    _ => {}
                }
                (start, keys)
            },
        );

    Ok(Maze { grid, start, keys })
}

fn transform_to_multi_maze(maze: &Maze) -> MultiMaze {
    let mut grid = maze.grid.clone();
    let start = maze.start;

    let pattern = [('@', '#', '@'), ('#', '#', '#'), ('@', '#', '@')];

    pattern.iter().enumerate().for_each(|(dy, &row)| {
        [row.0, row.1, row.2]
            .iter()
            .enumerate()
            .for_each(|(dx, &ch)| {
                grid[start.y + dy - 1][start.x + dx - 1] = ch;
            });
    });

    let starts = [
        Point {
            x: start.x - 1,
            y: start.y - 1,
        },
        Point {
            x: start.x + 1,
            y: start.y - 1,
        },
        Point {
            x: start.x - 1,
            y: start.y + 1,
        },
        Point {
            x: start.x + 1,
            y: start.y + 1,
        },
    ];

    MultiMaze {
        grid,
        starts,
        keys: maze.keys.clone(),
    }
}

fn compute_direct_paths_from<G: Grid>(grid: &G, start: Point) -> HashMap<char, PathInfo> {
    let mut paths = HashMap::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back((start, 0, 0u32));
    visited.insert(start);

    while let Some((pos, dist, required_keys)) = queue.pop_front() {
        let cell = grid.grid()[pos.y][pos.x];

        if cell.is_ascii_lowercase() && pos != start {
            paths.entry(cell).or_insert(PathInfo {
                distance: dist,
                required_keys,
            });
        }

        for neighbor in grid.neighbors(pos) {
            if visited.contains(&neighbor) {
                continue;
            }

            let neighbor_cell = grid.grid()[neighbor.y][neighbor.x];
            if neighbor_cell == '#' {
                continue;
            }

            let new_required = if neighbor_cell.is_ascii_uppercase() {
                required_keys | (1 << (neighbor_cell.to_ascii_lowercase() as u8 - b'a'))
            } else {
                required_keys
            };

            visited.insert(neighbor);
            queue.push_back((neighbor, dist + 1, new_required));
        }
    }

    paths
}

impl Maze {
    fn precompute_all_paths(&self) -> HashMap<char, HashMap<char, PathInfo>> {
        std::iter::once(('@', self.start))
            .chain(self.keys.iter().map(|(&k, &v)| (k, v)))
            .map(|(key, pos)| (key, compute_direct_paths_from(self, pos)))
            .collect()
    }
}

impl MultiMaze {
    fn precompute_all_paths(&self) -> HashMap<char, HashMap<char, PathInfo>> {
        self.starts
            .iter()
            .enumerate()
            .map(|(i, &start)| (char::from(b'0' + i as u8), start))
            .chain(self.keys.iter().map(|(&k, &v)| (k, v)))
            .map(|(key, pos)| (key, compute_direct_paths_from(self, pos)))
            .collect()
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct SearchState {
    current_key: char,
    collected_keys: u32,
}

#[derive(Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct MultiSearchState {
    robot_positions: [char; 4],
    collected_keys: u32,
}

trait State: Clone + Eq + std::hash::Hash + Ord {
    fn collected_keys(&self) -> u32;
    fn get_reachable_keys<'a>(
        &self,
        paths: &'a HashMap<char, HashMap<char, PathInfo>>,
    ) -> Vec<(char, &'a PathInfo, Self)>;
}

impl State for SearchState {
    fn collected_keys(&self) -> u32 {
        self.collected_keys
    }

    fn get_reachable_keys<'a>(
        &self,
        paths: &'a HashMap<char, HashMap<char, PathInfo>>,
    ) -> Vec<(char, &'a PathInfo, Self)> {
        paths
            .get(&self.current_key)
            .into_iter()
            .flat_map(|from_paths| from_paths.iter())
            .filter(|(key_char, path_info)| {
                let key_bit = 1 << (**key_char as u8 - b'a');
                (self.collected_keys & key_bit) == 0
                    && (self.collected_keys & path_info.required_keys) == path_info.required_keys
            })
            .map(|(key_char, path_info)| {
                let key_bit = 1 << (*key_char as u8 - b'a');
                let new_state = SearchState {
                    current_key: *key_char,
                    collected_keys: self.collected_keys | key_bit,
                };
                (*key_char, path_info, new_state)
            })
            .collect()
    }
}

impl State for MultiSearchState {
    fn collected_keys(&self) -> u32 {
        self.collected_keys
    }

    fn get_reachable_keys<'a>(
        &self,
        paths: &'a HashMap<char, HashMap<char, PathInfo>>,
    ) -> Vec<(char, &'a PathInfo, Self)> {
        (0..4)
            .flat_map(|robot_idx| {
                paths
                    .get(&self.robot_positions[robot_idx])
                    .into_iter()
                    .flat_map(|from_paths| from_paths.iter())
                    .filter(|(key_char, path_info)| {
                        let key_bit = 1 << (**key_char as u8 - b'a');
                        (self.collected_keys & key_bit) == 0
                            && (self.collected_keys & path_info.required_keys)
                                == path_info.required_keys
                    })
                    .map(move |(key_char, path_info)| {
                        let key_bit = 1 << (*key_char as u8 - b'a');
                        let mut new_robot_positions = self.robot_positions;
                        new_robot_positions[robot_idx] = *key_char;

                        let new_state = MultiSearchState {
                            robot_positions: new_robot_positions,
                            collected_keys: self.collected_keys | key_bit,
                        };
                        (*key_char, path_info, new_state)
                    })
            })
            .collect()
    }
}

fn dijkstra_search<S: State>(
    initial_state: S,
    paths: &HashMap<char, HashMap<char, PathInfo>>,
    all_keys_mask: u32,
) -> usize {
    let mut distances = HashMap::new();
    let mut heap = BinaryHeap::new();

    heap.push(Reverse((0, initial_state.clone())));
    distances.insert(initial_state, 0);

    while let Some(Reverse((dist, state))) = heap.pop() {
        if state.collected_keys() == all_keys_mask {
            return dist;
        }

        if distances.get(&state).is_some_and(|&best| best < dist) {
            continue;
        }

        for (_, path_info, new_state) in state.get_reachable_keys(paths) {
            let new_dist = dist + path_info.distance;

            if distances
                .get(&new_state)
                .is_none_or(|&best| new_dist < best)
            {
                distances.insert(new_state.clone(), new_dist);
                heap.push(Reverse((new_dist, new_state)));
            }
        }
    }

    0
}

#[aoc(day18, part1)]
fn part1(maze: &Maze) -> usize {
    assert!(maze.keys.len() <= 32);

    let initial_state = SearchState {
        current_key: '@',
        collected_keys: 0,
    };

    dijkstra_search(
        initial_state,
        &maze.precompute_all_paths(),
        (1 << maze.keys.len()) - 1,
    )
}

#[aoc(day18, part2)]
fn part2(maze: &Maze) -> usize {
    assert!(maze.keys.len() <= 32);

    let multi_maze = transform_to_multi_maze(maze);

    let initial_state = MultiSearchState {
        robot_positions: ['0', '1', '2', '3'],
        collected_keys: 0,
    };

    dijkstra_search(
        initial_state,
        &multi_maze.precompute_all_paths(),
        (1 << maze.keys.len()) - 1,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_example() {
        let input = "#########
#b.A.@.a#
#########";
        let maze = parse(input).unwrap();
        assert_eq!(part1(&maze), 8);
    }

    #[test]
    fn test_trace_simple() {
        let input = "#######
#a...b#
#.###.#
#.....#
#..@..#
#######";
        let maze = parse(input).unwrap();
        let result = part1(&maze);
        assert_eq!(result, 9);
    }

    #[test]
    fn test_debug_simple() {
        let input = "#####
#a.b#
#.@.#
#...#
#####";
        let maze = parse(input).unwrap();
        assert_eq!(part1(&maze), 4);
    }

    #[test]
    fn test_single_key() {
        let input = "###
#@#
#a#
###";
        let maze = parse(input).unwrap();
        assert_eq!(part1(&maze), 1);
    }

    #[test]
    fn test_key_behind_door() {
        let input = "#####
#@.A#
#..a#
#####";
        let maze = parse(input).unwrap();
        assert_eq!(part1(&maze), 3);
    }

    #[test]
    fn test_larger_example() {
        let input = "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";
        let maze = parse(input).unwrap();
        assert_eq!(part1(&maze), 86);
    }

    #[test]
    fn test_132_steps() {
        let input = "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################";
        let maze = parse(input).unwrap();
        assert_eq!(part1(&maze), 132);
    }

    #[test]
    fn test_136_steps() {
        let input = "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";
        let maze = parse(input).unwrap();
        assert_eq!(part1(&maze), 136);
    }

    #[test]
    fn test_81_steps() {
        let input = "########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################";
        let maze = parse(input).unwrap();
        assert_eq!(part1(&maze), 81);
    }

    #[test]
    fn test_part2_example_8_steps() {
        let input = "#######
#a.#Cd#
##...##
##.@.##
##...##
#cB#Ab#
#######";
        let maze = parse(input).unwrap();
        assert_eq!(part2(&maze), 8);
    }
}
