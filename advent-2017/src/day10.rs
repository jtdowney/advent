use std::num::ParseIntError;

use aoc_runner_derive::aoc;

#[derive(Debug)]
struct KnotHasher {
    index: usize,
    skip: usize,
    state: [u8; 256],
}

impl Default for KnotHasher {
    fn default() -> Self {
        let mut state = [0u8; 256];
        for i in 0u8..=255 {
            state[i as usize] = i;
        }

        Self {
            index: 0,
            skip: 0,
            state,
        }
    }
}

impl KnotHasher {
    fn mix(&mut self, n: u8) {
        let length = self.state.len();
        let i = n as usize;
        for j in 0..i / 2 {
            let a = (self.index + j) % length;
            let b = (self.index + i - j - 1) % length;
            self.state.swap(a, b);
        }

        self.index = (self.index + i + self.skip) % length;
        self.skip += 1;
    }

    fn mix_all(&mut self, input: &[u8]) {
        for &i in input {
            self.mix(i);
        }
    }

    fn hash(&mut self, input: &[u8]) -> String {
        for _ in 0..64 {
            self.mix_all(input);
            self.mix(17);
            self.mix(31);
            self.mix(73);
            self.mix(47);
            self.mix(23);
        }

        self.state
            .chunks(16)
            .map(|block| block.iter().fold(0, |acc, b| acc ^ b))
            .map(|b| format!("{:02x}", b))
            .collect()
    }
}

#[aoc(day10, part1)]
fn part1(input: &str) -> Result<u32, ParseIntError> {
    let data = input
        .split(',')
        .map(|part| part.parse())
        .collect::<Result<Vec<u8>, _>>()?;
    let mut hasher = KnotHasher::default();
    hasher.mix_all(&data);

    let product = hasher.state.iter().take(2).map(|&n| u32::from(n)).product();
    Ok(product)
}

#[aoc(day10, part2)]
fn part2(input: &[u8]) -> String {
    let mut hasher = KnotHasher::default();
    hasher.hash(input)
}
