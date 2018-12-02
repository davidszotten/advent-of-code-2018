use aoc2018::{dispatch, Result};
use std::collections::HashMap;

fn main() {
    dispatch(&part1, &part2)
}

fn part1(input: &str) -> Result<i32> {
    let mut found2s = 0;
    let mut found3s = 0;
    for row in input.split('\n') {
        let mut chars = HashMap::new();
        for c in row.chars() {
            let count = chars.entry(c).or_insert(0);
            *count += 1;
        }
        let mut found2 = false;
        let mut found3 = false;
        for v in chars.values() {
            match v {
                2 => found2 = true,
                3 => found3 = true,
                _ => {}
            }
        }
        if found2 {
            found2s += 1;
        }
        if found3 {
            found3s += 1;
        }
    }
    Ok(found2s * found3s)
}

fn part2(_input: &str) -> Result<i32> {
    let mut c = 0;
    for _ in vec![1, 2, 3].iter().cycle() {
        c += 1;
        if c == 10 {
            return Ok(c);
        }
    }
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
