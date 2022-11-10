use aoc_runner_derive::{aoc, aoc_generator};
use serde_json::Value;

#[aoc_generator(day12)]
fn generator(input: &str) -> serde_json::Result<Value> {
    serde_json::from_str(input)
}

#[aoc(day12, part1)]
fn part1(input: &Value) -> i64 {
    let mut search = Vec::new();
    search.push(input.clone());

    let mut value = 0;
    while let Some(current) = search.pop() {
        match current {
            Value::Number(n) => value += n.as_i64().unwrap(),
            Value::Array(values) => {
                search.extend_from_slice(&values);
            }
            Value::Object(map) => {
                search.extend(map.values().cloned());
            }
            _ => {}
        }
    }

    value
}

#[aoc(day12, part2)]
fn part2(input: &Value) -> i64 {
    let mut search = Vec::new();
    search.push(input.clone());

    let mut value = 0;
    while let Some(current) = search.pop() {
        match current {
            Value::Number(n) => value += n.as_i64().unwrap(),
            Value::Array(values) => {
                search.extend_from_slice(&values);
            }
            Value::Object(map) => {
                let red = map.values().any(|v| v == &Value::from("red"));
                if !red {
                    search.extend(map.values().cloned());
                }
            }
            _ => {}
        }
    }

    value
}
