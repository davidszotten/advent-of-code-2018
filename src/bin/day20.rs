use aoc2018::{dispatch, Result};
use std::mem;

fn main() {
    dispatch(&part1, &part2)
}

// ^E(N|S)W$
fn walk(input: &str, level: usize, so_far: Vec<Vec<char>>) -> (usize, Vec<Vec<char>>) {
    // println!("start: {}, {}", input, level);
    let mut res: Vec<Vec<char>> = so_far;
    // for prefix in so_far.iter() {
    // let mut current: Vec<char> = prefix.iter().collect();
    // let mut current = prefix.clone();
    let mut current: Vec<char> = vec![];
    let mut skip = 0;
    let mut consumed = 0;

    for (idx, c) in input.chars().enumerate() {
        if skip > 0 {
            println!("skipping {}", c);
            skip -= 1;
            continue
        }
        consumed += 1;
        match c {
            '^' => {}
            c @ 'N' | c @ 'S' | c @ 'E' | c @ 'W' => {
                current.push(c);
                // for entry in res.iter_mut() {
                //     entry.push(c);
                // }
            }
            '$' => {}
            '(' => {
                for entry in res.iter_mut() {
                    entry.extend(current.clone());
                }
                current = vec![];

                let mut new_res = vec![];
                // let walked = walk(&input[idx + 1..], level + 1, res.clone());
                let (to_skip, walked) = walk(&input[idx + 1..], level + 1, vec![]);
                skip += to_skip;
                println!("walked: {:?}", walked);
                for option in walked.into_iter() {
                    for mut a in res.clone() {
                        println!("have a: {:?}, option: {:?}", a, option);
                        a.extend(option.clone());
                        println!("push {:?}", a);
                        new_res.push(a);
                    }
                }
                mem::swap(&mut new_res, &mut res);
                println!("copied: {:?}", res);
                // let mut copy1 = res.clone();
                // let copy2 = res.clone();
                // for (a, b) in copy1.iter_mut().cartesian_product(walk(&input[idx + 1..], level + 1, copy2).into_iter()) {
                    // a.extend(b);
                    // new_res.push(a);
                // }
                // println!("push1: {:?}", current.iter().collect::<Vec<char>>());
                // res.push(current.iter().collect());
                // println!("current1: {:?}", current);
                // current = vec![];
                // for option in walk(&input[idx + 1..], level + 1, res.clone()).into_iter() {
                //     println!("push2: {:?}", option);
                //     res.push(option);
                // }
            }
            ')' => {
                println!("closing bracket on lvl {}", level);
                break;
            },
            '|' => {
                // for entry in res.iter_mut() {
                    // entry.extend(current.clone());
                // }
                println!("before | {:?}", res);
                res.push(current);
                current = vec![];
                println!("after | {:?}", res);
                // println!("push3: {:?}", current.iter().collect::<Vec<char>>());
                // res.push(current.iter().collect());
                // println!("push3: {:?}", current);
                // res.push(current);
                // println!("current2: {:?}", current);
                // current = vec![];
            }
            _ => unreachable!(),
        }

    }
    // for entry in res.iter_mut() {
        // entry.extend(current.clone());
    // }

    res.push(current);

    // println!("current3: {:?}", current);
    // println!("push4: {:?}", current.iter().collect::<Vec<char>>());
    // res.push(current.iter().collect());
    // println!("push4: {:?}", current);
    // res.push(current);
    // }
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

    // #[test]
    // fn test_walk3() {
    //     assert_eq!(
    //         walk("^ENWWW(NEEE|SSE(EE|N))$", 0, vec![vec![]]),
    //         vec!["W", "N", "E"]
    //     );
    // }

    #[test]
    fn test_part1() -> Result<()> {
        Ok(assert_eq!(part1("")?, 0))
    }
}
