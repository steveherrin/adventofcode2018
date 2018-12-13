use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

static SIZE: usize = 32768;
static OFFSET: usize = 16384;

fn main() {
    let args: Vec<String> = env::args().collect();
    let task = &args[1];
    let filename = &args[2];

    let mut row = read_file(filename).expect("Couldn't read file!");

    if task == "sum" {
        for _ in 0..20 {
            row.tick();
        }
        println!("{}", row.sum_of_plants());
    } else if task == "longsum" {
        // Hope the sum is increasing linearly over an
        // interval that evenly divides 50 billion.
        // Empirically, it seems to do it when I
        // run it, so I can solve the problem that way.
        // I need to think about if there's a proof this
        // has to happen, or if there are critria on the
        // rules, or what.
        let mut last_diff = 1;
        let mut last_sum = row.sum_of_plants();
        for i in 0..1000 {
            let skip = 100;
            if i % skip == 0 {
                let sum = row.sum_of_plants();
                let diff = sum - last_sum;
                if diff != last_diff {
                    last_diff = diff;
                    last_sum = sum;
                } else {
                    let extrapolation = sum + diff / skip * (50_000_000_000 - i);
                    println!("{}", extrapolation);
                    break;
                }
            }
            row.tick();
        }
    } else {
        panic!("Don't know how to '{}'", task);
    }
}

#[derive(Debug)]
enum ReadError {
    IO(std::io::Error),
    Malformed(String),
}

impl From<std::io::Error> for ReadError {
    fn from(err: std::io::Error) -> ReadError {
        ReadError::IO(err)
    }
}

fn read_file(filename: &str) -> Result<Row, ReadError> {
    let f = File::open(filename)?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();

    reader.read_line(&mut line)?;
    let state_str = line.split(": ").nth(1);
    if state_str.is_none() {
        return Result::Err(ReadError::Malformed(String::from(
            "State line seems malformed",
        )));
    }
    let state = parse_str(state_str.unwrap());

    let mut rules: HashMap<[Pot; 5], Pot> = HashMap::new();
    for mut maybe_line in reader.lines() {
        let rule_line = maybe_line?;
        if rule_line.trim().is_empty() {
            continue;
        }
        let rule_to_final: Vec<&str> = rule_line.trim().split(" => ").collect();
        let rule_str = rule_to_final[0];
        let mut rule_key = [Pot::None; 5];
        assert!(rule_str.len() == 5);
        for (i, c) in rule_str.chars().enumerate() {
            rule_key[i] = parse_char(c);
        }
        let rule_value = parse_str(rule_to_final[1].trim())[0];

        rules.insert(rule_key, rule_value);
    }
    Ok(Row::new(&state, rules))
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
    s.trim().chars().map(parse_char).collect()
}

#[cfg(test)]
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
        let region_with_plants: Vec<Pot> = row
            .pots
            .iter()
            .skip(OFFSET)
            .take(expected.len())
            .cloned()
            .collect();

        assert_eq!(expected, region_with_plants);
    }
}
