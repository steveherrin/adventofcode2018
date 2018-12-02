use std::collections::HashMap;
use std::env;
use std::fs;
use std::iter::FromIterator;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let ids: Vec<String> = Vec::from_iter(
        fs::read_to_string(filename)
            .unwrap()
            .split_whitespace()
            .map(|s| s.to_owned()),
    );
    println!("{}", checksum(&ids));
}

fn count_letters(id: &String) -> HashMap<char, u64> {
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

fn id_to_checksum_item(id: &String) -> ChecksumItem {
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

fn checksum(ids: &Vec<String>) -> u64 {
    let mut n_2: u64 = 0;
    let mut n_3: u64 = 0;
    for id in ids {
        let item_for_id = id_to_checksum_item(id);
        if item_for_id.contains_2 {
            n_2 += 1;
        }
        if item_for_id.contains_3 {
            n_3 += 1;
        }
    }
    n_2 * n_3
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
}
