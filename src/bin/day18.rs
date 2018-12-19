use aoc2018::{dispatch, Result};
use failure::Error;
use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    dispatch(&part1, &part2)
}

type Coor = (i32, i32);

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
enum Terrain {
    Open,
    Trees,
    Lumberyard,
}

impl Terrain {
    fn from_char(c: char) -> Self {
        use self::Terrain::*;
        match c {
            '.' => Open,
            '|' => Trees,
            '#' => Lumberyard,
            _ => unreachable!(),
        }
    }
}

struct Area {
    terrain: HashMap<Coor, Terrain>,
    history: HashMap<i64, usize>,
    resources: HashMap<usize, usize>,
    pos: usize,
}

impl Area {
    fn new(terrain: HashMap<Coor, Terrain>) -> Self {
        Area {
            terrain: terrain,
            history: HashMap::new(),
            resources: HashMap::new(),
            pos: 0,
        }
    }
    fn adjacent(&self, coor: Coor) -> impl Iterator<Item = &Terrain> {
        (-1..=1)
            .cartesian_product(-1..=1)
            .filter(|&(dx, dy)| dx != 0 || dy != 0)
            .filter_map(move |(dx, dy)| self.terrain.get(&(coor.0 + dx, coor.1 + dy)))
    }
    fn tick(&mut self) {
        use self::Terrain::*;
        let mut new_terrain = HashMap::new();
        let max = self.terrain.keys().max().unwrap();
        for y in 0..=max.1 {
            for x in 0..=max.0 {
                let coor = (x, y);
                let new = match self.terrain.get(&coor).expect("missing coor") {
                    Open => {
                        if self.adjacent(coor).filter(|&&c| c == Trees).count() >= 3 {
                            Trees
                        } else {
                            Open
                        }
                    }
                    Trees => {
                        if self.adjacent(coor).filter(|&&c| c == Lumberyard).count() >= 3 {
                            Lumberyard
                        } else {
                            Trees
                        }
                    }
                    Lumberyard => {
                        if self.adjacent(coor).filter(|&&c| c == Lumberyard).count() >= 1
                            && self.adjacent(coor).filter(|&&c| c == Trees).count() >= 1
                        {
                            Lumberyard
                        } else {
                            Open
                        }
                    }
                };
                new_terrain.insert(coor, new);
            }
        }
        self.history.insert(self.terrain_id(), self.pos);
        self.resources.insert(self.pos, self.resource_number());
        self.pos += 1;
        self.terrain = new_terrain;
    }

    fn terrain_id(&self) -> i64 {
        use self::Terrain::*;
        let mut current = self
            .terrain
            .clone()
            .into_iter()
            .collect::<Vec<(Coor, Terrain)>>();
        current.sort_by_key(|&(c, _)| c);
        current.iter().map(|&(_, c)| c).enumerate().map(|(i, c)| match c {
            Open => 0,
            Trees => 1,
            Lumberyard => 2,
        } * i as i64).sum()
    }

    fn resource_number(&self) -> usize {
        use self::Terrain::*;

        let trees = self.terrain.values().filter(|&&t| t == Trees).count();
        let lumberyards = self.terrain.values().filter(|&&t| t == Lumberyard).count();
        trees * lumberyards
    }

    // fn print(&self) {
    //     use self::Terrain::*;
    //     let max = self.terrain.keys().max().expect("print");
    //     for y in 0..=max.1 {
    //         for x in 0..=max.0 {
    //             let coor = &(x, y);
    //             match self.terrain.get(coor) {
    //                 Some(Open) => print!("."),
    //                 Some(Trees) => print!("|"),
    //                 Some(Lumberyard) => print!("#"),
    //                 None => unreachable!(),
    //             }
    //         }
    //         println!("");
    //     }
    // }
}

impl FromStr for Area {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut terrain_map = HashMap::new();

        for (y, row) in s.split('\n').enumerate() {
            for (x, c) in row.chars().enumerate() {
                let terrain = Terrain::from_char(c);
                terrain_map.insert((x as i32, y as i32), terrain);
            }
        }
        Ok(Area::new(terrain_map))
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut area: Area = input.parse()?;
    let end = 10;
    for _ in 0..(end+1) {
        area.tick();
    }

    let rn = area.resources.get(&end).expect("end found");
    Ok(*rn)
}


fn part2(input: &str) -> Result<usize> {
    let mut area: Area = input.parse()?;
    let end = 1_000_000_000;
    loop {
        if let Some(previous) = area.history.get(&area.terrain_id()) {
            let idx = previous + ((end - previous) % (area.pos - previous));
            let rn = area.resources.get(&idx).unwrap();
            break Ok(*rn);
        }
        area.tick();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.";

    #[test]
    fn test_part1() -> Result<()> {
        Ok(assert_eq!(part1(INPUT)?, 1147))
    }
}
