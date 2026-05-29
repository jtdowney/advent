use std::{collections::BTreeSet, str::FromStr};

use anyhow::anyhow;
use aoc_runner_derive::{aoc, aoc_generator};
use winnow::{
    ascii::{dec_uint, line_ending, space1},
    combinator::{repeat, separated, seq, terminated},
    prelude::*,
    token::take_while,
};

type Point = (i32, i32);
type Cells = BTreeSet<Point>;
type Placement = Vec<usize>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Shape {
    cells: Cells,
}

impl Shape {
    fn rotate_90(&self) -> Self {
        self.cells.iter().map(|(x, y)| (*y, -x)).collect()
    }

    fn reflect(&self) -> Self {
        self.cells.iter().map(|(x, y)| (-x, *y)).collect()
    }

    fn variants(&self) -> Vec<Self> {
        (0..4)
            .scan(self.clone(), |current, _| {
                let result = current.clone();
                *current = current.rotate_90();
                Some(result)
            })
            .flat_map(|rotated| [rotated.clone(), rotated.reflect()])
            .fold(Vec::with_capacity(8), |mut acc, variant| {
                if !acc.contains(&variant) {
                    acc.push(variant);
                }
                acc
            })
    }

    fn placements(&self, width: usize, height: usize) -> Vec<Placement> {
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        let (width_i32, height_i32) = (width as i32, height as i32);

        let mut placements = self
            .variants()
            .iter()
            .flat_map(|variant| {
                let max_x = variant
                    .cells
                    .iter()
                    .map(|(x, _)| *x)
                    .max()
                    .unwrap_or_default();
                let max_y = variant
                    .cells
                    .iter()
                    .map(|(_, y)| *y)
                    .max()
                    .unwrap_or_default();

                (0..=(height_i32 - 1 - max_y)).flat_map(move |y| {
                    (0..=(width_i32 - 1 - max_x)).map(move |x| {
                        variant
                            .cells
                            .iter()
                            .map(|(dx, dy)| {
                                #[allow(clippy::cast_sign_loss)]
                                {
                                    (y + dy) as usize * width + (x + dx) as usize
                                }
                            })
                            .collect::<Vec<_>>()
                    })
                })
            })
            .collect::<Vec<_>>();

        placements.sort_by_key(|cells| cells.iter().copied().min().unwrap_or_default());
        placements
    }
}

impl FromIterator<Point> for Shape {
    fn from_iter<T: IntoIterator<Item = Point>>(iter: T) -> Self {
        let cells = iter.into_iter().collect::<Cells>();
        let min_x = cells.iter().map(|(x, _)| *x).min().unwrap_or_default();
        let min_y = cells.iter().map(|(_, y)| *y).min().unwrap_or_default();
        let cells = cells.iter().map(|(x, y)| (x - min_x, y - min_y)).collect();
        Self { cells }
    }
}

fn backtrack_search(shape_placements: &[(usize, Vec<Placement>)], board: &mut [bool]) -> bool {
    fn place_copies(
        remaining: usize,
        start_idx: usize,
        placements: &[Placement],
        rest: &[(usize, Vec<Placement>)],
        board: &mut [bool],
    ) -> bool {
        if remaining == 0 {
            return backtrack_search(rest, board);
        }

        for i in start_idx..placements.len() {
            let cells = &placements[i];
            if cells.iter().all(|&idx| !board[idx]) {
                for &idx in cells {
                    board[idx] = true;
                }

                if place_copies(remaining - 1, i + 1, placements, rest, board) {
                    return true;
                }

                for &idx in cells {
                    board[idx] = false;
                }
            }
        }

        false
    }

    let Some(((qty, placements), rest)) = shape_placements.split_first() else {
        return true;
    };

    place_copies(*qty, 0, placements, rest, board)
}

struct Region {
    width: usize,
    height: usize,
    quantities: Vec<usize>,
}

impl Region {
    fn can_fit(&self, shapes: &[Shape]) -> bool {
        let grid_size = self.width * self.height;
        let total_cells_needed = self
            .quantities
            .iter()
            .zip(shapes.iter())
            .map(|(&qty, shape)| qty * shape.cells.len())
            .sum::<usize>();

        if total_cells_needed > grid_size {
            return false;
        }

        // return true - YOLO
        self.can_fit_packing(shapes)
    }

    fn can_fit_packing(&self, shapes: &[Shape]) -> bool {
        let shape_placements = self
            .quantities
            .iter()
            .zip(shapes.iter())
            .filter(|&(&quantity, _)| quantity > 0)
            .try_fold(vec![], |mut acc, (&qty, shape)| {
                let placements = shape.placements(self.width, self.height);
                if placements.is_empty() {
                    return None;
                }

                acc.push((qty, placements));
                Some(acc)
            });

        let Some(mut shape_placements) = shape_placements else {
            return false;
        };

        shape_placements.sort_by_key(|(quantity, placements)| placements.len() / quantity);

        let mut board = vec![false; self.width * self.height];
        backtrack_search(&shape_placements, &mut board)
    }
}

struct Input {
    shapes: Vec<Shape>,
    regions: Vec<Region>,
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn shape(input: &mut &str) -> winnow::Result<Shape> {
            let _index: usize = terminated(dec_uint, (":", line_ending)).parse_next(input)?;
            let lines: Vec<&str> =
                separated(1.., take_while(1.., |c| c == '#' || c == '.'), line_ending)
                    .parse_next(input)?;

            let cells = lines
                .iter()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .filter(|&(_, c)| c == '#')
                        .map(move |(x, _)| {
                            #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
                            (x as i32, y as i32)
                        })
                })
                .collect();

            Ok(Shape { cells })
        }

        fn region(input: &mut &str) -> winnow::Result<Region> {
            seq!(Region {
                width: dec_uint,
                _: "x",
                height: dec_uint,
                _: ":",
                _: space1,
                quantities: separated(1.., dec_uint::<_, usize, _>, space1),
            })
            .parse_next(input)
        }

        fn input(input: &mut &str) -> winnow::Result<Input> {
            let shapes: Vec<Shape> =
                separated(1.., shape, (line_ending, line_ending)).parse_next(input)?;
            let _: () = repeat(1.., line_ending).parse_next(input)?;
            let regions: Vec<Region> = separated(1.., region, line_ending).parse_next(input)?;

            Ok(Input { shapes, regions })
        }

        input
            .parse(s)
            .map_err(|e| anyhow!("error parsing input:\n{e}"))
    }
}

#[aoc_generator(day12)]
fn generator(input: &str) -> anyhow::Result<Input> {
    input.parse()
}

#[aoc(day12, part1)]
fn part1(input: &Input) -> usize {
    input
        .regions
        .iter()
        .filter(|region| region.can_fit(&input.shapes))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

    #[test]
    fn test_parse_shapes() {
        let input = generator(EXAMPLE).unwrap();

        assert_eq!(input.shapes.len(), 6);

        let shape0 = [(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (0, 2), (1, 2)]
            .into_iter()
            .collect::<Cells>();
        assert_eq!(input.shapes[0].cells, shape0);

        let shape4 = [(0, 0), (1, 0), (2, 0), (0, 1), (0, 2), (1, 2), (2, 2)]
            .into_iter()
            .collect::<Cells>();
        assert_eq!(input.shapes[4].cells, shape4);
    }

    #[test]
    fn test_parse_regions() {
        let input = generator(EXAMPLE).unwrap();

        assert_eq!(input.regions.len(), 3);
        assert_eq!(input.regions[0].width, 4);
        assert_eq!(input.regions[0].height, 4);
        assert_eq!(input.regions[0].quantities, vec![0, 0, 0, 0, 2, 0]);
        assert_eq!(input.regions[1].width, 12);
        assert_eq!(input.regions[1].height, 5);
        assert_eq!(input.regions[1].quantities, vec![1, 0, 1, 0, 2, 2]);
        assert_eq!(input.regions[2].width, 12);
        assert_eq!(input.regions[2].height, 5);
        assert_eq!(input.regions[2].quantities, vec![1, 0, 1, 0, 3, 2]);
    }

    #[test]
    fn test_variants() {
        let l_shape = Shape {
            cells: [(0, 0), (0, 1), (1, 1)].into_iter().collect(),
        };

        let variants = l_shape.variants();
        assert_eq!(variants.len(), 4);

        for variant in &variants {
            assert_eq!(variant.cells.len(), 3);
        }
    }

    #[test]
    fn test_variants_symmetric() {
        let line = Shape {
            cells: [(0, 0), (1, 0)].into_iter().collect(),
        };

        assert_eq!(line.variants().len(), 2);
    }

    #[test]
    fn test_can_fit_example_regions() {
        let input = generator(EXAMPLE).unwrap();

        assert!(input.regions[0].can_fit(&input.shapes));
        assert!(input.regions[1].can_fit(&input.shapes));
        assert!(!input.regions[2].can_fit(&input.shapes));
    }

    #[test]
    fn test_part1() {
        let input = generator(EXAMPLE).unwrap();
        assert_eq!(part1(&input), 2);
    }
}
