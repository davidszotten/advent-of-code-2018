use aoc2018::{dispatch, Result};
use lazy_static::lazy_static;
use regex::{CaptureMatches, Captures, Regex};

fn main() {
    dispatch(&part1, &part2)
}

#[derive(Debug, Clone)]
struct Unit {
    units: u32,
    hit_points: u32,
    immune: Vec<String>,
    weak: Vec<String>,
    damage: u32,
    damage_type: String,
    initiative: u32,
}

struct UnitWalker<'r, 't> {
    caps: CaptureMatches<'r, 't>,
}

impl<'r, 't> UnitWalker<'r, 't> {
    fn new<'s: 't + 'r>(s: &'s str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?P<units>\d+) units each with (?P<hit_points>\d+) hit points (\((?P<key1>\w+) to (?P<values1>(\w+)(, \w+)*)(; (?P<key2>\w+) to (?P<values2>(\w+)(, \w+)*))?\) )?with an attack that does (?P<damage>\d+) (?P<damage_type>\w+) damage at initiative (?P<initiative>\d+)").unwrap();
        }

        let caps = RE.captures_iter(s);
        UnitWalker { caps }
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
            Some(Unit {
                units: get_cap_int(&caps, "units"),
                hit_points: get_cap_int(&caps, "hit_points"),
                immune,
                weak,
                damage: get_cap_int(&caps, "damage"),
                damage_type: get_cap_str(&caps, "damage_type").unwrap(),
                initiative: get_cap_int(&caps, "initiative"),
            })
        } else {
            None
        }
    }
}

fn part1(input: &str) -> Result<i32> {
    let units: Vec<_> = UnitWalker::new(input).collect();
    println!("{}", units.len());
    for unit in UnitWalker::new(input) {
        println!("{:?}", unit);
    }
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
