use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{Itertools, iproduct};

type Point = (i16, i16);
type Grid = HashMap<Point, char>;
type Location = (Point, i16);

#[aoc_generator(day3)]
fn generator(input: &str) -> Grid {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i16, y as i16), c))
        })
        .collect()
}

fn part_number_locations(grid: &Grid) -> Vec<Location> {
    let (startx, endx) = grid.keys().map(|&(x, _)| x).minmax().into_option().unwrap();
    let (starty, endy) = grid.keys().map(|&(_, y)| y).minmax().into_option().unwrap();

    let (_, locations) = iproduct!(starty..=endy, startx..=endx)
        .map(|(y, x)| (x, y))
        .fold((None, vec![]), |(current, mut acc), point @ (x, _)| {
            match grid.get(&point) {
                Some(c) if c.is_ascii_digit() => {
                    if current.is_none() {
                        return (Some(point), acc);
                    }
                }
                _ => {
                    if let Some(start @ (startx, _)) = current {
                        let length = x - startx;
                        acc.push((start, length));
                        return (None, acc);
                    }
                }
            }

            (current, acc)
        });

    locations
}

fn part_number_neighbors(((startx, y), length): Location) -> Vec<Point> {
    let endx = startx + length - 1;
    (startx..=endx).fold(Vec::with_capacity(8), |mut acc, x| {
        acc.push((x, y - 1));
        acc.push((x, y + 1));

        if x == startx {
            acc.push((x - 1, y - 1));
            acc.push((x - 1, y));
            acc.push((x - 1, y + 1));
        }

        if x == endx {
            acc.push((x + 1, y - 1));
            acc.push((x + 1, y));
            acc.push((x + 1, y + 1));
        }

        acc
    })
}

fn part_number_symbols(location: Location, grid: &Grid) -> impl Iterator<Item = Point> + '_ {
    let neighbors = part_number_neighbors(location);
    neighbors.into_iter().filter(|neighbor| {
        grid.get(neighbor)
            .map(|&c| !c.is_ascii_digit() && c != '.')
            .unwrap_or_default()
    })
}

fn part_number(((startx, y), length): Location, grid: &Grid) -> Option<u32> {
    let endx = startx + length - 1;
    (startx..=endx)
        .map(|x| grid.get(&(x, y)))
        .collect::<Option<String>>()?
        .parse()
        .ok()
}

#[aoc(day3, part1)]
fn part1(input: &Grid) -> Option<u32> {
    let locations = part_number_locations(input);
    locations
        .into_iter()
        .filter(|&location| part_number_symbols(location, input).next().is_some())
        .map(|location| part_number(location, input))
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &Grid) -> Option<u32> {
    let locations = part_number_locations(input);
    let gears: HashMap<Point, Vec<Location>> =
        locations
            .into_iter()
            .fold(HashMap::new(), |mut acc, location| {
                let symbol_points =
                    part_number_symbols(location, input).filter(|p| input.get(p) == Some(&'*'));
                for point in symbol_points {
                    acc.entry(point).or_default().push(location)
                }

                acc
            });

    gears
        .into_values()
        .filter(|locations| locations.len() == 2)
        .map(|locations| {
            locations
                .into_iter()
                .map(|location| part_number(location, input))
                .product::<Option<u32>>()
        })
        .sum()
}
