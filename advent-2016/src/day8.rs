use anyhow::anyhow;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{Itertools, iproduct};
use nom::{Finish, IResult, Parser};

const WIDTH: usize = 50;
const HEIGHT: usize = 6;

enum Instruction {
    Rectangle(usize, usize),
    RotateRow(usize, usize),
    RotateColumn(usize, usize),
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    use nom::{
        branch::alt, bytes::complete::tag, character::complete::u8, combinator::map,
        sequence::preceded,
    };

    let rectangle = map(
        preceded(tag("rect "), |input| {
            let (input, x) = u8(input)?;
            let (input, _) = tag("x")(input)?;
            let (input, y) = u8(input)?;
            Ok((input, (x, y)))
        }),
        |(x, y)| Instruction::Rectangle(x as usize, y as usize),
    );
    let rotate_row = map(
        preceded(tag("rotate row y="), |input| {
            let (input, x) = u8(input)?;
            let (input, _) = tag(" by ")(input)?;
            let (input, y) = u8(input)?;
            Ok((input, (x, y)))
        }),
        |(x, y)| Instruction::RotateRow(x as usize, y as usize),
    );
    let rotate_column = map(
        preceded(tag("rotate column x="), |input| {
            let (input, x) = u8(input)?;
            let (input, _) = tag(" by ")(input)?;
            let (input, y) = u8(input)?;
            Ok((input, (x, y)))
        }),
        |(x, y)| Instruction::RotateColumn(x as usize, y as usize),
    );

    alt((rectangle, rotate_row, rotate_column)).parse(input)
}

#[aoc_generator(day8)]
fn generator(input: &str) -> anyhow::Result<Vec<Instruction>> {
    input
        .lines()
        .map(|line| {
            instruction
                .parse(line)
                .finish()
                .map(|(_, i)| i)
                .map_err(|_| anyhow!("Parse error: {}", line))
        })
        .collect()
}

fn render_screen(instructions: &[Instruction]) -> [[bool; WIDTH]; HEIGHT] {
    instructions
        .iter()
        .fold([[false; WIDTH]; HEIGHT], |mut screen, instruction| {
            match instruction {
                Instruction::Rectangle(x, y) => {
                    for (i, j) in iproduct!(0..*x, 0..*y) {
                        screen[j][i] = true;
                    }
                }
                Instruction::RotateRow(y, n) => {
                    let mut row = [false; WIDTH];
                    for i in 0..WIDTH {
                        row[(i + n) % WIDTH] = screen[*y][i];
                    }
                    screen[*y] = row;
                }
                Instruction::RotateColumn(x, n) => {
                    let mut column = [false; HEIGHT];
                    for i in 0..HEIGHT {
                        column[(i + n) % HEIGHT] = screen[i][*x];
                    }
                    for i in 0..HEIGHT {
                        screen[i][*x] = column[i];
                    }
                }
            }
            screen
        })
}

#[aoc(day8, part1)]
fn part1(input: &[Instruction]) -> usize {
    let screen = render_screen(input);
    screen.into_iter().flatten().filter(|&b| b).count()
}

#[aoc(day8, part2)]
fn part2(input: &[Instruction]) -> String {
    let screen = render_screen(input)
        .iter()
        .map(|row| {
            row.iter()
                .map(|&b| if b { '#' } else { '.' })
                .collect::<String>()
        })
        .join("\n");

    advent_of_code_ocr::parse_string_to_letters(&screen)
}
