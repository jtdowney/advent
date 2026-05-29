use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Point = (i32, i32);
type Bounds = (Point, Point);

#[derive(Debug, Clone)]
struct Maze {
    passages: HashSet<Point>,
    portals: HashMap<String, Vec<Point>>,
    start: Point,
    end: Point,
}

impl Maze {
    fn find_bounds(&self) -> Bounds {
        let (min_x, max_x) = self
            .passages
            .iter()
            .map(|&(x, _)| x)
            .minmax()
            .into_option()
            .unwrap();
        let (min_y, max_y) = self
            .passages
            .iter()
            .map(|&(_, y)| y)
            .minmax()
            .into_option()
            .unwrap();
        ((min_x, min_y), (max_x, max_y))
    }

    fn is_outer_portal(&self, (x, y): Point, ((min_x, min_y), (max_x, max_y)): &Bounds) -> bool {
        x == *min_x || x == *max_x || y == *min_y || y == *max_y
    }

    fn neighbors(&self, (x, y): Point) -> impl Iterator<Item = Point> + '_ {
        [(0, 1), (0, -1), (1, 0), (-1, 0)]
            .iter()
            .map(move |&(dx, dy)| (x + dx, y + dy))
            .filter(|next_pos| self.passages.contains(next_pos))
    }

    fn portal_destination(&self, pos: Point) -> Option<Point> {
        self.portals.values().find_map(|positions| {
            positions
                .iter()
                .position(|&p| p == pos)
                .and_then(|idx| positions.get(1 - idx))
                .copied()
        })
    }
}

#[aoc_generator(day20)]
fn generator(input: &str) -> Result<Maze> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let passages: HashSet<Point> = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, &ch)| (ch == '.').then_some((x as i32, y as i32)))
        })
        .collect();

    let letter_positions: HashMap<Point, char> = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, &ch)| {
                ch.is_ascii_uppercase()
                    .then_some(((x as i32, y as i32), ch))
            })
        })
        .collect();

    let mut portals = HashMap::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut processed: HashSet<Point> = HashSet::new();

    for (&(x, y), &ch1) in &letter_positions {
        let pos = (x, y);
        if processed.contains(&pos) {
            continue;
        }

        let portal_info = [(1, 0), (0, 1)].iter().find_map(|&(dx, dy)| {
            let adjacent = (x + dx, y + dy);
            letter_positions.get(&adjacent).map(|&ch2| {
                let label = format!("{}{}", ch1, ch2);
                let positions = vec![pos, adjacent];
                (label, positions)
            })
        });

        if let Some((label, positions)) = portal_info {
            processed.extend(&positions);

            let portal_pos = positions
                .iter()
                .flat_map(|&(x, y)| {
                    [(-1, 0), (2, 0), (0, -1), (0, 2)]
                        .iter()
                        .map(move |&(dx, dy)| (x + dx, y + dy))
                })
                .find(|pos| passages.contains(pos));

            if let Some(pos) = portal_pos {
                match label.as_str() {
                    "AA" => start = pos,
                    "ZZ" => end = pos,
                    _ => {
                        portals.entry(label).or_insert_with(Vec::new).push(pos);
                    }
                }
            }
        }
    }

    Ok(Maze {
        passages,
        portals,
        start,
        end,
    })
}

fn bfs<F, G, S>(start: S, is_goal: F, successors: G) -> Option<usize>
where
    F: Fn(&S) -> bool,
    G: Fn(&S) -> Vec<S>,
    S: Eq + std::hash::Hash + Clone,
{
    let mut queue = VecDeque::from([(start, 0)]);
    let mut visited = HashSet::new();
    visited.insert(queue[0].0.clone());

    while let Some((state, steps)) = queue.pop_front() {
        if is_goal(&state) {
            return Some(steps);
        }

        for next_state in successors(&state) {
            if visited.insert(next_state.clone()) {
                queue.push_back((next_state, steps + 1));
            }
        }
    }

    None
}

#[aoc(day20, part1)]
fn part1(input: &Maze) -> usize {
    bfs(
        input.start,
        |&pos| pos == input.end,
        |&pos| {
            input
                .neighbors(pos)
                .chain(input.portal_destination(pos))
                .collect()
        },
    )
    .unwrap_or(0)
}

#[aoc(day20, part2)]
fn part2(input: &Maze) -> usize {
    let bounds = input.find_bounds();

    bfs(
        (input.start, 0),
        |&(pos, level)| pos == input.end && level == 0,
        |&(pos, level)| {
            let neighbors: Vec<_> = input
                .neighbors(pos)
                .map(|next_pos| (next_pos, level))
                .collect();

            let portal_transitions = input
                .portal_destination(pos)
                .and_then(|dest| {
                    let is_outer = input.is_outer_portal(pos, &bounds);
                    match (is_outer, level) {
                        (true, 0) => None,
                        (true, l) => Some((dest, l - 1)),
                        (false, l) => Some((dest, l + 1)),
                    }
                })
                .into_iter()
                .collect::<Vec<_>>();

            neighbors.into_iter().chain(portal_transitions).collect()
        },
    )
    .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "         A
         A
  #######.#########
  #######.........#
  #######.#######.#
  #######.#######.#
  #######.#######.#
  #####  B    ###.#
BC...##  C    ###.#
  ##.##       ###.#
  ##...DE  F  ###.#
  #####    G  ###.#
  #########.#####.#
DE..#######...###.#
  #.#########.###.#
FG..#########.....#
  ###########.#####
             Z
             Z       ";

    const EXAMPLE2: &str = "                   A
                   A
  #################.#############
  #.#...#...................#.#.#
  #.#.#.###.###.###.#########.#.#
  #.#.#.......#...#.....#.#.#...#
  #.#########.###.#####.#.#.###.#
  #.............#.#.....#.......#
  ###.###########.###.#####.#.#.#
  #.....#        A   C    #.#.#.#
  #######        S   P    #####.#
  #.#...#                 #......VT
  #.#.#.#                 #.#####
  #...#.#               YN....#.#
  #.###.#                 #####.#
DI....#.#                 #.....#
  #####.#                 #.###.#
ZZ......#               QG....#..AS
  ###.###                 #######
JO..#.#.#                 #.....#
  #.#.#.#                 ###.#.#
  #...#..DI             BU....#..LF
  #####.#                 #.#####
YN......#               VT..#....QG
  #.###.#                 #.###.#
  #.#...#                 #.....#
  ###.###    J L     J    #.#.###
  #.....#    O F     P    #.#...#
  #.###.#####.#.#####.#####.###.#
  #...#.#.#...#.....#.....#.#...#
  #.#####.###.###.#.#.#########.#
  #...#.#.....#...#.#.#.#.....#.#
  #.###.#####.###.###.#.#.#######
  #.#.........#...#.............#
  #########.###.###.#############
           B   J   C
           U   P   P               ";

    #[test]
    fn test_parse_maze() {
        let maze = generator(EXAMPLE1).unwrap();

        assert_eq!(maze.start, (9, 2));
        assert_eq!(maze.end, (13, 16));

        assert!(maze.portals.contains_key("BC"));
        assert!(maze.portals.contains_key("DE"));
        assert!(maze.portals.contains_key("FG"));
        assert_eq!(maze.portals.len(), 3);

        let bc_portals = &maze.portals["BC"];
        assert_eq!(bc_portals.len(), 2);
        assert!(bc_portals.contains(&(9, 6)));
        assert!(bc_portals.contains(&(2, 8)));
    }

    #[test]
    fn test_part1_example1() {
        let maze = generator(EXAMPLE1).unwrap();
        assert_eq!(part1(&maze), 23);
    }

    #[test]
    fn test_part1_example2() {
        let maze = generator(EXAMPLE2).unwrap();
        assert_eq!(part1(&maze), 58);
    }

    #[test]
    fn test_is_outer_portal() {
        let maze = generator(EXAMPLE1).unwrap();

        let bounds = maze.find_bounds();

        assert!(maze.is_outer_portal((2, 8), &bounds));
        assert!(!maze.is_outer_portal((9, 6), &bounds));
        assert!(maze.is_outer_portal((2, 13), &bounds));
        assert!(!maze.is_outer_portal((11, 12), &bounds));
    }

    #[test]
    fn test_part2_example1() {
        let maze = generator(EXAMPLE1).unwrap();
        assert_eq!(part2(&maze), 26);
    }

    const EXAMPLE3: &str = "             Z L X W       C
             Z P Q B       K
  ###########.#.#.#.#######.###############
  #...#.......#.#.......#.#.......#.#.#...#
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###
  #.#...#.#.#...#.#.#...#...#...#.#.......#
  #.###.#######.###.###.#.###.###.#.#######
  #...#.......#.#...#...#.............#...#
  #.#########.#######.#.#######.#######.###
  #...#.#    F       R I       Z    #.#.#.#
  #.###.#    D       E C       H    #.#.#.#
  #.#...#                           #...#.#
  #.###.#                           #.###.#
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#
CJ......#                           #.....#
  #######                           #######
  #.#....CK                         #......IC
  #.###.#                           #.###.#
  #.....#                           #...#.#
  ###.###                           #.#.#.#
XF....#.#                         RF..#.#.#
  #####.#                           #######
  #......CJ                       NM..#...#
  ###.#.#                           #.###.#
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#
  #.....#        F   Q       P      #.#.#.#
  ###.###########.###.#######.#########.###
  #.....#...#.....#.......#...#.....#.#...#
  #####.#.###.#######.#######.###.###.#.#.#
  #.......#.......#.#.#.#.#...#...#...#.#.#
  #####.###.#####.#.#.#.#.###.###.#.###.###
  #.......#.....#.#...#...............#...#
  #############.#.#.###.###################
               A O F   N
               A A D   M                     ";

    #[test]
    fn test_part2_example3() {
        let maze = generator(EXAMPLE3).unwrap();
        assert_eq!(part2(&maze), 396);
    }
}
