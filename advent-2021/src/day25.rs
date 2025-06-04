use anyhow::bail;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, PartialEq)]
struct Grid {
    width: usize,
    height: usize,
    east: Vec<Vec<bool>>,
    south: Vec<Vec<bool>>,
}

impl Grid {
    fn step(&mut self) -> bool {
        let east_moves: Vec<_> = (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| (y, x)))
            .filter(|&(y, x)| self.east[y][x])
            .map(|(y, x)| {
                let next_x = (x + 1) % self.width;
                let can_move = !self.east[y][next_x] && !self.south[y][next_x];
                (
                    (y, x),
                    if can_move { (y, next_x) } else { (y, x) },
                    can_move,
                )
            })
            .collect();

        let moved_east = east_moves.iter().any(|&(_, _, moved)| moved);

        let mut new_east = vec![vec![false; self.width]; self.height];
        for (_, (y, x), _) in &east_moves {
            new_east[*y][*x] = true;
        }
        self.east = new_east;

        let south_moves: Vec<_> = (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| (y, x)))
            .filter(|&(y, x)| self.south[y][x])
            .map(|(y, x)| {
                let next_y = (y + 1) % self.height;
                let can_move = !self.east[next_y][x] && !self.south[next_y][x];
                (
                    (y, x),
                    if can_move { (next_y, x) } else { (y, x) },
                    can_move,
                )
            })
            .collect();

        let moved_south = south_moves.iter().any(|&(_, _, moved)| moved);

        let mut new_south = vec![vec![false; self.width]; self.height];
        for (_, (y, x), _) in &south_moves {
            new_south[*y][*x] = true;
        }
        self.south = new_south;

        moved_east || moved_south
    }
}

#[aoc_generator(day25)]
fn generator(input: &str) -> anyhow::Result<Grid> {
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let width = lines.first().map(|l| l.len()).unwrap_or(0);

    let positions = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, ch)| match ch {
                '>' => Ok((y, x, true, false)),
                'v' => Ok((y, x, false, true)),
                '.' => Ok((y, x, false, false)),
                _ => bail!("Invalid character: {}", ch),
            })
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    let mut east = vec![vec![false; width]; height];
    let mut south = vec![vec![false; width]; height];

    for (y, x, is_east, is_south) in positions {
        if is_east {
            east[y][x] = true;
        }
        if is_south {
            south[y][x] = true;
        }
    }

    Ok(Grid {
        width,
        height,
        east,
        south,
    })
}

#[aoc(day25, part1)]
fn part1(grid: &Grid) -> Option<usize> {
    let mut grid = grid.clone();
    (1..).find(|_| !grid.step())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>"#;

    #[test]
    fn test_parse() {
        let grid = generator(EXAMPLE).unwrap();
        assert_eq!(grid.width, 10);
        assert_eq!(grid.height, 9);
        assert!(grid.south[0][0]);
        assert!(!grid.east[0][0]);
        assert!(grid.east[0][4]);
        assert!(!grid.south[0][4]);
    }

    #[test]
    fn test_single_step() {
        let mut grid = generator(EXAMPLE).unwrap();
        let moved = grid.step();
        assert!(moved);

        let expected = generator(
            r#"....>.>v.>
v.v>.>v.v.
>v>>..>v..
>>v>v>.>.v
.>v.v...v.
v>>.>vvv..
..v...>>..
vv...>>vv.
>.v.v..v.v"#,
        )
        .unwrap();

        assert_eq!(grid, expected);
    }

    #[test]
    fn test_part1() {
        let grid = generator(EXAMPLE).unwrap();
        assert_eq!(part1(&grid).unwrap(), 58);
    }
}
