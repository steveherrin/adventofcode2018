#![allow(unused_doc_comments)]

use std::collections::HashMap;
use std::env;
use std::fs;
use std::iter::FromIterator;

fn main() {
    let args: Vec<String> = env::args().collect();
    let task = &args[1];
    let filename = &args[2];

    let ids: Vec<String> = Vec::from_iter(
        fs::read_to_string(filename)
            .unwrap()
            .split_whitespace()
            .map(|s| s.to_owned()),
    );
    match task.as_str() {
        "checksum" => println!("{}", checksum(&ids)),
        "common" => println!("{}", common_for_similar_ids(&ids)),
        _ => panic!("Don't know how to '{}'", task),
    }
}

fn count_letters(id: &str) -> HashMap<char, u64> {
    /// Given an id, return a map of letter to number of occurrences in the id
    let mut counts: HashMap<char, u64> = HashMap::new();
    for c in id.chars() {
        let n_c = counts.entry(c).or_insert(0);
        *n_c += 1;
    }
    counts
}

#[derive(Debug, PartialEq, Eq)]
struct ChecksumItem {
    contains_2: bool,
    contains_3: bool,
}

fn id_to_checksum_item(id: &str) -> ChecksumItem {
    /// Given an id, return a struct of the information needed to compute the checksum
    let mut item: ChecksumItem = ChecksumItem {
        contains_2: false,
        contains_3: false,
    };
    for count in count_letters(id).values() {
        match count {
            2 => item.contains_2 = true,
            3 => item.contains_3 = true,
            _ => (),
        }
    }
    item
}

fn checksum(ids: &[String]) -> u64 {
    /// Given ids, return the checksum of the set
    let (n_2, n_3) = ids
        .iter()
        .map(|id| id_to_checksum_item(id))
        .fold((0, 0), |(n_2, n_3), item| {
            (n_2 + item.contains_2 as u64, n_3 + item.contains_3 as u64)
        });
    n_2 * n_3
}

fn are_similar(s1: &str, s2: &str) -> bool {
    /// return if the two strings are similar (differing in at most 1 place)
    let n_dif = Iterator::zip(s1.chars(), s2.chars())
        .filter(|(c1, c2)| c1 != c2)
        .count();
    n_dif <= 1
}

fn common_for_similar_ids(ids: &[String]) -> String {
    /// return the common characters between two similar ids
    for i in 1..ids.len() {
        let s1 = &ids[i - 1];
        for s2 in ids[i..].iter() {
            if are_similar(&s1, &s2) {
                return Iterator::zip(s1.chars(), s2.chars())
                    .filter(|(c1, c2)| c1 == c2)
                    .map(|(c1, _c2)| c1)
                    .collect::<String>();
            }
        }
    }
    panic!("No similar ids!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_letters() {
        struct TestCase {
            output: HashMap<char, u64>,
            input: String,
        }

        let cases: Vec<TestCase> = vec![
            TestCase {
                input: String::from("aaa"),
                output: [('a', 3)].iter().cloned().collect(),
            },
            TestCase {
                input: String::from("ababa"),
                output: [('a', 3), ('b', 2)].iter().cloned().collect(),
            },
        ];

        for ref case in &cases[..] {
            assert_eq!(case.output, count_letters(&case.input))
        }
    }

    #[test]
    fn test_id_to_checksum_item() {
        struct TestCase {
            output: ChecksumItem,
            input: String,
        }

        let cases: Vec<TestCase> = vec![
            TestCase {
                input: String::from("abcdef"),
                output: ChecksumItem {
                    contains_2: false,
                    contains_3: false,
                },
            },
            TestCase {
                input: String::from("bababc"),
                output: ChecksumItem {
                    contains_2: true,
                    contains_3: true,
                },
            },
            TestCase {
                input: String::from("abbcde"),
                output: ChecksumItem {
                    contains_2: true,
                    contains_3: false,
                },
            },
            TestCase {
                input: String::from("abcccd"),
                output: ChecksumItem {
                    contains_2: false,
                    contains_3: true,
                },
            },
            TestCase {
                input: String::from("aabcdd"),
                output: ChecksumItem {
                    contains_2: true,
                    contains_3: false,
                },
            },
            TestCase {
                input: String::from("abcdee"),
                output: ChecksumItem {
                    contains_2: true,
                    contains_3: false,
                },
            },
            TestCase {
                input: String::from("ababab"),
                output: ChecksumItem {
                    contains_2: false,
                    contains_3: true,
                },
            },
        ];

        for ref case in &cases[..] {
            assert_eq!(case.output, id_to_checksum_item(&case.input))
        }
    }

    #[test]
    fn test_checksum() {
        struct TestCase {
            output: u64,
            input: Vec<String>,
        }

        let cases: Vec<TestCase> = vec![TestCase {
            input: vec![
                String::from("abcdef"),
                String::from("bababc"),
                String::from("abbcde"),
                String::from("abcccd"),
                String::from("aabcdd"),
                String::from("abcdee"),
                String::from("ababab"),
            ],
            output: 12,
        }];

        for ref case in &cases[..] {
            assert_eq!(case.output, checksum(&case.input))
        }
    }

    #[test]
    fn test_are_similar() {
        struct TestCase {
            output: bool,
            input1: String,
            input2: String,
        }

        let cases: Vec<TestCase> = vec![
            TestCase {
                input1: String::from("abcde"),
                input2: String::from("axcye"),
                output: false,
            },
            TestCase {
                input1: String::from("fghij"),
                input2: String::from("fguij"),
                output: true,
            },
        ];

        for ref case in &cases[..] {
            assert_eq!(case.output, are_similar(&case.input1, &case.input2))
        }
    }

    #[test]
    fn test_common_for_similar_ids() {
        struct TestCase {
            output: String,
            input: Vec<String>,
        }

        let cases: Vec<TestCase> = vec![TestCase {
            input: vec![
                String::from("abcde"),
                String::from("fghij"),
                String::from("klmno"),
                String::from("pqrst"),
                String::from("fguij"),
                String::from("axcye"),
                String::from("wvxyz"),
            ],
            output: String::from("fgij"),
        }];

        for ref case in &cases[..] {
            assert_eq!(case.output, common_for_similar_ids(&case.input))
        }
    }
}
