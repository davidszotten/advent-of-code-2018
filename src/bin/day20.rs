use aoc2018::{dispatch, Result};
// use failure::{err_msg, Error};
use failure::{Error};
use std::collections::HashSet;
use std::fmt;
use std::ops;
use std::str::FromStr;

fn main() {
    dispatch(&part1, &part2)
}

fn part1(input: &str) -> Result<usize> {
    let pattern: Pattern = input.parse()?;
    println!("{:?}", pattern);
    let seen = pattern.walk();
    println!("{:?}", seen);
    let min_x = seen.iter().map(|(Coor { x, y: _ }, _)| *x).min().unwrap();
    let max_x = seen.iter().map(|(Coor { x, y: _ }, _)| *x).max().unwrap();
    let min_y = seen.iter().map(|(Coor { x: _, y }, _)| *y).min().unwrap();
    let max_y = seen.iter().map(|(Coor { x: _, y }, _)| *y).max().unwrap();
    println!("{}/{}, {}/{}", min_x, max_x, min_y, max_y);
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let north = seen.contains(&(Coor { x, y }, 'N'));
            let south = seen.contains(&(Coor { x, y }, 'S'));
            let east = seen.contains(&(Coor { x, y }, 'E'));
            let west = seen.contains(&(Coor { x, y }, 'W'));
            let found = north || south || east || west;
            let hmarker = |b| if b { "|" } else { "?" };
            let cmarker = if (x, y) == (0, 0) {
                "X"
            } else if found {
                "."
            } else {
                " "
            };
            print!("{}{}{}", hmarker(west), cmarker, hmarker(east));
        }
        println!("");
    }
    Ok(seen.len())
}

fn part2(_input: &str) -> Result<i32> {
    Ok(0)
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coor {
    x: i32,
    y: i32,
}

impl fmt::Debug for Coor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl ops::Add for Coor {
    type Output = Coor;

    fn add(self, other: Coor) -> Self::Output {
        Coor {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Coor {
    fn new() -> Self {
        Coor { x: 0, y: 0 }
    }
    fn from_char(c: char) -> Self {
        let (x, y) = match c {
            'N' => (0, 1),
            'S' => (0, -1),
            'E' => (-1, 0),
            'W' => (1, 0),
            _ => panic!("bad from_char"),
        };
        Coor { x, y }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Pattern {
    Literal(String),
    Concat(Vec<Pattern>),
    Or(Vec<Pattern>),
    Partial(String),
}

impl Pattern {
    fn walk(&self) -> HashSet<(Coor, char)> {
        fn inner_walk(
            pattern: &Pattern,
            mut seen: HashSet<(Coor, char)>,
            mut pos: Coor,
        ) -> (HashSet<(Coor, char)>, Coor) {
            use self::Pattern::*;
            match pattern {
                Literal(s) => {
                    for c in s.chars() {
                        let direction = Coor::from_char(c);
                        seen.insert((pos, c));
                        pos = pos + direction;
                    }
                }
                Concat(v) => {
                    for p in v {
                        let ret = inner_walk(p, seen, pos);
                        seen = ret.0;
                        pos = ret.1;
                    }
                }
                Or(v) => {
                    for p in v {
                        let ret = inner_walk(p, seen, pos);
                        seen = ret.0;
                        // pos = ret.1;
                    }
                }
                Partial(_) => panic!("can't walk partial"),
            }
            (seen, pos)
        }
        let seen = HashSet::new();
        let pos = Coor::new();
        inner_walk(self, seen, pos).0
    }
}

impl FromStr for Pattern {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        use self::Pattern::*;
        let mut chunks = vec![];
        let mut chunk_start = 0;
        let mut literal_chunks = vec![];
        let mut literal_start = 0;
        let mut or_positions = vec![];
        let mut bracket_chunks = vec![];
        let mut bracket_start = 0;
        let mut pos = 0;

        let mut bracket = 0;

        let mut chars = s.chars();
        while let Some(c) = chars.next() {
            match c {
                '(' => {
                    if bracket == 0 {
                        bracket_start = pos;
                        if literal_start != pos {
                            literal_chunks.push((literal_start, pos));
                        }
                        if chunk_start != pos {
                            chunks.push((chunk_start, pos));
                            chunk_start = pos;
                        }
                    }
                    bracket += 1;
                },
                ')' => {
                    bracket -= 1;
                    if bracket == 0 {
                        chunks.push((chunk_start, pos + 1));
                        bracket_chunks.push((bracket_start, pos + 1));
                        literal_start = pos + 1;
                        chunk_start = pos + 1;
                    }
                },
                '|' => {
                    if bracket == 0 {
                        or_positions.push(pos);
                    }
                },
                _ => {},
            }
            pos += 1;
        };

        if literal_start != pos {
            literal_chunks.push((literal_start, pos));
        }
        if chunk_start != pos {
            chunks.push((chunk_start, pos));
        }

        println!("{}", s);
        println!("or: {:?}", or_positions);
        let mut ostart = 0;
        for &opos in or_positions.iter() {
            println!("\t{:?}", &s[ostart..opos]);
            ostart = opos + 1;
        }
        println!("\t{:?}", &s[ostart..]);

        // println!("literals: {:?}", literal_chunks);
        // for (start, end) in literal_chunks {
        //     println!("\t{:?}", &s[start..end]);
        // }
        // println!("brackets: {:?}", bracket_chunks);
        // for &(start, end) in bracket_chunks.iter() {
        //     println!("\t{:?}", &s[start..end]);
        // }
        println!("chunks: {:?}", chunks);
        for &(start, end) in chunks.iter() {
            println!("\t{:?}", &s[start..end]);
        }

        if !or_positions.is_empty() {
            let mut ors: Vec<Pattern> = vec![];

            let mut ostart = 0;
            for &opos in or_positions.iter() {
                ors.push(s[ostart..opos].parse()?);
                ostart = opos + 1;
            }

            return Ok(Or(ors));
        }

        for (start, end) in bracket_chunks {
            &s[start+1..end-1].parse::<Pattern>();
        }

        Ok(Literal("A".into()))
    }

    // fn from_str(s: &str) -> Result<Self> {
    //     // println!("parsing `{}`", s);

    //     use self::Pattern::*;
    //     let mut res: Vec<Pattern> = vec![];
    //     let mut current = vec![];
    //     let mut or_mode = false;
    //     let mut chars = s.chars();

    //     while let Some(c) = chars.next() {
    //         match c {
    //             '^' | '$' => {}
    //             c @ 'N' | c @ 'S' | c @ 'E' | c @ 'W' => {
    //                 current.push(c);
    //             }
    //             '(' => {
    //                 if !current.is_empty() {
    //                     res.push(Partial(current.iter().collect()));
    //                     current = vec![];
    //                 }

    //                 let mut pending = vec![];
    //                 let mut level = 0;
    //                 while let Some(c) = chars.next() {
    //                     if c == '(' {
    //                         level += 1;
    //                     }
    //                     if c == ')' {
    //                         if level == 0 {
    //                             break;
    //                         }
    //                         level -= 1;
    //                     }
    //                     pending.push(c);
    //                 }
    //                 if !pending.is_empty() {
    //                     res.push(Partial(pending.iter().collect()));
    //                 }
    //             }
    //             '|' => {
    //                 or_mode = true;
    //                 if !current.is_empty() {
    //                     res.push(Partial(current.iter().collect()));
    //                     current = vec![];
    //                 }
    //                 let mut pending = vec![];
    //                 let mut level = 0;
    //                 while let Some(c) = chars.next() {
    //                     if c == '(' {
    //                         level += 1;
    //                     }
    //                     if c == ')' {
    //                         if level == 0 {
    //                             res.push(Partial(pending.iter().collect()));
    //                             pending = vec![];
    //                         }
    //                         level -= 1;
    //                     }
    //                     pending.push(c);
    //                 }
    //                 if !pending.is_empty() {
    //                     res.push(Partial(pending.iter().collect()));
    //                 }
    //             }
    //             ')' => {
    //                 if !current.is_empty() {
    //                     res.push(Partial(current.iter().collect()));
    //                     current = vec![];
    //                 }
    //             }
    //             c => Err(err_msg(format!("parse fail: `{}`", c)))?,
    //         }
    //     }

    //     let is_literal = |s: &str| {
    //         s.chars()
    //             .all(|c| c == 'N' || c == 'S' || c == 'E' || c == 'W')
    //     };

    //     if !current.is_empty() {
    //         res.push(Partial(current.iter().collect()));
    //     }

    //     // let mode_str = if or_mode { "or" } else { "concat" };
    //     // println!("res ({}): {:?}", mode_str, res);
    //     let inside = res
    //         .into_iter()
    //         .map(|p| match p {
    //             Partial(ref s) if is_literal(&s) => Literal(s.clone()),
    //             Partial(s) => s.parse().expect("sub parsing"),
    //             p => p,
    //         })
    //         // .filter(|p| if let Literal(s) = p && s == "" {false} else {true})
    //         .collect();
    //     let complete = if or_mode { Or(inside) } else { Concat(inside) };

    //     println!("complete: {} -> {:?}", s, complete);
    //     Ok(complete)
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse1() -> Result<()> {
        use self::Pattern::*;
        Ok(assert_eq!(
            "^SNEW$".parse::<Pattern>()?,
            Concat(vec![Literal("SNEW".into())])
        ))
    }
    #[test]
    fn test_parse2() -> Result<()> {
        use self::Pattern::*;
        Ok(assert_eq!(
            "^E(N|S)W$".parse::<Pattern>()?,
            Concat(vec![
                Literal("E".into()),
                Or(vec![Literal("N".into()), Literal("S".into())]),
                Literal("W".into()),
            ])
        ))
    }

    #[test]
    fn test_parse3() -> Result<()> {
        use self::Pattern::*;
        Ok(assert_eq!(
            "^E(N|S|W)$".parse::<Pattern>()?,
            Concat(vec![
                Literal("E".into()),
                Or(vec![Literal("E".into()), Literal("N".into()), Literal("W".into())]),
            ])
        ))
    }

    // #[test]
    fn test_parse4() -> Result<()> {
        use self::Pattern::*;
        Ok(assert_eq!(
            "^E(N|S|)W$".parse::<Pattern>()?,
            Concat(vec![
                Literal("E".into()),
                Or(vec![
                    Literal("N".into()),
                    Literal("S".into()),
                    Literal("".into())
                ]),
                Literal("W".into()),
            ])
        ))
    }

    #[test]
    fn test_parse5() -> Result<()> {
        use self::Pattern::*;
        Ok(assert_eq!(
            "^E(N|S(E|N))$".parse::<Pattern>()?,
            Concat(vec![
                Literal("E".into()),
                Or(vec![
                    Literal("N".into()),
                    Concat(vec![
                        Literal("S".into()),
                        Or(vec![Literal("E".into()), Literal("N".into())]),
                    ]),
                ])
            ])
        ))
    }

    // #[test]
    fn test_part1() -> Result<()> {
        Ok(assert_eq!(part1("")?, 0))
    }
}
