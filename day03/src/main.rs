#![allow(unused_doc_comments)]

use std::env;
use std::fs;
use std::num::ParseIntError;
use std::result::Result;
use std::str;
use std::str::FromStr;

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;
const SIZE: usize = WIDTH * HEIGHT;

fn main() {
    let args: Vec<String> = env::args().collect();
    let task = &args[1];
    let filename = &args[2];

    let claims: Vec<Claim> = fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<Claim>().unwrap())
        .collect();
    if task == "overlap" {
        println!("{}", count_overlapping(&claims));
    } else if task == "intact" {
        for id in find_intact(&claims) {
            println!("{}", id);
        }
    } else {
        panic!("Don't know how to '{}'", task);
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Claim {
    id: u64,
    x: usize, // distance between left edge of fabric and edge of claim
    y: usize, // distance between top edge of fabric and edge of claim
    width: usize,
    height: usize,
}

impl FromStr for Claim {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // rather than really deal with the Options from .get()
        // I abuse `unwrap_or` and  rely on the integer
        // parsing to avoid having to return Errors manually
        let fields: Vec<&str> = s.split_whitespace().collect();

        let raw_id = fields.get(0).unwrap_or(&"");
        let id = raw_id.trim_left_matches('#').parse::<u64>()?;

        let raw_edges = fields.get(2).unwrap_or(&"");
        let edges: Vec<&str> = raw_edges.trim_end_matches(':').split(',').collect();
        let x = edges.get(0).unwrap_or(&"").parse::<usize>()?;
        let y = edges.get(1).unwrap_or(&"").parse::<usize>()?;

        let raw_size = fields.get(3).unwrap_or(&"");
        let size: Vec<&str> = raw_size.split('x').collect();
        let width = size.get(0).unwrap_or(&"").parse::<usize>()?;
        let height = size.get(1).unwrap_or(&"").parse::<usize>()?;

        Ok(Self {
            id,
            x,
            y,
            width,
            height,
        })
    }
}

fn count_claims(claims: &[Claim]) -> Vec<u32> {
    /// Given claims, return an array of how many claims claim a given square
    let mut n_claims: Vec<u32> = vec![0; SIZE];
    for claim in claims {
        for x in claim.x..(claim.x + claim.width) {
            for y in claim.y..(claim.y + claim.height) {
                let idx = WIDTH * y + x;
                n_claims[idx] += 1
            }
        }
    }
    n_claims
}

fn count_overlapping(claims: &[Claim]) -> usize {
    /// Given claims, figure out how much overlapping area there is
    count_claims(claims).iter().filter(|&&n| n > 1).count()
}

fn find_intact(claims: &[Claim]) -> Vec<u64> {
    /// Given claims, return IDs that are intact
    let mut intact: Vec<u64> = Vec::new();
    let n_claims = count_claims(claims);
    for claim in claims {
        let mut is_intact = true;
        for x in claim.x..(claim.x + claim.width) {
            for y in claim.y..(claim.y + claim.height) {
                let idx = WIDTH * y + x;
                if n_claims[idx] != 1 {
                    is_intact = false;
                }
            }
        }
        if is_intact {
            intact.push(claim.id);
        }
    }
    intact
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claim_parse() {
        struct TestCase {
            input: String,
            output: Claim,
        }

        let cases: Vec<TestCase> = vec![TestCase {
            input: String::from("#123 @ 3,2: 5x4"),
            output: Claim {
                id: 123,
                x: 3,
                y: 2,
                width: 5,
                height: 4,
            },
        }];

        for ref case in &cases[..] {
            assert_eq!(case.output, case.input.parse::<Claim>().unwrap());
        }
    }

    #[test]
    fn test_claim_parse_error() {
        let cases: Vec<String> = vec![
            String::from("@ 3,2: 5x4"),
            String::from("#123 3,2: 5x4"),
            String::from("#123 @ 5x4"),
            String::from("#123 @ 3, 5x4"),
            String::from("#123 @ ,2 5x4"),
            String::from("#123 @ 3 5x4"),
            String::from("#123 @ 3,2"),
            String::from("#123 @ 3,2 5x"),
            String::from("#123 @ 3,2 x4"),
            String::from("#123 @ 3,2 5"),
        ];

        for ref case in &cases[..] {
            assert!(case.parse::<Claim>().is_err());
        }
    }

    #[test]
    fn test_count_overlapping() {
        struct TestCase {
            input: Vec<Claim>,
            output: usize,
        }

        let cases: Vec<TestCase> = vec![TestCase {
            input: vec![
                Claim {
                    id: 1,
                    x: 1,
                    y: 3,
                    width: 4,
                    height: 4,
                },
                Claim {
                    id: 2,
                    x: 3,
                    y: 1,
                    width: 4,
                    height: 4,
                },
                Claim {
                    id: 3,
                    x: 5,
                    y: 5,
                    width: 2,
                    height: 2,
                },
            ],
            output: 4,
        }];

        for ref case in &cases[..] {
            assert_eq!(case.output, count_overlapping(&case.input));
        }
    }

    #[test]
    fn test_find_intact() {
        struct TestCase {
            input: Vec<Claim>,
            output: Vec<u64>,
        }

        let cases: Vec<TestCase> = vec![TestCase {
            input: vec![
                Claim {
                    id: 1,
                    x: 1,
                    y: 3,
                    width: 4,
                    height: 4,
                },
                Claim {
                    id: 2,
                    x: 3,
                    y: 1,
                    width: 4,
                    height: 4,
                },
                Claim {
                    id: 3,
                    x: 5,
                    y: 5,
                    width: 2,
                    height: 2,
                },
            ],
            output: vec![3],
        }];

        for ref case in &cases[..] {
            assert_eq!(case.output, find_intact(&case.input));
        }
    }
}
