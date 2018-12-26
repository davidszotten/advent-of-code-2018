use aoc2018::{dispatch, Result};
use std::collections::HashMap;

fn main() {
    dispatch(&part1, &part2)
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Point {
    fn distance(&self, other: &Point) -> i32 {
        (other.x - self.x).abs() + (other.y - self.y).abs() + (other.z - self.z).abs() + (other.w - self.w).abs()
    }
}

fn part1(input: &str) -> Result<usize> {
    let points: Vec<_> = input
        .split('\n')
        .map(|r| r.split(',').filter_map(|s| s.parse::<i32>().ok()))
        .map(|mut ns| Point{x: ns.next().expect("x"), y: ns.next().expect("y"), z: ns.next().expect("z"), w: ns.next().expect("w")}).collect()
    ;

    // let constellations: Vec<Vec<usize>> = vec![];
    let mut constellations: HashMap<usize, Vec<usize>> = HashMap::new();
    for point_id in 0..points.len() {
        let mut can_chain = vec![];
        for constellation_id in constellations.keys().map(|i| *i) {
            let constellation_point_ids = constellations.get(&constellation_id).unwrap();
            for potential_id in constellation_point_ids {
                if points[point_id].distance(&points[*potential_id]) <= 3 {
                    can_chain.push(constellation_id);
                    break;
                }
            }
        }
        let mut new_constellation = vec![point_id];
        let new_constellation_id = constellations.keys().max().unwrap_or(&0) + 1;
        for constellation_id in can_chain {
            let mut constellation = constellations.remove(&constellation_id).unwrap();
            new_constellation.append(&mut constellation);
        }
        constellations.insert(new_constellation_id, new_constellation);
    }

    // println!("{:?}", points);
    Ok(constellations.keys().count())
}

fn part2(_input: &str) -> Result<i32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0,0,0,0
3,0,0,0
0,3,0,0
0,0,3,0
0,0,0,3
0,0,0,6
9,0,0,0
12,0,0,0";

    #[test]
    fn test_part1() -> Result<()> {
        Ok(assert_eq!(part1(INPUT)?, 2))
    }

    #[test]
    fn test_part1b() -> Result<()> {
        Ok(assert_eq!(part1("-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0")?, 4))
    }

    #[test]
    fn test_part1c() -> Result<()> {
        Ok(assert_eq!(part1("1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2")?, 3))
    }

    #[test]
    fn test_part1d() -> Result<()> {
        Ok(assert_eq!(part1("1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2")?, 8))
    }

}
