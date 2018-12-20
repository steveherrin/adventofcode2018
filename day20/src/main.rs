#![allow(unused_doc_comments)]
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let task = &args[1];
    let filename = &args[2];

    let regex = fs::read_to_string(filename).expect("Couldn't read file");

    if task == "shortest" {
        let facility = parse_regex(&regex);
        println!("{}", shortest_path(&facility));
    } else {
        panic!("Don't know how to '{}'", task);
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Facility {
    paths: Vec<Path>,
}

impl Facility {
    fn new() -> Facility {
        Facility { paths: Vec::new() }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Path {
    children: Vec<usize>,
    directions: String,
}

impl Path {
    fn new(s: &str) -> Path {
        Path {
            children: Vec::new(),
            directions: s.to_string(),
        }
    }
}

fn split_regex(s: &str) -> (&str, Vec<&str>, &str) {
    /// take a string, split it into (head, branches, tail)
    let mut lparen: Option<usize> = None;
    let mut rparen: Option<usize> = None;
    let mut pipes: Vec<usize> = Vec::new();
    let mut open_parens = 0;
    for (i, c) in s.char_indices() {
        match c {
            '(' => {
                if open_parens == 0 {
                    lparen = Some(i);
                }
                open_parens += 1;
            }
            ')' => {
                assert!(open_parens > 0);
                open_parens -= 1;
                if open_parens == 0 {
                    rparen = Some(i);
                    break;
                }
            }
            '|' => {
                if open_parens == 1 {
                    pipes.push(i);
                }
            }
            _ => {}
        };
    }
    match (lparen, rparen) {
        (Some(l), Some(r)) => {
            let head = &s[..l];
            let tail = &s[(r + 1)..];
            let mut branches: Vec<&str> = vec![];
            if !pipes.is_empty() {
                let mut last = l + 1;
                for pipe in pipes {
                    branches.push(&s[last..pipe]);
                    last = pipe + 1;
                }
                branches.push(&s[last..r])
            }
            (head, branches, tail)
        }
        (None, None) => (s, Vec::new(), ""),
        (_, _) => {
            panic!("mismatched parentheses");
        }
    }
}

fn _parse_regex(regex: &str) -> Facility {
    let mut fac = Facility::new();
    let mut queue: VecDeque<(Option<usize>, &str)> = VecDeque::from(vec![(None, regex)]);
    while !queue.is_empty() {
        let (parent, s) = queue.pop_front().unwrap();
        let (head, branches, tail) = split_regex(s);

        let last_added = fac.paths.len();
        fac.paths.push(Path::new(head));
        match parent {
            Some(i) => {
                fac.paths[i].children.push(last_added);
            }
            None => {
                // if we just finished handling branches, need to mark
                // the new path as a child to all of them. Conveniently,
                // these will be the only ones with no children.
                for i in 0..last_added {
                    if fac.paths[i].children.is_empty() {
                        fac.paths[i].children.push(last_added);
                    }
                }
            }
        }

        for branch in branches {
            queue.push_back((Some(last_added), branch));
        }
        if !tail.is_empty() {
            queue.push_back((None, tail));
        }
    }
    fac
}

fn parse_regex(s: &str) -> Facility {
    match (s.find('^'), s.find('$')) {
        (Some(c), Some(d)) => _parse_regex(&s[(c + 1)..d]),
        (_, _) => {
            panic!("Invalid regex");
        }
    }
}
/*
fn shortest_path_no_recursion(fac: &Facility) -> usize {
    let mut longest_from: HashMap<usize, usize> = HashMap::new();
    let mut stack: Vec<(usize, usize)> = vec![(0, 0)];
    let mut longest = 0;
    while !stack.is_empty() {
        let (path_idx, len) = stack.pop().unwrap();
        let path = &fac.paths[path_idx];
        let new_len = len + path.directions.len();
        if path.children.is_empty() {
            longest_from.insert(path_idx, path.directions.len());
        }
        let memoized_len = longest_from.entry(path_idx).or_insert(new_len);
        *memoized_len = min(*memoized_len, new_len);
        match path
            .children
            .iter()
            .filter(|&c| fac.paths[*c].directions.len() == 0)
            .next()
        {
            Some(child) => {
                if !stack.iter().any(|(i, _)| i == child) {
                    stack.push((*child, new_len));
                }
            }
            None => {
                for child in &path.children {
                    if !stack.iter().any(|(i, _)| i == child) {
                        stack.push((*child, new_len));
                    }
                }
            }
        }
    }
    longest
}
*/

fn _shortest_path(fac: &Facility, start: usize, memo: &mut HashMap<usize, usize>) -> usize {
    if let Some(longest) = memo.get(&start) {
        return *longest;
    }
    let path = &fac.paths[start];
    let len = path.directions.len();
    if path.children.is_empty() {
        memo.insert(start, len);
        return len;
    }
    match path
        .children
        .iter()
        .find(|&c| fac.paths[*c].directions.is_empty())
    {
        Some(child) => {
            // in the case we have an empty option and can skip straight on
            let next_len = len + _shortest_path(fac, *child, memo);
            memo.insert(start, next_len);
            next_len
        }
        None => {
            let next_len = path
                .children
                .iter()
                .map(|c| _shortest_path(fac, *c, memo))
                .max()
                .unwrap_or(0)
                + len;
            memo.insert(start, next_len);
            next_len
        }
    }
}

fn shortest_path(fac: &Facility) -> usize {
    let mut hm: HashMap<usize, usize> = HashMap::new();
    _shortest_path(fac, 0, &mut hm)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split() {
        struct TestCase {
            input: String,
            expected: (String, Vec<String>, String),
        }

        let cases = vec![
            TestCase {
                input: "NEW".to_string(),
                expected: ("NEW".to_string(), vec![], "".to_string()),
            },
            TestCase {
                input: "N(E|S|)W".to_string(),
                expected: (
                    "N".to_string(),
                    vec!["E".to_string(), "S".to_string(), "".to_string()],
                    "W".to_string(),
                ),
            },
            TestCase {
                input: "(E|S)W".to_string(),
                expected: (
                    "".to_string(),
                    vec!["E".to_string(), "S".to_string()],
                    "W".to_string(),
                ),
            },
            TestCase {
                input: "N(E|S)W(N|E)".to_string(),
                expected: (
                    "N".to_string(),
                    vec!["E".to_string(), "S".to_string()],
                    "W(N|E)".to_string(),
                ),
            },
            TestCase {
                input: "N(E|(S|W))W".to_string(),
                expected: (
                    "N".to_string(),
                    vec!["E".to_string(), "(S|W)".to_string()],
                    "W".to_string(),
                ),
            },
        ];

        for case in cases {
            let (ex_head, ex_branches, ex_tail) = case.expected;
            let (head, branches, tail) = split_regex(&case.input);
            assert_eq!(ex_head, head);
            assert_eq!(ex_branches, branches);
            assert_eq!(ex_tail, tail);
        }
    }

    #[test]
    fn test_parse() {
        struct TestCase {
            input: String,
            expected: Facility,
        }

        let cases = vec![
            TestCase {
                input: "^WNE$".to_string(),
                expected: Facility {
                    paths: vec![Path {
                        children: vec![],
                        directions: "WNE".to_string(),
                    }],
                },
            },
            TestCase {
                input: "^N(NESW|WNES|)N$".to_string(),
                expected: Facility {
                    paths: vec![
                        Path {
                            children: vec![1, 2, 3],
                            directions: "N".to_string(),
                        },
                        Path {
                            children: vec![4],
                            directions: "NESW".to_string(),
                        },
                        Path {
                            children: vec![4],
                            directions: "WNES".to_string(),
                        },
                        Path {
                            children: vec![4],
                            directions: "".to_string(),
                        },
                        Path {
                            children: vec![],
                            directions: "N".to_string(),
                        },
                    ],
                },
            },
            TestCase {
                input: String::from("^ENWWW(NEEE|SSE(EE|N))$"),
                expected: Facility {
                    paths: vec![
                        Path {
                            children: vec![1, 2],
                            directions: "ENWWW".to_string(),
                        },
                        Path {
                            children: vec![],
                            directions: "NEEE".to_string(),
                        },
                        Path {
                            children: vec![3, 4],
                            directions: "SSE".to_string(),
                        },
                        Path {
                            children: vec![],
                            directions: "EE".to_string(),
                        },
                        Path {
                            children: vec![],
                            directions: "N".to_string(),
                        },
                    ],
                },
            },
        ];

        for case in cases {
            assert_eq!(case.expected, parse_regex(&case.input));
        }
    }

    #[test]
    fn test_shortest_path() {
        struct TestCase {
            input: String,
            expected: usize,
        }

        let cases = vec![
            TestCase {
                input: "^WNE$".to_string(),
                expected: 3,
            },
            TestCase {
                input: "^ENWWW(NEEE|SSE(EE|N))$".to_string(),
                expected: 10,
            },
            TestCase {
                input: "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$".to_string(),
                expected: 18,
            },
            TestCase {
                input: "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$".to_string(),
                expected: 23,
            },
            TestCase {
                input: "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$"
                    .to_string(),
                expected: 31,
            },
        ];

        for case in cases {
            let facility = parse_regex(&case.input);
            assert_eq!(case.expected, shortest_path(&facility));
        }
    }
}
