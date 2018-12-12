use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

static SIZE: usize = 29;
static OFFSET: usize = 2;

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