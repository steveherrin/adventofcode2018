#![allow(unused_doc_comments)]
use std::collections::HashSet;
use std::iter::FromIterator;
use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();
    let task = &args[1];
    let filename = &args[2];

    let edges: Vec<Edge> = fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .map(|x| read_step(x))
        .collect();
    let nodes: HashSet<char> = edges
        .iter()
        .flat_map(|e| vec![e.first, e.second])
        .collect();

    if task == "foo" {
        println!("bar");
    } else {
        panic!("Don't know how to '{}'", task);
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
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

fn topological_sort(nodes: HashSet<char>, edges: &[Edge]) -> Vec<char> {
    // Kahn's Algorithm
    let mut sorted: Vec<char> = Vec::new();
    let mut start_nodes: HashSet<char> = nodes.clone();
    let mut remaining_edges: HashSet<Edge> = HashSet::from_iter(edges.iter().cloned());

    for edge in edges {
        start_nodes.remove(&edge.second);
    }

    while !start_nodes.is_empty() {
        let node = start_nodes.iter().next().unwrap().clone();
        start_nodes.remove(&node);

        sorted.push(node);

        let edges_to_check: Vec<Edge> = remaining_edges.iter().filter(|e| e.first == node).cloned().collect();
        for edge in edges_to_check {
            remaining_edges.remove(&edge);
            let n = edge.second;
            if !remaining_edges.iter().any(|e| n == e.second) {
                start_nodes.insert(n);
            }
        }
    }
    if !remaining_edges.is_empty() {
        panic!("circular dependency");
    }
    sorted
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
