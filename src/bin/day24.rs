use aoc2018::{dispatch, Result};
use lazy_static::lazy_static;
use regex::{CaptureMatches, Captures, Regex};
use std::collections::{HashSet, HashMap};

fn main() {
    dispatch(&part1, &part2)
}

#[derive(Debug, Clone)]
struct Reindeer {
    // immune_system: Vec<Unit>,
    // infection: Vec<Unit>,
    units: Vec<Unit>,
}

impl Reindeer {
    fn new(input: &str) -> Self {
        let immune_system_start = input.find("Immune System:").unwrap();
        let infection_start = input.find("Infection").unwrap();

        let mut immune_system: Vec<_> = UnitWalker::new(UnitType::ImmuneSystem, &input[immune_system_start..infection_start]).collect();

        let mut infection: Vec<_> = UnitWalker::new(UnitType::Infection, &input[infection_start..]).collect();
        let mut units = vec![];
        units.append(&mut immune_system);
        units.append(&mut infection);
        // Reindeer {immune_system, infection}
        Reindeer {units}
    }

    fn fight(&mut self) {
        // target selection
        // let immune_system_target_map = HashMap::new();
        // let infection_target_map = HashMap::new();
        let mut target_map = HashMap::new();
        let mut target_set = TargetSet::new();
        let mut units = self.units.clone();
        let (_immune_system, _infection): (Vec<_>, Vec<_>) = self.units.iter().partition(|u| match u.unit_type {UnitType::ImmuneSystem => true, UnitType::Infection => false});
        units.sort_by_key(|u| (u.effective_power(), u.initiative));
        units.reverse();
        for unit in units.iter() {
            // let target_map = target_map.for_type(unit.unit_type);
            // let target_set = target_set.for_type(unit.unit_type);
            let mut units_left: Vec<_> = self.units.iter().filter(|&u| u.unit_type != unit.unit_type).filter(|&u| !target_set.contains_for_type(unit.unit_type, &u.id)).collect();
            units_left.sort_by_key(|&u| (unit.damage_to(u), u.effective_power(), u.initiative));
            units_left.reverse();
            println!("{:?} {} left {:?}", unit.unit_type, unit.id, units_left.iter().map(|u| (u.id, unit.damage_to(u))).collect::<Vec<_>>());
            if let Some(best) = units_left.first() {
                target_map.insert((unit.unit_type, unit.id), (best.unit_type, best.id));
                target_set.insert_for_type(unit.unit_type, best.id);
                println!("{:?} {} -> {}", unit.unit_type, unit.id, best.id);
            }
        }
        units.sort_by_key(|u| u.initiative);
        for unit in units {
            if let Some(_target) = target_map.get(&(unit.unit_type, unit.id)) {

            }
        }
    }
}

// struct TargetMap {
//     immune_system: HashMap<usize, usize>,
//     infection: HashMap<usize, usize>,
// }

// impl TargetMap {
//     fn new() -> Self {
//         TargetMap{ immune_system: HashMap::new(),
//         infection: HashMap::new(),}
//     }

//     // fn for_type(&self, unit_type: UnitType) -> &mut HashMap<usize, usize> {
//     //     match unit_type {
//     //         UnitType::ImmuneSystem => &mut self.immune_system,
//     //         UnitType::Infection => &mut self.infection,
//     //     }
//     // }
// }

struct TargetSet {
    immune_system: HashSet<usize>,
    infection: HashSet<usize>,
}

impl TargetSet {
    fn new() -> Self {
        TargetSet{ immune_system: HashSet::new(),
        infection: HashSet::new(),}
    }

    fn contains_for_type(&mut self, unit_type: UnitType, key: &usize) -> bool {
        match unit_type {
            UnitType::ImmuneSystem => self.immune_system.contains(key),
            UnitType::Infection => self.infection.contains(key),
        }
    }


    fn insert_for_type(&mut self, unit_type: UnitType, key: usize) {
        match unit_type {
            UnitType::ImmuneSystem => self.immune_system.insert(key),
            UnitType::Infection => self.infection.insert(key),
        };
    }

    // fn for_type(&self, unit_type: UnitType) -> &mut HashSet<usize> {
    //     match unit_type {
    //         UnitType::ImmuneSystem => &mut self.immune_system,
    //         UnitType::Infection => &mut self.infection,
    //     }
    // }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum UnitType {
    ImmuneSystem,
    Infection,
}

#[derive(Debug, Clone)]
struct Unit {
    id: usize,
    units: u32,
    hit_points: u32,
    immune: Vec<String>,
    weak: Vec<String>,
    damage: u32,
    damage_type: String,
    initiative: u32,
    unit_type: UnitType,
}

impl Unit {
    fn effective_power(&self) -> u32 {
        self.units * self.damage
    }

    fn damage_to(&self, other: &Unit) -> u32 {
        let factor = if other.immune.contains(&self.damage_type) {
            0
        } else
        if other.weak.contains(&self.damage_type) {
            2
        } else {
            1
        };
        factor * self.effective_power()
    }

}

struct UnitWalker<'r, 't> {
    caps: CaptureMatches<'r, 't>,
    unit_type: UnitType,
    counter: usize,
}

impl<'r, 't> UnitWalker<'r, 't> {
    fn new<'s: 't + 'r>(unit_type: UnitType, s: &'s str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?P<units>\d+) units each with (?P<hit_points>\d+) hit points (\((?P<key1>\w+) to (?P<values1>(\w+)(, \w+)*)(; (?P<key2>\w+) to (?P<values2>(\w+)(, \w+)*))?\) )?with an attack that does (?P<damage>\d+) (?P<damage_type>\w+) damage at initiative (?P<initiative>\d+)").unwrap();
        }

        let caps = RE.captures_iter(s);
        UnitWalker { caps, unit_type, counter: 0 }
    }
}

impl<'r, 't> Iterator for UnitWalker<'r, 't> {
    type Item = Unit;

    fn next(&mut self) -> Option<Unit> {
        if let Some(caps) = self.caps.next() {
            fn get_cap_str(caps: &Captures, name: &str) -> Option<String> {
                caps.name(name).map(|m| m.as_str().into())
            }
            fn get_cap_str_split(caps: &Captures, name: &str) -> Vec<String> {
                match caps.name(name) {
                    None => vec![],
                    Some(m) => m.as_str().split(", ").map(|s| s.into()).collect::<Vec<_>>(),
                }
            }
            fn get_cap_int(caps: &Captures, name: &str) -> u32 {
                caps.name(name)
                    .expect("regex match fail")
                    .as_str()
                    .parse()
                    .expect("cap parse fail")
            }
            let mut immune = vec![];
            let mut weak = vec![];
            for pos in 1..=2 {
                let key_name = format!("key{}", pos);
                let value_name = format!("values{}", pos);
                if let Some(key) = get_cap_str(&caps, &key_name) {
                    let values = get_cap_str_split(&caps, &value_name);
                    if key == "immune" {
                        immune = values;
                    } else if key == "weak" {
                        weak = values;
                    } else {
                        panic!("unexpected key");
                    }
                }
            }
            self.counter += 1;
            Some(Unit {
                id: self.counter,
                units: get_cap_int(&caps, "units"),
                hit_points: get_cap_int(&caps, "hit_points"),
                immune,
                weak,
                damage: get_cap_int(&caps, "damage"),
                damage_type: get_cap_str(&caps, "damage_type").unwrap(),
                initiative: get_cap_int(&caps, "initiative"),
                unit_type: self.unit_type.clone(),
            })
        } else {
            None
        }
    }
}

fn part1(input: &str) -> Result<i32> {
    let mut reindeer = Reindeer::new(input);
    // println!("{:?}", reindeer);
    reindeer.fight();
    Ok(1)
}

fn part2(_input: &str) -> Result<i32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        Ok(assert_eq!(part1("Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4")?, 0))
    }
}
