use aoc2018::shared::{dispatch, AppResult};

fn main() {
    dispatch(&part1, &part2)
}

fn part1(_input: &str) -> AppResult<u32> {
    Ok(1)
}

fn part2(_input: &str) -> AppResult<u32> {
    Ok(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("").unwrap(), 1);
    }
}
