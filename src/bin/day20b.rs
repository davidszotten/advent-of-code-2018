use aoc2018::{dispatch, Result};
use failure::{err_msg, Error};
use std::str::FromStr;

fn main() {
    dispatch(&part1, &part2)
}

fn part1(_input: &str) -> Result<i32> {
    Ok(0)
}

fn part2(_input: &str) -> Result<i32> {
    Ok(0)
}


#[derive(Debug, Clone, PartialEq)]
enum Pattern {
    Literal(String),
    Concat(Vec<Pattern>),
    Or(Vec<Pattern>),
    Partial(String),
}

impl FromStr for Pattern {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        println!("parsing `{}`", s);

        use self::Pattern::*;
        let mut res: Vec<Pattern> = vec![];
        let mut current = vec![];
        let mut or_mode = false;
        let mut chars = s.chars();

        while let Some(c) = chars.next() {
            match c {
                '^' | '$' => {}
                c @ 'N' | c @ 'S' | c @ 'E' | c @ 'W' => {
                    current.push(c);
                }
                '(' => {
                    if !current.is_empty() {
                        res.push(Partial(current.iter().collect()));
                        current = vec![];
                    }

                    let mut pending = vec![];
                    let mut level = 0;
                    while let Some(c) = chars.next() {
                        if c == '(' {
                            level += 1;
                        }
                        if c == ')' {
                            if level == 0 {
                                break
                            }
                            level -= 1;
                        }
                        pending.push(c);
                    }
                    if !pending.is_empty() {
                        res.push(Partial(pending.iter().collect()));
                    }
                }
                '|' => {
                    or_mode = true;
                    if !current.is_empty() {
                        res.push(Partial(current.iter().collect()));
                        current = vec![];
                    }
                    let mut pending = vec![];
                    let mut level = 0;
                    while let Some(c) = chars.next() {
                        if c == '(' {
                            level += 1;
                        }
                        if c == ')' {
                            if level == 0 {
                                res.push(Partial(pending.iter().collect()));
                                pending = vec![];
                            }
                            level -= 1;
                        }
                        pending.push(c);
                    }
                    if !pending.is_empty() {
                        res.push(Partial(pending.iter().collect()));
                    }
                }
                ')' => {
                    if !current.is_empty() {
                        res.push(Partial(current.iter().collect()));
                        current = vec![];
                    }
                }
                _ => Err(err_msg("parse fail"))?,
            }
        }

        let is_literal = |s: &str| {
            s.chars()
                .all(|c| c == 'N' || c == 'S' || c == 'E' || c == 'W')
        };

        if !current.is_empty() {
            res.push(Partial(current.iter().collect()));
        }

        let mode_str = if or_mode { "or" } else { "concat" };
        println!("res ({}): {:?}", mode_str, res);
        let inside = res
            .into_iter()
            .map(|p| match p {
                Partial(ref s) if is_literal(&s) => Literal(s.clone()),
                Partial(s) => s.parse().expect("sub parsing"),
                p => p,
            })
            // .filter(|p| if let Literal(s) = p && s == "" {false} else {true})
            .collect();
        let complete = if or_mode { Or(inside) } else { Concat(inside) };

        println!("complete: {:?}", complete);
        Ok(complete)
    }
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

    #[test]
    fn test_part1() -> Result<()> {
        Ok(assert_eq!(part1("")?, 0))
    }
}
