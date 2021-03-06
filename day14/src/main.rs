use std::env;
use std::fmt;

fn main() {
    let args: Vec<String> = env::args().collect();
    let task = &args[1];

    let initial = Kitchen {
        scoreboard: vec![3, 7],
        elf_0_idx: 0,
        elf_1_idx: 1,
    };

    if task == "tenafter" {
        let n = &args[2].parse::<usize>().unwrap();
        let result = next_10_scores_after(&initial, *n);
        for i in result {
            print!("{}", i);
        }
        println!();
    } else if task == "before" {
        let desired: Vec<u8> = args[2]
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        let result = recipes_before(&initial, &desired);
        println!("{}", result);
    } else {
        panic!("Don't know how to '{}'", task);
    }
}

fn new_recipes(recipe_0: u8, recipe_1: u8) -> Vec<u8> {
    let sum = recipe_0 + recipe_1;
    let tens = sum / 10;
    let ones = sum % 10;
    if tens == 0 {
        vec![ones]
    } else {
        vec![tens, ones]
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Kitchen {
    scoreboard: Vec<u8>,
    elf_0_idx: usize,
    elf_1_idx: usize,
}

impl Kitchen {
    fn cooking_round(&mut self) {
        let recipe_0 = self.scoreboard[self.elf_0_idx];
        let recipe_1 = self.scoreboard[self.elf_1_idx];
        self.scoreboard.extend(new_recipes(recipe_0, recipe_1));
        self.elf_0_idx = (self.elf_0_idx + (recipe_0 as usize) + 1) % self.scoreboard.len();
        self.elf_1_idx = (self.elf_1_idx + (recipe_1 as usize) + 1) % self.scoreboard.len();
        if self.elf_0_idx == self.elf_1_idx {
            panic!("{}", self);
        }
    }
}

impl fmt::Display for Kitchen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, value) in self.scoreboard.iter().enumerate() {
            if i == self.elf_0_idx {
                write!(f, "({}) ", value);
            } else if i == self.elf_1_idx {
                write!(f, "[{}] ", value);
            } else {
                write!(f, "{} ", value);
            }
        }
        Ok(())
    }
}

fn next_10_scores_after(initial: &Kitchen, n_recipes: usize) -> Vec<u8> {
    let desired_recipes = n_recipes + 10;
    let mut kitchen = initial.clone();
    while kitchen.scoreboard.len() <= desired_recipes {
        kitchen.cooking_round();
    }
    kitchen.scoreboard[n_recipes..desired_recipes].to_vec()
}

fn recipes_before(initial: &Kitchen, desired: &[u8]) -> usize {
    let d_l = desired.len();
    let mut kitchen = initial.clone();
    let mut i: u64 = 0;
    loop {
        kitchen.cooking_round();

        // we only add at most 2 elements per loop
        // so check for our desired sequence right at
        // the end, and one off of the end
        let l = kitchen.scoreboard.len();
        if l > d_l && kitchen.scoreboard[(l - d_l)..] == *desired {
            return l - d_l;
        } else if l > d_l + 1 && kitchen.scoreboard[(l - d_l - 1)..(l - 1)] == *desired {
            return l - d_l - 1;
        }
        i += 1;
        if i > 1_000_000_000 {
            panic!("not terminating");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_recipes() {
        struct TestCase {
            input: (u8, u8),
            output: Vec<u8>,
        }

        let cases = vec![
            TestCase {
                input: (3, 7),
                output: vec![1, 0],
            },
            TestCase {
                input: (7, 3),
                output: vec![1, 0],
            },
            TestCase {
                input: (2, 3),
                output: vec![5],
            },
            TestCase {
                input: (9, 7),
                output: vec![1, 6],
            },
            TestCase {
                input: (0, 0),
                output: vec![0],
            },
            TestCase {
                input: (5, 6),
                output: vec![1, 1],
            },
        ];

        for case in cases {
            assert_eq!(case.output, new_recipes(case.input.0, case.input.1));
        }
    }

    #[test]
    fn test_cooking_rounds() {
        let mut state = Kitchen {
            scoreboard: vec![3, 7],
            elf_0_idx: 0,
            elf_1_idx: 1,
        };
        let expecteds: Vec<Kitchen> = vec![
            Kitchen {
                scoreboard: vec![3, 7, 1, 0],
                elf_0_idx: 0,
                elf_1_idx: 1,
            },
            Kitchen {
                scoreboard: vec![3, 7, 1, 0, 1, 0],
                elf_0_idx: 4,
                elf_1_idx: 3,
            },
            Kitchen {
                scoreboard: vec![3, 7, 1, 0, 1, 0, 1],
                elf_0_idx: 6,
                elf_1_idx: 4,
            },
            Kitchen {
                scoreboard: vec![3, 7, 1, 0, 1, 0, 1, 2],
                elf_0_idx: 0,
                elf_1_idx: 6,
            },
        ];

        for expected in expecteds {
            state.cooking_round();
            assert_eq!(expected, state);
        }
    }

    #[test]
    fn test_next_10_scores_after() {
        let initial = Kitchen {
            scoreboard: vec![3, 7],
            elf_0_idx: 0,
            elf_1_idx: 1,
        };
        struct TestCase {
            n_recipes: usize,
            output: Vec<u8>,
        }

        let cases = vec![
            TestCase {
                n_recipes: 9,
                output: vec![5, 1, 5, 8, 9, 1, 6, 7, 7, 9],
            },
            TestCase {
                n_recipes: 5,
                output: vec![0, 1, 2, 4, 5, 1, 5, 8, 9, 1],
            },
            TestCase {
                n_recipes: 18,
                output: vec![9, 2, 5, 1, 0, 7, 1, 0, 8, 5],
            },
            TestCase {
                n_recipes: 2018,
                output: vec![5, 9, 4, 1, 4, 2, 9, 8, 8, 2],
            },
        ];

        for case in cases {
            assert_eq!(case.output, next_10_scores_after(&initial, case.n_recipes));
        }
    }

    #[test]
    fn test_recipes_before() {
        let initial = Kitchen {
            scoreboard: vec![3, 7],
            elf_0_idx: 0,
            elf_1_idx: 1,
        };
        struct TestCase {
            desired: Vec<u8>,
            output: usize,
        }

        let cases = vec![
            TestCase {
                desired: vec![5, 1, 5, 8, 9],
                output: 9,
            },
            TestCase {
                desired: vec![0, 1, 2, 4, 5],
                output: 5,
            },
            TestCase {
                desired: vec![9, 2, 5, 1, 0],
                output: 18,
            },
            TestCase {
                desired: vec![5, 9, 4, 1, 4],
                output: 2018,
            },
        ];

        for case in cases {
            assert_eq!(case.output, recipes_before(&initial, &case.desired));
        }
    }

}
