use std::collections::HashSet;

use anyhow::{Result, anyhow};
use aoc_runner_derive::{aoc, aoc_generator};

type Point = (i32, i32);

#[derive(Debug, Clone)]
struct Warehouse {
    walls: HashSet<Point>,
    boxes: HashSet<Point>,
    robot: Point,
    moves: Vec<char>,
}

#[aoc_generator(day15)]
fn generator(input: &str) -> Result<Warehouse> {
    let mut parts = input.split("\n\n");
    let map_str = parts.next().ok_or_else(|| anyhow!("No map found"))?;
    let moves_str = parts.next().ok_or_else(|| anyhow!("No moves found"))?;

    let mut walls = HashSet::new();
    let mut boxes = HashSet::new();
    let mut robot = None;

    for (y, line) in map_str.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let point = (x as i32, y as i32);
            match ch {
                '#' => {
                    walls.insert(point);
                }
                'O' => {
                    boxes.insert(point);
                }
                '@' => {
                    robot = Some(point);
                }
                _ => {}
            }
        }
    }

    let robot = robot.ok_or_else(|| anyhow!("No robot found"))?;
    let moves: Vec<char> = moves_str.chars().filter(|&c| "<>^v".contains(c)).collect();

    Ok(Warehouse {
        walls,
        boxes,
        robot,
        moves,
    })
}

#[aoc(day15, part1)]
fn part1(warehouse: &Warehouse) -> i32 {
    let final_state = simulate_moves(warehouse);
    final_state.boxes.iter().copied().map(calculate_gps).sum()
}

#[aoc(day15, part2)]
fn part2(warehouse: &Warehouse) -> i32 {
    let wide_warehouse = scale_warehouse(warehouse);
    let final_state = simulate_wide_moves(&wide_warehouse);
    final_state
        .wide_boxes
        .iter()
        .copied()
        .map(calculate_gps)
        .sum()
}

fn scale_warehouse(warehouse: &Warehouse) -> WideWarehouse {
    let walls = warehouse
        .walls
        .iter()
        .flat_map(|&(x, y)| [(x * 2, y), (x * 2 + 1, y)])
        .collect();

    let wide_boxes = warehouse.boxes.iter().map(|&(x, y)| (x * 2, y)).collect();

    let (robot_x, robot_y) = warehouse.robot;
    WideWarehouse {
        walls,
        wide_boxes,
        robot: (robot_x * 2, robot_y),
        moves: warehouse.moves.clone(),
    }
}

#[derive(Debug, Clone)]
struct WideWarehouse {
    walls: HashSet<Point>,
    wide_boxes: HashSet<Point>,
    robot: Point,
    moves: Vec<char>,
}

fn simulate_wide_moves(warehouse: &WideWarehouse) -> WideWarehouse {
    let (robot, wide_boxes) = warehouse.moves.iter().fold(
        (warehouse.robot, warehouse.wide_boxes.clone()),
        |(robot, mut wide_boxes), &move_char| {
            let dir = get_direction(move_char);
            let new_robot_pos = add_points(robot, dir);

            if warehouse.walls.contains(&new_robot_pos) {
                return (robot, wide_boxes);
            }

            if !is_box_at_position(&wide_boxes, new_robot_pos) {
                return (new_robot_pos, wide_boxes);
            }

            if can_push_wide_boxes(&wide_boxes, &warehouse.walls, new_robot_pos, dir) {
                push_wide_boxes(&mut wide_boxes, new_robot_pos, dir);
                (new_robot_pos, wide_boxes)
            } else {
                (robot, wide_boxes)
            }
        },
    );

    WideWarehouse {
        walls: warehouse.walls.clone(),
        wide_boxes,
        robot,
        moves: warehouse.moves.clone(),
    }
}

fn is_box_at_position(wide_boxes: &HashSet<Point>, pos: Point) -> bool {
    let (x, y) = pos;
    wide_boxes.contains(&pos) || wide_boxes.contains(&(x - 1, y))
}

fn get_box_left_position(wide_boxes: &HashSet<Point>, pos: Point) -> Option<Point> {
    let (x, y) = pos;
    [pos, (x - 1, y)]
        .into_iter()
        .find(|p| wide_boxes.contains(p))
}

fn collect_boxes_to_move(
    wide_boxes: &HashSet<Point>,
    start_pos: Point,
    dir: Point,
) -> HashSet<Point> {
    let mut boxes_to_move = HashSet::new();
    let mut to_check = vec![start_pos];

    while let Some(pos) = to_check.pop() {
        if let Some(box_pos) = get_box_left_position(wide_boxes, pos)
            && boxes_to_move.insert(box_pos)
        {
            let (box_x, box_y) = box_pos;
            let new_positions = [box_pos, (box_x + 1, box_y)]
                .into_iter()
                .map(|p| add_points(p, dir));
            to_check.extend(new_positions);
        }
    }

    boxes_to_move
}

fn can_push_wide_boxes(
    wide_boxes: &HashSet<Point>,
    walls: &HashSet<Point>,
    start_pos: Point,
    dir: Point,
) -> bool {
    let boxes_to_move = collect_boxes_to_move(wide_boxes, start_pos, dir);
    boxes_to_move.iter().all(|&box_pos| {
        let (box_x, box_y) = box_pos;
        [box_pos, (box_x + 1, box_y)]
            .into_iter()
            .map(|p| add_points(p, dir))
            .all(|pos| !walls.contains(&pos))
    })
}

fn push_wide_boxes(wide_boxes: &mut HashSet<Point>, start_pos: Point, dir: Point) {
    let boxes_to_move = collect_boxes_to_move(wide_boxes, start_pos, dir);

    for &box_pos in &boxes_to_move {
        wide_boxes.remove(&box_pos);
    }

    for &box_pos in &boxes_to_move {
        wide_boxes.insert(add_points(box_pos, dir));
    }
}

fn get_direction(c: char) -> Point {
    match c {
        '^' => (0, -1),
        'v' => (0, 1),
        '<' => (-1, 0),
        '>' => (1, 0),
        _ => unreachable!(),
    }
}

fn add_points((x1, y1): Point, (x2, y2): Point) -> Point {
    (x1 + x2, y1 + y2)
}

fn calculate_gps((x, y): Point) -> i32 {
    100 * y + x
}

fn simulate_moves(warehouse: &Warehouse) -> Warehouse {
    let (robot, boxes) = warehouse.moves.iter().fold(
        (warehouse.robot, warehouse.boxes.clone()),
        |(robot, mut boxes), &move_char| {
            let dir = get_direction(move_char);
            let new_robot_pos = add_points(robot, dir);

            if warehouse.walls.contains(&new_robot_pos) {
                return (robot, boxes);
            }

            if !boxes.contains(&new_robot_pos) {
                return (new_robot_pos, boxes);
            }

            let boxes_to_push = collect_line_boxes(&boxes, new_robot_pos, dir);
            let final_pos = add_points(*boxes_to_push.last().unwrap(), dir);
            let can_push = !warehouse.walls.contains(&final_pos);

            if can_push {
                boxes_to_push.iter().rev().for_each(|&box_pos| {
                    boxes.remove(&box_pos);
                    boxes.insert(add_points(box_pos, dir));
                });
                (new_robot_pos, boxes)
            } else {
                (robot, boxes)
            }
        },
    );

    Warehouse {
        walls: warehouse.walls.clone(),
        boxes,
        robot,
        moves: warehouse.moves.clone(),
    }
}

fn collect_line_boxes(boxes: &HashSet<Point>, start_pos: Point, dir: Point) -> Vec<Point> {
    let mut result = vec![start_pos];
    let mut check_pos = start_pos;

    while boxes.contains(&add_points(check_pos, dir)) {
        check_pos = add_points(check_pos, dir);
        result.push(check_pos);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_EXAMPLE: &str = r"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const LARGE_EXAMPLE: &str = r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn test_parse_warehouse() {
        let warehouse = generator(SMALL_EXAMPLE).unwrap();
        assert_eq!(warehouse.robot, (2, 2));
        assert_eq!(warehouse.boxes.len(), 6);
        assert_eq!(warehouse.moves.len(), 15);
        assert_eq!(warehouse.moves[0], '<');
    }

    #[test]
    fn test_simulate_movement() {
        let warehouse = generator(SMALL_EXAMPLE).unwrap();
        let final_state = simulate_moves(&warehouse);
        assert_eq!(final_state.boxes.len(), 6);
    }

    #[test]
    fn test_gps_calculation() {
        assert_eq!(calculate_gps((4, 1)), 104);
    }

    #[test]
    fn test_part1_small_example() {
        let warehouse = generator(SMALL_EXAMPLE).unwrap();
        assert_eq!(part1(&warehouse), 2028);
    }

    #[test]
    fn test_part1_large_example() {
        let warehouse = generator(LARGE_EXAMPLE).unwrap();
        assert_eq!(part1(&warehouse), 10092);
    }

    #[test]
    fn test_part2_small_example() {
        let warehouse = generator(SMALL_EXAMPLE).unwrap();
        assert_eq!(part2(&warehouse), 1751);
    }

    #[test]
    fn test_part2_large_example() {
        let warehouse = generator(LARGE_EXAMPLE).unwrap();
        assert_eq!(part2(&warehouse), 9021);
    }
}
