#![allow(unused_doc_comments)]
use std::cmp::max;
use std::collections::VecDeque;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let task = &args[1];
    let filename = &args[2];

    let regex = fs::read_to_string(filename).expect("Couldn't read file");

    if task == "shortest" {
        println!("{}", direct_shortest_regex(&regex));
    } else {
        panic!("Don't know how to '{}'", task);
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

fn _direct_shortest_regex(regex: &str) -> usize {
    let mut queue: VecDeque<(usize, &str)> = VecDeque::from(vec![(0, regex)]);
    let mut shortest = 0;
    while !queue.is_empty() {
        let (l, s) = queue.pop_front().unwrap();
        let (head, branches, tail) = split_regex(s);

        let new_l = l + head.len();
        shortest = max(shortest, new_l);
        if !branches.iter().any(|b| b.is_empty()) {
            for branch in branches {
                queue.push_back((new_l, branch));
            }
        }
        if !tail.is_empty() {
            queue.push_back((new_l, tail));
        }
    }
    shortest
}

fn direct_shortest_regex(s: &str) -> usize {
    match (s.find('^'), s.find('$')) {
        (Some(c), Some(d)) => _direct_shortest_regex(&s[(c + 1)..d]),
        (_, _) => {
            panic!("Invalid regex");
        }
    }
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
            assert_eq!(case.expected, direct_shortest_regex(&case.input));
        }
    }
}
