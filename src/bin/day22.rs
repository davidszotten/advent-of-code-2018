use aoc2018::{dispatch, Result};
use cached::cached;
use std::cmp::{Ord, Ordering};
use std::collections::{BinaryHeap, HashSet};
// use std::collections::VecDeque;

fn main() {
    dispatch(&part1, &part2)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Type {
    Rocky,
    Wet,
    Narrow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Gear {
    Neither,
    Torch,
    ClimbingGear,
}

impl Gear {
    fn overlap(t1: Type, t2: Type) -> Self {
        use self::Gear::{ClimbingGear, Neither, Torch};
        use self::Type::{Narrow, Rocky, Wet};
        match (t1, t2) {
            (Rocky, Wet) => ClimbingGear,
            (Rocky, Narrow) => Torch,
            (Wet, Rocky) => ClimbingGear,
            (Wet, Narrow) => Neither,
            (Narrow, Rocky) => Torch,
            (Narrow, Wet) => Neither,
            _ => panic!("can't call overlap with t1 == t2"),
        }
    }
}

impl Type {
    fn from_erosion(erosion: u32) -> Self {
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

    fn get(depth: u32, target: (u32, u32), x: u32, y: u32) -> Self {
        let gi = geo_index(depth, target, x, y);
        let er = erosion(depth, gi);
        Type::from_erosion(er)
    }

    // fn sufficient_gear(&self) -> bool {

    // }
}

fn erosion(depth: u32, geo_index: u32) -> u32 {
    (geo_index + depth) % 20183
}

cached! {
    GEO_INDEX;
    fn geo_index(depth: u32, target: (u32, u32), x: u32, y: u32) -> u32 = {
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
}

fn calculate(depth: u32, target: (u32, u32)) -> u32 {
    let (target_x, target_y) = target;

    let mut risk = 0;

    for y in 0..=target_y {
        for x in 0..=target_x {
            risk += Type::get(depth, target, x, y).risk();
        }
    }

    risk
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Input {
    depth: u32,
    target: (u32, u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    x: u32,
    y: u32,
    gear: Gear,
    time: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Offset {
    Incr,
    Decr,
    Noop,
}

impl Offset {
    fn apply(&self, n: u32) -> u32 {
        use self::Offset::*;
        match self {
            Decr => n - 1,
            Noop => n,
            Incr => n + 1,
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        // self.height.cmp(&other.height)
        // nb reverse order
        use self::Gear::Torch;
        let gear_order = |t| match t {
            Torch => 0,
            _ => 1,
        };
        (other.time, gear_order(other.gear)).cmp(&(self.time, gear_order(self.gear)))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// struct NeighbourIterator {

// }

impl State {
    fn new(x: u32, y: u32, gear: Gear, time: u32) -> Self {
        State { x, y, gear, time }
    }
    fn type_(&self, input: Input) -> Type {
        Type::get(input.depth, input.target, self.x, self.y)
    }
    fn neighbours(&self, input: Input, seen: &HashSet<(u32, u32, Gear)>) -> Vec<State> {
        use self::Offset::*;
        let mut res = vec![];
        let x = self.x;
        let y = self.y;
        let time = self.time;
        let type_ = self.type_(input);

        if (self.x, self.y) == input.target && self.gear != Gear::Torch {
            return vec![State::new(x, y, Gear::Torch, time + 7)];
        }

        for (offset_x, offset_y) in [(Decr, Noop), (Incr, Noop), (Noop, Decr), (Noop, Incr)].iter()
        {
            if (x == 0 && *offset_x == Decr) || (y == 0 && *offset_y == Decr) {
                continue;
            }

            // println!("{:?}", (offset_x, offset_y));

            let next_x = offset_x.apply(x);
            let next_y = offset_y.apply(y);
            // println!("{} {}   {} {}", x, y, next_x, next_y);
            let next_type = Type::get(input.depth, input.target, next_x, next_y);
            let gear = if type_ == next_type {
                self.gear
            } else {
                Gear::overlap(type_, next_type)
            };
            let delay = if gear == self.gear { 0 } else { 7 };

            // println!("gear: {:?} -> {:?}", self.gear, gear);
            // println!("seen: {:?}", seen);
            // println!("e: {:?}", (next_x, next_y, gear));
            if seen.contains(&(next_x, next_y, gear)) {
                continue;
            }

            // println!("push: {:?}", (next_x, next_y, gear, time + 1 + delay));
            res.push(State::new(next_x, next_y, gear, time + 1 + delay));
        }
        res
    }
}

fn part1(_input: &str) -> Result<u32> {
    let depth = 7305;
    let target = (13, 734);
    Ok(calculate(depth, target))
}

fn calculate2(input: Input) -> u32 {
    // let mut queue = VecDeque::new();
    let mut queue = BinaryHeap::new();
    let mut seen = HashSet::new();
    let start = State {
        x: 0,
        y: 0,
        gear: Gear::Torch,
        time: 0,
    };
    // queue.push_back(start);
    queue.push(start);
    loop {
        // let pos = queue.pop_front().expect("Out of moves");
        let pos = queue.pop().expect("Out of moves");
        // if seen.contains(&(pos.x, pos.y, pos.gear)) {
        //     println!("skip");
        //     continue;
        // }
        seen.insert((pos.x, pos.y, pos.gear));
        // println!("");
        // println!("seen seen: {:?}", seen);
        // println!("{:?}", pos);
        if (pos.x, pos.y) == input.target && pos.gear == Gear::Torch {
            break pos.time;
        }
        for next in pos.neighbours(input, &seen) {
            // queue.push_back(next);
            queue.push(next);
        }
    }
}

fn part2(_input: &str) -> Result<u32> {
    let input = Input {
        depth: 7305,
        target: (13, 734),
    };
    Ok(calculate2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate() {
        assert_eq!(calculate(510, (10, 10)), 114);
    }

    #[test]
    fn test_ord() {
        let mut queue = BinaryHeap::new();
        let s1 = State {
            x: 0,
            y: 0,
            gear: Gear::Torch,
            time: 0,
        };
        let s2 = State {
            x: 0,
            y: 0,
            gear: Gear::ClimbingGear,
            time: 0,
        };
        let s3 = State {
            x: 0,
            y: 0,
            gear: Gear::Neither,
            time: 2,
        };
        queue.push(s3);
        queue.push(s1);
        queue.push(s2);
        assert_eq!(queue.pop().unwrap(), s1);
        assert_eq!(queue.pop().unwrap(), s2);
        assert_eq!(queue.pop().unwrap(), s3);
    }

    #[test]
    fn test_calculate2() {
        let input = Input {
            depth: 510,
            target: (10, 10),
        };
        assert_eq!(calculate2(input), 45);
    }

    // #[test]
    // fn test_part1() -> Result<()> {
    //     Ok(assert_eq!(part1("")?, 0))
    // }
}
