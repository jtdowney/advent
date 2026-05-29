use std::str::FromStr;

use anyhow::{Result, anyhow, ensure};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Vec3 = (f64, f64, f64);
type IVec3 = (i64, i64, i64);

#[derive(Debug, Clone)]
struct Hailstone {
    position: Vec3,
    velocity: Vec3,
}

impl FromStr for Hailstone {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (pos_str, vel_str) = s
            .split_once(" @ ")
            .ok_or_else(|| anyhow!("Invalid format: missing ' @ '"))?;

        let position = pos_str
            .split(", ")
            .map(|x| x.trim().parse::<f64>())
            .collect::<Result<Vec<_>, _>>()?;
        let velocity = vel_str
            .split(", ")
            .map(|x| x.trim().parse::<f64>())
            .collect::<Result<Vec<_>, _>>()?;

        ensure!(
            position.len() == 3 || velocity.len() == 3,
            "Expected 3D coordinates"
        );

        Ok(Hailstone {
            position: (position[0], position[1], position[2]),
            velocity: (velocity[0], velocity[1], velocity[2]),
        })
    }
}

#[aoc_generator(day24)]
fn generator(input: &str) -> Result<Vec<Hailstone>> {
    input.lines().map(str::parse).collect()
}

fn find_2d_intersection(h1: &Hailstone, h2: &Hailstone) -> Option<(f64, f64, f64, f64)> {
    let ((x1, y1, _), (vx1, vy1, _)) = (h1.position, h1.velocity);
    let ((x2, y2, _), (vx2, vy2, _)) = (h2.position, h2.velocity);

    let det = vx1 * vy2 - vy1 * vx2;
    if det.abs() < 1e-10 {
        return None;
    }

    let (dx, dy) = (x2 - x1, y2 - y1);
    let t1 = (dx * vy2 - dy * vx2) / det;
    let t2 = (dx * vy1 - dy * vx1) / det;

    Some((x1 + t1 * vx1, y1 + t1 * vy1, t1, t2))
}

#[aoc(day24, part1)]
fn part1(hailstones: &[Hailstone]) -> usize {
    const MIN: f64 = 200_000_000_000_000.0;
    const MAX: f64 = 400_000_000_000_000.0;

    (0..hailstones.len())
        .tuple_combinations()
        .filter_map(|(i, j)| find_2d_intersection(&hailstones[i], &hailstones[j]))
        .filter(|&(x, y, t1, t2)| {
            t1 >= 0.0 && t2 >= 0.0 && (MIN..=MAX).contains(&x) && (MIN..=MAX).contains(&y)
        })
        .count()
}

fn to_ivec3((x, y, z): Vec3) -> IVec3 {
    (x as i64, y as i64, z as i64)
}

fn check_velocity(hailstones: &[Hailstone], rock_vel: IVec3) -> Option<IVec3> {
    if hailstones.len() < 2 {
        return None;
    }

    let (rvx, rvy, rvz) = rock_vel;
    let h0 = &hailstones[0];
    let h1 = &hailstones[1];

    let (hx0, hy0, hz0) = to_ivec3(h0.position);
    let (vx0, vy0, vz0) = to_ivec3(h0.velocity);
    let (hx1, hy1, _) = to_ivec3(h1.position);
    let (vx1, vy1, vz1) = to_ivec3(h1.velocity);

    if (vx0, vy0, vz0) == rock_vel || (vx1, vy1, vz1) == rock_vel {
        return None;
    }

    let a = vy0 - rvy;
    let b = rvx - vx0;
    let c = hx0 * (vy0 - rvy) - hy0 * (vx0 - rvx);

    let d = vy1 - rvy;
    let e = rvx - vx1;
    let f = hx1 * (vy1 - rvy) - hy1 * (vx1 - rvx);

    let det = a * e - b * d;
    if det == 0 {
        return None;
    }

    let rx_num = c * e - b * f;
    let ry_num = a * f - c * d;

    if rx_num % det != 0 || ry_num % det != 0 {
        return None;
    }

    let (rx, ry) = (rx_num / det, ry_num / det);

    let t0_num = rx - hx0;
    let t0_den = vx0 - rvx;

    if t0_den == 0 || t0_num % t0_den != 0 || t0_num * t0_den <= 0 {
        return None;
    }

    let t0 = t0_num / t0_den;
    let rz = hz0 + t0 * vz0 - t0 * rvz;

    Some((rx, ry, rz))
}

fn verify_solution(hailstones: &[Hailstone], rock_pos: IVec3, rock_vel: IVec3) -> bool {
    let (rx, ry, rz) = rock_pos;
    let (rvx, rvy, rvz) = rock_vel;

    hailstones.iter().all(|h| {
        let (hx, hy, hz) = to_ivec3(h.position);
        let (vx, vy, vz) = to_ivec3(h.velocity);

        if vx == rvx && hx != rx {
            return false;
        }

        let t_num = rx - hx;
        let t_den = vx - rvx;

        if t_den != 0 {
            if t_num % t_den != 0 || t_num * t_den < 0 {
                return false;
            }
            let t = t_num / t_den;
            hy + t * vy == ry + t * rvy && hz + t * vz == rz + t * rvz
        } else {
            true
        }
    })
}

#[aoc(day24, part2)]
fn part2(hailstones: &[Hailstone]) -> i64 {
    (-300i64..=300)
        .flat_map(|vx| {
            (-300i64..=300).flat_map(move |vy| (-300i64..=300).map(move |vz| (vx, vy, vz)))
        })
        .find_map(|vel| {
            check_velocity(hailstones, vel)
                .filter(|&pos| verify_solution(hailstones, pos, vel))
                .map(|(x, y, z)| x + y + z)
        })
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[test]
    fn test_part1() {
        let hailstones = generator(EXAMPLE).unwrap();
        let result = (0..hailstones.len())
            .tuple_combinations()
            .filter_map(|(i, j)| find_2d_intersection(&hailstones[i], &hailstones[j]))
            .filter(|&(x, y, t1, t2)| {
                t1 >= 0.0 && t2 >= 0.0 && (7.0..=27.0).contains(&x) && (7.0..=27.0).contains(&y)
            })
            .count();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part2() {
        let hailstones = generator(EXAMPLE).unwrap();
        assert_eq!(part2(&hailstones), 47);
    }
}
