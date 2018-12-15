use aoc2018::{dispatch, Result};

fn main() {
    dispatch(&part1, &part2)
}

fn part1(input: &str) -> Result<String> {
    let rounds = input.parse::<usize>().unwrap();

    let mut e1 = 0;
    let mut e2 = 1;

    let mut recipes = vec![3, 7];

    for _ in 0..(rounds + 10) {
        let r1 = recipes[e1];
        let r2 = recipes[e2];
        let sum = r1 + r2;
        if sum >= 10 {
            recipes.push(sum / 10)
        }
        recipes.push(sum % 10);
        // println!("{:?}", recipes);
        e1 = (e1 + 1 + r1) % recipes.len();
        e2 = (e2 + 1 + r2) % recipes.len();
    }
    // println!("{:?}", recipes);
    let mut output = String::new();
    for r in recipes.iter().skip(rounds).take(10) {
        output = format!("{}{}", output, r);
    }
    // println!("{}", output);
    Ok(output)
}

fn part2(input: &str) -> Result<usize> {
    let mut end = input.parse::<u32>().unwrap();

    let mut target = vec![];
    for pos in (0..input.len()).rev() {
        let factor = (10 as u32).pow(pos as u32);
        target.push(end / factor);
        end -= (end / factor) * factor;
    }
    target.reverse();
    println!("{:?}", target);

    let mut e1: u32 = 0;
    let mut e2 = 1;

    let mut recipes: Vec<u32> = vec![3, 7];
    let mut rounds = 0;

    loop {
        let r1: u32 = recipes[e1 as usize];
        let r2 = recipes[e2 as usize];
        let sum = r1 + r2;
        if sum >= 10 {
            recipes.push(sum / 10);
            rounds += 1;
        }

        if recipes
            .iter()
            .rev()
            .take(input.len())
            .zip(&target)
            .all(|(a, b)| *a == *b)
        {
            break;
        }

        recipes.push(sum % 10);
        rounds += 1;

        e1 = (e1 + 1 + r1) % (recipes.len() as u32);
        e2 = (e2 + 1 + r2) % (recipes.len() as u32);

        if recipes
            .iter()
            .rev()
            .take(input.len())
            .zip(&target)
            .all(|(a, b)| *a == *b)
        {
            break;
        }
    }
    Ok(rounds - input.len() + 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        Ok(assert_eq!(part1("9")?, "5158916779"))
    }

    #[test]
    fn test_part1b() -> Result<()> {
        Ok(assert_eq!(part1("5")?, "0124515891"))
    }
    #[test]
    fn test_part1c() -> Result<()> {
        Ok(assert_eq!(part1("18")?, "9251071085"))
    }
    #[test]
    fn test_part1d() -> Result<()> {
        Ok(assert_eq!(part1("2018")?, "5941429882"))
    }

    #[test]
    fn test_part2a() -> Result<()> {
        Ok(assert_eq!(part2("51589")?, 9))
    }
    #[test]
    fn test_part2b() -> Result<()> {
        Ok(assert_eq!(part2("01245")?, 5))
    }
    #[test]
    fn test_part2c() -> Result<()> {
        Ok(assert_eq!(part2("92510")?, 18))
    }
    #[test]
    fn test_part2d() -> Result<()> {
        Ok(assert_eq!(part2("59414")?, 2018))
    }

    // 107073195 too high
}
