use aoc2018::{dispatch, Result};
use itertools::Itertools;
use std::cmp;

fn main() {
    dispatch(&part1, &part2)
}

fn power(x: i32, y: i32, serial: i32) -> i32 {
    let rack_id = x + 10;
    let part1 = rack_id * y;
    let part2 = part1 + serial;
    let part3 = ((part2 % 1000) * (rack_id % 1000) % 1000) / 100;
    part3 - 5
}

fn part1(input: &str) -> Result<String> {
    let serial = input.parse()?;
    let mut max_coor = (-1, -1);
    let mut max = 0;
    for x in 1..299 {
        for y in 0..299 {
            let sum = (0..=2)
                .cartesian_product(0..=2)
                .map(|(dx, dy)| power(x + dx, y + dy, serial))
                .sum();
            if sum > max {
                max_coor = (x, y);
                max = sum;
            }
        }
    }
    Ok(format!("{}, {}", max_coor.0, max_coor.1))
}

fn part2(input: &str) -> Result<String> {
    let serial = input.parse()?;
    let mut max_coor = None;
    let mut max = 0;

    //     123
    //
    //  1  112
    //  2  112
    //  3  222
    //
    //  x=1, y=1, size=3
    //  (1,3), (2,3)  (3,1), (3, 2)  (3,3)

    for x in 1..=300 {
        for y in 1..=300 {
            let mut square = 0;
            for size in 1..=(300 - cmp::max(x, y)) {
                square += (0..size - 1)
                    .map(|d| {
                        power(x + d, y + size - 1, serial) + power(x + size - 1, y + d, serial)
                    })
                    .sum::<i32>()
                    + power(x + size - 1, y + size - 1, serial);

                if square > max {
                    max_coor = Some((x, y, size));
                    max = square;
                }
            }
        }
    }
    let max_coor = max_coor.unwrap();
    Ok(format!("{},{},{}", max_coor.0, max_coor.1, max_coor.2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power() {
        assert_eq!(power(3, 5, 8), 4);
        assert_eq!(power(122, 79, 57), -5);
        assert_eq!(power(217, 196, 39), 0);
        assert_eq!(power(101, 153, 71), 4);
    }

    #[test]
    fn test_part1a() -> Result<()> {
        Ok(assert_eq!(part1("18")?, "33, 45"))
    }

    #[test]
    fn test_part1b() -> Result<()> {
        Ok(assert_eq!(part1("42")?, "21, 61"))
    }

    // #[test]
    // fn test_part2() -> Result<()> {
    // Ok(assert_eq!(part2("18")?, "90,269,16"))
    // }
}
