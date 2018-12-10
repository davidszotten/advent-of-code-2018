use aoc2018::{dispatch, Result};
use failure::Error;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    px: i64,
    py: i64,
    vx: i64,
    vy: i64,
}

impl Point {
    fn mv(&mut self) {
        self.px += self.vx;
        self.py += self.vy;
    }

    fn mv_back(&mut self) {
        self.px -= self.vx;
        self.py -= self.vy;
    }
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex =
            //position=<-3, 11> velocity=< 1, -2>
                Regex::new(r"position=< *(-?\d+), *(-?\d+)> velocity=< *(-?\d+), *(-?\d+)>")
                    .expect("regex create");
        }

        let caps = RE.captures(s).expect("regex match");
        Ok(Self {
            px: caps[1].parse().expect("regex match 1"),
            py: caps[2].parse().expect("regex match 1"),
            vx: caps[3].parse().expect("regex match 1"),
            vy: caps[4].parse().expect("regex match 1"),
        })
    }
}

fn main() {
    dispatch(&part1, &part2)
}

fn size(points: &Vec<Point>) -> i64 {
    let (minx, maxx, miny, maxy) = bounding_box(points);
    (maxx - minx) * (maxy - miny)
}
fn bounding_box(points: &Vec<Point>) -> (i64, i64, i64, i64) {
    let maxx = points.iter().map(|p| p.px).max().unwrap();
    let minx = points.iter().map(|p| p.px).min().unwrap();
    let maxy = points.iter().map(|p| p.py).max().unwrap();
    let miny = points.iter().map(|p| p.py).min().unwrap();
    (minx, maxx, miny, maxy)
}

fn mv(points: &mut Vec<Point>) {
    for point in points.iter_mut() {
        (*point).mv();
    }
}

fn mv_back(points: &mut Vec<Point>) {
    for point in points.iter_mut() {
        (*point).mv_back();
    }
}

fn part1(input: &str) -> Result<i64> {
    let mut coors: Vec<Point> = input
        .split('\n')
        .filter_map(|row| row.parse().ok())
        .collect();
    let mut steps = 0;
    let mut bb1 = size(&coors);
    mv(&mut coors);
    let mut bb2 = size(&coors);
    while bb2 < bb1 {
        steps += 1;
        bb1 = bb2;
        mv(&mut coors);
        bb2 = size(&coors);
    }
    mv_back(&mut coors);
    let taken: HashSet<_> = coors.iter().map(|p| (p.px, p.py)).collect();
    let (minx, maxx, miny, maxy) = bounding_box(&coors);
    for y in miny..=maxy {
        for x in minx..=maxx {
            if taken.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    Ok(steps)
}

fn part2(input: &str) -> Result<i64> {
    part1(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";

    #[test]
    fn test_partse() {
        assert_eq!(
            "position=<-3, 11> velocity=< 1, -2>"
                .parse::<Point>()
                .unwrap(),
            Point {
                px: -3,
                py: 11,
                vx: 1,
                vy: -2
            }
        )
    }

    #[test]
    fn test_part1() -> Result<()> {
        Ok(assert_eq!(part1(INPUT)?, 3))
    }
}
