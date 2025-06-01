use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day16)]
fn generator(input: &str) -> Vec<i32> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect()
}

fn fft_phase(input: &[i32]) -> Vec<i32> {
    let base_pattern = [0, 1, 0, -1];

    (0..input.len())
        .map(|i| {
            input
                .iter()
                .enumerate()
                .map(|(j, &digit)| digit * base_pattern[((j + 1) / (i + 1)) % 4])
                .sum::<i32>()
                .abs()
                % 10
        })
        .collect()
}

fn fft(input: &[i32], phases: usize) -> Vec<i32> {
    (0..phases).fold(input.to_vec(), |signal, _| fft_phase(&signal))
}

#[aoc(day16, part1)]
fn part1(input: &[i32]) -> String {
    fft(input, 100).iter().take(8).join("")
}

#[aoc(day16, part2)]
fn part2(input: &[i32]) -> String {
    let offset = input[0..7].iter().fold(0, |acc, &d| acc * 10 + d as usize);

    let signal: Vec<i32> = input
        .iter()
        .cycle()
        .take(input.len() * 10000)
        .copied()
        .collect();

    let len = signal.len();

    if offset >= len / 2 {
        let result = (0..100).fold(signal[offset..].to_vec(), |mut acc, _| {
            (0..acc.len() - 1)
                .rev()
                .for_each(|i| acc[i] = (acc[i] + acc[i + 1]) % 10);
            acc
        });

        result.iter().take(8).join("")
    } else {
        panic!("Offset in first half not implemented");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fft_4_phases() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let result = fft(&input, 4);
        let first_8: String = result.iter().take(8).map(|d| d.to_string()).collect();
        assert_eq!(first_8, "01029498");
    }

    #[test]
    fn test_part1_examples() {
        let input1 = generator("80871224585914546619083218645595");
        let result1 = part1(&input1);
        assert_eq!(result1, "24176176");

        let input2 = generator("19617804207202209144916044189917");
        let result2 = part1(&input2);
        assert_eq!(result2, "73745418");

        let input3 = generator("69317163492948606335995924319873");
        let result3 = part1(&input3);
        assert_eq!(result3, "52432133");
    }

    #[test]
    fn test_part2_examples() {
        let input1 = generator("03036732577212944063491565474664");
        let result1 = part2(&input1);
        assert_eq!(result1, "84462026");

        let input2 = generator("02935109699940807407585447034323");
        let result2 = part2(&input2);
        assert_eq!(result2, "78725270");

        let input3 = generator("03081770884921959731165446850517");
        let result3 = part2(&input3);
        assert_eq!(result3, "53553731");
    }
}
