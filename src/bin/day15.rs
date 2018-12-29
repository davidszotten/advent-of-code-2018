use aoc2018::{dispatch, Result};
use failure::Error;
use std::collections::{HashMap, VecDeque, HashSet};
use std::str::FromStr;
use std::mem;

fn main() {
    dispatch(&part1, &part2)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UnitType {
    Elf,
    Goblin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Terrain {
    Wall,
    Open,
}

struct Input {
    terrain: Terrain,
    unit_type: Option<UnitType>,
}

type Coor = (usize, usize);

#[derive(Debug, Clone, Copy)]
struct Unit {
    hit_points: i16,
    unit_type: UnitType,
}

impl Unit {
    fn new(unit_type: UnitType) -> Self {
        Unit { unit_type, hit_points: 200 }
    }
}

struct Game {
    units: HashMap<Coor, Unit>,
    terrain: HashMap<Coor, Terrain>,
}

impl Game {
    fn round(&mut self) -> Option<()> {
        // println!("{:?}", self.units);
        let mut order: Vec<_> = self.units.keys().map(|&c| c).collect();
        order.sort_by_key(|&(x, y)| (y, x));
        for coor in order {
            let unit = self.units.get(&coor).expect("missing at 1");

            // TODO: make sure there are enemies remaining

            let mut neighbour_units = vec![];
            for neighbour in self.adjacent(&coor).iter() {
                if let Some(neighbour_unit) = self.units.get(neighbour) {
                    if neighbour_unit.unit_type != unit.unit_type {
                        // neighbour_units.push(neighbour_unit.clone());
                        neighbour_units.push(*neighbour);
                    }
                }
            }
            if neighbour_units.len() > 0 {
                // println!("attack: {:?}", coor);
                // let unit = self.units.get(coor).expect("get unit");
                self.choose_and_attack(&neighbour_units[..]);
            } else {
                // println!("move: {:?}", coor);
                self.choose_and_move(coor);
            }
        }
        Some(())
    }

    fn choose_and_attack(&mut self, neighbour_units: &[Coor]) {
        let chosen_coor = neighbour_units.iter().min_by_key(|&u| self.units.get(u).unwrap().hit_points).unwrap();
        let chosen_unit = self.units.get_mut(chosen_coor).unwrap();
        chosen_unit.hit_points -= 3;
    }

    fn choose_and_move(&mut self, unit_coor: Coor) {
        let unit = self.units.get(&unit_coor).expect("missing at 2");
        let distances = self.distances(unit_coor);

        let target_coors = self.units.iter().filter(|(_, target)| target.unit_type != unit.unit_type).map(|(coor, _)| coor);
        let mut in_range = vec![];
        for target_coor in target_coors {
                in_range.append(&mut self.in_range(*target_coor));
        }
        let mut reachable = vec![];
        let mut min_distance = None;
        for in_range_coor in in_range {
            if let Some(distance) = distances.get(&in_range_coor) {
                if let Some(current_min) = min_distance {
                    if distance < current_min {
                        min_distance = Some(distance)
                    }
                } else {
                    min_distance = Some(distance);
                }
                reachable.push((distance, in_range_coor));
            }
        }
        let chosen = match min_distance {
            None => None,
            Some(ref min_distance) => {
                reachable.iter().filter(|&(d, _)| d == min_distance).map(|&(_, c)| c).min_by_key(|&(x, y)| (y, x))
            },
        };
        if let Some(coor) = chosen {
            let new_coor = self.next_step(unit_coor, coor);
            // println!("{:?} -> {:?} ({:?})", unit_coor, coor, new_coor);
            let unit = self.units.remove(&unit_coor).expect("unit missing");
            self.units.insert(new_coor, unit);
        } else {
            return
        }
    }

    fn in_range(&self, coor: Coor) -> Vec<Coor> {
        let mut res = vec![];
        for neighbour in self.adjacent(&coor).iter() {
            if let Some(_) = self.units.get(neighbour) {
                continue;
            }
            res.push(*neighbour);
        }
        res
    }

    fn distances(&self, coor: Coor) -> HashMap<Coor, usize> {
        let mut queue = VecDeque::new();
        let mut distances = HashMap::new();
        queue.push_back(coor);
        distances.insert(coor, 0);
        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            let current_distance = distances
                .get(&current)
                .expect("current missing in distances")
                .clone();

            for neighbour in self.adjacent(&current).iter() {
                if let Some(_) = self.units.get(neighbour) {
                    continue;
                }
                if self.terrain.get(neighbour) != Some(&Terrain::Open) {
                    continue;
                }

                distances.entry(*neighbour).or_insert_with(|| {
                    if queue.push_back(*neighbour) == () {
                        current_distance + 1
                    } else {
                        panic!("haxx")
                    }
                });
            }
        }
        distances
    }

    fn next_step(&self, from: Coor, to: Coor) -> Coor {
        // find all shortest paths

        let mut queue = VecDeque::new();
        let mut distances = HashMap::new();
        let mut paths: HashMap<Coor, Vec<Coor>> = HashMap::new();
        queue.push_back(from);
        distances.insert(from, 0);
        paths.insert(from, vec![]);
        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            let current_distance = distances
                .get(&current)
                .expect("current missing in distances")
                .clone();

            if let Some(&to_distance) = distances.get(&to) {
                if to_distance < current_distance {
                    break;
                }
            }

            for neighbour in self.adjacent(&current).iter() {
                if let Some(_) = self.units.get(neighbour) {
                    continue;
                }
                if self.terrain.get(neighbour) != Some(&Terrain::Open) {
                    continue;
                }

                match distances.get(neighbour) {
                    None => {
                        queue.push_back(*neighbour);
                        distances.insert(*neighbour, current_distance + 1);
                        let path = paths.entry(*neighbour).or_insert(vec![]);
                        (*path).push(current);
                    },
                    Some(distance) => {
                        if *distance == current_distance + 1 {
                            let path = paths.entry(*neighbour).or_insert(vec![]);
                            (*path).push(current);
                        }

                    }
                }
            }
        }
        let mut possible_paths: HashSet<Coor> = HashSet::new();
        // for path in paths.get(&to).expect("no shortest path found") {
        //     possible_paths.insert(*path);
        // }
        possible_paths.insert(to);
        let mut new_possible: HashSet<Coor> = HashSet::new();
        // println!("{:?} -> {:?}:  {:?}", from, to, possible_paths);
        'outer: loop {
            new_possible.clear();
            for possible in possible_paths.iter() {
                let next_paths: Vec<Coor> = paths.get(&possible).expect("shortest missing").clone();
                for path in next_paths.iter() {
                    // all paths are the same distance from `from` so if any
                    // matches we are done
                    if *path == from {
                        break 'outer;
                    }
                    new_possible.insert(*path);
                }
            }
            mem::swap(&mut possible_paths, &mut new_possible);
            // println!("pp: {:?}", possible_paths);
        }
        *possible_paths.iter().min_by_key(|&(x, y)| (y, x)).expect("shortest path is empty")
    }

    fn adjacent(&self, coor: &Coor) -> [Coor; 4] {
        let (x, y) = *coor;
        [(x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1)]
    }

    fn print(&self) {
        use self::Terrain::*;
        use self::UnitType::*;
        let max = self.terrain.keys().max().unwrap();
        for y in 0..=max.1 {
            for x in 0..=max.0 {
                let coor = &(x, y);
                if let Some(unit) = self.units.get(coor) {
                    match unit.unit_type {
                        Goblin => print!("G"),
                        Elf => print!("E"),
                    }
                } else {
                    match self.terrain.get(coor) {
                        Some(Open) => print!("."),
                        Some(Wall) => print!("#"),
                        None => unreachable!(),
                    }
                }
            }
            println!("");
        }
    }
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut terrain = HashMap::new();
        let mut units = HashMap::new();

        for (y, row) in s.split('\n').enumerate() {
            for (x, c) in row.chars().enumerate() {
                let input = Input::from_char(c);
                terrain.insert((x, y), input.terrain);
                if let Some(unit_type) = input.unit_type {
                    units.insert((x, y), Unit::new(unit_type));
                }
            }
        }
        Ok(Game { terrain, units })
    }
}

impl Input {
    fn from_char(c: char) -> Self {
        use self::Terrain::*;
        use self::UnitType::*;
        match c {
            '#' => Input {
                terrain: Wall,
                unit_type: None,
            },
            '.' => Input {
                terrain: Open,
                unit_type: None,
            },
            'G' => Input {
                terrain: Open,
                unit_type: Some(Goblin),
            },
            'E' => Input {
                terrain: Open,
                unit_type: Some(Elf),
            },
            _ => unreachable!(),
        }
    }
}

fn part1(input: &str) -> Result<i32> {
    let mut game: Game = input.parse()?;
    game.print();
    game.round();
    game.print();
    game.round();
    game.print();
    game.round();
    game.print();
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
        Ok(assert_eq!(part1("")?, 0))
    }
}
