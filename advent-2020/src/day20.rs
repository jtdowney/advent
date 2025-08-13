use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::{Result, anyhow};

#[derive(Debug, Clone)]
struct Tile {
    id: u64,
    data: Vec<Vec<char>>,
}

impl Tile {
    fn borders(&self) -> [String; 4] {
        let top = self.data[0].iter().collect();
        let bottom = self.data.last().unwrap().iter().collect();
        let left = self.data.iter().map(|row| row[0]).collect();
        let right = self.data.iter().map(|row| row.last().unwrap()).collect();
        [top, right, bottom, left]
    }

    fn all_possible_borders(&self) -> Vec<String> {
        self.borders()
            .into_iter()
            .flat_map(|border| vec![border.clone(), border.chars().rev().collect()])
            .collect()
    }

    fn rotate(&self) -> Self {
        let n = self.data.len();
        let data = (0..n)
            .map(|j| (0..n).map(|i| self.data[n - 1 - i][j]).collect())
            .collect();

        Tile { id: self.id, data }
    }

    fn flip_horizontal(&self) -> Self {
        let data = self
            .data
            .iter()
            .map(|row| row.iter().rev().copied().collect())
            .collect();

        Tile { id: self.id, data }
    }

    fn all_orientations(&self) -> Vec<Self> {
        (0..4)
            .flat_map(|rotations| {
                let rotated = (0..rotations).fold(self.clone(), |acc, _| acc.rotate());
                vec![rotated.clone(), rotated.flip_horizontal()]
            })
            .collect()
    }

    fn remove_borders(&self) -> Vec<Vec<char>> {
        self.data[1..self.data.len() - 1]
            .iter()
            .map(|row| row[1..row.len() - 1].to_vec())
            .collect()
    }
}

impl FromStr for Tile {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.lines();
        let header = lines.next().ok_or_else(|| anyhow!("Missing tile header"))?;

        let id = header
            .strip_prefix("Tile ")
            .and_then(|s| s.strip_suffix(":"))
            .and_then(|s| s.parse().ok())
            .ok_or_else(|| anyhow!("Invalid tile header: {}", header))?;

        let data = lines.map(|line| line.chars().collect()).collect();

        Ok(Tile { id, data })
    }
}

#[aoc_generator(day20)]
fn generator(input: &str) -> Result<Vec<Tile>> {
    input
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(str::parse)
        .collect()
}

#[aoc(day20, part1)]
fn part1(tiles: &[Tile]) -> Result<u64> {
    let border_matches = tiles
        .iter()
        .map(|tile| {
            let borders = tile.all_possible_borders();
            let matches = tiles
                .iter()
                .filter(|other| other.id != tile.id)
                .filter(|other| {
                    let other_borders = other.all_possible_borders();
                    borders.iter().any(|b| other_borders.contains(b))
                })
                .count();
            (tile.id, matches)
        })
        .collect::<HashMap<_, _>>();

    let corners: Vec<u64> = border_matches
        .into_iter()
        .filter(|(_, matches)| *matches == 2)
        .map(|(id, _)| id)
        .collect();

    if corners.len() != 4 {
        return Err(anyhow!("Expected 4 corners, found {}", corners.len()));
    }

    Ok(corners.iter().product())
}

fn find_matching_tiles(tiles: &[Tile]) -> HashMap<u64, Vec<u64>> {
    tiles
        .iter()
        .map(|tile| {
            let borders = tile.all_possible_borders();
            let matching_ids = tiles
                .iter()
                .filter(|other| other.id != tile.id)
                .filter(|other| {
                    let other_borders = other.all_possible_borders();
                    borders.iter().any(|b| other_borders.contains(b))
                })
                .map(|other| other.id)
                .collect();
            (tile.id, matching_ids)
        })
        .collect()
}

fn assemble_grid(tiles: &[Tile]) -> Result<Vec<Vec<Tile>>> {
    let matches = find_matching_tiles(tiles);
    let tile_map: HashMap<u64, &Tile> = tiles.iter().map(|t| (t.id, t)).collect();

    let corners: Vec<u64> = matches
        .iter()
        .filter(|(_, m)| m.len() == 2)
        .map(|(id, _)| *id)
        .collect();

    let grid_size = (tiles.len() as f64).sqrt() as usize;
    let mut grid = vec![vec![None; grid_size]; grid_size];
    let mut used = HashSet::new();

    let first_corner = corners[0];
    let first_tile = tile_map[&first_corner];

    for orientation in first_tile.all_orientations() {
        grid[0][0] = Some(orientation.clone());
        used.insert(first_corner);

        if fill_grid(&mut grid, &matches, &tile_map, &mut used, 0, 1) {
            break;
        }

        used.clear();
    }

    let result: Vec<Vec<Tile>> = grid
        .into_iter()
        .map(|row| row.into_iter().map(|t| t.unwrap()).collect())
        .collect();

    Ok(result)
}

fn fill_grid(
    grid: &mut Vec<Vec<Option<Tile>>>,
    matches: &HashMap<u64, Vec<u64>>,
    tile_map: &HashMap<u64, &Tile>,
    used: &mut HashSet<u64>,
    mut row: usize,
    mut col: usize,
) -> bool {
    let grid_size = grid.len();

    if col >= grid_size {
        col = 0;
        row += 1;
    }

    if row >= grid_size {
        return true;
    }

    let required_neighbors = get_required_neighbors(grid, row, col);

    for (tile_id, neighbors) in matches {
        if used.contains(tile_id) {
            continue;
        }

        let has_required_neighbors = required_neighbors.iter().all(|req| neighbors.contains(req));

        if !has_required_neighbors {
            continue;
        }

        let tile = tile_map[tile_id];

        for orientation in tile.all_orientations() {
            if fits_in_grid(grid, &orientation, row, col) {
                grid[row][col] = Some(orientation);
                used.insert(*tile_id);

                if fill_grid(grid, matches, tile_map, used, row, col + 1) {
                    return true;
                }

                used.remove(tile_id);
            }
        }
    }

    grid[row][col] = None;
    false
}

fn get_required_neighbors(grid: &[Vec<Option<Tile>>], row: usize, col: usize) -> Vec<u64> {
    let mut neighbors = vec![];

    if row > 0
        && let Some(tile) = &grid[row - 1][col]
    {
        neighbors.push(tile.id);
    }

    if col > 0
        && let Some(tile) = &grid[row][col - 1]
    {
        neighbors.push(tile.id);
    }

    neighbors
}

fn fits_in_grid(grid: &[Vec<Option<Tile>>], tile: &Tile, row: usize, col: usize) -> bool {
    if row > 0
        && let Some(above) = &grid[row - 1][col]
    {
        let above_bottom: String = above.data.last().unwrap().iter().collect();
        let tile_top: String = tile.data[0].iter().collect();
        if above_bottom != tile_top {
            return false;
        }
    }

    if col > 0
        && let Some(left) = &grid[row][col - 1]
    {
        let left_right: String = left.data.iter().map(|row| row.last().unwrap()).collect();
        let tile_left: String = tile.data.iter().map(|row| row[0]).collect();
        if left_right != tile_left {
            return false;
        }
    }

    true
}

fn create_image(grid: &[Vec<Tile>]) -> Vec<Vec<char>> {
    grid.iter()
        .flat_map(|grid_row| {
            let inner_tiles: Vec<_> = grid_row.iter().map(|tile| tile.remove_borders()).collect();
            (0..inner_tiles[0].len()).map(move |row_idx| {
                inner_tiles
                    .iter()
                    .flat_map(|tile| tile[row_idx].iter().copied())
                    .collect()
            })
        })
        .collect()
}

fn rotate_image(image: &[Vec<char>]) -> Vec<Vec<char>> {
    let n = image.len();
    (0..n)
        .map(|j| (0..n).map(|i| image[n - 1 - i][j]).collect())
        .collect()
}

fn flip_image(image: &[Vec<char>]) -> Vec<Vec<char>> {
    image
        .iter()
        .map(|row| row.iter().rev().copied().collect())
        .collect()
}

fn find_sea_monsters(image: &[Vec<char>]) -> HashSet<(usize, usize)> {
    let monster = [
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   ",
    ];

    let monster_positions: Vec<(usize, usize)> = monster
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.chars()
                .enumerate()
                .filter(|(_, ch)| *ch == '#')
                .map(move |(j, _)| (i, j))
        })
        .collect();

    let mut found_positions = HashSet::new();
    let height = image.len();
    let width = image[0].len();
    let monster_height = monster.len();
    let monster_width = monster[0].len();

    for i in 0..=height.saturating_sub(monster_height) {
        for j in 0..=width.saturating_sub(monster_width) {
            let is_monster = monster_positions
                .iter()
                .all(|(di, dj)| image[i + di][j + dj] == '#');

            if is_monster {
                for (di, dj) in &monster_positions {
                    found_positions.insert((i + di, j + dj));
                }
            }
        }
    }

    found_positions
}

#[aoc(day20, part2)]
fn part2(tiles: &[Tile]) -> Result<usize> {
    let grid = assemble_grid(tiles)?;
    let mut image = create_image(&grid);

    let mut max_monsters = 0;

    for _ in 0..4 {
        let monsters = find_sea_monsters(&image);
        max_monsters = max_monsters.max(monsters.len());

        let flipped = flip_image(&image);
        let flipped_monsters = find_sea_monsters(&flipped);
        max_monsters = max_monsters.max(flipped_monsters.len());

        image = rotate_image(&image);
    }

    let total_hashes = image
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&ch| ch == '#')
        .count();

    Ok(total_hashes - max_monsters)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";

    #[test]
    fn test_part1() {
        let tiles = generator(EXAMPLE).unwrap();
        assert_eq!(part1(&tiles).unwrap(), 20899048083289);
    }

    #[test]
    fn test_part2() {
        let tiles = generator(EXAMPLE).unwrap();
        assert_eq!(part2(&tiles).unwrap(), 273);
    }
}
