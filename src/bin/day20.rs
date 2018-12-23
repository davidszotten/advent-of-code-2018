use aoc2018::{dispatch, Result};
use std::mem;

fn main() {
    dispatch(&part1, &part2)
}

fn indent(level: usize) {
    for _ in 0..level {
        print!("\t");
    }
}

// ^E(N|S)W$
// ^E(N|S(E|N))$
fn walk(input: &str, level: usize, so_far: Vec<Vec<char>>) -> (usize, Vec<Vec<char>>) {
    let mut res: Vec<Vec<char>> = so_far;
    let mut current: Vec<char> = vec![];
    let mut skip = 0;
    let mut consumed = 0;

    for (idx, c) in input.chars().enumerate() {
        if skip > 0 {
            indent(level);
            println!("skipping {}", c);
            skip -= 1;
            continue;
        }
        consumed += 1;
        match c {
            '^' => {}
            c @ 'N' | c @ 'S' | c @ 'E' | c @ 'W' => {
                current.push(c);
            }
            '$' => {
                indent(level);
                println!("$: {:?} {:?}", res, current);
                if res.len() > 0 {
                    for entry in res.iter_mut() {
                        entry.extend(current.clone());
                    }
                    current = vec![];
                } else {
                    res.push(current);
                    current = vec![];
                }
                indent(level);
                println!("$: {:?} {:?}", res, current);
            }
            '(' => {
                for entry in res.iter_mut() {
                    entry.extend(current.clone());
                }
                indent(level);
                println!("( {:?}", res);
                current = vec![];

                let mut new_res = vec![];
                let (to_skip, walked) = walk(&input[idx + 1..], level + 1, vec![]);
                skip += to_skip;
                indent(level);
                println!("walked: {:?}", walked);
                for option in walked.into_iter() {
                    for mut a in res.clone() {
                        indent(level);
                        println!("have a: {:?}, option: {:?}", a, option);
                        a.extend(option.clone());
                        indent(level);
                        println!("push {:?}", a);
                        new_res.push(a);
                    }
                }
                mem::swap(&mut new_res, &mut res);
                indent(level);
                println!("copied: {:?}", res);
            }
            ')' => {
                indent(level);
                println!("closing bracket on lvl {}", level);
                break;
            }
            '|' => {
                indent(level);
                println!("before | {:?}", res);
                res.push(current);
                current = vec![];
                indent(level);
                println!("after | {:?}", res);
            }
            _ => unreachable!(),
        }
    }

    if current.len() > 0 {
        res.push(current);
    }
    indent(level);
    println!("returning lvl {} ({}, {:?})", level, consumed, res);
    (consumed, res)
}

fn part1(input: &str) -> Result<i32> {
    walk(input, 0, vec![vec![]]);
    Ok(0)
}

fn part2(_input: &str) -> Result<i32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_walk1() {
        assert_eq!(walk("^WNE$", 0, vec![]).1, vec![vec!['W', 'N', 'E']]);
    }

    #[test]
    fn test_walk2() {
        assert_eq!(
            walk("^E(N|S)W$", 0, vec![vec![]]).1,
            vec![vec!['E', 'N', 'W'], vec!['E', 'S', 'W']]
        );
    }

    #[test]
    fn test_walk3() {
        assert_eq!(
            walk("^E(N|S(E|N))$", 0, vec![vec![]]).1,
            vec![vec!['E', 'N'], vec!['E', 'S', 'E'], vec!['E', 'S', 'N'],]
        );
    }

    // #[test]
    // fn test_walk4() {
    //     assert_eq!(
    //         walk("^ENWWW(NEEE|SSE(EE|N))$", 0, vec![vec![]]).1,
    //         vec![
    //             vec!['E', 'N',  'W', 'W', 'W', 'N', 'E', 'E', 'E'],
    //             vec!['E', 'N',  'W', 'W', 'W', 'S', 'S', 'E', 'E', 'E'],
    //             vec!['E', 'N',  'W', 'W', 'W', 'S', 'S', 'E', 'I', 'N'],
    //         ]
    //     );
    // }

    #[test]
    fn test_part1() -> Result<()> {
        Ok(assert_eq!(part1("")?, 0))
    }
}
