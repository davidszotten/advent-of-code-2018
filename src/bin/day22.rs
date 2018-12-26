use aoc2018::{dispatch, Result};
use std::collections::HashMap;
// use cached::cached;

fn main() {
    dispatch(&part1, &part2)
}

enum Type {
    Rocky,
    Wet,
    Narrow,
}

impl Type {
    fn new(erosion: u32) -> Self {
        use self::Type::*;
        match erosion % 3 {
            0 => Rocky,
            1 => Wet,
            2 => Narrow,
            _ => panic!("bad modulo"),
        }
    }

    fn risk(&self) -> u32 {
        use self::Type::*;
        match self {
            Rocky => 0,
            Wet => 1,
            Narrow => 2,
        }
    }
}

fn erosion(depth: u32, geo_index: u32) -> u32 {
    (geo_index + depth) % 20183
}

fn geo_index(depth: u32, target: (u32, u32), x: u32, y: u32) -> u32 {
    if (x, y) == (0, 0) {
        0
    } else if (x, y) == target {
        0
    } else if y == 0 {
        x * 16807
    } else if x == 0 {
        y * 48271
    } else {
        erosion(depth, geo_index(depth, target, x - 1, y))
            * erosion(depth, geo_index(depth, target, x, y - 1))
    }
}

fn calculate(depth: u32, target: (u32, u32)) -> u32 {
    let (target_x, target_y) = target;

    let mut risk = 0;

    let get_erosion = |geo_index: u32| (geo_index + depth) % 20183;

    let mut erosion_map = HashMap::new();

    for y in 0..=target_y {
        for x in 0..=target_x {
            let geo_index = if (x, y) == (0, 0) {
                0
            } else if (x, y) == target {
                0
            } else if y == 0 {
                x * 16807
            } else if x == 0 {
                y * 48271
            } else {
                erosion_map.get(&(x - 1, y)).expect("coor missing")
                    * erosion_map.get(&(x, y - 1)).expect("coor missing")
            };
            let erosion = get_erosion(geo_index);
            erosion_map.insert((x, y), erosion);
            risk += erosion % 3;
        }
    }

    risk
}

fn part1(_input: &str) -> Result<u32> {
    let depth = 7305;
    let target = (13, 734);
    Ok(calculate(depth, target))
}

fn part2(_input: &str) -> Result<i32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate() {
        assert_eq!(calculate(510, (10, 10)), 114);
    }

    // #[test]
    // fn test_part1() -> Result<()> {
    //     Ok(assert_eq!(part1("")?, 0))
    // }
}
