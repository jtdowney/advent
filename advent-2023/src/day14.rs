use std::collections::{BTreeMap, HashMap};

use aoc_runner_derive::{aoc, aoc_generator};

type Point = (i16, i16);

#[derive(Clone, PartialEq, Eq, Hash)]
struct Map {
    cells: BTreeMap<Point, char>,
    width: i16,
    height: i16,
}

impl Map {
    fn transpose(&self) -> Self {
        let cells = self.cells.iter().map(|(&(x, y), &c)| ((y, x), c)).collect();
        Self {
            cells,
            width: self.height,
            height: self.width,
        }
    }

    fn reverse_x(&self) -> Self {
        let cells = (0..=self.height)
            .flat_map(|y| {
                (0..=self.width)
                    .zip((0..=self.width).rev())
                    .filter_map(move |(x, new_x)| self.cells.get(&(x, y)).map(|&c| ((new_x, y), c)))
            })
            .collect();
        Self {
            cells,
            width: self.width,
            height: self.height,
        }
    }

    fn rotate(&self) -> Self {
        self.transpose().reverse_x()
    }

    fn cycle(&self) -> Self {
        let mut result = self.tilt();
        for _ in 0..3 {
            result = result.rotate().tilt();
        }

        result.rotate()
    }

    fn tilt(&self) -> Self {
        let mut result = BTreeMap::new();
        for y in 0..=self.height {
            for x in 0..=self.width {
                if let Some(&c) = self.cells.get(&(x, y)) {
                    if c != 'O' {
                        result.insert((x, y), c);
                        continue;
                    }

                    let new_y = (0..=y)
                        .rev()
                        .take_while(|&new_y| !result.contains_key(&(x, new_y)))
                        .last()
                        .unwrap();
                    result.insert((x, new_y), c);
                }
            }
        }

        Self {
            cells: result,
            width: self.width,
            height: self.height,
        }
    }

    fn total_load(&self) -> usize {
        (0..=self.height)
            .zip((1..=self.height + 1).rev())
            .map(|(y, load)| {
                (0..=self.width)
                    .filter(|&x| self.cells.get(&(x, y)) == Some(&'O'))
                    .count()
                    * load as usize
            })
            .sum()
    }
}

#[aoc_generator(day14)]
fn generator(input: &str) -> Map {
    let cells = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, c)| c != '.')
                .map(move |(x, c)| ((x as i16, y as i16), c))
        })
        .collect::<BTreeMap<_, _>>();

    let width = cells.keys().map(|&(x, _)| x).max().unwrap();
    let height = cells.keys().map(|&(_, y)| y).max().unwrap();

    Map {
        cells,
        width,
        height,
    }
}

#[aoc(day14, part1)]
fn part1(input: &Map) -> usize {
    input.tilt().total_load()
}

#[aoc(day14, part2)]
fn part2(input: &Map) -> usize {
    let mut cycles = HashMap::new();
    let mut map = input.clone();
    let count: usize = 1_000_000_000;
    for i in 0..count {
        map = map.cycle();
        if let Some(&prev) = cycles.get(&map) {
            let size = i - prev;
            let remaining = count - i - 1;
            let extra = remaining % size;
            for _ in 0..extra {
                map = map.cycle();
            }

            return map.total_load();
        } else {
            cycles.insert(map.clone(), i);
        }
    }

    unreachable!()
}
