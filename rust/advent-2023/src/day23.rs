use std::collections::{HashMap, HashSet};

use anyhow::{Result, anyhow, bail};
use aoc_runner_derive::{aoc, aoc_generator};

type Point = (i32, i32);

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Path,
    Forest,
    SlopeUp,
    SlopeDown,
    SlopeLeft,
    SlopeRight,
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let tile = match value {
            '.' => Tile::Path,
            '#' => Tile::Forest,
            '^' => Tile::SlopeUp,
            'v' => Tile::SlopeDown,
            '<' => Tile::SlopeLeft,
            '>' => Tile::SlopeRight,
            _ => bail!("Invalid tile character: {}", value),
        };

        Ok(tile)
    }
}

#[derive(Debug)]
struct Map {
    tiles: HashMap<Point, Tile>,
    width: i32,
    height: i32,
}

#[aoc_generator(day23)]
#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
fn generator(input: &str) -> Result<Map> {
    let tiles = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, ch)| {
                Tile::try_from(ch)
                    .map(|tile| ((x as i32, y as i32), tile))
                    .map_err(|e| anyhow!("Invalid character at ({}, {}): {}", x, y, e))
            })
        })
        .collect::<Result<HashMap<_, _>>>()?;

    let mut lines = input.lines();
    let width = lines.next().ok_or_else(|| anyhow!("Input is empty"))?.len() as i32;
    let height = input.lines().count() as i32;

    Ok(Map {
        tiles,
        width,
        height,
    })
}

fn neighbors((x, y): Point) -> impl Iterator<Item = Point> {
    const DIRECTIONS: [Point; 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

    DIRECTIONS.into_iter().map(move |(dx, dy)| (x + dx, y + dy))
}

impl Map {
    fn valid_neighbors(&self, pos: Point) -> impl Iterator<Item = Point> + '_ {
        neighbors(pos).filter(|&p| self.tiles.get(&p).is_some_and(|t| *t != Tile::Forest))
    }

    fn slope_neighbors(&self, pos: Point) -> Vec<Point> {
        match self.tiles.get(&pos) {
            Some(Tile::Path) => self.valid_neighbors(pos).collect(),
            Some(Tile::SlopeUp) => vec![(pos.0, pos.1 - 1)],
            Some(Tile::SlopeDown) => vec![(pos.0, pos.1 + 1)],
            Some(Tile::SlopeLeft) => vec![(pos.0 - 1, pos.1)],
            Some(Tile::SlopeRight) => vec![(pos.0 + 1, pos.1)],
            _ => vec![],
        }
    }
}

fn find_longest_path_slopes(
    map: &Map,
    pos: Point,
    target: Point,
    visited: &mut HashSet<Point>,
) -> Option<usize> {
    if pos == target {
        return Some(0);
    }

    visited.insert(pos);

    let neighbors: Vec<_> = map
        .slope_neighbors(pos)
        .into_iter()
        .filter(|p| !visited.contains(p))
        .collect();

    let max_length = neighbors
        .into_iter()
        .filter_map(|next_pos| {
            find_longest_path_slopes(map, next_pos, target, visited).map(|length| length + 1)
        })
        .max();

    visited.remove(&pos);
    max_length
}

#[aoc(day23, part1)]
fn part1(map: &Map) -> usize {
    let start = (1, 0);
    let target = (map.width - 2, map.height - 1);

    find_longest_path_slopes(map, start, target, &mut HashSet::new()).unwrap_or(0)
}

fn find_intersections(map: &Map, start: Point, target: Point) -> HashSet<Point> {
    map.tiles
        .iter()
        .filter(|(_, tile)| **tile != Tile::Forest)
        .map(|(pos, _)| *pos)
        .filter(|&pos| pos == start || pos == target || map.valid_neighbors(pos).count() > 2)
        .collect()
}

fn trace_path(
    map: &Map,
    start: Point,
    from: Point,
    intersections: &HashSet<Point>,
) -> Option<(Point, usize)> {
    let (mut current, mut prev, mut distance) = (start, from, 1);

    if intersections.contains(&current) {
        return Some((current, distance));
    }

    loop {
        let next: Vec<_> = map
            .valid_neighbors(current)
            .filter(|&n| n != prev)
            .collect();

        match next.len() {
            1 => {
                prev = current;
                current = next[0];
                distance += 1;

                if intersections.contains(&current) {
                    return Some((current, distance));
                }
            }
            _ => return None,
        }
    }
}

fn build_graph(map: &Map, intersections: &HashSet<Point>) -> HashMap<Point, Vec<(Point, usize)>> {
    intersections
        .iter()
        .map(|&start| {
            let edges = map
                .valid_neighbors(start)
                .filter_map(|neighbor| trace_path(map, neighbor, start, intersections))
                .collect();
            (start, edges)
        })
        .collect()
}

fn find_longest_path_graph(
    graph: &HashMap<Point, Vec<(Point, usize)>>,
    pos: Point,
    target: Point,
    visited: &mut HashSet<Point>,
) -> Option<usize> {
    if pos == target {
        return Some(0);
    }

    visited.insert(pos);

    let edges: Vec<_> = graph
        .get(&pos)?
        .iter()
        .filter(|(next_pos, _)| !visited.contains(next_pos))
        .copied()
        .collect();

    let max_length = edges
        .into_iter()
        .filter_map(|(next_pos, distance)| {
            find_longest_path_graph(graph, next_pos, target, visited)
                .map(|length| length + distance)
        })
        .max();

    visited.remove(&pos);
    max_length
}

#[aoc(day23, part2)]
fn part2(map: &Map) -> usize {
    let start = (1, 0);
    let target = (map.width - 2, map.height - 1);

    let intersections = find_intersections(map, start, target);
    let graph = build_graph(map, &intersections);

    find_longest_path_graph(&graph, start, target, &mut HashSet::new()).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn test_part1() {
        let map = generator(EXAMPLE).unwrap();
        assert_eq!(part1(&map), 94);
    }

    #[test]
    fn test_part2() {
        let map = generator(EXAMPLE).unwrap();
        assert_eq!(part2(&map), 154);
    }

    #[test]
    fn test_invalid_character() {
        let input = "#.X#\n#..#";
        let result = generator(input);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Invalid character at (2, 0)"));
    }
}
