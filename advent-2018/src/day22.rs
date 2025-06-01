use std::collections::{BinaryHeap, HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug, PartialEq)]
struct Cave {
    depth: usize,
    target: (usize, usize),
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum RegionType {
    Rocky,
    Wet,
    Narrow,
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
enum Tool {
    Torch,
    ClimbingGear,
    Neither,
}

struct CaveSystem<'a> {
    cave: &'a Cave,
    erosion_cache: HashMap<(usize, usize), usize>,
}

impl<'a> CaveSystem<'a> {
    fn new(cave: &'a Cave) -> Self {
        Self {
            cave,
            erosion_cache: HashMap::new(),
        }
    }

    fn geologic_index(&mut self, x: usize, y: usize) -> usize {
        let (target_x, target_y) = self.cave.target;

        match (x, y) {
            (0, 0) => 0,
            pos if pos == (target_x, target_y) => 0,
            (x, 0) => x * 16807,
            (0, y) => y * 48271,
            (x, y) => {
                let left_erosion = self.erosion_level(x - 1, y);
                let up_erosion = self.erosion_level(x, y - 1);
                left_erosion * up_erosion
            }
        }
    }

    fn erosion_level(&mut self, x: usize, y: usize) -> usize {
        if let Some(&level) = self.erosion_cache.get(&(x, y)) {
            return level;
        }

        let geologic = self.geologic_index(x, y);
        let erosion = (geologic + self.cave.depth) % 20183;
        self.erosion_cache.insert((x, y), erosion);
        erosion
    }

    fn region_type(&mut self, x: usize, y: usize) -> RegionType {
        match self.erosion_level(x, y) % 3 {
            0 => RegionType::Rocky,
            1 => RegionType::Wet,
            2 => RegionType::Narrow,
            _ => unreachable!(),
        }
    }

    fn risk_level(&mut self, x: usize, y: usize) -> usize {
        match self.region_type(x, y) {
            RegionType::Rocky => 0,
            RegionType::Wet => 1,
            RegionType::Narrow => 2,
        }
    }
}

fn is_tool_valid_for_region(region: RegionType, tool: Tool) -> bool {
    use RegionType::*;
    use Tool::*;

    matches!(
        (region, tool),
        (Rocky, Torch)
            | (Rocky, ClimbingGear)
            | (Wet, ClimbingGear)
            | (Wet, Neither)
            | (Narrow, Torch)
            | (Narrow, Neither)
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    x: usize,
    y: usize,
    tool: Tool,
}

impl State {
    fn new(x: usize, y: usize, tool: Tool) -> Self {
        Self { x, y, tool }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct SearchNode {
    cost: usize,
    state: State,
}

impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(cave: &Cave) -> usize {
    let mut system = CaveSystem::new(cave);
    let (target_x, target_y) = cave.target;

    let start_state = State::new(0, 0, Tool::Torch);
    let target_state = State::new(target_x, target_y, Tool::Torch);

    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();
    let mut distances = HashMap::new();

    heap.push(SearchNode {
        cost: 0,
        state: start_state,
    });
    distances.insert(start_state, 0);

    while let Some(SearchNode { cost, state }) = heap.pop() {
        if state == target_state {
            return cost;
        }

        if !visited.insert(state) {
            continue;
        }

        let current_region = system.region_type(state.x, state.y);

        for new_tool in [Tool::Torch, Tool::ClimbingGear, Tool::Neither] {
            if new_tool != state.tool && is_tool_valid_for_region(current_region, new_tool) {
                let new_state = State::new(state.x, state.y, new_tool);
                let new_cost = cost + 7;

                if new_cost < *distances.get(&new_state).unwrap_or(&usize::MAX) {
                    distances.insert(new_state, new_cost);
                    heap.push(SearchNode {
                        cost: new_cost,
                        state: new_state,
                    });
                }
            }
        }

        let moves = [
            (state.x.wrapping_sub(1), state.y),
            (state.x + 1, state.y),
            (state.x, state.y.wrapping_sub(1)),
            (state.x, state.y + 1),
        ];

        for (new_x, new_y) in moves {
            if new_x == usize::MAX || new_y == usize::MAX {
                continue;
            }

            let new_region = system.region_type(new_x, new_y);
            if is_tool_valid_for_region(new_region, state.tool) {
                let new_state = State::new(new_x, new_y, state.tool);
                let new_cost = cost + 1;

                if new_cost < *distances.get(&new_state).unwrap_or(&usize::MAX) {
                    distances.insert(new_state, new_cost);
                    heap.push(SearchNode {
                        cost: new_cost,
                        state: new_state,
                    });
                }
            }
        }
    }

    usize::MAX
}

#[aoc_generator(day22)]
fn generator(input: &str) -> anyhow::Result<Cave> {
    let mut lines = input.lines();

    let depth = lines
        .next()
        .and_then(|line| line.strip_prefix("depth: "))
        .ok_or_else(|| anyhow::anyhow!("Invalid depth line"))?
        .parse::<usize>()?;

    let (target_x, target_y) = lines
        .next()
        .and_then(|line| line.strip_prefix("target: "))
        .ok_or_else(|| anyhow::anyhow!("Invalid target line"))?
        .split(',')
        .collect_tuple()
        .ok_or_else(|| anyhow::anyhow!("Invalid target format"))?;

    let target = (target_x.parse()?, target_y.parse()?);

    Ok(Cave { depth, target })
}

#[aoc(day22, part1)]
fn part1(input: &Cave) -> usize {
    let mut system = CaveSystem::new(input);
    let (target_x, target_y) = input.target;

    (0..=target_y)
        .cartesian_product(0..=target_x)
        .map(|(y, x)| system.risk_level(x, y))
        .sum()
}

#[aoc(day22, part2)]
fn part2(input: &Cave) -> usize {
    shortest_path(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "depth: 510\ntarget: 10,10";
        let cave = generator(input).unwrap();
        assert_eq!(cave.depth, 510);
        assert_eq!(cave.target, (10, 10));
    }

    #[test]
    fn test_geologic_index_at_mouth() {
        let cave = Cave {
            depth: 510,
            target: (10, 10),
        };
        let mut system = CaveSystem::new(&cave);
        assert_eq!(system.geologic_index(0, 0), 0);
    }

    #[test]
    fn test_geologic_index_at_target() {
        let cave = Cave {
            depth: 510,
            target: (10, 10),
        };
        let mut system = CaveSystem::new(&cave);
        assert_eq!(system.geologic_index(10, 10), 0);
    }

    #[test]
    fn test_geologic_index_y_zero() {
        let cave = Cave {
            depth: 510,
            target: (10, 10),
        };
        let mut system = CaveSystem::new(&cave);
        assert_eq!(system.geologic_index(1, 0), 16807);
        assert_eq!(system.geologic_index(2, 0), 2 * 16807);
    }

    #[test]
    fn test_geologic_index_x_zero() {
        let cave = Cave {
            depth: 510,
            target: (10, 10),
        };
        let mut system = CaveSystem::new(&cave);
        assert_eq!(system.geologic_index(0, 1), 48271);
        assert_eq!(system.geologic_index(0, 2), 2 * 48271);
    }

    #[test]
    fn test_erosion_level() {
        let cave = Cave {
            depth: 510,
            target: (10, 10),
        };
        let mut system = CaveSystem::new(&cave);
        assert_eq!(system.erosion_level(0, 0), 510);
        assert_eq!(system.erosion_level(1, 0), 17317);
        assert_eq!(system.erosion_level(0, 1), 8415);
    }

    #[test]
    fn test_region_type() {
        let cave = Cave {
            depth: 510,
            target: (10, 10),
        };
        let mut system = CaveSystem::new(&cave);
        assert_eq!(system.region_type(0, 0), RegionType::Rocky);
        assert_eq!(system.region_type(1, 0), RegionType::Wet);
        assert_eq!(system.region_type(1, 1), RegionType::Narrow);
    }

    #[test]
    fn test_risk_level() {
        let cave = Cave {
            depth: 510,
            target: (10, 10),
        };
        let mut system = CaveSystem::new(&cave);
        assert_eq!(system.risk_level(0, 0), 0);
        assert_eq!(system.risk_level(1, 0), 1);
        assert_eq!(system.risk_level(1, 1), 2);
    }

    #[test]
    fn test_part1_example() {
        let cave = Cave {
            depth: 510,
            target: (10, 10),
        };
        assert_eq!(part1(&cave), 114);
    }

    #[test]
    fn test_tool_valid_for_region() {
        assert!(is_tool_valid_for_region(RegionType::Rocky, Tool::Torch));
        assert!(is_tool_valid_for_region(
            RegionType::Rocky,
            Tool::ClimbingGear
        ));
        assert!(!is_tool_valid_for_region(RegionType::Rocky, Tool::Neither));
        assert!(!is_tool_valid_for_region(RegionType::Wet, Tool::Torch));
        assert!(is_tool_valid_for_region(
            RegionType::Wet,
            Tool::ClimbingGear
        ));
        assert!(is_tool_valid_for_region(RegionType::Wet, Tool::Neither));
        assert!(is_tool_valid_for_region(RegionType::Narrow, Tool::Torch));
        assert!(!is_tool_valid_for_region(
            RegionType::Narrow,
            Tool::ClimbingGear
        ));
        assert!(is_tool_valid_for_region(RegionType::Narrow, Tool::Neither));
    }

    #[test]
    fn test_part2_example() {
        let cave = Cave {
            depth: 510,
            target: (10, 10),
        };
        assert_eq!(part2(&cave), 45);
    }
}
