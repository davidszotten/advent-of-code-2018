use aoc2018::{dispatch, Result};

fn main() {
    dispatch(&part1, &part2)
}

fn walk(input: &str, level: usize, so_far: Vec<String>) -> Vec<String> {
    // println!("start: {}, {}", input, level);
    let mut res: Vec<String> = vec![];
    for prefix in so_far.iter() {
        let mut current: Vec<char> = prefix.chars().collect();
        for (idx, c) in input.chars().enumerate() {
            match c {
                '^' => {}
                'N' => current.push('N'),
                'S' => current.push('S'),
                'E' => current.push('E'),
                'W' => current.push('W'),
                '$' => {}
                '(' => {
                    // println!("push1: {:?}", current.iter().collect::<String>());
                    // res.push(current.iter().collect());
                    // println!("current1: {:?}", current);
                    // current = vec![];
                    for option in walk(&input[idx + 1..], level + 1, res.clone()).into_iter() {
                        println!("push2: {:?}", option);
                        res.push(option);
                    }
                }
                ')' => break,
                '|' => {
                    println!("push3: {:?}", current.iter().collect::<String>());
                    res.push(current.iter().collect());
                    // println!("current2: {:?}", current);
                    current = vec![];
                }
                _ => unreachable!(),
            }
        }
        // println!("current3: {:?}", current);
        println!("push4: {:?}", current.iter().collect::<String>());
        res.push(current.iter().collect());
    }
    // println!("returning lvl {} {:?}", level, res);
    res
}

fn part1(input: &str) -> Result<i32> {
    walk(input, 0, vec!["".into()]);
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
        assert_eq!(walk("^WNE$", 0, vec!["".into()]), vec!["WNE"]);
    }

    #[test]
    fn test_walk2() {
        assert_eq!(
            walk("^E(N|S)W$", 0, vec!["".into()]),
            vec!["ENW", "ESW"]
        );
    }

    // #[test]
    fn test_walk3() {
        assert_eq!(
            walk("^ENWWW(NEEE|SSE(EE|N))$", 0, vec!["".into()]),
            vec!["W", "N", "E"]
        );
    }

    #[test]
    fn test_part1() -> Result<()> {
        Ok(assert_eq!(part1("")?, 0))
    }
}
