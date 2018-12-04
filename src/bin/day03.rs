use aoc2018::{dispatch, Result};
use nom::{call, digit, do_parse, error_position, map_res, named, tag};
// use std::str::{self, FromStr};
use nom::types::CompleteStr;

#[derive(Debug, PartialEq, Clone)]
struct Claim {
    top: usize,
    left: usize,
    width: usize,
    height: usize,
}

impl Claim {
    fn walk(self) -> ClaimWalker {
        ClaimWalker {
            claim: self.clone(),
            y: self.top,
            x: self.left,
        }
    }
}

struct ClaimWalker {
    claim: Claim,
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Iterator for ClaimWalker {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        let res = Some(Point {
            x: self.x,
            y: self.y,
        });

        if self.y >= self.claim.top + self.claim.height {
            return None;
        }
        self.x += 1;

        if self.x >= self.claim.left + self.claim.width {
            self.x = self.claim.left;
            self.y += 1;
        }

        res
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

fn part1(_input: &str) -> Result<i32> {
    let (_, claim) = parse(CompleteStr("#123 @ 3,2: 5x4"))?;
    let v: Vec<_> = claim.walk().collect();
    println!("{:?}", v);
    Ok(0)
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
