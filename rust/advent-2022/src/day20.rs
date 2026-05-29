use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day20)]
fn generator(input: &str) -> anyhow::Result<Vec<i64>> {
    input
        .lines()
        .map(|line| line.parse::<i64>().map_err(Into::into))
        .collect()
}

fn mix(numbers: &[i64], rounds: usize, key: i64) -> i64 {
    let mut indexed: Vec<(usize, i64)> = numbers
        .iter()
        .enumerate()
        .map(|(i, &n)| (i, n * key))
        .collect();

    let len = indexed.len() as i64;

    for _ in 0..rounds {
        for i in 0..numbers.len() {
            let pos = indexed.iter().position(|&(idx, _)| idx == i).unwrap();
            let (idx, value) = indexed.remove(pos);
            let new_pos = (pos as i64 + value).rem_euclid(len - 1) as usize;
            indexed.insert(new_pos, (idx, value));
        }
    }

    let zero_pos = indexed.iter().position(|&(_, v)| v == 0).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|&offset| indexed[(zero_pos + offset) % indexed.len()].1)
        .sum()
}

#[aoc(day20, part1)]
fn part1(input: &[i64]) -> i64 {
    mix(input, 1, 1)
}

#[aoc(day20, part2)]
fn part2(input: &[i64]) -> i64 {
    const DECRYPTION_KEY: i64 = 811_589_153;
    mix(input, 10, DECRYPTION_KEY)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "1
2
-3
3
-2
0
4";
        let parsed = generator(input).unwrap();
        assert_eq!(part1(&parsed), 3);
    }

    #[test]
    fn test_part2() {
        let input = "1
2
-3
3
-2
0
4";
        let parsed = generator(input).unwrap();
        assert_eq!(part2(&parsed), 1623178306);
    }
}
