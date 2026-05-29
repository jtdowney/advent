use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Point = (i64, i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Machine {
    button_a: Point,
    button_b: Point,
    prize: Point,
}

#[aoc_generator(day13)]
fn generator(input: &str) -> Option<Vec<Machine>> {
    input
        .split("\n\n")
        .map(|block| {
            let mut lines = block.lines();
            Some(Machine {
                button_a: parse_coords(lines.next()?)?,
                button_b: parse_coords(lines.next()?)?,
                prize: parse_coords(lines.next()?)?,
            })
        })
        .collect()
}

fn parse_coords(line: &str) -> Option<Point> {
    let (_, coords) = line.split_once(": ")?;
    let (x, y) = coords.split_once(", ")?;
    Some((x[2..].parse().ok()?, y[2..].parse().ok()?))
}

#[aoc(day13, part1)]
fn part1(machines: &[Machine]) -> i64 {
    machines
        .iter()
        .filter_map(|m| solve_with_limit(m, 100))
        .sum()
}

#[aoc(day13, part2)]
fn part2(machines: &[Machine]) -> i64 {
    const OFFSET: i64 = 10_000_000_000_000;
    machines
        .iter()
        .filter_map(|m| {
            solve_linear_system(
                m.button_a,
                m.button_b,
                (m.prize.0 + OFFSET, m.prize.1 + OFFSET),
            )
        })
        .sum()
}

fn solve_with_limit(machine: &Machine, limit: i64) -> Option<i64> {
    (0..=limit)
        .cartesian_product(0..=limit)
        .filter(|&(a, b)| {
            let (ax, ay) = machine.button_a;
            let (bx, by) = machine.button_b;
            let (px, py) = machine.prize;
            a * ax + b * bx == px && a * ay + b * by == py
        })
        .map(|(a, b)| a * 3 + b)
        .min()
}

fn solve_linear_system((ax, ay): Point, (bx, by): Point, (px, py): Point) -> Option<i64> {
    let det = ax * by - ay * bx;
    if det == 0 {
        return None;
    }

    let a_num = px * by - py * bx;
    let b_num = ax * py - ay * px;

    if a_num % det != 0 || b_num % det != 0 {
        return None;
    }

    let a = a_num / det;
    let b = b_num / det;

    (a >= 0 && b >= 0).then_some(a * 3 + b)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_generator() {
        let machines = generator(EXAMPLE).unwrap();
        assert_eq!(machines.len(), 4);

        assert_eq!(machines[0].button_a, (94, 34));
        assert_eq!(machines[0].button_b, (22, 67));
        assert_eq!(machines[0].prize, (8400, 5400));

        assert_eq!(machines[3].button_a, (69, 23));
        assert_eq!(machines[3].button_b, (27, 71));
        assert_eq!(machines[3].prize, (18641, 10279));
    }

    #[test]
    fn test_solve_with_limit() {
        let machine = Machine {
            button_a: (94, 34),
            button_b: (22, 67),
            prize: (8400, 5400),
        };
        assert_eq!(solve_with_limit(&machine, 100), Some(280));

        let machine = Machine {
            button_a: (26, 66),
            button_b: (67, 21),
            prize: (12748, 12176),
        };
        assert_eq!(solve_with_limit(&machine, 100), None);

        let machine = Machine {
            button_a: (17, 86),
            button_b: (84, 37),
            prize: (7870, 6450),
        };
        assert_eq!(solve_with_limit(&machine, 100), Some(200));
    }

    #[test]
    fn test_solve_linear_system() {
        assert_eq!(
            solve_linear_system((94, 34), (22, 67), (8400, 5400)),
            Some(280)
        );
    }

    #[test]
    fn test_part1() {
        let machines = generator(EXAMPLE).unwrap();
        assert_eq!(part1(&machines), 480);
    }

    #[test]
    fn test_part2() {
        let machines = generator(EXAMPLE).unwrap();
        assert!(part2(&machines) > 0);
    }
}
