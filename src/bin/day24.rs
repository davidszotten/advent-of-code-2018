use aoc2018::{dispatch, Result};
use lazy_static::lazy_static;
use regex::{CaptureMatches, Captures, Regex};
use std::cmp::min;
use std::collections::{HashMap, HashSet};

fn main() {
    dispatch(&part1, &part2)
}

#[derive(Debug, Clone)]
struct Reindeer {
    units: HashMap<UnitKey, Unit>,
}

impl Reindeer {
    fn new(input: &str, boost: u32) -> Self {
        let immune_system_start = input.find("Immune System:").unwrap();
        let infection_start = input.find("Infection").unwrap();

        let mut units = HashMap::new();
        for mut unit in UnitWalker::new(
            UnitType::ImmuneSystem,
            &input[immune_system_start..infection_start],
        ) {
            unit.damage += boost;
            units.insert((unit.unit_type, unit.id), unit);
        }

        for unit in UnitWalker::new(UnitType::Infection, &input[infection_start..]) {
            units.insert((unit.unit_type, unit.id), unit);
        }
        Reindeer { units }
    }

    fn target(&self) -> HashMap<UnitKey, UnitKey> {
        let mut target_map = HashMap::new();
        let mut target_set = TargetSet::new();
        let mut units = self.units.values().collect::<Vec<_>>();
        units.sort_by_key(|u| (u.effective_power(), u.initiative));
        units.reverse();
        // println!("{:?}", self.units);
        // println!("order: {:?}", units.iter().map(|u| (u.unit_type, u.id, u.effective_power(), u.initiative)).collect::<Vec<_>>());
        for unit in units.iter() {
            let mut units_left: Vec<_> = self
                .units
                .values()
                .filter(|&u| u.unit_type != unit.unit_type)
                .filter(|&u| u.units > 0)
                .filter(|&u| unit.damage_to(u) > 0)
                .filter(|&u| !target_set.contains_for_type(unit.unit_type, &u.id))
                .collect();
            units_left.sort_by_key(|&u| (unit.damage_to(u), u.effective_power(), u.initiative));
            units_left.reverse();
            // println!(
            //     "{:?} {} left {:?}",
            //     unit.unit_type,
            //     unit.id,
            //     units_left
            //         .iter()
            //         .map(|u| (u.id, unit.damage_to(u)))
            //         .collect::<Vec<_>>()
            // );
            if let Some(best) = units_left.first() {
                target_map.insert((unit.unit_type, unit.id), (best.unit_type, best.id));
                target_set.insert_for_type(unit.unit_type, best.id);
                // println!("{:?} {} -> {}", unit.unit_type, unit.id, best.id);
            }
        }

        target_map
    }

    fn fight(&mut self) -> u32 {
        loop {
            // println!("");
            let (immune_system, infection): (Vec<_>, Vec<_>) =
                self.units.values().partition(|u| match u.unit_type {
                    UnitType::ImmuneSystem => true,
                    UnitType::Infection => false,
                });
            let immune_remaining = immune_system.iter().map(|u| u.units).sum::<u32>();
            let infection_remaining = infection.iter().map(|u| u.units).sum::<u32>();
            // println!("{}, {}", immune_remaining, infection_remaining);
            if immune_remaining == 0 || infection_remaining == 0 {
                break;
            }
            let target_map = self.target();
            let mut unit_order: Vec<_> = self
                .units
                .values()
                .map(|u| (u.initiative, u.key()))
                .collect();
            unit_order.sort_by_key(|t| t.0);
            unit_order.reverse();

            for (_, unit_key) in unit_order {
                let unit = self.units.get(&unit_key).expect("attacker missing").clone();
                if unit.units == 0 {
                    continue;
                }
                if let Some(target_key) = target_map.get(&unit_key) {
                    if let Some(entry) = self.units.get_mut(target_key) {
                        (*entry).attacked_by(&unit);
                    } else {
                        panic!("target missing");
                    }
                }
            }
        }
        self.units.values().map(|u| u.units).sum()
    }

    fn remaining(&self) -> UnitType {
        self.units.values().filter(|u| u.units > 0).next().unwrap().unit_type
    }
}

struct TargetSet {
    immune_system: HashSet<usize>,
    infection: HashSet<usize>,
}

impl TargetSet {
    fn new() -> Self {
        TargetSet {
            immune_system: HashSet::new(),
            infection: HashSet::new(),
        }
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum UnitType {
    ImmuneSystem,
    Infection,
}

type UnitKey = (UnitType, usize);

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
        } else if other.weak.contains(&self.damage_type) {
            2
        } else {
            1
        };
        factor * self.effective_power()
    }

    fn attacked_by(&mut self, other: &Unit) {
        let damage = other.damage_to(self);
        let potential_killed_units = damage / self.hit_points;
        let killed_units = min(potential_killed_units, self.units);
        // println!("{:?} {} attacks {}: damage: {}, units: {}, hit points: {}, potential: {}, killed {}", other.unit_type, other.id, self.id, damage, self.units, self.hit_points, potential_killed_units, killed_units);
        self.units -= killed_units;
    }

    fn key(&self) -> UnitKey {
        (self.unit_type, self.id)
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
        UnitWalker {
            caps,
            unit_type,
            counter: 0,
        }
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

fn part1(input: &str) -> Result<u32> {
    let mut reindeer = Reindeer::new(input, 0);
    // println!("{:?}", reindeer);
    Ok(reindeer.fight())
}

fn part2(input: &str) -> Result<u32> {
    let mut boost = 61;
    Ok(loop {
        let mut reindeer = Reindeer::new(input, boost);
        let res = reindeer.fight();
        if reindeer.remaining() == UnitType::ImmuneSystem {
            break res
        }
        boost += 1;
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";

    #[test]
    fn test_part1() -> Result<()> {
        Ok(assert_eq!(part1(INPUT)?, 5216))
    }

    #[test]
    fn test_boost() {
        let mut reindeer = Reindeer::new(INPUT, 1570);
        // println!("{:?}", reindeer);
        let res = reindeer.fight();
        assert_eq!(reindeer.remaining(), UnitType::ImmuneSystem);
        assert_eq!(res, 51);
    }
}
