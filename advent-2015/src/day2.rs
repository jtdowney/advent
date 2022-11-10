use aoc_runner_derive::{aoc, aoc_generator};
use eyre::ContextCompat;

type Dimension = (usize, usize, usize);

#[aoc_generator(day2)]
fn generator(input: &str) -> eyre::Result<Vec<Dimension>> {
    let dimensions = input
        .lines()
        .map(|line| {
            let mut gift = line.trim().split('x').map(|d| d.parse().ok());
            Ok((
                gift.next().flatten().context("unable to read length")?,
                gift.next().flatten().context("unable to read width")?,
                gift.next().flatten().context("unable to read height")?,
            ))
        })
        .collect::<eyre::Result<Vec<Dimension>>>()?;

    Ok(dimensions)
}

#[aoc(day2, part1)]
fn part1(dimensions: &[Dimension]) -> usize {
    dimensions
        .iter()
        .map(|&(l, w, h)| {
            let sides = [l * w, w * h, h * l];
            let smallest = sides.iter().copied().min().unwrap();
            sides.iter().map(|&n| n * 2).sum::<usize>() + smallest
        })
        .sum()
}

#[aoc(day2, part2)]
fn part2(dimensions: &[Dimension]) -> usize {
    dimensions
        .iter()
        .map(|&(l, w, h)| {
            let mut sides = [l, w, h];
            sides.sort_unstable();

            sides.iter().copied().take(2).map(|n| n * 2).sum::<usize>()
                + sides.iter().product::<usize>()
        })
        .sum()
}
