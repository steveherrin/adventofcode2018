#![allow(unused_doc_comments)]

extern crate chrono;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

use chrono::{NaiveDateTime, ParseError, Timelike};

fn main() {
    let args: Vec<String> = env::args().collect();
    let task = &args[1];
    let filename = &args[2];

    let records: Vec<GuardRecord> = fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .map(|s| s.parse::<GuardRecord>())
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())  // I feel like there should be an idiom for this and the line above
        .collect();

    let naps = collect_naps(&records);

    if task == "sleepiest" {
        let guard = sleepiest_guard(&naps);
        let minutes = most_asleep(guard, &naps);
        println!("{} * {} = {}", guard, minutes, guard * minutes);
    } else {
        panic!("Don't know how to '{}'", task);
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum GuardEvent {
    StartShift(u32),
    FallAsleep,
    WakeUp,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct GuardRecord {
    timestamp: NaiveDateTime,
    event: GuardEvent,
}

impl Ord for GuardRecord {
    fn cmp(&self, other: &GuardRecord) -> Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

impl PartialOrd for GuardRecord {
    fn partial_cmp(&self, other: &GuardRecord) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
enum ParseRecordError {
    Chrono(ParseError),
    Number(ParseIntError),
    Empty,
}

impl From<ParseError> for ParseRecordError {
    fn from(err: ParseError) -> ParseRecordError {
        ParseRecordError::Chrono(err)
    }
}

impl From<ParseIntError> for ParseRecordError {
    fn from(err: ParseIntError) -> ParseRecordError {
        ParseRecordError::Number(err)
    }
}

impl FromStr for GuardRecord {
    type Err = ParseRecordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseRecordError::Empty);
        }

        // assume starts with "[YYYY-mm-dd HH:MM]"
        let timestamp = NaiveDateTime::parse_from_str(&s[1..17], "%Y-%m-%d %H:%M")?;

        let event = {
            if s.contains("falls asleep") {
                GuardEvent::FallAsleep
            } else if s.contains("wakes up") {
                GuardEvent::WakeUp
            } else {
                // assume first number starting with '#' is the guard number
                let n = s
                    .split_whitespace()
                    .filter(|&x| x.starts_with('#'))
                    .next()
                    .unwrap_or("")
                    .trim_left_matches('#')
                    .parse::<u32>()?;
                GuardEvent::StartShift(n)
            }
        };

        Ok(Self { timestamp, event })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Nap {
    guard: u32,
    start: u32,
    end: u32,
}

fn collect_naps(records: &[GuardRecord]) -> Vec<Nap> {
    /// Collect a set of observations of guards into a collection of naps
    let mut naps: Vec<Nap> = Vec::new();

    let mut sorted_records = records.to_vec();
    sorted_records.sort();

    let mut guard = 0;
    let mut start = sorted_records[0].timestamp;

    for record in sorted_records {
        match record.event {
            GuardEvent::StartShift(g) => guard = g,
            GuardEvent::FallAsleep => start = record.timestamp,
            GuardEvent::WakeUp => {
                let end = record.timestamp;
                if start.date() != end.date() {
                    panic!("A guard's nap extended past the observation period");
                }
                if start.time().hour() != end.time().hour() {
                    panic!("Multiple hours are involved");
                }
                naps.push(Nap { guard, start: start.time().minute(), end: end.time().minute() });
            },
        }
    }
    naps
}

fn sleepiest_guard(naps: &[Nap]) -> u32 {
    /// Given a bunch of naps, return the guard that sleeps the most (total)
    let mut guard_naptime: HashMap<u32, u64> = HashMap::new();
    for nap in naps {
        let t = guard_naptime.entry(nap.guard).or_insert(0);
        *t += (nap.end - nap.start) as u64;
    }
    let (sleepiest_guard, _) = guard_naptime.iter().max_by_key(|(&_guard, &time)| time).unwrap();
    sleepiest_guard.to_owned()
}

fn most_asleep(guard: u32, naps: &[Nap]) -> u32 {
    /// Given a guard number and a bunch of naps, return when the guard is most often asleep
    let mut minutes: [u64; 60] = [0; 60];
    for nap in naps.iter().filter(|n| n.guard == guard) {
        for min in nap.start..nap.end {
            minutes[min as usize] += 1
        }
    }
    let (min, _) = minutes.iter().enumerate().max_by_key(|&(_min, t)| t).unwrap();
    min as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::{NaiveDate};

    #[test]
    fn test_record_parse() {
        struct TestCase {
            input: String,
            output: GuardRecord,
        }

        let cases: Vec<TestCase> = vec![
            TestCase {
                input: String::from("[1518-11-01 00:00] Guard #10 begins shift"),
                output: GuardRecord {
                    timestamp: NaiveDate::from_ymd(1518, 11, 1).and_hms(0, 0, 0),
                    event: GuardEvent::StartShift(10),
                },
            },
            TestCase {
                input: String::from("[1518-11-01 00:30] falls asleep"),
                output: GuardRecord {
                    timestamp: NaiveDate::from_ymd(1518, 11, 1).and_hms(0, 30, 0),
                    event: GuardEvent::FallAsleep,
                },
            },
            TestCase {
                input: String::from("[1518-11-02 00:40] wakes up"),
                output: GuardRecord {
                    timestamp: NaiveDate::from_ymd(1518, 11, 2).and_hms(0, 40, 0),
                    event: GuardEvent::WakeUp,
                },
            },
        ];

        for ref case in &cases[..] {
            assert_eq!(case.output, case.input.parse::<GuardRecord>().unwrap());
        }
    }

    #[test]
    fn test_record_parse_error() {
        let cases: Vec<String> = vec![String::from("")];

        for ref case in &cases[..] {
            assert!(case.parse::<GuardRecord>().is_err());
        }
    }

    #[test]
    fn test_collect_naps() {
        struct TestCase {
            input: Vec<GuardRecord>,
            output: Vec<Nap>,
        }

        let cases: Vec<TestCase> = vec![
            TestCase {
                input: vec![
                    GuardRecord {
                        timestamp: NaiveDate::from_ymd(1518, 11, 5).and_hms(0, 55, 0),
                        event: GuardEvent::WakeUp,
                    },
                    GuardRecord {
                        timestamp: NaiveDate::from_ymd(1518, 11, 4).and_hms(0, 46, 0),
                        event: GuardEvent::WakeUp,
                    },
                    GuardRecord {
                        timestamp: NaiveDate::from_ymd(1518, 11, 3).and_hms(0, 29, 0),
                        event: GuardEvent::WakeUp,
                    },
                    GuardRecord {
                        timestamp: NaiveDate::from_ymd(1518, 11, 2).and_hms(0, 50, 0),
                        event: GuardEvent::WakeUp,
                    },
                    GuardRecord {
                        timestamp: NaiveDate::from_ymd(1518, 11, 1).and_hms(0, 55, 0),
                        event: GuardEvent::WakeUp,
                    },
                    GuardRecord {
                        timestamp: NaiveDate::from_ymd(1518, 11, 1).and_hms(0, 25, 0),
                        event: GuardEvent::WakeUp,
                    },
                    GuardRecord {
                        timestamp: NaiveDate::from_ymd(1518, 11, 1).and_hms(0, 5, 0),
                        event: GuardEvent::FallAsleep,
                    },
                    GuardRecord {
                        timestamp: NaiveDate::from_ymd(1518, 11, 1).and_hms(0, 30, 0),
                        event: GuardEvent::FallAsleep,
                    },
                    GuardRecord {
                        timestamp: NaiveDate::from_ymd(1518, 11, 2).and_hms(0, 40, 0),
                        event: GuardEvent::FallAsleep,
                    },
                    GuardRecord {
                        timestamp: NaiveDate::from_ymd(1518, 11, 3).and_hms(0, 24, 0),
                        event: GuardEvent::FallAsleep,
                    },
                    GuardRecord {
                        timestamp: NaiveDate::from_ymd(1518, 11, 4).and_hms(0, 36, 0),
                        event: GuardEvent::FallAsleep,
                    },
                    GuardRecord {
                        timestamp: NaiveDate::from_ymd(1518, 11, 5).and_hms(0, 45, 0),
                        event: GuardEvent::FallAsleep,
                    },
                    GuardRecord {
                        timestamp: NaiveDate::from_ymd(1518, 11, 5).and_hms(0, 3, 0),
                        event: GuardEvent::StartShift(99),
                    },
                    GuardRecord {
                        timestamp: NaiveDate::from_ymd(1518, 11, 4).and_hms(0, 2, 0),
                        event: GuardEvent::StartShift(99),
                    },
                    GuardRecord {
                        timestamp: NaiveDate::from_ymd(1518, 11, 3).and_hms(0, 5, 0),
                        event: GuardEvent::StartShift(10),
                    },
                    GuardRecord {
                        timestamp: NaiveDate::from_ymd(1518, 11, 1).and_hms(23, 58, 0),
                        event: GuardEvent::StartShift(99),
                    },
                    GuardRecord {
                        timestamp: NaiveDate::from_ymd(1518, 11, 1).and_hms(0, 0, 0),
                        event: GuardEvent::StartShift(10),
                    },
                ],
                output: vec![
                    Nap { guard: 10, start: 5, end: 25 },
                    Nap { guard: 10, start: 30, end: 55 },
                    Nap { guard: 99, start: 40, end: 50 },
                    Nap { guard: 10, start: 24, end: 29 },
                    Nap { guard: 99, start: 36, end: 46 },
                    Nap { guard: 99, start: 45, end: 55 },
                ],
            },
        ];
        for ref case in &cases[..] {
            assert_eq!(case.output, collect_naps(&case.input));
        }
    }


    #[test]
    fn test_sleepiest_guard() {
        struct TestCase {
            input: Vec<Nap>,
            output: u32,
        }

        let cases: Vec<TestCase> = vec![
            TestCase {
                input : vec![
                    Nap { guard: 10, start: 0, end: 30 },
                ],
                output: 10,
            },
            TestCase {
                input: vec![
                    Nap { guard: 10, start: 5, end: 25 },
                    Nap { guard: 10, start: 30, end: 55 },
                    Nap { guard: 99, start: 40, end: 50 },
                    Nap { guard: 10, start: 24, end: 29 },
                    Nap { guard: 99, start: 36, end: 46 },
                    Nap { guard: 99, start: 45, end: 55 },
                ],
                output: 10
            },
        ];

        for ref case in &cases[..] {
            assert_eq!(case.output, sleepiest_guard(&case.input));
        }
    }

    #[test]
    fn test_most_asleep() {
        struct TestCase {
            input: Vec<Nap>,
            output: u32,
        }

        let cases: Vec<TestCase> = vec![
            TestCase {
                input : vec![
                    Nap { guard: 10, start: 0, end: 31 },
                    Nap { guard: 10, start: 30, end: 32 },
                ],
                output: 30,
            },
            TestCase {
                input: vec![
                    Nap { guard: 10, start: 5, end: 25 },
                    Nap { guard: 10, start: 30, end: 55 },
                    Nap { guard: 99, start: 40, end: 50 },
                    Nap { guard: 10, start: 24, end: 29 },
                    Nap { guard: 99, start: 36, end: 46 },
                    Nap { guard: 99, start: 45, end: 55 },
                ],
                output: 24
            },
        ];

        for ref case in &cases[..] {
            assert_eq!(case.output, most_asleep(10, &case.input));
        }
    }
}
