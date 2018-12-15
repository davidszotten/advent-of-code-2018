use aoc2018::{dispatch, Result};
use failure::err_msg;
use std::collections::HashMap;

fn main() {
    dispatch(&part1, &part2)
}

#[derive(Debug, Clone, Copy)]
enum Track {
    Horizontal,
    Vertical,
    TopLeft,
    TopRight,
    Intersection,
}

#[derive(Debug, Clone, Copy)]
enum TrainDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Input {
    track: Track,
    train: Option<TrainDirection>,
}

#[derive(Debug, Clone, Copy)]
enum IntersectionStrategy {
    Left,
    Straight,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Train {
    direction: TrainDirection,
    intersection_strategy: IntersectionStrategy,
}

impl Train {
    fn new(direction: TrainDirection) -> Self {
        use self::IntersectionStrategy::*;
        let intersection_strategy = Left;
        Train {
            direction,
            intersection_strategy,
        }
    }
    fn step(&self, pos: (usize, usize)) -> (usize, usize) {
        use self::TrainDirection::*;
        match self.direction {
            Up => (pos.0, pos.1 - 1),
            Down => (pos.0, pos.1 + 1),
            Left => (pos.0 - 1, pos.1),
            Right => (pos.0 + 1, pos.1),
        }
    }

    fn intersection_turn(&mut self) -> TrainDirection {
        use self::TrainDirection::*;
        match self.intersection_strategy {
            IntersectionStrategy::Straight => {
                self.intersection_strategy = IntersectionStrategy::Right;
                self.direction
            }
            IntersectionStrategy::Left => {
                self.intersection_strategy = IntersectionStrategy::Straight;
                match self.direction {
                    Up => Left,
                    Down => Right,
                    Left => Down,
                    Right => Up,
                }
            }
            IntersectionStrategy::Right => {
                self.intersection_strategy = IntersectionStrategy::Left;
                match self.direction {
                    Up => Right,
                    Down => Left,
                    Left => Up,
                    Right => Down,
                }
            }
        }
    }

    fn turn(&mut self, track: &Track) {
        use self::Track::*;
        use self::TrainDirection::*;
        match track {
            Horizontal => {}
            Vertical => {}
            TopLeft => match self.direction {
                Up => self.direction = Left,
                Left => self.direction = Up,
                Down => self.direction = Right,
                Right => self.direction = Down,
            },
            TopRight => match self.direction {
                Up => self.direction = Right,
                Right => self.direction = Up,
                Down => self.direction = Left,
                Left => self.direction = Down,
            },
            Intersection => self.direction = self.intersection_turn(),
        }
    }
}

/*
/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/
*/

fn parse(c: &char) -> Result<Input> {
    use self::Track::*;
    use self::TrainDirection::*;

    match c {
        '/' => Ok(Input {
            track: TopRight,
            train: None,
        }),
        '|' => Ok(Input {
            track: Vertical,
            train: None,
        }),
        '-' => Ok(Input {
            track: Horizontal,
            train: None,
        }),
        '\\' => Ok(Input {
            track: TopLeft,
            train: None,
        }),
        '+' => Ok(Input {
            track: Intersection,
            train: None,
        }),
        '^' => Ok(Input {
            track: Vertical,
            train: Some(Up),
        }),
        'v' => Ok(Input {
            track: Vertical,
            train: Some(Down),
        }),
        '>' => Ok(Input {
            track: Horizontal,
            train: Some(Right),
        }),
        '<' => Ok(Input {
            track: Horizontal,
            train: Some(Left),
        }),
        _ => Err(err_msg("parse fail")),
    }
}

fn part1(input: &str) -> Result<String> {
    let mut map = HashMap::new();
    let mut trains = HashMap::new();

    for (y, row) in input.split('\n').enumerate() {
        for (x, c) in row.chars().enumerate() {
            if let Ok(parsed) = parse(&c) {
                map.insert((x, y), parsed.track);
                if let Some(direction) = parsed.train {
                    trains.insert((x, y), Train::new(direction));
                }
            }
        }
    }

    loop {
        let mut order: Vec<_> = trains.keys().map(|&t| t.clone()).collect();
        order.sort_by_key(|&(x,y)| (y, x));
        for pos in order {
            let mut train = trains.remove(&pos).expect("coor missing for remove");
            let next_pos = train.step(pos);
            if trains.contains_key(&next_pos) {
                return Ok(format!("{:?}", next_pos));
            }
            let track = map.get(&next_pos).expect("next coor missing");
            train.turn(track);
            trains.insert(next_pos, train);
        }

    }
}

fn part2(_input: &str) -> Result<i32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   ";

    #[test]
    fn test_part1() -> Result<()> {
        Ok(assert_eq!(part1(INPUT)?, "(7, 3)"))
    }
}
