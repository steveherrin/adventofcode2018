#![allow(unused_doc_comments)]

use std::env;
use std::fmt;

fn main() {
    let args: Vec<String> = env::args().collect();
    let task = &args[1];

    if task == "highscore" {
        let n_players = args[2].parse::<usize>().unwrap();
        let last_marble = args[3].parse::<u32>().unwrap();
        println!("{}", high_score(n_players, last_marble));
    } else {
        panic!("Don't know how to '{}'", task);
    }
}

#[derive(Debug)]
struct GameBoard {
    board: Vec<u32>,
    current: usize,
}

impl GameBoard {
    fn new() -> GameBoard {
        GameBoard {
            board: vec![0],
            current: 0,
        }
    }

    fn insert(&mut self, value: u32) {
        let size = self.board.len();
        let insert_at = (self.current + 2) % size;
        if insert_at == 0 {
            self.board.push(value);
            self.current = size;
        } else {
            self.board.insert(insert_at, value);
            self.current = insert_at;
        }
    }

    fn remove(&mut self) -> u32 {
        let size = self.board.len();
        let remove_at = (self.current + size - 7) % size;
        self.current = remove_at;
        self.board.remove(remove_at)
    }
}

impl fmt::Display for GameBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, value) in self.board.iter().enumerate() {
            if i == self.current {
                write!(f, "({}) ", value);
            } else {
                write!(f, "{} ", value);
            }
        }
        Ok(())
    }
}

fn high_score(n_players: usize, last_marble: u32) -> u32 {
    let mut board = GameBoard::new();
    let mut scores: Vec<u32> = vec![0; n_players];

    let mut player: usize = 0;
    for marble in 1..=last_marble {
        if marble % 23 != 0 {
            board.insert(marble);
        } else {
            scores[player] += marble;
            scores[player] += board.remove();
        }
        player = (player + 1) % n_players;
    }
    *scores.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_high_score() {
        struct TestCase {
            n_players: usize,
            last_marble: u32,
            output: u32,
        }

        let cases: Vec<TestCase> = vec![
            TestCase {
                n_players: 9,
                last_marble: 25,
                output: 32,
            },
            TestCase {
                n_players: 10,
                last_marble: 1618,
                output: 8317,
            },
            TestCase {
                n_players: 13,
                last_marble: 7999,
                output: 146373,
            },
            TestCase {
                n_players: 17,
                last_marble: 1104,
                output: 2764,
            },
            TestCase {
                n_players: 21,
                last_marble: 6111,
                output: 54718,
            },
            TestCase {
                n_players: 30,
                last_marble: 5807,
                output: 37305,
            },
        ];

        for ref case in &cases[..] {
            assert_eq!(case.output, high_score(case.n_players, case.last_marble));
        }
    }
}
