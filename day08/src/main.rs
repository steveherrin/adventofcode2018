#![allow(unused_doc_comments)]

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let task = &args[1];
    let filename = &args[2];

    let tree: Vec<i32> = fs::read_to_string(filename)
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    if task == "meta" {
        println!("{}", sum_metadata(&tree));
    } else {
        panic!("Don't know how to '{}'", task);
    }
}

fn sum_metadata(tree: &[i32]) -> i32 {
    /// sum all the "metadata" items from the tree
    let (s, _) = _sum_metadata(tree);
    s
}

fn _sum_metadata(tree: &[i32]) -> (i32, usize) {
    /// recursive helper that returns both the sum and how many items are consumed
    let n_children = tree[0];
    let n_metadata = tree[1] as usize;

    if n_children == 0 {
        (
            tree[2..(2 + n_metadata)].iter().sum(),
            2 + (n_metadata as usize),
        )
    } else {
        let mut sum = 0;
        let mut idx = 2;
        // recursively sum the child nodes, tracking how many items we use up
        for _i in 0..n_children {
            let (child_sum, child_idx) = _sum_metadata(&tree[idx..]);
            sum += child_sum;
            idx += child_idx;
        }
        // use up the remaining metadata items
        for _i in 0..n_metadata {
            sum += tree[idx + _i];
        }
        idx += n_metadata;
        (sum, idx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_metadata() {
        struct TestCase {
            input: Vec<i32>,
            output: i32,
        }

        let cases: Vec<TestCase> = vec![
            TestCase {
                input: vec![0, 2, 4, 6],
                output: 10,
            },
            TestCase {
                input: vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2],
                output: 138,
            },
        ];

        for ref case in &cases[..] {
            assert_eq!(case.output, sum_metadata(&case.input));
        }
    }
}
