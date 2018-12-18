use aoc2018::{dispatch, Result};
use failure::Error;
use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    dispatch(&part1, &part2)
}

enum UnitType {
    Elf,
    Goblin,
}

enum Terrain {
    Wall,
    Open,
}

struct Input {
    terrain: Terrain,
    unit_type: Option<UnitType>,
}

type Coor = (usize, usize);

struct Unit {
    unit_type: UnitType,
}

impl Unit {
    fn new(unit_type: UnitType) -> Self {
        Unit { unit_type }
    }
}

struct Game {
    units: HashMap<Coor, Unit>,
    terrain: HashMap<Coor, Terrain>,
}

impl Game {
    fn round(&mut self) {
        let mut order: Vec<_> = self.units.iter().collect();
        order.sort_by_key(|((x,y), _)| (y, x));
        for (coor, _) in order {
            let mut neighbour_units = vec![];
            for neighbour in self.adjacent(&coor).iter() {
                if let Some(neighbour_unit) = self.units.get(neighbour) {
                    neighbour_units.push(neighbour_unit.clone());
                }
            }
            if neighbour_units.len() > 0 {
                let unit = self.units.get(coor).expect("get unit");
                // self.choose_and_attack(unit, *neighbour_units);
            }
        }

    }

    // fn choose_and_attack(&mut self, unit: &Unit, neighbour_units: &[Unit]) {

    // }

    fn adjacent(&self, coor: &Coor) -> [Coor; 4] {
        let (x, y) = *coor;
        [(x, y-1), (x- 1, y), (x+1, y), (x, y+1)]
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
