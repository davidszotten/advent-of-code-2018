use aoc2018::{dispatch, Result};
use itertools::Itertools;

const DIFF: i8 = 'a' as i8 - 'A' as i8;

fn main() {
    dispatch(&part1, &part2)
}

fn reduce(input: Vec<char>) -> Vec<char> {
    let mut res = vec![];
    let mut skip = false;
    for (a, b) in input.iter().tuple_windows() {
        if skip {
            skip = false;
            continue;
        }
        if (*b as i8 - *a as i8).abs() == DIFF {
            skip = true;
        } else {
            res.push(*a);
        }
    }
    if let Some(last) = input.last() {
        if !skip {
            res.push(*last);
        }
    }
    res
}

fn part1(input: &str) -> Result<usize> {
    let mut chars: Vec<_> = input.chars().collect();
    let mut prev_len = chars.len();
    loop {
        chars = reduce(chars);
        if chars.len() == prev_len {
            break;
        }
        prev_len = chars.len();
    }
    // reduce(input)
    Ok(prev_len)
}

fn part2(_input: &str) -> Result<i32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn split(input: &str) -> Vec<char> {
        input.chars().collect()
    }

    #[test]
    fn test_reduce1() {
        assert_eq!(reduce(split("aA")), split(""));
    }

    #[test]
    fn test_reduce2() {
        assert_eq!(reduce(split("abBA")), split("aA"));
    }

    #[test]
    fn test_reduce3() {
        assert_eq!(reduce(split("abAB")), split("abAB"));
    }

    #[test]
    fn test_reduce4() {
        assert_eq!(reduce(split("aAab")), split("ab"));
    }

    #[test]
    fn test_reduce5() {
        let v = split("dabAcCaCBAcCcaDA");
        let v = reduce(v);
        let v = reduce(v);
        let v = reduce(v);
        assert_eq!(v, split("dabCBAcaDA"));
        let v = reduce(v);
        assert_eq!(v, split("dabCBAcaDA"));
    }
}
