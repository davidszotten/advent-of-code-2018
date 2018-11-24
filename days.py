contents = """use crate::shared::AppResult;

pub fn part1(_input: &str) -> AppResult<u32> {
    Ok(0)
}


pub fn part2(_input: &str) -> AppResult<u32> {
    Ok(0)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1("").unwrap(), 0);
    }
}"""

for counter in range(1, 26):
    filename = f'src/day{counter:02d}.rs'
    with open(filename, 'w') as handle:
        handle.write(contents)
