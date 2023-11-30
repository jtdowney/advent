use num_integer::Integer;

struct State {
    start_time: i64,
    busses: Vec<(i64, i64)>,
}

#[aoc_generator(day13)]
fn generator(input: &str) -> State {
    let mut lines = input.lines();
    let start_time = lines.next().and_then(|l| l.parse().ok()).unwrap();
    let busses = lines
        .next()
        .map(|l| {
            l.split(',')
                .enumerate()
                .filter_map(|(i, n)| n.parse().ok().map(|n| (i as i64, n)))
                .collect::<Vec<(i64, i64)>>()
        })
        .unwrap();

    State { start_time, busses }
}

#[aoc(day13, part1)]
fn part1(state: &State) -> i64 {
    let (bus, end_time) = (state.start_time..)
        .find_map(|t| {
            state
                .busses
                .iter()
                .copied()
                .find(|&(_, b)| t.gcd(&b) != 1)
                .map(|(_, b)| (b, t))
        })
        .unwrap();

    (end_time - state.start_time) * bus
}

#[aoc(day13, part2)]
fn part2(state: &State) -> i64 {
    let product: i64 = state.busses.iter().map(|(_, b)| b).product();
    state
        .busses
        .iter()
        .map(|(i, b)| (b - i, b))
        .map(|(residue, modulus)| {
            let p = product / modulus;
            let g = p.extended_gcd(modulus);
            residue * (g.x % modulus + modulus) * p
        })
        .sum::<i64>()
        % product
}
