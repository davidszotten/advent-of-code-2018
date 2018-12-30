use aoc2018::{dispatch, Result};
use lazy_static::lazy_static;
use regex::{CaptureMatches, Captures, Regex};
use std::cmp;
use std::collections::HashMap;
use std::fmt;
use std::ops;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coor {
    x: u32,
    y: u32,
}

impl Coor {
    fn new(x: u32, y: u32) -> Coor {
        Coor { x, y }
    }

    fn below(&self) -> Coor {
        Coor::new(self.x, self.y + 1)
    }

    fn left(&self, offset: u32) -> Coor {
        Coor::new(self.x - offset, self.y)
    }
    fn right(&self, offset: u32) -> Coor {
        Coor::new(self.x + offset, self.y)
    }
}

impl ops::Add for Coor {
    type Output = Coor;

    fn add(self, other: Coor) -> Self::Output {
        Coor {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl fmt::Debug for Coor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Scan {
    Horizontal(HorizontalScan),
    Vertical(VerticalScan),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HorizontalScan {
    x: u32,
    y0: u32,
    y1: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct VerticalScan {
    x0: u32,
    x1: u32,
    y: u32,
}

struct InputWalker<'r, 't> {
    caps: CaptureMatches<'r, 't>,
}

impl<'r, 't> InputWalker<'r, 't> {
    fn new<'s: 't + 'r>(s: &'s str) -> Self {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(x=(?P<x>\d+), y=(?P<y0>\d+)..(?P<y1>\d+))|(y=(?P<y>\d+), x=(?P<x0>\d+)..(?P<x1>\d+))").unwrap();
        }

        let caps = RE.captures_iter(s);
        InputWalker { caps }
    }
}

impl<'r, 't> Iterator for InputWalker<'r, 't> {
    type Item = Scan;

    fn next(&mut self) -> Option<Scan> {
        if let Some(caps) = self.caps.next() {
            fn get_cap_int(caps: &Captures, pos: &str) -> u32 {
                caps.name(pos)
                    .expect("name missing")
                    .as_str()
                    .parse()
                    .unwrap()
            }
            Some(match caps.name("x") {
                Some(_) => Scan::Horizontal(HorizontalScan {
                    x: get_cap_int(&caps, "x"),
                    y0: get_cap_int(&caps, "y0"),
                    y1: get_cap_int(&caps, "y1"),
                }),
                None => Scan::Vertical(VerticalScan {
                    x0: get_cap_int(&caps, "x0"),
                    x1: get_cap_int(&caps, "x1"),
                    y: get_cap_int(&caps, "y"),
                }),
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Terrain {
    Clay,
    SettledWater,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map {
    terrain: HashMap<Coor, Terrain>,
    min_x: u32,
    max_x: u32,
    min_y: u32,
    max_y: u32,
    flowing_water: HashMap<Coor, usize>,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut terrain = HashMap::new();
        let mut min_x = std::u32::MAX;
        let mut max_x = 0;
        let mut min_y = std::u32::MAX;
        let mut max_y = 0;
        let flowing_water = HashMap::new();
        for scan in InputWalker::new(input) {
            match scan {
                Scan::Horizontal(scan) => {
                    min_x = cmp::min(min_x, scan.x);
                    max_x = cmp::max(max_x, scan.x);
                    for y in scan.y0..=scan.y1 {
                        min_y = cmp::min(min_y, y);
                        max_y = cmp::max(max_y, y);
                        terrain.insert(Coor::new(scan.x, y), Terrain::Clay);
                    }
                }
                Scan::Vertical(scan) => {
                    min_y = cmp::min(min_y, scan.y);
                    max_y = cmp::max(max_y, scan.y);
                    for x in scan.x0..=scan.x1 {
                        min_x = cmp::min(min_x, x);
                        max_x = cmp::max(max_x, x);
                        terrain.insert(Coor::new(x, scan.y), Terrain::Clay);
                    }
                }
            }
        }
        Map {
            terrain,
            min_x,
            max_x,
            min_y,
            max_y,
            flowing_water,
        }
    }

    fn is_open(&self, coor: &Coor) -> bool {
        !(self.terrain.contains_key(&coor))
    }

    fn move_down(&mut self, from: &Coor) {
        let distance = self.flowing_water.remove(from).expect("move_down missing");
        self.flowing_water.insert(from.below(), distance + 1);
    }

    #[allow(dead_code)]
    fn print(&self) {
        // let mut ycount = 0;
        for y in (self.min_y)..=(self.max_y + 1) {
            // ycount += 1;
            // if ycount > 100 {
            //     break;
            // }
            // let mut xcount = 0;
            for x in (self.min_x - 3)..=(self.max_x + 3) {
                // xcount += 1;
                // if xcount > 100 {
                //     break;
                // }
                let coor = Coor::new(x, y);
                let c = match self.terrain.get(&coor) {
                    Some(Terrain::SettledWater) => '~',
                    Some(Terrain::Clay) => '#',
                    None => {
                        if self.flowing_water.contains_key(&coor) {
                            '|'
                        } else {
                            '.'
                        }
                    }
                };
                print!("{}", c);
            }
            println!("");
        }
        println!("");
    }

    fn inside_bb(&self, coor: &Coor) -> bool {
        coor.y >= self.min_y && coor.y <= self.max_y
    }

    fn water_count(&self) -> usize {
        self.terrain
            .iter()
            .filter(|&(c, &t)| t == Terrain::SettledWater && self.inside_bb(c))
            .count()
            + self
                .flowing_water
                .iter()
                .filter(|&(c, _)| self.inside_bb(c))
                .count()
    }

    fn settled_water_count(&self) -> usize {
        self.terrain
            .iter()
            .filter(|&(c, &t)| t == Terrain::SettledWater && self.inside_bb(c))
            .count()
    }

    fn contained(&self, coor: &Coor) -> Option<Vec<Coor>> {
        let mut res = vec![];
        let mut offset = 0;
        loop {
            let left = coor.left(offset);
            if !self.is_open(&left) {
                break;
            }
            if self.is_open(&left.below()) {
                return None;
            }
            res.push(left);
            offset += 1;
        }
        let mut offset = 0;
        loop {
            let right = coor.right(offset);
            if !self.is_open(&right) {
                break;
            }
            if self.is_open(&right.below()) {
                return None;
            }
            res.push(right);
            offset += 1;
        }
        Some(res)
    }

    fn round(&mut self) -> bool {
        // spring
        self.flowing_water.insert(Coor::new(500, 0), 0);
        let mut order: Vec<(Coor, usize)> = self
            .flowing_water
            .iter()
            .map(|(&c, &d)| (c.clone(), d))
            .collect();
        order.sort_by_key(|&(_, d)| d);
        order.reverse();
        let mut moved = false;
        for (coor, distance) in order {
            if self.is_open(&coor.below()) {
                if coor.below().y <= self.max_y {
                    if !self.flowing_water.contains_key(&coor.below()) {
                        self.move_down(&coor);
                        moved = true;
                    }
                }
            } else {
                if let Some(contained) = self.contained(&coor) {
                    moved = true;

                    self.flowing_water.remove(&coor);
                    for contained_coor in contained {
                        self.terrain.insert(contained_coor, Terrain::SettledWater);
                    }
                }
                let mut flowed = false;
                if self.is_open(&coor.left(1)) {
                    let new_coor = coor.left(1);
                    if !self.flowing_water.contains_key(&new_coor) {
                        self.flowing_water.insert(new_coor, distance + 1);
                        flowed = true;
                    }
                }
                if self.is_open(&coor.right(1)) {
                    let new_coor = coor.right(1);
                    if !self.flowing_water.contains_key(&new_coor) {
                        self.flowing_water.insert(new_coor, distance + 1);
                        flowed = true;
                    }
                }
                if flowed {
                    moved = true;
                    self.flowing_water.remove(&coor);
                }
            }
        }

        moved
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut map = Map::new(input);
    loop {
        if !map.round() {
            break;
        };
    }
    Ok(map.water_count())
}

fn part2(input: &str) -> Result<usize> {
    let mut map = Map::new(input);
    let mut water_count = map.water_count();
    loop {
        if !map.round() {
            break;
        };
        let new_water_count = map.water_count();
        if new_water_count > water_count {
            water_count = new_water_count
        }
    }
    Ok(map.settled_water_count())
}

fn main() {
    dispatch(&part1, &part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        Ok(assert_eq!(
            part1(
                "x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504"
            )?,
            57
        ))
    }
    #[test]
    fn test_part1_split() -> Result<()> {
        Ok(assert_eq!(
            part1(
                "y=7, x=495..505
y=3, x=498..502
y=0, x=505..505
y=0, x=495..495"
            )?,
            0
        ))
    }
}
