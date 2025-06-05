use std::collections::HashMap;

use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day19)]
fn generator(input: &str) -> anyhow::Result<(Vec<String>, Vec<String>)> {
    let (patterns_line, designs_part) = input.split_once("\n\n").context("invalid input format")?;

    let patterns = patterns_line.split(", ").map(String::from).collect();
    let designs = designs_part.lines().map(String::from).collect();

    Ok((patterns, designs))
}

fn memoized_recursion<T, F>(
    design: &str,
    patterns: &[String],
    cache: &mut HashMap<String, T>,
    base_case: T,
    combine: F,
) -> T
where
    T: Copy,
    F: Fn(&str, &[String], &mut HashMap<String, T>) -> T,
{
    if design.is_empty() {
        return base_case;
    }

    if let Some(&result) = cache.get(design) {
        return result;
    }

    let result = combine(design, patterns, cache);
    cache.insert(design.to_string(), result);
    result
}

fn can_make_design(design: &str, patterns: &[String], cache: &mut HashMap<String, bool>) -> bool {
    memoized_recursion(design, patterns, cache, true, |design, patterns, cache| {
        patterns
            .iter()
            .filter(|pattern| design.starts_with(pattern.as_str()))
            .any(|pattern| can_make_design(&design[pattern.len()..], patterns, cache))
    })
}

fn count_ways_to_make_design(
    design: &str,
    patterns: &[String],
    cache: &mut HashMap<String, usize>,
) -> usize {
    memoized_recursion(design, patterns, cache, 1, |design, patterns, cache| {
        patterns
            .iter()
            .filter(|pattern| design.starts_with(pattern.as_str()))
            .map(|pattern| count_ways_to_make_design(&design[pattern.len()..], patterns, cache))
            .sum()
    })
}

#[aoc(day19, part1)]
fn part1((patterns, designs): &(Vec<String>, Vec<String>)) -> usize {
    let mut cache = HashMap::new();
    designs
        .iter()
        .filter(|design| can_make_design(design, patterns, &mut cache))
        .count()
}

#[aoc(day19, part2)]
fn part2((patterns, designs): &(Vec<String>, Vec<String>)) -> usize {
    let mut cache = HashMap::new();
    designs
        .iter()
        .map(|design| count_ways_to_make_design(design, patterns, &mut cache))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_generator() {
        let (patterns, designs) = generator(EXAMPLE_INPUT).unwrap();

        assert_eq!(patterns, vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"]);
        assert_eq!(designs.len(), 8);
        assert_eq!(designs[0], "brwrr");
        assert_eq!(designs[4], "ubwu");
    }

    #[test]
    fn test_can_make_design() {
        let patterns = vec![
            "r".to_string(),
            "wr".to_string(),
            "b".to_string(),
            "g".to_string(),
            "bwu".to_string(),
            "rb".to_string(),
            "gb".to_string(),
            "br".to_string(),
        ];
        let mut cache = HashMap::new();

        assert!(can_make_design("brwrr", &patterns, &mut cache));
        assert!(can_make_design("bggr", &patterns, &mut cache));
        assert!(can_make_design("gbbr", &patterns, &mut cache));
        assert!(can_make_design("rrbgbr", &patterns, &mut cache));
        assert!(!can_make_design("ubwu", &patterns, &mut cache));
        assert!(can_make_design("bwurrg", &patterns, &mut cache));
        assert!(can_make_design("brgr", &patterns, &mut cache));
        assert!(!can_make_design("bbrgwb", &patterns, &mut cache));
    }

    #[test]
    fn test_part1() {
        let (patterns, designs) = generator(EXAMPLE_INPUT).unwrap();
        assert_eq!(part1(&(patterns, designs)), 6);
    }

    #[test]
    fn test_single_pattern_match() {
        let patterns = vec!["abc".to_string()];
        let mut cache = HashMap::new();
        assert!(can_make_design("abc", &patterns, &mut cache));
        assert!(!can_make_design("ab", &patterns, &mut cache));
        assert!(!can_make_design("abcd", &patterns, &mut cache));
    }

    #[test]
    fn test_multiple_ways_to_make_design() {
        let patterns = vec!["a".to_string(), "aa".to_string(), "aaa".to_string()];
        let mut cache = HashMap::new();
        assert!(can_make_design("aaa", &patterns, &mut cache));
    }

    #[test]
    fn test_edge_cases() {
        let patterns = vec!["ab".to_string(), "bc".to_string(), "abc".to_string()];
        let mut cache = HashMap::new();

        assert!(can_make_design("abc", &patterns, &mut cache));
        assert!(can_make_design("abbc", &patterns, &mut cache));
        assert!(!can_make_design("ac", &patterns, &mut cache));
        assert!(can_make_design("ababc", &patterns, &mut cache));
        assert!(can_make_design("abcabc", &patterns, &mut cache));
    }

    #[test]
    fn test_no_patterns() {
        let patterns = vec![];
        let mut cache = HashMap::new();

        assert!(can_make_design("", &patterns, &mut cache));
        assert!(!can_make_design("a", &patterns, &mut cache));
    }

    #[test]
    fn test_count_ways_to_make_design() {
        let patterns = vec![
            "r".to_string(),
            "wr".to_string(),
            "b".to_string(),
            "g".to_string(),
            "bwu".to_string(),
            "rb".to_string(),
            "gb".to_string(),
            "br".to_string(),
        ];
        let mut cache = HashMap::new();

        assert_eq!(count_ways_to_make_design("brwrr", &patterns, &mut cache), 2);
        assert_eq!(count_ways_to_make_design("bggr", &patterns, &mut cache), 1);
        assert_eq!(count_ways_to_make_design("gbbr", &patterns, &mut cache), 4);
        assert_eq!(
            count_ways_to_make_design("rrbgbr", &patterns, &mut cache),
            6
        );
        assert_eq!(count_ways_to_make_design("ubwu", &patterns, &mut cache), 0);
        assert_eq!(
            count_ways_to_make_design("bwurrg", &patterns, &mut cache),
            1
        );
        assert_eq!(count_ways_to_make_design("brgr", &patterns, &mut cache), 2);
        assert_eq!(
            count_ways_to_make_design("bbrgwb", &patterns, &mut cache),
            0
        );
    }

    #[test]
    fn test_part2() {
        let (patterns, designs) = generator(EXAMPLE_INPUT).unwrap();
        assert_eq!(part2(&(patterns, designs)), 16);
    }

    #[test]
    fn test_count_ways_simple_cases() {
        let patterns = vec!["a".to_string(), "aa".to_string(), "aaa".to_string()];
        let mut cache = HashMap::new();

        assert_eq!(count_ways_to_make_design("", &patterns, &mut cache), 1);
        assert_eq!(count_ways_to_make_design("a", &patterns, &mut cache), 1);
        assert_eq!(count_ways_to_make_design("aa", &patterns, &mut cache), 2);
        assert_eq!(count_ways_to_make_design("aaa", &patterns, &mut cache), 4);
    }
}
