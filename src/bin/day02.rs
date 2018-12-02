use aoc2018::{dispatch, Result};
use std::collections::HashMap;
use std::ops;

fn main() {
    dispatch(&part1, &part2)
}

#[derive(Debug, PartialEq)]
struct Found {
    found2: u32,
    found3: u32,
}

impl Found {
    fn new() -> Self {
        Found {
            found2: 0,
            found3: 0,
        }
    }
}

impl ops::Add for Found {
    type Output = Found;

    fn add(self, other: Found) -> Found {
        Found {
            found2: self.found2 + other.found2,
            found3: self.found3 + other.found3,
        }
    }
}

impl ops::AddAssign for Found {
    fn add_assign(&mut self, other: Found) {
        *self = Found {
            found2: self.found2 + other.found2,
            found3: self.found3 + other.found3,
        }
    }
}

fn find(input: &str) -> Found {
    let mut chars = HashMap::new();
    for c in input.chars() {
        let count = chars.entry(c).or_insert(0);
        *count += 1;
    }
    let mut found2 = 0;
    let mut found3 = 0;
    for v in chars.values() {
        match v {
            2 => found2 = 1,
            3 => found3 = 1,
            _ => {}
        }
    }
    Found { found2, found3 }
}

fn part1(input: &str) -> Result<i32> {
    let mut found = Found::new();
    for row in input.split('\n') {
        found += find(row);
    }
    Ok((found.found2 * found.found3) as i32)
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
    fn test_find() {
        assert_eq!(
            find("abcdef"),
            Found {
                found2: false,
                found3: false
            }
        );
        assert_eq!(
            find("bababc"),
            Found {
                found2: true,
                found3: true
            }
        );
        assert_eq!(
            find("abbcde"),
            Found {
                found2: true,
                found3: false
            }
        );
        assert_eq!(
            find("abcccd"),
            Found {
                found2: false,
                found3: true
            }
        );
        assert_eq!(
            find("aabcdd"),
            Found {
                found2: true,
                found3: false
            }
        );
        assert_eq!(
            find("abcdee"),
            Found {
                found2: true,
                found3: false
            }
        );
        assert_eq!(
            find("ababab"),
            Found {
                found2: false,
                found3: true
            }
        );
    }
}
