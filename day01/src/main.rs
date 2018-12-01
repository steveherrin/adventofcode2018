#![allow(unused_doc_comments)]

use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let task = &args[1];
    let filename = &args[2];

    let freq_changes = fs::read_to_string(filename).unwrap();

    match task.as_str() {
        "change" => println!("{}", change_freq(&freq_changes)),
        "calibrate" => println!("{}", calibrate_freq(&freq_changes)),
        _ => panic!("Don't know how to '{}'", task),
    }
}

fn change_freq(changes: &String) -> i64 {
    /// Given frequency changes, return the final frequency
    ///
    /// # Arguments
    ///
    /// * `changes` - a whitespace-separated list of ±integer frequency changes
    changes
        .split_whitespace()
        .fold(0, |freq, change| freq + change.parse::<i64>().unwrap())
}

fn calibrate_freq(input: &String) -> i64 {
    /// Given frequency changes, find the first repeated frequency after applying them repeatedly
    ///
    /// # Arguments
    ///
    /// * `changes` - a whitespace-separated list of ±integer frequency changes
    let mut past_freqs: HashSet<i64> = HashSet::new();

    let mut freq: i64 = 0;

    for change in input.split_whitespace().cycle() {
        past_freqs.insert(freq);
        freq = freq + change.parse::<i64>().unwrap();
        if past_freqs.contains(&freq) {
            break;
        }
    }
    freq
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prob1_examples() {
        struct TestCase {
            output: i64,
            input: String,
        }

        let cases: Vec<TestCase> = vec![
            TestCase {
                input: String::from("+1\n-2\n+3\n+1\n"),
                output: 3,
            },
            TestCase {
                input: String::from("+1\n+1\n+1\n"),
                output: 3,
            },
            TestCase {
                input: String::from("+1\n+1\n-2\n"),
                output: 0,
            },
            TestCase {
                input: String::from("-1\n-2\n-3\n"),
                output: -6,
            },
        ];

        for ref case in &cases[..] {
            assert_eq!(case.output, change_freq(&case.input))
        }
    }

    #[test]
    fn test_prob2_examples() {
        struct TestCase {
            output: i64,
            input: String,
        }

        let cases: Vec<TestCase> = vec![
            TestCase {
                input: String::from("+1\n-1\n"),
                output: 0,
            },
            TestCase {
                input: String::from("+3\n+3\n+4\n-2\n-4\n"),
                output: 10,
            },
            TestCase {
                input: String::from("-6\n+3\n+8\n+5\n-6\n"),
                output: 5,
            },
            TestCase {
                input: String::from("+7\n+7\n-2\n-7\n-4\n"),
                output: 14,
            },
        ];

        for ref case in &cases[..] {
            assert_eq!(case.output, calibrate_freq(&case.input))
        }
    }
}
