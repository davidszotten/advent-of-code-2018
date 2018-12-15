use aoc2018::{dispatch, Result};
use failure::Error;
use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    dispatch(&part1, &part2)
}

struct Rule {
    before: Vec<bool>,
    after: bool,
}

fn char_to_bool(c: char) -> Option<bool> {
    match c {
        '.' => Some(false),
        '#' => Some(true),
        _ => None,
    }
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split_whitespace();
        let left: &str = parts.nth(0).expect("left");
        let right = parts.nth(1).expect("right");

        let before = left.chars().filter_map(char_to_bool).collect::<Vec<_>>();
        let after = right
            .chars()
            .filter_map(char_to_bool)
            .nth(0)
            .expect("after");

        Ok(Rule { before, after })
    }
}

fn part1(input: &str) -> Result<i32> {
    let mut rows = input.split('\n');
    let initial = rows
        .next()
        .expect("next")
        .split_whitespace()
        .nth(2)
        .expect("1")
        .chars()
        .filter_map(char_to_bool)
        .collect::<Vec<_>>();
    rows.next();
    let rules: Vec<Rule> = rows.filter_map(|row| row.parse().ok()).collect();
    let rule_map: HashMap<_, bool> = rules.iter().map(|r| (&r.before[..], r.after)).collect();
    // println!("{:?}", rule_map);
    // println!("{:?}", initial.iter().map(|&b| if b {'#'} else {'.'}).collect::<String>());
    let mut pots = vec![false; 60];
    pots.extend(&initial);
    pots.extend(&vec![false; 60]);
    // println!("{}", pots.len());
    for _ in 0..20 {
        // fn foo(w: Vec<char>) -> char {
        let foo = |w| {
            // println!("{:?}", w);
            *rule_map.get(&w).unwrap_or(&false)
        };
        pots = pots.windows(5).map(foo).collect();
        // initial = initial.windows(5).map(|w| .expect("missing")).collect();
        // println!("{}", pots.len());
    }
    // println!("{}", pots.iter().map(|&b| if b {'#'} else {'.'}).collect::<String>());
    // println!("{:?}", (-20..(20+initial.len() as i32)).zip(pots.clone()).map(|(i, p)| if p {i} else {0}).collect::<Vec<_>>());
    Ok((-20..(20 + initial.len() as i32))
        .zip(pots)
        .map(|(i, p)| if p { i } else { 0 })
        .sum())
}

fn part2(input: &str) -> Result<String> {
    let mut rows = input.split('\n');
    let initial = rows
        .next()
        .expect("next")
        .split_whitespace()
        .nth(2)
        .expect("1")
        .chars()
        .filter_map(char_to_bool)
        .collect::<Vec<_>>();
    rows.next();
    let rules: Vec<Rule> = rows.filter_map(|row| row.parse().ok()).collect();
    let rule_map: HashMap<_, bool> = rules.iter().map(|r| (&r.before[..], r.after)).collect();
    let mut start_pos = 0;
    let mut prev;
    let mut prev_score = 0;
    let mut pots = vec![];
    pots.extend(&initial);
    let mut generation = 1;
    loop {
        let step = |w| *rule_map.get(&w).unwrap_or(&false);
        prev = pots.clone();
        for _ in 0..4 {
            pots.insert(0, false);
            pots.push(false);
        }
        start_pos -= 2;
        pots = pots.windows(5).map(step).collect();
        let score = pots
            .iter()
            .enumerate()
            .map(|(i, &p)| if p { i as i64 + start_pos } else { 0 })
            .sum();

        if prev.iter().zip(pots.iter().skip(3)).all(|(a, b)| a == b) {
            return Ok(format!(
                "{}",
                score + (50_000_000_000 - generation) * (score - prev_score)
            ));
        }
        prev_score = score;
        generation += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";

    #[test]
    fn test_part1() -> Result<()> {
        Ok(assert_eq!(part1(INPUT)?, 325))
    }
}
