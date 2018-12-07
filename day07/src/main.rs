#![allow(unused_doc_comments)]
use std::collections::HashSet;
use std::env;
use std::fs;
use std::iter::FromIterator;

fn main() {
    let args: Vec<String> = env::args().collect();
    let task = &args[1];
    let filename = &args[2];

    let dependencies: Vec<Dependency> = fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| Dependency::from_str(l))
        .collect();
    let steps: HashSet<char> = dependencies
        .iter()
        .flat_map(|d| vec![d.first, d.second])
        .collect();

    if task == "sort" {
        println!(
            "{}",
            topological_sort(&steps, &dependencies)
                .iter()
                .collect::<String>()
        );
    } else {
        panic!("Don't know how to '{}'", task);
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Dependency {
    first: char,
    second: char,
}

impl Dependency {
    fn new(first: char, second: char) -> Dependency {
        Dependency { first, second }
    }

    fn from_str(dependency: &str) -> Dependency {
        // assumes step reads "Step A must be finished before step B can begin."
        let chars: Vec<char> = dependency.chars().collect();
        let first = chars[5];
        let second = chars[36];
        Dependency::new(first, second)
    }
}

fn topological_sort(nodes: &HashSet<char>, dependencies: &[Dependency]) -> Vec<char> {
    // Kahn's Algorithm
    let mut sorted: Vec<char> = Vec::new();
    let mut remaining_deps: HashSet<Dependency> = HashSet::from_iter(dependencies.iter().cloned());

    loop {
        let mut start_nodes: HashSet<char> = nodes.clone();
        for edge in &remaining_deps {
            start_nodes.remove(&edge.second);
        }
        for used_node in &sorted {
            start_nodes.remove(&used_node);
        }

        let node = *start_nodes.iter().min().unwrap();
        start_nodes.remove(&node);
        sorted.push(node);

        let deps_satisfied: Vec<Dependency> = remaining_deps
            .iter()
            .filter(|e| e.first == node)
            .cloned()
            .collect();
        for edge in deps_satisfied {
            remaining_deps.remove(&edge);
        }

        if sorted.len() == nodes.len() {
            break;
        }
    }
    if !remaining_deps.is_empty() {
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
            output: Dependency,
        }

        let cases: Vec<TestCase> = vec![
            TestCase {
                input: String::from("Step A must be finished before step B can begin."),
                output: Dependency::new('A', 'B'),
            },
            TestCase {
                input: String::from("Step D must be finished before step Z can begin."),
                output: Dependency::new('D', 'Z'),
            },
        ];

        for ref case in &cases[..] {
            assert_eq!(case.output, Dependency::from_str(&case.input));
        }
    }

    #[test]
    fn test_topo_sort() {
        struct TestCase {
            nodes: HashSet<char>,
            edges: Vec<Dependency>,
            output: Vec<char>,
        }

        let cases: Vec<TestCase> = vec![
            TestCase {
                nodes: HashSet::from_iter(['A', 'B', 'C'].iter().cloned()),
                edges: vec![Dependency::new('A', 'C'), Dependency::new('C', 'B')],
                output: vec!['A', 'C', 'B'],
            },
            TestCase {
                nodes: HashSet::from_iter(['A', 'B', 'C'].iter().cloned()),
                edges: vec![Dependency::new('A', 'C'), Dependency::new('A', 'B')],
                output: vec!['A', 'B', 'C'],
            },
            TestCase {
                nodes: HashSet::from_iter(['A', 'B', 'C', 'D', 'E', 'F'].iter().cloned()),
                edges: vec![
                    Dependency::new('C', 'A'),
                    Dependency::new('C', 'F'),
                    Dependency::new('A', 'B'),
                    Dependency::new('A', 'D'),
                    Dependency::new('B', 'E'),
                    Dependency::new('D', 'E'),
                    Dependency::new('F', 'E'),
                ],
                output: vec!['C', 'A', 'B', 'D', 'F', 'E'],
            },
        ];

        for case in cases {
            assert_eq!(case.output, topological_sort(&case.nodes, &case.edges));
        }
    }
}
