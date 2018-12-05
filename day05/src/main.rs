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
}
