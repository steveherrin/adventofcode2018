use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let task = &args[1];
    let filename = &args[2];

    if task == "something" {
        println!("do something");
    } else {
        panic!("Don't know how to '{}'", task);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        struct TestCase {
            input: i64,
            expected: i64,
        }

        let cases = vec![
            TestCase {
                input: 0,
                expected: 0,
            },
        ];

        for case in cases {
            assert_eq!(case.expected, case.input);
        }
    }
}
