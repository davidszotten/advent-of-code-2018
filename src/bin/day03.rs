use aoc2018::{dispatch, Result};
use failure::{err_msg, Error};
use itertools::{Itertools, Product};
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::collections::HashMap;
use std::ops::Range;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
struct Claim {
    id: usize,
    top: usize,
    left: usize,
    width: usize,
    height: usize,
}

impl Claim {
    fn walk(&self) -> Product<Range<usize>, Range<usize>> {
        (self.left..(self.left + self.width)).cartesian_product(self.top..(self.top + self.height))
    }
}

impl FromStr for Claim {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"#(?P<id>\d+) @ (?P<left>\d+),(?P<top>\d+): (?P<width>\d+)+x(?P<height>\d+)+"
            )
            .unwrap();
        }

        let caps = RE.captures(s).unwrap();
        fn get_cap_int(caps: &Captures, name: &str) -> Result<usize> {
            Ok(caps
                .name(name)
                .ok_or(err_msg("parse fail"))?
                .as_str()
                .parse()?)
        }
        Ok(Claim {
            id: get_cap_int(&caps, "id")?,
            top: get_cap_int(&caps, "top")?,
            left: get_cap_int(&caps, "left")?,
            width: get_cap_int(&caps, "width")?,
            height: get_cap_int(&caps, "height")?,
        })
    }
}

fn main() {
    dispatch(&part1, &part2)
}

fn part1(input: &str) -> Result<usize> {
    let mut fabric = HashMap::new();
    for row in input.split('\n') {
        let claim: Claim = row.parse()?;
        for point in claim.walk() {
            let count = fabric.entry(point).or_insert(0);
            *count += 1;
        }
    }

    Ok(fabric.values().filter(|&x| *x > 1).count())
}

fn part2(input: &str) -> Result<usize> {
    let mut fabric = HashMap::new();
    for row in input.split('\n') {
        let claim: Claim = row.parse()?;
        for point in claim.walk() {
            let count = fabric.entry(point).or_insert(0);
            *count += 1;
        }
    }

    for row in input.split('\n') {
        let claim: Claim = row.parse()?;
        if claim
            .walk()
            .filter_map(|point| fabric.get(&point))
            .all(|&count| count == 1)
        {
            return Ok(claim.id);
        }
    }
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() -> Result<()> {
        let claim: Claim = "#12 @ 385,951: 10x7".parse()?;
        assert_eq!(
            claim,
            Claim {
                id: 12,
                top: 951,
                left: 385,
                width: 10,
                height: 7
            }
        );
        Ok(())
    }
}
