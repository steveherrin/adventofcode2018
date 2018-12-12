use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

static SIZE: usize = 2048;
static OFFSET: usize = 1024;

fn main() {
    let args: Vec<String> = env::args().collect();
    let task = &args[1];
    let filename = &args[2];

    let f = File::open(filename).unwrap();
    let mut reader = BufReader::new(f);
    let mut line = String::new();

    reader.read_line(&mut line).expect("Problem reading file");
    let parts: Vec<&str> = line.split(": ").collect();
    let state = parse_str(parts[1]);

    let mut rules: HashMap<[Pot; 5], Pot> = HashMap::new();
    for mut rule_line in reader.lines() {
        let rl = rule_line.unwrap();
        if rl.trim().is_empty() {
            continue;
        }
        let parts: Vec<&str> = rl.trim().split(" => ").collect();
        let rule_str = parts[0];
        let mut key = [Pot::None; 5];
        assert!(rule_str.len() == 5);
        for (i, c) in rule_str.chars().enumerate() {
            key[i] = parse_char(c);
        }
        let value = parse_str(parts[1].trim())[0];

        rules.insert(key, value);
    }

    let mut row = Row::new(&state, rules);
    if task == "sum" {
        for _ in 0..20 {
            row.tick();
        }
        println!("{}", row.sum_of_plants());
    } else {
        panic!("Don't know how to '{}'", task);
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Pot {
    Plant,
    None,
}

struct Row {
    pots: Vec<Pot>,
    rules: HashMap<[Pot; 5], Pot>,
}

impl Row {
    fn new(state: &[Pot], rules: HashMap<[Pot; 5], Pot>) -> Row {
        let pots = vec![Pot::None; SIZE];
        let mut row = Row { pots, rules };
        for (i, s) in state.iter().enumerate() {
            row.pots[i + OFFSET] = *s;
        }
        row
    }

    fn tick(&mut self) {
        let len = self.pots.len();
        let mut new_pots = vec![Pot::None; SIZE];
        for i in 2..(len - 2) {
            //TODO check limits
            let five_pots = &self.pots[(i - 2)..=(i + 2)];
            new_pots[i] = match self.rules.get(five_pots) {
                None => Pot::None,
                Some(p) => *p,
            }
        }
        self.pots = new_pots;
    }

    fn sum_of_plants(&self) -> i64 {
        self.pots
            .iter()
            .enumerate()
            .map(|(i, p)| match p {
                Pot::None => 0,
                Pot::Plant => (i as i64) - (OFFSET as i64),
            }).sum()
    }
}

fn parse_char(c: char) -> Pot {
    match c {
        '#' => Pot::Plant,
        _ => Pot::None,
    }
}

fn parse_str(s: &str) -> Vec<Pot> {
    s.trim()
        .chars()
        .map(|c| parse_char(c))
        .collect()
}

mod tests {
    use super::*;
    #[test]
    fn test_sum_of_plants() {
        struct TestCase {
            state: Vec<Pot>,
            expected: i64,
        }

        let cases = vec![
            TestCase {
                state: vec![Pot::None, Pot::None, Pot::None],
                expected: 0,
            },
            TestCase {
                state: vec![Pot::None, Pot::None, Pot::Plant],
                expected: 2,
            },
            TestCase {
                state: vec![Pot::Plant, Pot::Plant, Pot::Plant],
                expected: 3,
            },
            TestCase {
                state: vec![Pot::Plant, Pot::Plant, Pot::Plant, Pot::Plant],
                expected: 6,
            },
        ];

        for case in cases {
            let rules: HashMap<[Pot; 5], Pot> = HashMap::new();
            let row = Row::new(&case.state, rules);
            assert_eq!(case.expected, row.sum_of_plants());
        }
    }

    #[test]
    fn test_parse_str() {
        struct TestCase {
            input: String,
            output: Vec<Pot>,
        }

        let cases = vec![
            TestCase {
                input: String::from("#.#"),
                output: vec![Pot::Plant, Pot::None, Pot::Plant],
            },
            TestCase {
                input: String::from(" ..#\n"),
                output: vec![Pot::None, Pot::None, Pot::Plant],
            },
            TestCase {
                input: String::from(""),
                output: vec![],
            },
        ];

        for case in cases {
            assert_eq!(case.output, parse_str(&case.input));
        }
    }

    #[test]
    fn test_tick() {
        let mut rules: HashMap<[Pot; 5], Pot> = HashMap::new();
        rules.insert(
            [Pot::None, Pot::None, Pot::None, Pot::Plant, Pot::Plant],
            Pot::Plant,
        );
        rules.insert(
            [Pot::None, Pot::None, Pot::Plant, Pot::None, Pot::None],
            Pot::Plant,
        );
        rules.insert(
            [Pot::None, Pot::Plant, Pot::None, Pot::None, Pot::None],
            Pot::Plant,
        );
        rules.insert(
            [Pot::None, Pot::Plant, Pot::None, Pot::Plant, Pot::None],
            Pot::Plant,
        );
        rules.insert(
            [Pot::None, Pot::Plant, Pot::None, Pot::Plant, Pot::Plant],
            Pot::Plant,
        );
        rules.insert(
            [Pot::None, Pot::Plant, Pot::Plant, Pot::None, Pot::None],
            Pot::Plant,
        );
        rules.insert(
            [Pot::None, Pot::Plant, Pot::Plant, Pot::Plant, Pot::Plant],
            Pot::Plant,
        );
        rules.insert(
            [Pot::Plant, Pot::None, Pot::Plant, Pot::None, Pot::Plant],
            Pot::Plant,
        );
        rules.insert(
            [Pot::Plant, Pot::None, Pot::Plant, Pot::Plant, Pot::Plant],
            Pot::Plant,
        );
        rules.insert(
            [Pot::Plant, Pot::Plant, Pot::None, Pot::Plant, Pot::None],
            Pot::Plant,
        );
        rules.insert(
            [Pot::Plant, Pot::Plant, Pot::None, Pot::Plant, Pot::Plant],
            Pot::Plant,
        );
        rules.insert(
            [Pot::Plant, Pot::Plant, Pot::Plant, Pot::None, Pot::None],
            Pot::Plant,
        );
        rules.insert(
            [Pot::Plant, Pot::Plant, Pot::Plant, Pot::None, Pot::Plant],
            Pot::Plant,
        );
        rules.insert(
            [Pot::Plant, Pot::Plant, Pot::Plant, Pot::Plant, Pot::None],
            Pot::Plant,
        );

        let state = vec![
            Pot::Plant,
            Pot::None,
            Pot::None,
            Pot::Plant,
            Pot::None,
            Pot::Plant,
            Pot::None,
            Pot::None,
            Pot::Plant,
            Pot::Plant,
            Pot::None,
            Pot::None,
            Pot::None,
            Pot::None,
            Pot::None,
            Pot::None,
            Pot::Plant,
            Pot::Plant,
            Pot::Plant,
            Pot::None,
            Pot::None,
            Pot::None,
            Pot::Plant,
            Pot::Plant,
            Pot::Plant,
        ];

        let expected = vec![
            Pot::None,
            Pot::None,
            Pot::Plant,
            Pot::None,
            Pot::None,
            Pot::None,
            Pot::Plant,
            Pot::None,
            Pot::None,
            Pot::None,
            Pot::None,
            Pot::Plant,
            Pot::None,
            Pot::None,
            Pot::None,
            Pot::None,
            Pot::None,
            Pot::Plant,
            Pot::None,
            Pot::None,
            Pot::Plant,
            Pot::None,
            Pot::None,
            Pot::Plant,
            Pot::None,
            Pot::None,
            Pot::Plant,
            Pot::None,
            Pot::None,
        ];

        let mut row = Row::new(&state, rules);
        row.tick();
        assert_eq!(expected, row.pots);
    }
}
