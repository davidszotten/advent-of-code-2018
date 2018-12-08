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

fn part1(input: &str) -> Result<i32> {
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
        // for x in 0..maxx {
            // println!("{:?}", distances.get(&(x + maxx * y)));
        // }
    }
    Ok(0)
}

fn part2(_input: &str) -> Result<i32> {
    Ok(0)
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
            1
        ))
    }
}
