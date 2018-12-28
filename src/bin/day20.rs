use aoc2018::{dispatch, Result};
use failure::Error;
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::ops;
use std::str::FromStr;

fn main() {
    dispatch(&part1, &part2)
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
            'E' => (1, 0),
            'W' => (-1, 0),
            c => panic!(format!("bad from_char: {}", c)),
        };
        Coor { x, y }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Pattern {
    Literal(String),
    Concat(Vec<Pattern>),
    Or(Vec<Pattern>),
}

type Edges = HashMap<Coor, Vec<Coor>>;

impl Pattern {
    fn edges(&self) -> Edges {
        fn inner(pattern: &Pattern, mut edges: Edges, mut pos: Coor) -> (Edges, Coor) {
            use self::Pattern::*;
            match pattern {
                Literal(s) => {
                    for c in s.chars() {
                        let direction = Coor::from_char(c);
                        let next = pos + direction;
                        let entry = edges.entry(pos).or_insert(vec![]);
                        (*entry).push(next);
                        pos = next;
                    }
                }
                Concat(v) => {
                    for p in v {
                        let ret = inner(p, edges, pos);
                        edges = ret.0;
                        pos = ret.1;
                    }
                }
                Or(v) => {
                    for p in v {
                        let ret = inner(p, edges, pos);
                        edges = ret.0;
                    }
                }
            }
            (edges, pos)
        }
        let edges = HashMap::new();
        let pos = Coor::new();
        let edges = inner(self, edges, pos).0;
        edges
    }
}

impl FromStr for Pattern {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        use self::Pattern::*;
        let mut chunk_positions = vec![];
        let mut chunk_start = 0;
        let mut or_positions = vec![];
        let mut pos = 0;

        let mut bracket = 0;

        // println!("{}", s);

        if s.is_empty() {
            return Ok(Literal("".into()));
        }

        if &s[0..1] == "^" {
            return s[1..s.len() - 1].parse();
        }

        let mut chars = s.chars();
        while let Some(c) = chars.next() {
            match c {
                '(' => {
                    if bracket == 0 {
                        if chunk_start != pos {
                            chunk_positions.push((chunk_start, pos));
                        }
                        chunk_start = pos + 1;
                    }
                    bracket += 1;
                }
                ')' => {
                    bracket -= 1;
                    if bracket == 0 {
                        chunk_positions.push((chunk_start, pos));
                        chunk_start = pos + 1;
                    }
                }
                '|' => {
                    if bracket == 0 {
                        or_positions.push(pos);
                    }
                }
                _ => {}
            }
            pos += 1;
        }

        if chunk_start != pos {
            chunk_positions.push((chunk_start, pos));
        }

        // println!("or: {:?}", or_positions);
        // let mut ostart = 0;
        // for &opos in or_positions.iter() {
        //     println!("\t{:?}", &s[ostart..opos]);
        //     ostart = opos + 1;
        // }
        // println!("\t{:?}", &s[ostart..]);

        // println!("chunk_positions: {:?}", chunk_positions);
        // for &(start, end) in chunk_positions.iter() {
        //     println!("\t{:?}", &s[start..end]);
        // }

        if !or_positions.is_empty() {
            let mut ors: Vec<Pattern> = vec![];

            let mut ostart = 0;
            for &opos in or_positions.iter() {
                ors.push(s[ostart..opos].parse()?);
                ostart = opos + 1;
            }
            ors.push(s[ostart..].parse()?);

            // println!("returning or: {:?}", ors);
            return Ok(Or(ors));
        }

        if chunk_positions.len() == 1 {
            assert!(!s.contains('('));
            return Ok(Literal(s.into()));
        }

        let mut chunks: Vec<Pattern> = vec![];
        for &(start, end) in chunk_positions.iter() {
            chunks.push(s[start..end].parse()?);
        }
        // println!("returning concat: {:?}", chunks);
        Ok(Concat(chunks))
    }
}

fn get_distances(pattern: &Pattern) -> HashMap<Coor, usize> {
    let edges = pattern.edges();

    let mut queue = VecDeque::new();
    let start = Coor { x: 0, y: 0 };
    queue.push_back(start);
    let mut distances = HashMap::new();
    distances.insert(start, 0);
    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        let current_distance = distances
            .get(&current)
            .expect("current missing in distances")
            .clone();
        if let Some(neighbours) = edges.get(&current) {
            for &next in neighbours {
                distances.entry(next).or_insert_with(|| {
                    if queue.push_back(next) == () {
                        current_distance + 1
                    } else {
                        panic!("haxx")
                    }
                });
            }
        }
    }
    distances
}

fn part1(input: &str) -> Result<usize> {
    let pattern: Pattern = input.parse()?;
    let distances = get_distances(&pattern);
    Ok(*distances.values().max().unwrap())
}

fn part2(input: &str) -> Result<usize> {
    let pattern: Pattern = input.parse()?;
    let distances = get_distances(&pattern);
    Ok(distances.values().filter(|&&d| d >= 1000).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_brackets1() -> Result<()> {
        use self::Pattern::*;
        Ok(assert_eq!(
            "^(SN)E$".parse::<Pattern>()?,
            Concat(vec![Literal("SN".into()), Literal("E".into())])
        ))
    }

    #[test]
    fn test_parse1() -> Result<()> {
        use self::Pattern::*;
        Ok(assert_eq!(
            "^SNEW$".parse::<Pattern>()?,
            Literal("SNEW".into())
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
                Or(vec![
                    Literal("N".into()),
                    Literal("S".into()),
                    Literal("W".into())
                ]),
            ])
        ))
    }

    #[test]
    fn test_parse4() -> Result<()> {
        use self::Pattern::*;
        Ok(assert_eq!(
            "^E(N|S|)W$".parse::<Pattern>()?,
            Concat(vec![
                Literal("E".into()),
                Or(vec![
                    Literal("N".into()),
                    Literal("S".into()),
                    Literal("".into()),
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

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1("^ENWWW(NEEE|SSE(EE|N))$")?, 10);
        assert_eq!(part1("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$")?, 18);
        assert_eq!(
            part1("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))")?,
            23
        );
        assert_eq!(
            part1("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$")?,
            31
        );
        Ok(())
    }
}
