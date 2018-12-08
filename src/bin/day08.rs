use aoc2018::{dispatch, Result};

fn main() {
    dispatch(&part1, &part2)
}

fn parse(data: &mut impl Iterator<Item=u32>) -> u32 {
    let n_children = data.next().expect("not enough data to read n_children");
    let n_metadata = data.next().expect("not enough data to read n_metadata");
    let mut metadata = 0;
    for _ in 0..n_children {
        metadata += parse(data);
    }
    for _ in 0..n_metadata {
        metadata += data.next().expect("not enough metadata");
    }
    metadata
}

fn part1(input: &str) -> Result<u32> {
    let mut data = input.split_whitespace().filter_map(|d| d.parse::<u32>().ok());
    Ok(parse(&mut data))
}

fn parse2(data: &mut impl Iterator<Item=u32>) -> u32 {
    let n_children = data.next().expect("not enough data to read n_children");
    let n_metadata = data.next().expect("not enough data to read n_metadata");
    let mut metadata = 0;
    if n_children == 0 {
        for _ in 0..n_metadata {
            metadata += data.next().expect("not enough metadata");
        }
        return metadata;
    }
    let mut child_data = vec![];
    for _ in 0..n_children {
        child_data.push(parse2(data));
    }
    for _ in 0..n_metadata {
        let child_id = data.next().expect("not enough metadata") - 1;
        if let Some(&child_meta) = child_data.get(child_id as usize) {
            metadata += child_meta;
        }
    }
    metadata
}

fn part2(input: &str) -> Result<u32> {
    let mut data = input.split_whitespace().filter_map(|d| d.parse::<u32>().ok());
    Ok(parse2(&mut data))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        Ok(assert_eq!(part1("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2")?, 138))
    }

    #[test]
    fn test_part2() -> Result<()> {
        Ok(assert_eq!(part2("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2")?, 66))
    }
}
