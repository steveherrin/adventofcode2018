#![allow(unused_doc_comments)]

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let task = &args[1];

    if task == "best3" {
        let serial = args[2].parse::<u64>().unwrap();
        let (x, y) = best_3_square(serial);
        println!("{},{}", x, y);
    } else if task == "bestn" {
        let serial = args[2].parse::<u64>().unwrap();
        let (x, y, d) = best_square(serial);
        println!("{},{},{}", x, y, d);
    } else {
        panic!("Don't know how to '{}'", task);
    }
}
static BOARD_X: usize = 300;
static BOARD_Y: usize = 300;

struct Board {
    cells: Vec<i64>,
}

impl Board {
    fn new(serial: u64) -> Board {
        let mut cells: Vec<i64> = vec![0; BOARD_X * BOARD_Y];
        for i in 0..BOARD_X {
            for j in 0..BOARD_Y {
                cells[i + BOARD_X * j] = power_level(i + 1, j + 1, serial);
            }
        }
        Board { cells }
    }

    fn power_in_square(&self, x: usize, y: usize, d: usize) -> i64 {
        let mut sum: i64 = 0;
        for i in x..(x + d) {
            for j in y..(y + d) {
                let idx = (i - 1) + BOARD_X * (j - 1);
                let p = self.cells[idx];
                sum += p;
            }
        }
        sum
    }
}

fn power_level(x: usize, y: usize, serial: u64) -> i64 {
    let rack_id = (x + 10) as u64;
    let mut power_level = serial + y as u64 * rack_id;
    power_level *= rack_id;
    power_level = (power_level / 100) % 10; // select 3rd digit
    (power_level as i64) - 5
}

fn best_3_square(serial: u64) -> (usize, usize) {
    let board = Board::new(serial);
    let mut best_x: usize = 0;
    let mut best_y: usize = 0;
    let mut max_power: i64 = 0;

    for x in 1..=(BOARD_X - 3) {
        for y in 1..=(BOARD_Y - 3) {
            let power = board.power_in_square(x, y, 3);
            if power > max_power {
                best_x = x;
                best_y = y;
                max_power = power;
            }
        }
    }
    (best_x, best_y)
}

fn best_square(serial: u64) -> (usize, usize, usize) {
    let board = Board::new(serial);
    let mut best_d: usize = 0;
    let mut best_x: usize = 0;
    let mut best_y: usize = 0;
    let mut max_power: i64 = 0;


    for d in 1..=BOARD_X {
        for x in 1..=(BOARD_X - d + 1) {
            for y in 1..=(BOARD_Y - d + 1) {
                let power = board.power_in_square(x, y, d);
                if power > max_power {
                    best_d = d;
                    best_x = x;
                    best_y = y;
                    max_power = power;
                }
            }
        }
    }
    (best_x, best_y, best_d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_level() {
        struct TestCase {
            x: usize,
            y: usize,
            serial: u64,
            output: i64,
        }

        let cases: Vec<TestCase> = vec![
            TestCase {
                x: 3,
                y: 5,
                serial: 8,
                output: 4,
            },
            TestCase {
                x: 122,
                y: 79,
                serial: 57,
                output: -5,
            },
            TestCase {
                x: 217,
                y: 196,
                serial: 39,
                output: 0,
            },
            TestCase {
                x: 101,
                y: 153,
                serial: 71,
                output: 4,
            },
        ];

        for ref case in cases {
            assert_eq!(case.output, power_level(case.x, case.y, case.serial));
        }
    }

    #[test]
    fn test_power_in_square() {
        struct TestCase {
            x: usize,
            y: usize,
            serial: u64,
            output: i64,
        }

        let cases: Vec<TestCase> = vec![
            TestCase {
                x: 33,
                y: 45,
                serial: 18,
                output: 29,
            },
            TestCase {
                x: 21,
                y: 61,
                serial: 42,
                output: 30,
            },
        ];

        for ref case in cases {
            let board = Board::new(case.serial);
            assert_eq!(case.output, board.power_in_square(case.x, case.y, 3));
        }
    }
}
