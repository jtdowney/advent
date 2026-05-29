# Advent of Code

My solutions to [Advent of Code](https://adventofcode.com/) challenges in Rust.

## Structure

Each year is organized as a separate Rust crate (`advent-2015`, `advent-2016`, etc.) using [cargo-aoc](https://github.com/gobanos/cargo-aoc) to manage daily solutions.

## Running Solutions

```bash
# Run last day implemented for a year
cd advent-2024
cargo aoc

# Run a specific day
cargo aoc -d 5
```

## Development

Repository-wide commands are available through [just](https://github.com/casey/just):

```bash
just fmt      # Format all code
just clippy   # Run linter
just test     # Run all tests
```
