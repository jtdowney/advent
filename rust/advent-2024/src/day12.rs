use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Point = (i32, i32);
type Garden = HashMap<Point, char>;

#[aoc_generator(day12)]
fn generator(input: &str) -> Garden {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, ch)| ((x as i32, y as i32), ch))
        })
        .collect()
}

#[aoc(day12, part1)]
fn part1(garden: &Garden) -> u32 {
    find_regions(garden)
        .iter()
        .map(|region| region.len() as u32 * calculate_perimeter(region))
        .sum()
}

#[aoc(day12, part2)]
fn part2(garden: &Garden) -> u32 {
    find_regions(garden)
        .iter()
        .map(|region| region.len() as u32 * calculate_sides(region))
        .sum()
}

fn neighbors((x, y): Point) -> [Point; 4] {
    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
}

fn find_regions(garden: &Garden) -> Vec<HashSet<Point>> {
    let mut regions = Vec::new();
    let mut visited = HashSet::new();

    for (&point, &plant) in garden {
        if visited.contains(&point) {
            continue;
        }

        let mut region = HashSet::new();
        let mut queue = VecDeque::from([point]);

        while let Some(current) = queue.pop_front() {
            if !visited.insert(current) {
                continue;
            }

            region.insert(current);

            neighbors(current)
                .into_iter()
                .filter(|&neighbor| {
                    garden.get(&neighbor).is_some_and(|&p| p == plant)
                        && !visited.contains(&neighbor)
                })
                .for_each(|neighbor| queue.push_back(neighbor));
        }

        regions.push(region);
    }

    regions
}

fn calculate_perimeter(region: &HashSet<Point>) -> u32 {
    region
        .iter()
        .flat_map(|&point| neighbors(point))
        .filter(|neighbor| !region.contains(neighbor))
        .count() as u32
}

fn calculate_sides(region: &HashSet<Point>) -> u32 {
    let top_edges: HashSet<Point> = region
        .iter()
        .filter(|&&(x, y)| !region.contains(&(x, y - 1)))
        .copied()
        .collect();

    let bottom_edges: HashSet<Point> = region
        .iter()
        .filter(|&&(x, y)| !region.contains(&(x, y + 1)))
        .copied()
        .collect();

    let left_edges: HashSet<Point> = region
        .iter()
        .filter(|&&(x, y)| !region.contains(&(x - 1, y)))
        .copied()
        .collect();

    let right_edges: HashSet<Point> = region
        .iter()
        .filter(|&&(x, y)| !region.contains(&(x + 1, y)))
        .copied()
        .collect();

    count_segments_horizontal(&top_edges)
        + count_segments_horizontal(&bottom_edges)
        + count_segments_vertical(&left_edges)
        + count_segments_vertical(&right_edges)
}

fn count_segments_horizontal(edges: &HashSet<Point>) -> u32 {
    count_segments(edges, |&(x, y)| (y, x))
}

fn count_segments_vertical(edges: &HashSet<Point>) -> u32 {
    count_segments(edges, |&(x, y)| (x, y))
}

fn count_segments<K, F>(edges: &HashSet<Point>, key_fn: F) -> u32
where
    K: Hash + Eq,
    F: Fn(&Point) -> (K, i32),
{
    edges
        .iter()
        .map(key_fn)
        .into_group_map()
        .values()
        .map(|coords| count_continuous_segments(coords))
        .sum()
}

fn count_continuous_segments(coords: &[i32]) -> u32 {
    let mut sorted = coords.to_vec();
    sorted.sort_unstable();
    sorted.windows(2).filter(|w| w[1] - w[0] > 1).count() as u32 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "AAAA
BBCD
BBCC
EEEC";

    const EXAMPLE2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    const EXAMPLE3: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn test_generator() {
        let garden = generator(EXAMPLE1);
        assert_eq!(garden.len(), 16);
        assert_eq!(garden[&(0, 0)], 'A');
        assert_eq!(garden[&(3, 0)], 'A');
        assert_eq!(garden[&(0, 1)], 'B');
        assert_eq!(garden[&(3, 3)], 'C');
    }

    #[test]
    fn test_find_regions() {
        let garden = generator(EXAMPLE1);
        let regions = find_regions(&garden);
        assert_eq!(regions.len(), 5);
    }

    #[test]
    fn test_region_area() {
        let garden = generator(EXAMPLE1);
        let regions = find_regions(&garden);

        let a_region = regions
            .iter()
            .find(|r| garden[r.iter().next().unwrap()] == 'A')
            .unwrap();
        let d_region = regions
            .iter()
            .find(|r| garden[r.iter().next().unwrap()] == 'D')
            .unwrap();

        assert_eq!(a_region.len(), 4);
        assert_eq!(d_region.len(), 1);
    }

    #[test]
    fn test_region_perimeter() {
        let garden = generator(EXAMPLE1);
        let regions = find_regions(&garden);

        let a_region = regions
            .iter()
            .find(|r| garden[r.iter().next().unwrap()] == 'A')
            .unwrap();
        let d_region = regions
            .iter()
            .find(|r| garden[r.iter().next().unwrap()] == 'D')
            .unwrap();

        assert_eq!(calculate_perimeter(a_region), 10);
        assert_eq!(calculate_perimeter(d_region), 4);
    }

    #[test]
    fn test_part1_example1() {
        let garden = generator(EXAMPLE1);
        assert_eq!(part1(&garden), 140);
    }

    #[test]
    fn test_part1_example2() {
        let garden = generator(EXAMPLE2);
        assert_eq!(part1(&garden), 772);
    }

    #[test]
    fn test_part1_example3() {
        let garden = generator(EXAMPLE3);
        assert_eq!(part1(&garden), 1930);
    }

    #[test]
    fn test_calculate_sides() {
        let garden = generator(EXAMPLE1);
        let regions = find_regions(&garden);

        let a_region = regions
            .iter()
            .find(|r| garden[r.iter().next().unwrap()] == 'A')
            .unwrap();
        let c_region = regions
            .iter()
            .find(|r| garden[r.iter().next().unwrap()] == 'C')
            .unwrap();

        assert_eq!(calculate_sides(a_region), 4);
        assert_eq!(calculate_sides(c_region), 8);
    }

    #[test]
    fn test_part2_example1() {
        let garden = generator(EXAMPLE1);
        assert_eq!(part2(&garden), 80);
    }

    #[test]
    fn test_part2_example2() {
        let garden = generator(EXAMPLE2);
        assert_eq!(part2(&garden), 436);
    }

    #[test]
    fn test_part2_example3() {
        let garden = generator(EXAMPLE3);
        assert_eq!(part2(&garden), 1206);
    }

    #[test]
    fn test_part2_e_shape() {
        let input = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        let garden = generator(input);
        assert_eq!(part2(&garden), 236);
    }

    #[test]
    fn test_part2_nested() {
        let input = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
        let garden = generator(input);
        assert_eq!(part2(&garden), 368);
    }
}
