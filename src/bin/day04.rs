use aoc2018::{dispatch, Result};
use failure::{err_msg, Error};
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
enum Action {
    StartShift(u32),
    WakesUp,
    FallsAsleep,
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
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

fn part1(input: &str) -> Result<u32> {
    let mut records: Vec<Record> = input.split('\n').filter_map(|row| row.parse().ok()).collect();
    records.sort();

    let mut guard_sleeps: HashMap<u32, [usize; 60]> = HashMap::new();

    // let mut guard = 0;
    let mut sleep = 0;
    let mut sleeps = &mut[0; 60];
    for record in records {
        match record.action {
            Action::StartShift(id) => {
                // println!("a {}, {:?}", id, &sleeps[..]);
                sleeps = guard_sleeps.entry(id).or_insert([0; 60]);
            }
            Action::FallsAsleep => sleep = record.minute as usize,
            Action::WakesUp => {
                let wake = record.minute as usize;
                if sleep <= wake {
                    for min in sleep..wake {
                        sleeps[min] += 1;
                    }

                } else {
                    for min in sleep..60 {
                        sleeps[min] += 1;
                    }
                    for min in 0..wake {
                        sleeps[min] += 1;
                    }
                }
            },
        }
    }
    let (&longest_sleeper, _) = guard_sleeps.iter().max_by_key(|&(_,  &sleeps)| sleeps.iter().sum::<usize>()).unwrap();
    println!("longest_sleeper {:?}", longest_sleeper);
    let sleeps = guard_sleeps.get(&longest_sleeper).unwrap();

    let (minute, _) = sleeps.iter().enumerate().max_by_key(|&(_, x)| x).unwrap();

    Ok(longest_sleeper * minute as u32)
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

    #[test]
    fn test_part1() -> Result<()> {
        let res = part1("[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up")?;
        assert_eq!(res, 240);
        Ok(())
    }
}
