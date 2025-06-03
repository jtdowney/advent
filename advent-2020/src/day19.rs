use std::collections::HashMap;

use anyhow::{Result, anyhow};
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
enum Rule {
    Terminal(char),
    Sequence(Vec<Vec<usize>>),
}

#[derive(Debug)]
struct Input {
    rules: HashMap<usize, Rule>,
    messages: Vec<String>,
}

#[aoc_generator(day19)]
fn generator(input: &str) -> Result<Input> {
    let mut parts = input.split("\n\n");
    let rules_part = parts.next().ok_or_else(|| anyhow!("missing rules"))?;
    let messages_part = parts.next().ok_or_else(|| anyhow!("missing messages"))?;

    let rules = rules_part
        .lines()
        .map(|line| {
            let (id_str, rule_text) = line
                .split_once(": ")
                .ok_or_else(|| anyhow!("invalid rule format: {}", line))?;
            let id = id_str.parse()?;

            let rule = if let Some(ch) = rule_text.strip_prefix('"').and_then(|s| s.chars().next())
            {
                Rule::Terminal(ch)
            } else {
                Rule::Sequence(
                    rule_text
                        .split(" | ")
                        .map(|seq| {
                            seq.split(' ')
                                .map(str::parse)
                                .collect::<Result<Vec<_>, _>>()
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                )
            };

            Ok((id, rule))
        })
        .collect::<Result<HashMap<_, _>>>()?;

    let messages = messages_part.lines().map(String::from).collect();
    Ok(Input { rules, messages })
}

#[aoc(day19, part1)]
fn part1(input: &Input) -> usize {
    input
        .messages
        .iter()
        .filter(|msg| matches_rule(&input.rules, 0, msg))
        .count()
}

#[aoc(day19, part2)]
fn part2(input: &Input) -> usize {
    let mut rules = input.rules.clone();
    rules.insert(8, Rule::Sequence(vec![vec![42], vec![42, 8]]));
    rules.insert(11, Rule::Sequence(vec![vec![42, 31], vec![42, 11, 31]]));

    input
        .messages
        .iter()
        .filter(|msg| matches_rule(&rules, 0, msg))
        .count()
}

fn matches_rule(rules: &HashMap<usize, Rule>, rule_id: usize, message: &str) -> bool {
    matches_rule_recursive(rules, rule_id, message)
        .map(|remaining| remaining.iter().any(|s| s.is_empty()))
        .unwrap_or(false)
}

fn matches_rule_recursive<'a>(
    rules: &HashMap<usize, Rule>,
    rule_id: usize,
    message: &'a str,
) -> Option<Vec<&'a str>> {
    if message.is_empty() {
        return None;
    }

    match rules.get(&rule_id)? {
        Rule::Terminal(ch) => message
            .chars()
            .next()
            .filter(|c| c == ch)
            .map(|_| vec![&message[1..]]),
        Rule::Sequence(sequences) => {
            let results: Vec<&str> = sequences
                .iter()
                .flat_map(|sequence| {
                    sequence.iter().fold(vec![message], |current, &rule_id| {
                        current
                            .into_iter()
                            .flat_map(|msg| {
                                matches_rule_recursive(rules, rule_id, msg).unwrap_or_default()
                            })
                            .collect()
                    })
                })
                .collect();

            (!results.is_empty()).then_some(results)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_rules() {
        let input = r#"0: 1 2
1: "a"
2: 1 3 | 3 1
3: "b"

aab
aba"#;
        let parsed = generator(input).unwrap();
        assert_eq!(part1(&parsed), 2);
    }

    #[test]
    fn test_complex_rules() {
        let input = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;
        let parsed = generator(input).unwrap();
        assert_eq!(part1(&parsed), 2);
    }

    #[test]
    fn test_part2() {
        let input = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;
        let parsed = generator(input).unwrap();
        assert_eq!(part1(&parsed), 3);
        assert_eq!(part2(&parsed), 12);
    }
}
