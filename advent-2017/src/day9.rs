use aoc_runner_derive::aoc;

#[derive(Debug, Default, Clone, Copy)]
struct State {
    score: usize,
    depth: usize,
    ignore: bool,
    garbage: bool,
}

#[aoc(day9, part1)]
fn part1(input: &str) -> usize {
    let state = input.chars().fold(State::default(), |mut state, c| {
        if state.ignore {
            state.ignore = false;
            return state;
        }

        if state.garbage && (c != '>' && c != '!') {
            return state;
        }

        match c {
            '{' => {
                state.depth += 1;
            }
            '}' => {
                state.score += state.depth;
                state.depth -= 1;
            }
            '<' => {
                state.garbage = true;
            }
            '>' => {
                state.garbage = false;
            }
            '!' => {
                state.ignore = true;
            }
            _ => {}
        };

        state
    });

    state.score
}

#[aoc(day9, part2)]
fn part2(input: &str) -> usize {
    let state = input.chars().fold(State::default(), |mut state, c| {
        if state.ignore {
            state.ignore = false;
            return state;
        }

        if state.garbage && (c != '>' && c != '!') {
            state.score += 1;
            return state;
        }

        match c {
            '<' => {
                state.garbage = true;
            }
            '>' => {
                state.garbage = false;
            }
            '!' => {
                state.ignore = true;
            }
            _ => {}
        };

        state
    });

    state.score
}
