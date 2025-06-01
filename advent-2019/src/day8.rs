use aoc_runner_derive::{aoc, aoc_generator};

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

#[aoc_generator(day8)]
fn generate(input: &str) -> anyhow::Result<Vec<Vec<char>>> {
    let chars = input.trim().chars().collect::<Vec<char>>();
    let layers = chars
        .chunks(WIDTH * HEIGHT)
        .map(|layer| layer.to_vec())
        .collect::<Vec<Vec<char>>>();
    Ok(layers)
}

#[aoc(day8, part1)]
fn part1(layers: &[Vec<char>]) -> usize {
    let (layer, _) = layers
        .iter()
        .map(|layer| layer.iter().filter(|&c| *c == '0').count())
        .enumerate()
        .min_by_key(|&(_, count)| count)
        .unwrap();

    let ones = layers[layer].iter().filter(|&c| *c == '1').count();
    let twos = layers[layer].iter().filter(|&c| *c == '2').count();

    ones * twos
}

#[aoc(day8, part2)]
fn part2(layers: &[Vec<char>]) -> String {
    let mut result = String::new();
    result.push('\n');

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let offset = y * WIDTH + x;
            let pixel = layers
                .iter()
                .map(|layer| layer[offset])
                .find(|&pixel| pixel != '2');
            match pixel {
                Some('0') | None => result.push(' '),
                Some('1') => result.push('█'),
                _ => unreachable!(),
            }
        }
        result.push('\n');
    }

    result
}
