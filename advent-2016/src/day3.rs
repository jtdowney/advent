use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Copy, Clone, Debug)]
struct Triangle(u16, u16, u16);

impl Triangle {
    fn is_valid(&self) -> bool {
        let Triangle(x, y, z) = *self;
        x + y > z && x + z > y && y + z > x
    }
}

#[aoc_generator(day3)]
fn generator(input: &str) -> anyhow::Result<Vec<Vec<u16>>> {
    input
        .lines()
        .map(|line| {
            let row = line
                .split_whitespace()
                .map(|s| s.parse::<u16>())
                .collect::<Result<Vec<_>, _>>()?;
            Ok(row)
        })
        .collect()
}

#[aoc(day3, part1)]
fn part1(input: &[Vec<u16>]) -> usize {
    input
        .iter()
        .map(|sides| Triangle(sides[0], sides[1], sides[2]))
        .filter(Triangle::is_valid)
        .count()
}

#[aoc(day3, part2)]
fn part2(input: &[Vec<u16>]) -> usize {
    input
        .chunks(3)
        .flat_map(|chunk| (0..3).map(|i| Triangle(chunk[0][i], chunk[1][i], chunk[2][i])))
        .filter(Triangle::is_valid)
        .count()
}
