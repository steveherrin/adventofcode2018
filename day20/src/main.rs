#![allow(unused_doc_comments)]
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

#[derive(Debug, PartialEq, Eq)]
struct Path {
    children: Vec<usize>,
    directions: String,
}

fn parse_regex(s: &str) -> Facility {
    Facility { paths: vec![] }
}

fn shortest_path(fac: &Facility) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

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
