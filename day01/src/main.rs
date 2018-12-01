use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let freq_changes = fs::read_to_string(filename).unwrap();

    let final_freq = change_freq(&freq_changes);
    println!("{}", final_freq);
}

fn change_freq(input: &String) -> i64 {
    input.split_whitespace().fold(0, |freq, change| freq + change.parse::<i64>().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        struct TestCase {
            output: i64,
            input: String,
        }

        let cases: Vec<TestCase> = vec![
            TestCase { input: String::from("+1\n-2\n+3\n+1\n"), output: 3 },
            TestCase { input: String::from("+1\n+1\n+1\n"), output: 3 },
            TestCase { input: String::from("+1\n+1\n-2\n"), output: 0 },
            TestCase { input: String::from("-1\n-2\n-3\n"), output: -6 },
        ];

        for ref case in &cases[..] {
            assert_eq!(case.output, change_freq(&case.input))
        }
    }
}
