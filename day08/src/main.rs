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
    } else if task == "value" {
        println!("{}", node_value(&tree));
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

fn node_value(tree: &[i32]) -> i32 {
    /// return the value of the root node of the tree
    let (v, _) = _node_value(tree);
    v
}

fn _node_value(tree: &[i32]) -> (i32, usize) {
    /// recursive helper that returns both the value and how many items are consumed
    let n_children = tree[0];
    let n_metadata = tree[1] as usize;

    if n_children == 0 {
        (
            tree[2..(2 + n_metadata)].iter().sum(),
            2 + (n_metadata as usize),
        )
    } else {
        let mut children: Vec<i32> = Vec::new();
        let mut idx = 2;
        for _i in 0..n_children {
            let (child_value, child_idx) = _node_value(&tree[idx..]);
            children.push(child_value);
            idx += child_idx;
        }
        let mut value: i32 = 0;
        for child in tree[idx..(idx + n_metadata)].iter() {
            value += children.get((*child - 1) as usize).unwrap_or(&0);
        }
        idx += n_metadata;
        (value, idx)
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

    #[test]
    fn test_root_value() {
        struct TestCase {
            input: Vec<i32>,
            output: i32,
        }

        let cases: Vec<TestCase> = vec![
            TestCase {
                input: vec![1, 2, 0, 1, 7, 1, 1],
                output: 14, // i think
            },
            TestCase {
                input: vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2],
                output: 66,
            },
        ];

        for ref case in &cases[..] {
            assert_eq!(case.output, node_value(&case.input));
        }
    }
}
