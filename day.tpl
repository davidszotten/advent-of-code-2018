use aoc2018::args::{parse_input, Part};
use aoc2018::shared::AppResult;

fn main() -> AppResult<()> {
    let args = parse_input()?;
    match args.part {
        Part::Part1 => part1(&args.input),
        Part::Part2 => part2(&args.input),
    }
}

pub fn part1(_input: &str) -> AppResult<()> {
    Ok(())
}


pub fn part2(_input: &str) -> AppResult<()> {
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("").unwrap(), 0);
    }
}
