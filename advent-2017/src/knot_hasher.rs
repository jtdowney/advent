use std::fmt::{self, Display};

pub struct Digest(pub [u8; 16]);

impl Display for Digest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Digest(data) = self;
        for b in data {
            write!(f, "{:02x}", b)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct KnotHasher {
    index: usize,
    skip: usize,
    pub state: [u8; 256],
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

    pub fn mix_all(&mut self, input: &[u8]) {
        for &i in input {
            self.mix(i);
        }
    }

    pub fn hash(&mut self, input: &[u8]) -> Digest {
        for _ in 0..64 {
            self.mix_all(input);
            self.mix(17);
            self.mix(31);
            self.mix(73);
            self.mix(47);
            self.mix(23);
        }

        let mut digest = [0; 16];
        let bytes = self
            .state
            .chunks(16)
            .map(|block| block.iter().fold(0, |acc, b| acc ^ b))
            .enumerate();
        for (i, byte) in bytes {
            digest[i] = byte;
        }

        Digest(digest)
    }
}
