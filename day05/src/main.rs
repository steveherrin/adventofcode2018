#![allow(unused_doc_comments)]

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let task = &args[1];
    let filename = &args[2];

    let raw = fs::read_to_string(filename).unwrap();
    let polymer = raw.trim();

    if task == "react" {
        let reacted = react(&polymer);
        println!("{} down to {}", polymer.len(), reacted.len());
    } else if task == "improve" {
        let (unit, improved) = best_removal(&polymer);
        println!(
            "Remove '{}' and it reduces down to {}",
            unit,
            improved.len()
        );
    } else {
        panic!("Don't know how to '{}'", task);
    }
}

fn react(s_orig: &str) -> String {
    let mut reacted = true;
    let mut s = String::from(s_orig.trim());
    while reacted {
        reacted = false;
        let mut s_new: Vec<char> = Vec::new();

        s = {
            let mut chars = s.chars();
            let mut maybe_a = chars.next();
            loop {
                if maybe_a.is_none() {
                    break;
                }
                let a = maybe_a.unwrap();

                let maybe_b = chars.next();
                if maybe_b.is_none() {
                    // out of letters; nothing for a to react with
                    s_new.push(a);
                    break;
                } else {
                    let b = maybe_b.unwrap();
                    if a != b && (a.to_ascii_lowercase() == b || a.to_ascii_uppercase() == b) {
                        reacted = true;
                        maybe_a = chars.next();
                    } else {
                        s_new.push(a);
                        maybe_a = maybe_b;
                    }
                }
            }
            s_new.iter().collect::<String>()
        };
    }
    s
}

fn remove_unit(undesired: char, polymer: &str) -> String {
    let und_lo = undesired.to_ascii_lowercase();
    let und_up = undesired.to_ascii_uppercase();
    polymer
        .chars()
        .filter(|&c| c != und_lo && c != und_up)
        .collect::<String>()
}

fn best_removal(polymer: &str) -> (char, String) {
    let mut units: Vec<char> = polymer.to_lowercase().chars().collect();
    units.sort();
    units.dedup();

    let mut best_unit: char = ' ';
    let mut best_reacted = String::from(polymer);

    for unit in units {
        let removed = remove_unit(unit, polymer);
        let reacted = react(&removed);
        if reacted.len() < best_reacted.len() {
            best_unit = unit;
            best_reacted = reacted;
        }
    }
    (best_unit, best_reacted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_react() {
        struct TestCase {
            input: String,
            output: String,
        }

        let cases: Vec<TestCase> = vec![
            TestCase {
                input: String::from("aA"),
                output: String::from(""),
            },
            TestCase {
                input: String::from("abBA"),
                output: String::from(""),
            },
            TestCase {
                input: String::from("abAB"),
                output: String::from("abAB"),
            },
            TestCase {
                input: String::from("aabAAB"),
                output: String::from("aabAAB"),
            },
            TestCase {
                input: String::from("dabAcCaCBAcCcaDA"),
                output: String::from("dabCBAcaDA"),
            },
        ];

        for ref case in &cases[..] {
            assert_eq!(case.output, react(&case.input));
        }
    }

    #[test]
    fn test_remove_unit() {
        struct TestCase {
            input: char,
            output: String,
        }

        let polymer = "dabAcCaCBAcCcaDA";

        let cases: Vec<TestCase> = vec![
            TestCase {
                input: 'a',
                output: String::from("dbcCCBcCcD"),
            },
            TestCase {
                input: 'b',
                output: String::from("daAcCaCAcCcaDA"),
            },
            TestCase {
                input: 'c',
                output: String::from("dabAaBAaDA"),
            },
            TestCase {
                input: 'd',
                output: String::from("abAcCaCBAcCcaA"),
            },
        ];

        for ref case in &cases[..] {
            assert_eq!(case.output, remove_unit(case.input, polymer));
        }
    }

    #[test]
    fn test_best_removal() {
        struct TestCase {
            input: String,
            output: (char, String),
        }

        let cases: Vec<TestCase> = vec![TestCase {
            input: String::from("dabAcCaCBAcCcaDA"),
            output: ('c', String::from("daDA")),
        }];

        for ref case in &cases[..] {
            assert_eq!(case.output, best_removal(&case.input));
        }
    }
}
