use clap::{App, Arg};
use crate::shared;
use failure::bail;

#[derive(Debug)]
pub enum Part {
    Part1,
    Part2,
}

#[derive(Debug)]
pub struct Args {
    pub part: Part,
    pub input: String,
}

pub fn parse_input() -> shared::AppResult<Args> {
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
