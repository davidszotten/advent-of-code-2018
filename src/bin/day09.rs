use aoc2018::{dispatch, Result};
use std::collections::VecDeque;

fn main() {
    dispatch(&part1, &part2)
}

#[derive(Debug)]
struct Marbles {
    marbles: VecDeque<u64>,
    current_pos: usize,
}

impl Marbles {
    fn new() -> Self {
        let mut marbles = VecDeque::new();
        marbles.push_back(0);
        Marbles {
            marbles: marbles,
            current_pos: 0,
        }
    }

    fn insert(&mut self, next: u64) {
        let val = self.marbles.pop_front().unwrap();
        self.marbles.push_back(val);
        self.marbles.push_back(next);
    }

    fn remove(&mut self) -> u64 {
        for _ in 0..7 {
            let val = self.marbles.pop_back().unwrap();
            self.marbles.push_front(val);
        }
        let removed = self.marbles.pop_back().unwrap();
        let val = self.marbles.pop_front().unwrap();
        self.marbles.push_back(val);
        removed
    }
}

fn place(marbles: &mut Marbles, next: u64) -> u64 {
    if next > 0 && next % 23 == 0 {
        return next + marbles.remove();
    } else {
        marbles.insert(next);
        return 0;
    }
}

fn play(n_players: usize, n_marbles: u64) -> u64 {
    let mut scores = vec![0; n_players];
    let mut current_player = 0;
    let mut marbles = Marbles::new();
    for next in 1..=n_marbles {
        scores[current_player] += place(&mut marbles, next);
        current_player = (current_player + 1) % n_players;
    }
    *scores.iter().max().unwrap()
}

fn part1(input: &str) -> Result<u64> {
    let numbers: Vec<u64> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    Ok(play(numbers[0] as usize, numbers[1]))
}

fn part2(_input: &str) -> Result<i64> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        Ok(assert_eq!(
            part1("9 players; last marble is worth 25 points")?,
            32
        ))
    }

    #[test]
    fn test_play() {
        assert_eq!(play(9, 25), 32);
    }

    #[test]
    fn test_play2() {
        assert_eq!(play(10, 1618), 8317);
        assert_eq!(play(13, 7999), 146373);
        assert_eq!(play(17, 1104), 2764);
        assert_eq!(play(21, 6111), 54718);
        assert_eq!(play(30, 5807), 37305);
    }
}
