use aoc2018::{dispatch, Result};
use failure::err_msg;
use itertools::{Itertools, Product};
use lazy_static::lazy_static;
use nom::types::CompleteStr;
use nom::{call, digit, do_parse, error_position, map_res, named, tag};
use regex::Regex;
use std::collections::HashMap;
use std::ops::Range;

#[derive(Debug, PartialEq, Clone)]
struct Claim {
    top: usize,
    left: usize,
    width: usize,
    height: usize,
}

impl Claim {
    fn walk(self) -> Product<Range<usize>, Range<usize>> {
        (self.left..(self.left + self.width)).cartesian_product(self.top..(self.top + self.height))
    }
}


named!(pub positive_integer <CompleteStr, usize>,
    map_res!(digit, |d: CompleteStr| d.parse())
);

named!(parse<CompleteStr, Claim>,
  do_parse!(
    tag!("#")   >>
    positive_integer >>
    tag!(" @ ") >>
    left: positive_integer >>
    tag!(",") >>
    top: positive_integer >>
    tag!(": ") >>
    width: positive_integer >>
    tag!("x") >>
    height: positive_integer >>
    (Claim { top, left, width, height })
  )
);

fn main() {
    dispatch(&part1, &part2)
}

fn part1(input: &str) -> Result<usize> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"#\d+ @ (?P<left>\d+),(?P<top>\d+): (?P<width>\d+)+x(?P<height>\d+)+")
                .unwrap();
    }
    let mut fabric = HashMap::new();
    for row in input.split('\n') {
        let caps = RE.captures(row).unwrap();
        let claim = Claim {
            top: caps.name("top").ok_or(err_msg(""))?.as_str().parse()?,
            left: caps.name("left").ok_or(err_msg(""))?.as_str().parse()?,
            width: caps.name("width").ok_or(err_msg(""))?.as_str().parse()?,
            height: caps.name("height").ok_or(err_msg(""))?.as_str().parse()?,
        };
        for point in claim.walk() {
            let count = fabric.entry(point).or_insert(0);
            *count += 1;
        }
    }

    Ok(fabric.values().filter(|&x| *x > 1).count())
}

fn part2(_input: &str) -> Result<i32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_integer() -> Result<()> {
        (assert_eq!(
            positive_integer(CompleteStr("12")),
            Ok((CompleteStr(""), 12))
        ));
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        (assert_eq!(
            parse(CompleteStr("#12 @ 385,951: 10x7")),
            Ok((
                CompleteStr(""),
                Claim {
                    top: 951,
                    left: 385,
                    width: 10,
                    height: 7
                }
            ))
        ));
        Ok(())
    }
}
