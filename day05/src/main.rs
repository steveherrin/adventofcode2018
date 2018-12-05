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
    // Keep 2 stacks, one with the original string and one initially empty.
    // Compare the tops of the stacks. If they react, throw away both elements.
    // If they don't react, move the element from the original string onto
    // the new stack, and then repeat, until the original stack is exhausted
    let mut old: Vec<char> = s_orig.chars().collect();
    let mut new: Vec<char> = Vec::new();
    old.reverse(); // we want to pop from the start of the original
    while !old.is_empty() {
        match (old.pop(), new.pop()) {
            (None, None) => {}
            (None, Some(_n)) => {} // we've exhausted the original string
            (Some(o), None) => {
                new.push(o); // move the top of the original to the new stack
            }
            (Some(o), Some(n)) => {
                // compare the tops of the stacks
                if !(o != n && (o.to_ascii_lowercase() == n || o.to_ascii_uppercase() == n)) {
                    // no reaction, so put them both onto the new stack
                    new.push(n);
                    new.push(o);
                }
                // if they reacted, they're both gone; nothing to push
            }
        };
    }
    new.iter().collect::<String>()
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
