use aoc2018::{dispatch, Result};
use cached::cached;
use std::cmp::{Ord, Ordering};
use std::collections::{BinaryHeap, HashSet};

fn main() {
    dispatch(&part1, &part2)
}

type Target = (u32, u32);

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
    target: (u32, u32),
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
        // nb reverse order
        use self::Gear::Torch;
        let gear_order = |t| match t {
            Torch => 0,
            _ => 1,
        };
        (
            other.time,
            gear_order(other.gear),
            other.distance_to_target(),
        )
            .cmp(&(self.time, gear_order(self.gear), self.distance_to_target()))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct NeighbourIterator<'a> {
    input: &'a Input,
    seen: &'a HashSet<(u32, u32, Gear)>,
    state: &'a State,
    idx: usize,
}

impl<'a> NeighbourIterator<'a> {
    fn new(input: &'a Input, seen: &'a HashSet<(u32, u32, Gear)>, state: &'a State) -> Self {
        NeighbourIterator {
            input,
            seen,
            state,
            idx: 0,
        }
    }
}

impl<'a> Iterator for NeighbourIterator<'a> {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        use self::Offset::*;
        let x = self.state.x;
        let y = self.state.y;
        let time = self.state.time;
        let input = *self.input;
        let type_ = self.state.type_(input);

        loop {
            let (offset_x, offset_y) = match self.idx {
                0 => (Decr, Noop),
                1 => (Incr, Noop),
                2 => (Noop, Decr),
                3 => (Noop, Incr),
                _ => break None,
            };
            self.idx += 1;

            if (x == 0 && offset_x == Decr) || (y == 0 && offset_y == Decr) {
                continue;
            }

            let next_x = offset_x.apply(x);
            let next_y = offset_y.apply(y);
            let next_type = Type::get(input.depth, input.target, next_x, next_y);
            let gear = if type_ == next_type {
                self.state.gear
            } else {
                Gear::overlap(type_, next_type)
            };
            let delay = if gear == self.state.gear { 0 } else { 7 };

            if self.seen.contains(&(next_x, next_y, gear)) {
                continue;
            }

            break Some(State::new(
                next_x,
                next_y,
                gear,
                self.state.target,
                time + 1 + delay,
            ));
        }
    }
}

impl State {
    fn new(x: u32, y: u32, gear: Gear, target: Target, time: u32) -> Self {
        State {
            x,
            y,
            gear,
            target,
            time,
        }
    }
    fn type_(&self, input: Input) -> Type {
        Type::get(input.depth, input.target, self.x, self.y)
    }
    fn neighbours<'a>(
        &'a self,
        input: &'a Input,
        seen: &'a HashSet<(u32, u32, Gear)>,
    ) -> NeighbourIterator<'a> {
        NeighbourIterator::new(&input, &seen, &self)
    }

    fn distance_to_target(&self) -> u32 {
        ((self.x as i32 - self.target.0 as i32).abs()
            + (self.y as i32 - self.target.1 as i32).abs()) as u32
    }
}

fn part1(_input: &str) -> Result<u32> {
    let depth = 7305;
    let target = (13, 734);
    Ok(calculate(depth, target))
}

fn calculate2(input: Input) -> u32 {
    let mut queue = BinaryHeap::new();
    let mut seen = HashSet::new();
    let start = State {
        x: 0,
        y: 0,
        gear: Gear::Torch,
        target: input.target,
        time: 0,
    };
    queue.push(start);
    loop {
        let pos = queue.pop().expect("Out of moves");
        if seen.contains(&(pos.x, pos.y, pos.gear)) {
            continue;
        }
        seen.insert((pos.x, pos.y, pos.gear));
        // println!("{:?}", pos);
        if (pos.x, pos.y) == input.target && pos.gear == Gear::Torch {
            break pos.time;
        }
        if (pos.x, pos.y) == input.target && pos.gear != Gear::Torch {
            queue.push(State::new(
                pos.x,
                pos.y,
                Gear::Torch,
                pos.target,
                pos.time + 7,
            ));
        }

        for next in pos.neighbours(&input, &seen) {
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
            target: (0, 0),
            time: 0,
        };
        let s2 = State {
            x: 0,
            y: 0,
            gear: Gear::ClimbingGear,
            target: (0, 0),
            time: 0,
        };
        let s3 = State {
            x: 0,
            y: 0,
            gear: Gear::Neither,
            target: (0, 0),
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
    fn test_ord2() {
        let mut queue = BinaryHeap::new();
        let s1 = State {
            x: 0,
            y: 0,
            gear: Gear::Torch,
            target: (1, 1),
            time: 0,
        };
        let s2 = State {
            x: 0,
            y: 1,
            gear: Gear::Torch,
            target: (1, 1),
            time: 0,
        };
        queue.push(s1);
        queue.push(s2);
        queue.push(s1);
        assert_eq!(queue.pop().unwrap(), s2);
        assert_eq!(queue.pop().unwrap(), s1);
        assert_eq!(queue.pop().unwrap(), s1);
    }

    #[test]
    fn test_calculate2() {
        let input = Input {
            depth: 510,
            target: (10, 10),
        };
        assert_eq!(calculate2(input), 45);
    }
}
