use aoc2018::{dispatch, Result};
use failure::Error;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

fn main() {
    dispatch(&part1, &part2)
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    first: char,
    then: char,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"Step (.) must be finished before step (.) can begin.")
                    .expect("regex create");
        }

        let caps = RE.captures(s).expect("regex match");
        Ok(Instruction {
            first: caps[1].chars().next().expect("regex match 1"),
            then: caps[2].chars().next().expect("regex match 2"),
        })
    }
}

fn get_dependencies(input: &str) -> (HashMap<char, HashSet<char>>, HashSet<char>) {
    let instructions: Vec<Instruction> = input
        .split('\n')
        .filter_map(|row| row.parse().ok())
        .collect();
    let mut dependencies: HashMap<char, HashSet<char>> = HashMap::new();
    let mut all = HashSet::new();
    for instruction in instructions {
        all.insert(instruction.first);
        all.insert(instruction.then);
        let entry = dependencies
            .entry(instruction.then)
            .or_insert(HashSet::new());
        (*entry).insert(instruction.first);
    }
    (dependencies, all)
}

fn part1(input: &str) -> Result<String> {
    let (mut dependencies, mut all) = get_dependencies(input);
    let mut steps = vec![];
    loop {
        if all.is_empty() {
            break;
        }
        let dependees: HashSet<char> = dependencies.keys().map(|c| *c).collect();
        let mut ready: Vec<&char> = all.difference(&dependees).collect();
        ready.sort();
        let next = ready[0].clone();
        all.remove(&next);
        steps.push(next);
        let keys = dependencies.keys().map(|c| *c).collect::<Vec<_>>();
        for key in keys {
            let value = dependencies.get_mut(&key).unwrap();
            value.remove(&next);
            if value.is_empty() {
                dependencies.remove(&key);
            }
        }
    }
    Ok(steps.iter().collect())
}

fn part2(_input: &str) -> Result<i32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

    #[test]
    fn test_part1() -> Result<()> {
        Ok(assert_eq!(part1(INPUT)?, "CABDFE"))
    }
}
