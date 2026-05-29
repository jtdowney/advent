use std::collections::HashSet;

use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct HexCoord {
    q: i32,
    r: i32,
}

impl HexCoord {
    fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }

    fn move_direction(&self, dir: &str) -> Self {
        match dir {
            "e" => Self::new(self.q + 1, self.r),
            "w" => Self::new(self.q - 1, self.r),
            "ne" => Self::new(self.q + 1, self.r - 1),
            "nw" => Self::new(self.q, self.r - 1),
            "se" => Self::new(self.q, self.r + 1),
            "sw" => Self::new(self.q - 1, self.r + 1),
            _ => panic!("Invalid direction: {}", dir),
        }
    }

    fn neighbors(&self) -> Vec<HexCoord> {
        vec![
            Self::new(self.q + 1, self.r),
            Self::new(self.q - 1, self.r),
            Self::new(self.q + 1, self.r - 1),
            Self::new(self.q, self.r - 1),
            Self::new(self.q, self.r + 1),
            Self::new(self.q - 1, self.r + 1),
        ]
    }
}

fn parse_directions(line: &str) -> Vec<String> {
    let chars: Vec<char> = line.chars().collect();
    (0..chars.len())
        .scan(0, |pos, _| {
            if *pos >= chars.len() {
                return None;
            }

            match chars[*pos] {
                'e' | 'w' => {
                    let d = chars[*pos].to_string();
                    *pos += 1;
                    Some(d)
                }
                's' | 'n' if *pos + 1 < chars.len() => {
                    let d = format!("{}{}", chars[*pos], chars[*pos + 1]);
                    *pos += 2;
                    Some(d)
                }
                _ => None,
            }
        })
        .collect()
}

#[aoc_generator(day24)]
fn generator(input: &str) -> Result<Vec<Vec<String>>> {
    Ok(input
        .lines()
        .map(|line| parse_directions(line.trim()))
        .collect())
}

fn get_initial_black_tiles(directions_list: &[Vec<String>]) -> HashSet<HexCoord> {
    directions_list
        .iter()
        .map(|directions| {
            directions
                .iter()
                .fold(HexCoord::new(0, 0), |pos, dir| pos.move_direction(dir))
        })
        .fold(HashSet::new(), |mut tiles, pos| {
            if tiles.contains(&pos) {
                tiles.remove(&pos);
            } else {
                tiles.insert(pos);
            }
            tiles
        })
}

#[aoc(day24, part1)]
fn part1(directions_list: &[Vec<String>]) -> usize {
    get_initial_black_tiles(directions_list).len()
}

fn simulate_day(black_tiles: &HashSet<HexCoord>) -> HashSet<HexCoord> {
    black_tiles
        .iter()
        .flat_map(|tile| std::iter::once(*tile).chain(tile.neighbors()))
        .collect::<HashSet<_>>()
        .into_iter()
        .filter(|tile| {
            let black_neighbors = tile
                .neighbors()
                .into_iter()
                .filter(|n| black_tiles.contains(n))
                .count();

            matches!(
                (black_tiles.contains(tile), black_neighbors),
                (true, 1) | (true, 2) | (false, 2)
            )
        })
        .collect()
}

#[aoc(day24, part2)]
fn part2(directions_list: &[Vec<String>]) -> usize {
    (0..100)
        .fold(get_initial_black_tiles(directions_list), |tiles, _| {
            simulate_day(&tiles)
        })
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_directions() {
        assert_eq!(parse_directions("esew"), vec!["e", "se", "w"]);
        assert_eq!(parse_directions("nwwswee"), vec!["nw", "w", "sw", "e", "e"]);
    }

    #[test]
    fn test_hex_coord_movement() {
        let start = HexCoord::new(0, 0);

        let east = start.move_direction("e");
        assert_eq!(east, HexCoord::new(1, 0));

        let west = start.move_direction("w");
        assert_eq!(west, HexCoord::new(-1, 0));

        let ne = start.move_direction("ne");
        assert_eq!(ne, HexCoord::new(1, -1));

        let nw = start.move_direction("nw");
        assert_eq!(nw, HexCoord::new(0, -1));

        let se = start.move_direction("se");
        assert_eq!(se, HexCoord::new(0, 1));

        let sw = start.move_direction("sw");
        assert_eq!(sw, HexCoord::new(-1, 1));
    }

    #[test]
    fn test_tile_navigation() {
        let directions = parse_directions("esew");
        let position = directions
            .iter()
            .fold(HexCoord::new(0, 0), |pos, dir| pos.move_direction(dir));
        assert_eq!(position, HexCoord::new(0, 1));

        let directions = parse_directions("nwwswee");
        let position = directions
            .iter()
            .fold(HexCoord::new(0, 0), |pos, dir| pos.move_direction(dir));
        assert_eq!(position, HexCoord::new(0, 0));
    }

    #[test]
    fn test_part1_example() {
        let input = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

        let directions_list = generator(input).unwrap();
        assert_eq!(part1(&directions_list), 10);
    }

    #[test]
    fn test_hex_neighbors() {
        let tile = HexCoord::new(0, 0);
        let neighbors = tile.neighbors();
        assert_eq!(neighbors.len(), 6);
        assert!(neighbors.contains(&HexCoord::new(1, 0)));
        assert!(neighbors.contains(&HexCoord::new(-1, 0)));
        assert!(neighbors.contains(&HexCoord::new(1, -1)));
        assert!(neighbors.contains(&HexCoord::new(0, -1)));
        assert!(neighbors.contains(&HexCoord::new(0, 1)));
        assert!(neighbors.contains(&HexCoord::new(-1, 1)));
    }

    #[test]
    fn test_simulate_days() {
        let input = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

        let directions_list = generator(input).unwrap();
        let mut black_tiles = get_initial_black_tiles(&directions_list);

        assert_eq!(black_tiles.len(), 10);

        black_tiles = simulate_day(&black_tiles);
        assert_eq!(black_tiles.len(), 15);

        black_tiles = simulate_day(&black_tiles);
        assert_eq!(black_tiles.len(), 12);

        black_tiles = simulate_day(&black_tiles);
        assert_eq!(black_tiles.len(), 25);
    }

    #[test]
    fn test_part2_example() {
        let input = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

        let directions_list = generator(input).unwrap();
        assert_eq!(part2(&directions_list), 2208);
    }
}
