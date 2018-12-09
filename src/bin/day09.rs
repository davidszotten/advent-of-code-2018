use aoc2018::{dispatch, Result};

fn main() {
    dispatch(&part1, &part2)
}

#[derive(Debug)]
struct Marbles {
    marbles: Vec<u64>,
    current_pos: usize,
}

impl Marbles {
    fn new() -> Self {
        Marbles {
            marbles: vec![0],
            current_pos: 0,
        }
    }

    fn insert(&mut self, next: u64) {
        let next_pos = (self.current_pos + 2) % self.marbles.len();
        self.marbles.insert(next_pos, next);
        self.current_pos = next_pos;
    }

    fn remove(&mut self) -> u64 {
        let mut remove_num = (self.current_pos as i64 - 7) % self.marbles.len() as i64;
        if remove_num < 0 {
            remove_num += self.marbles.len() as i64;
        }
        let remove_pos = remove_num as usize;
        self.current_pos = remove_pos;
        self.marbles.remove(remove_pos)
    }
}

fn place(marbles: &mut Marbles, next: u64) -> u64 {
    // println!("{:?}", marbles);
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
    for next in 1..n_marbles {
        scores[current_player] += place(&mut marbles, next);
        current_player = (current_player + 1) % n_players;
    }
    *scores.iter().max().unwrap()
}

fn part1(input: &str) -> Result<u64> {
    let numbers: Vec<u64> = input.split_whitespace().filter_map(|x| x.parse().ok()).collect();
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
        Ok(assert_eq!(part1("9 players; last marble is worth 25 points")?, 32))
    }

    #[test]
    fn test_play() {
        assert_eq!(play(9, 25), 32);
    }

    #[test]
    fn test_play2() {
        assert_eq!(play(10, 1618), 8317);
        assert_eq!(play(13, 7999), 146373);
        // assert_eq!(play(17, 1104), 2764);
        assert_eq!(play(21, 6111), 54718);
        assert_eq!(play(30, 5807), 37305);
    }
}
