use aoc2018::{dispatch, Result};
use failure::{err_msg, Error};
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?P<x>\d+), (?P<y>\d+)").unwrap();
        }

        let caps = RE.captures(s).unwrap();
        fn get_cap_int(caps: &Captures, name: &str) -> Result<i32> {
            Ok(caps
                .name(name)
                .ok_or(err_msg("parse fail"))?
                .as_str()
                .parse()?)
        }
        Ok(Point::new(
            get_cap_int(&caps, "x")?,
            get_cap_int(&caps, "y")?,
        ))
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

fn main() {
    dispatch(&part1, &part2)
}

#[derive(Debug)]
enum Distance {
    Unset,
    Equal(i32),
    // distance, point #
    Best(i32, usize),
}

fn parse_distances(input: &str) -> (HashMap<i32, Distance>, Point) {
    use self::Distance::*;

    let mut distances = HashMap::new();
    let points: Vec<Point> = input
        .split('\n')
        .filter_map(|row| row.parse().ok())
        .collect();
    // println!("{:?}", points);
    let maxx = points.iter().map(|p| p.x).max().unwrap() + 1;
    let maxy = points.iter().map(|p| p.y).max().unwrap() + 1;
    for (pid, point) in points.iter().enumerate() {
        for x in 0..maxx {
            for y in 0..maxy {
                let distance = distances.entry(x + maxx * y).or_insert(Unset);
                let point_distance = (point.x - x).abs() + (point.y - y).abs();
                // println!("{:?}, {}", *distance, point_distance);
                *distance = match *distance {
                    Unset => Best(point_distance, pid),
                    Best(d, _) if d == point_distance => Equal(d),
                    Best(d, _) if d > point_distance => Best(point_distance, pid),
                    Best(d, i) if d < point_distance => Best(d, i),
                    Equal(d) if d <= point_distance => Equal(d),
                    Equal(d) if d > point_distance => Best(point_distance, pid),
                    _ => unreachable!(),
                }
            }
        }
    }
    (distances, Point::new(maxx, maxy))
}

fn _print(distances: &HashMap<i32, Distance>, max: &Point) {
    use self::Distance::*;
    let maxx = max.x;
    let maxy = max.y;
    // println!("{:?}", distances);
    for y in 0..maxy {
        for x in 0..maxx {
            let c = match distances.get(&(x + maxx * y)) {
                Some(Unset) => 'Q',
                Some(Best(0, i)) => ('A' as u8 + *i as u8) as char,
                Some(Best(_, i)) => ('a' as u8 + *i as u8) as char,
                Some(Equal(_)) => '.',
                None => unreachable!(),
            };
            print!("{}", c);
        }
        println!("");
    }
}

#[derive(Debug)]
enum Region {
    Infinite,
    Finite(u32),
}

fn part1(input: &str) -> Result<u32> {
    use self::Distance::*;
    use self::Region::*;
    let mut regions = HashMap::new();
    let (distances, max) = parse_distances(input);
    let maxx = max.x;
    let maxy = max.y;
    for y in 0..maxy {
        for x in 0..maxx {
            if let Some(Best(_, i)) = distances.get(&(x + maxx * y)) {
                let new = match regions.get(i) {
                    None => Finite(1),
                    Some(Infinite) => Infinite,
                    Some(Finite(count)) => Finite(count + 1),
                };
                if x == 0 || x == maxx - 1 || y == 0 || y == maxy - 1 {
                    regions.insert(i, Infinite);
                } else {
                    regions.insert(i, new);
                }
            };
        }
    }
    // _print(&distances, &max);
    // println!("{:?}", regions);
    Ok(*regions
        .values()
        .filter_map(|r| if let Finite(c) = r { Some(c) } else { None })
        .max()
        .unwrap())
}

fn area(input: &str, max_distance: i32) -> i32 {
    let points: Vec<Point> = input
        .split('\n')
        .filter_map(|row| row.parse().ok())
        .collect();

    let mut area = 0;
    let maxx = points.iter().map(|p| p.x).max().unwrap() + 1;
    let maxy = points.iter().map(|p| p.y).max().unwrap() + 1;
    for x in 0..maxx {
        for y in 0..maxy {
            let total_distance: i32 = points
                .iter()
                .map(|point| (point.x - x).abs() + (point.y - y).abs())
                .sum();

            if total_distance < max_distance {
                area += 1;
            }
        }
    }
    area
}

fn part2(input: &str) -> Result<i32> {
    Ok(area(input, 10000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        Ok(assert_eq!(
            part1(
                "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9"
            )?,
            17
        ))
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            area(
                "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9",
                32
            ),
            16
        )
    }
}
