#![allow(unused_doc_comments)]
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let task = &args[1];
    let filename = &args[2];

    let steps: Vec<Edge> = fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .map(|x| read_step(x))
        .collect();

    if task == "foo" {
        println!("bar");
    } else {
        panic!("Don't know how to '{}'", task);
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Edge {
    first: char,
    second: char,
}

impl Edge {
    fn new(first: char, second: char) -> Edge {
        Edge { first, second }
    }
}

fn read_step(step: &str) -> Edge {
    // assumes step reads "Step A must be finished before step B can begin."
    let chars: Vec<char> = step.chars().collect();
    let first = chars[5];
    let second = chars[36];
    Edge::new(first, second)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        struct TestCase {
            input: String,
            output: Edge,
        }

        let cases: Vec<TestCase> = vec![
            TestCase {
                input: String::from("Step A must be finished before step B can begin."),
                output: Edge::new('A', 'B'),
            },
            TestCase {
                input: String::from("Step D must be finished before step Z can begin."),
                output: Edge::new('D', 'Z'),
            },
        ];

        for ref case in &cases[..] {
            assert_eq!(case.output, read_step(&case.input));
        }
    }
}
