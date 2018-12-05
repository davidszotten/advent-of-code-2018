use aoc2018::{dispatch, Result};
use failure::{err_msg, Error};
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
enum Action {
    StartShift(u32),
    WakesUp,
    FallsAsleep,
}

#[derive(Debug, PartialEq, Clone)]
struct Record {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    action: Action,
}

impl FromStr for Record {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"\[(?P<year>\d+)-(?P<month>\d+)-(?P<day>\d+) (?P<hour>\d+):(?P<minute>\d+)\] ((Guard #(?P<guard>\d+) begins shift)|(?P<wake>wakes up)|(?P<sleep>falls asleep))"
            )
            .unwrap();
        }

        let caps = RE.captures(s).unwrap();
        let year = get_cap_int(&caps, "year")?;
        let month = get_cap_int(&caps, "month")?;
        let day = get_cap_int(&caps, "day")?;
        let hour = get_cap_int(&caps, "hour")?;
        let minute = get_cap_int(&caps, "minute")?;
        fn get_cap_int(caps: &Captures, name: &str) -> Result<u32> {
            Ok(caps
                .name(name)
                .ok_or(err_msg("parse fail"))?
                .as_str()
                .parse()?)
        }
        if caps.name("wake").is_some() {
            Ok(Record {
                year,
                month,
                day,
                hour,
                minute,
                action: Action::WakesUp,
            })
        } else if caps.name("sleep").is_some() {
            Ok(Record {
                year,
                month,
                day,
                hour,
                minute,
                action: Action::FallsAsleep,
            })
        }
        else {
            Ok(Record {
                year,
                month,
                day,
                hour,
                minute,
                action: Action::StartShift(get_cap_int(&caps, "guard")?),
            })
        }
    }
}

fn main() {
    dispatch(&part1, &part2)
}

fn part1(_input: &str) -> Result<i32> {
    Ok(0)
}

fn part2(_input: &str) -> Result<i32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_begin() -> Result<()> {
        let record: Record = "[1518-11-01 22:34] Guard #10 begins shift".parse()?;
        assert_eq!(
            record,
            Record {
                year: 1518,
                month: 11,
                day: 01,
                hour: 22,
                minute: 34,
                action: Action::StartShift(10),
            }
        );
        Ok(())
    }

    #[test]
    fn test_parse_wake() -> Result<()> {
        let record: Record = "[1518-11-01 00:25] wakes up".parse()?;
        assert_eq!(
            record,
            Record {
                year: 1518,
                month: 11,
                day: 01,
                hour: 0,
                minute: 25,
                action: Action::WakesUp,
            }
        );
        Ok(())
    }

    #[test]
    fn test_parse_sleep() -> Result<()> {
        let record: Record = "[1518-11-01 00:25] falls asleep".parse()?;
        assert_eq!(
            record,
            Record {
                year: 1518,
                month: 11,
                day: 01,
                hour: 0,
                minute: 25,
                action: Action::FallsAsleep,
            }
        );
        Ok(())
    }
}
