#![allow(unused_doc_comments)]
use std::collections::{HashMap, HashSet};
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
    } else if task == "time" {
        let timed_steps: Vec<Step> = steps.iter().map(|step| Step::new(*step, 60)).collect();
        println!("{}", time_needed(5, &timed_steps, &dependencies))
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

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Step {
    id: char,
    time_left: i32,
}

impl Step {
    fn new(id: char, extra_time: i32) -> Step {
        let id_time = ((id as u32) - ('A' as u32) + 1u32) as i32;
        Step {
            id,
            time_left: id_time + extra_time,
        }
    }

    fn tick(&mut self) {
        self.time_left -= 1;
    }

    fn is_done(self) -> bool {
        self.time_left <= 0
    }
}

fn topological_sort(nodes: &HashSet<char>, dependencies: &[Dependency]) -> Vec<char> {
    /// Given a set of nodes and edges, return a topologically sorted vec of nodes
    // I could probably clean up this interface to match the problem 2 one. Ah well.
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

fn steps_available(
    steps: &HashMap<char, Step>,
    dependencies: &HashMap<char, Vec<char>>,
) -> Vec<char> {
    /// Given a map of id to Step, and a map of id to vec of dependencies, return unblocked steps
    let mut available: Vec<char> = Vec::new();
    for &step in steps.values() {
        if step.is_done() {
            continue;
        }
        if dependencies
            .get(&step.id)
            .unwrap_or(&vec![])
            .iter()
            .all(|prereq| steps[prereq].is_done())
        {
            available.push(step.id);
        }
    }
    available
}

fn time_needed(n_workers: usize, all_steps: &[Step], dependency_edges: &[Dependency]) -> i64 {
    /// Given n workers, Step nodes, and dependency edges, how long does everything take?
    // I could probably clean up the data structures here. Like,
    // the HashMap indexed by IDs, mapping those IDs to structs
    // containing those IDs could probably do with some refactoring.

    // map step id to the Step struct
    let mut steps: HashMap<char, Step> = HashMap::new();
    for step in all_steps.iter().cloned() {
        steps.insert(step.id, step);
    }

    // map step id to the ids of other steps its dependent on
    let mut dependencies: HashMap<char, Vec<char>> = HashMap::new();
    for &edge in dependency_edges {
        let mut d = dependencies
            .entry(edge.second)
            .or_insert_with(|| [].to_vec());
        d.push(edge.first);
    }

    // an array keeps track of what's being worked on
    let mut work: Vec<Option<char>> = vec![None; n_workers];
    let mut t: i64 = 0;
    while !steps.values().all(|s| s.is_done()) {
        // figure out what can be worked on, and only one worker can work on a thing at a time
        let mut avail: Vec<char> = steps_available(&steps, &dependencies)
            .iter()
            .filter(|&s| !work.contains(&Some(*s)))
            .cloned()
            .collect();
        // work on the first alphabetically available
        avail.sort();
        avail.reverse();

        t += 1;

        for i in 0..work.len() {
            // clear out anything done
            match work[i] {
                None => (),
                Some(s) => {
                    if steps[&s].is_done() {
                        work[i] = None;
                    }
                }
            };

            // fill up any free work
            match work[i] {
                None => work[i] = avail.pop(),
                Some(_) => (),
            }

            // tick time on anything being worked on
            match work[i] {
                None => (),
                Some(s) => {
                    steps.get_mut(&s).unwrap().tick();
                }
            }
        }
    }
    t
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

    #[test]
    fn test_time_for_step() {
        struct TestCase {
            step: Step,
            output: i32,
        }

        let mut cases: Vec<TestCase> = vec![
            TestCase {
                step: Step::new('A', 0),
                output: 1,
            },
            TestCase {
                step: Step::new('A', 10),
                output: 11,
            },
            TestCase {
                step: Step::new('B', 2),
                output: 4,
            },
        ];

        for case in cases.iter_mut() {
            assert_eq!(case.output, case.step.time_left);
            assert!(!case.step.is_done());
            for _i in 0..case.step.time_left {
                case.step.tick();
            }
            assert!(case.step.is_done());
        }
    }

    #[test]
    fn test_steps_available() {
        let mut steps: HashMap<char, Step> = HashMap::new();
        steps.insert('A', Step::new('A', 0));
        steps.insert('B', Step::new('B', 0));
        steps.insert('C', Step::new('C', 0));
        let mut dependencies: HashMap<char, Vec<char>> = HashMap::new();
        dependencies.insert('C', vec!['B', 'A']);
        dependencies.insert('B', vec!['A']);

        assert_eq!(vec!['A'], steps_available(&steps, &dependencies));
        steps.get_mut(&'A').unwrap().tick();
        assert_eq!(vec!['B'], steps_available(&steps, &dependencies));
        steps.get_mut(&'B').unwrap().time_left = 0;
        assert_eq!(vec!['C'], steps_available(&steps, &dependencies));
    }

    #[test]
    fn test_work_together() {
        struct TestCase {
            n_workers: usize,
            steps: Vec<Step>,
            dependencies: Vec<Dependency>,
            output: i64,
        }

        let cases: Vec<TestCase> = vec![
            TestCase {
                n_workers: 1,
                steps: vec![Step::new('A', 0), Step::new('B', 0)],
                dependencies: vec![],
                output: 3,
            },
            TestCase {
                n_workers: 2,
                steps: vec![Step::new('A', 0), Step::new('B', 0)],
                dependencies: vec![],
                output: 2,
            },
            TestCase {
                n_workers: 3,
                steps: vec![Step::new('A', 0), Step::new('B', 0)],
                dependencies: vec![],
                output: 2,
            },
            TestCase {
                n_workers: 2,
                steps: vec![Step::new('A', 0), Step::new('B', 0)],
                dependencies: vec![Dependency::new('A', 'B')],
                output: 3,
            },
            TestCase {
                n_workers: 2,
                steps: vec![
                    Step::new('A', 0),
                    Step::new('B', 0),
                    Step::new('C', 0),
                    Step::new('D', 0),
                    Step::new('E', 0),
                    Step::new('F', 0),
                ],
                dependencies: vec![
                    Dependency::new('C', 'A'),
                    Dependency::new('C', 'F'),
                    Dependency::new('A', 'B'),
                    Dependency::new('A', 'D'),
                    Dependency::new('B', 'E'),
                    Dependency::new('D', 'E'),
                    Dependency::new('F', 'E'),
                ],
                output: 15,
            },
        ];

        for ref case in &cases[..] {
            assert_eq!(
                case.output,
                time_needed(case.n_workers, &case.steps, &case.dependencies)
            );
        }
    }
}
