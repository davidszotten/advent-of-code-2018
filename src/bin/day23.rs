use aoc2018::{dispatch, Result};
use lazy_static::lazy_static;
use regex::{CaptureMatches, Captures, Regex};

fn main() {
    dispatch(&part1, &part2)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Bot {
    x: i32,
    y: i32,
    z: i32,
    r: i32,
}

impl Bot {
    fn distance(&self, other: &Bot) -> i32 {
        self.p_distance(other.x, other.y, other.z)
    }

    fn p_distance(&self, x: i32, y: i32, z: i32) -> i32 {
        (x - self.x).abs() + (y - self.y).abs() + (z - self.z).abs()
    }
}

struct InputWalker<'r, 't> {
    caps: CaptureMatches<'r, 't>,
}

impl<'r, 't> InputWalker<'r, 't> {
    fn new<'s: 't + 'r>(s: &'s str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)").unwrap();
        }

        let caps = RE.captures_iter(s);
        InputWalker { caps }
    }
}

impl<'r, 't> Iterator for InputWalker<'r, 't> {
    type Item = Bot;

    fn next(&mut self) -> Option<Bot> {
        if let Some(caps) = self.caps.next() {
            fn get_cap_int(caps: &Captures, pos: usize) -> i32 {
                caps[pos].parse().unwrap()
            }
            Some(Bot {
                x: get_cap_int(&caps, 1),
                y: get_cap_int(&caps, 2),
                z: get_cap_int(&caps, 3),
                r: get_cap_int(&caps, 4),
            })
        } else {
            None
        }
    }
}

fn part1(input: &str) -> Result<usize> {
    let bots = InputWalker::new(input).collect::<Vec<_>>();
    let strongest = bots.iter().max_by_key(|b| b.r).unwrap();
    let count = bots
        .iter()
        .filter(|&b| b.distance(strongest) <= strongest.r)
        .count();
    Ok(count)
}

fn part2(input: &str) -> Result<i32> {
    let bots = InputWalker::new(input).collect::<Vec<_>>();
    let max_x = bots.iter().max_by_key(|b| b.x).unwrap().x;
    let max_y = bots.iter().max_by_key(|b| b.y).unwrap().y;
    let max_z = bots.iter().max_by_key(|b| b.z).unwrap().z;
    let min_x = bots.iter().min_by_key(|b| b.x).unwrap().x;
    let min_y = bots.iter().min_by_key(|b| b.y).unwrap().y;
    let min_z = bots.iter().min_by_key(|b| b.z).unwrap().z;
    // let count = bots.iter().filter(|&b| b.distance(strongest) <= strongest.r).count();
    println!(
        "{}/{} {}/{} {}/{}",
        min_x, max_x, min_y, max_y, min_z, max_z
    );
    // let zero = Bot {
    // x: 0,
    // y: 0,
    // z: 0,
    // r: 0,
    // };
    let mut bot_count = 0;
    let mut best_dist = None;
    // let mut best_pos = (0, 0, 0);
    for x in min_x..=max_x {
        println!("{}", x);
        for y in min_y..=max_y {
            println!("{}", y);
            for z in min_z..=max_z {
                if z % 1_000_000 == 0 {
                    println!("{}", z);
                }
                // let tmp = Bot { x, y, z, r: 0 };
                // let zero_dist = tmp.distance(&zero);
                let zero_dist = x.abs() + y.abs() + z.abs();
                let count = bots
                    .iter()
                    .filter(|&b| b.p_distance(x, y, z) <= b.r)
                    .count();
                // if count >= bot_count {
                if count >= bot_count {
                    // println!("bar: {} {}", count, bot_count);
                    // println!("bar: {}   {} {} {} ", count, x, y, z);
                    let mut best = false;
                    if let Some(best_dist) = best_dist {
                        if zero_dist < best_dist {
                            best = true;
                        }
                    } else {
                        best = true
                    }
                    if count > bot_count {
                        best = true;
                    }
                    if best {
                        // println!("{} {} {}: {}", x,y,z, count);
                        bot_count = count;
                        best_dist = Some(zero_dist);
                        // best_pos = (x, y, z);
                    }
                }
            }
        }
    }
    Ok(best_dist.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1";

    #[test]
    fn test_parse() {
        let bots = InputWalker::new(INPUT).collect::<Vec<_>>();
        assert_eq!(
            bots[1],
            Bot {
                x: 1,
                y: 0,
                z: 0,
                r: 1
            }
        );
    }

    #[test]
    fn test_part1() -> Result<()> {
        Ok(assert_eq!(part1(INPUT)?, 7))
    }

    #[test]
    fn test_part2() -> Result<()> {
        Ok(assert_eq!(
            part2(
                "pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5"
            )?,
            36
        ))
    }
}
