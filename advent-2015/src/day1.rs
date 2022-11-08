use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
fn part1(input: &str) -> isize {
    input.chars().fold(0, |acc, c| match c {
	'(' => acc + 1,
	')' => acc - 1,
	_ => unreachable!(),
    })
}

#[aoc(day1, part2)]
fn part2(input: &str) -> usize {
    input
	.chars()
	.scan(0, |state, c| {
	    if *state < 0 {
		return None;
	    }

	    *state = match c {
		'(' => *state + 1,
		')' => *state - 1,
		_ => unreachable!(),
	    };

	    Some(*state)
	})
	.count()
}
