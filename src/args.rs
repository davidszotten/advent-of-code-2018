use clap::{App, Arg};
use crate::shared::{self, AppResult};
use failure::bail;

#[derive(Debug)]
enum Part {
    Part1,
    Part2,
}

#[derive(Debug)]
struct Args {
    part: Part,
    input: String,
}

fn parse_input() -> shared::AppResult<Args> {
    let matches = App::new("adventofcode")
        .arg(Arg::with_name("part")
            .short("p")
            .takes_value(true)
            .default_value("1")
            .possible_values(&["1", "2"])
        )
        .arg(Arg::with_name("input")
            .help("Sets the input file to use, or `-` for stdin")
            .required(true)
            .index(1))
        .get_matches();

    let part = match matches.value_of("part").unwrap_or("1") {
        "1" => Part::Part1,
        "2" => Part::Part2,
        _ => bail!("Invalid part"),
    };
    let input = match matches
        .value_of("input")
        .expect("input is required but missing")
    {
        "-" => shared::read_stdin(),
        filename => shared::read_file(filename),
    }?;
    Ok(Args {
        part: part,
        input: input,
    })
}


fn run(
    part1: &Fn(&str) -> AppResult<u32>,
    part2: &Fn(&str) -> AppResult<u32>,
) -> AppResult<u32> {
    let args = parse_input()?;
    match args.part {
        Part::Part1 => part1(&args.input),
        Part::Part2 => part2(&args.input),
    }
}


pub fn dispatch(
    part1: &Fn(&str) -> AppResult<u32>,
    part2: &Fn(&str) -> AppResult<u32>,
) {
    match run(part1, part2) {
        Ok(result) => println!("{}", result),
        Err(err) => println!("{}", err),
    };
}
