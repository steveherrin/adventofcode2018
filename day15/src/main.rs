use std::env;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let task = &args[1];
    let filename = &args[2];

    let mut file = File::open(filename).expect("Couldn't open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Couldn't read from file");

    let mut board = read_board(&contents);

    if task == "echo" {
        print!("{}", board);
    } else {
        panic!("Don't know how to '{}'", task);
    }
}

static STARTING_HP: i32 = 200;

enum Piece {
    Open,
    Wall,
    Elf(i32),
    Goblin(i32),
}

struct Board {
    grid: Vec<Piece>,
    size_x: usize,
}

fn read_board(s: &str) -> Board {
    let mut size_x: Option<usize> = None;
    let mut grid: Vec<Piece> = Vec::with_capacity(s.len());

    let mut x = 0;
    for c in s.chars() {
        match c {
            '.' => grid.push(Piece::Open),
            '#' => grid.push(Piece::Wall),
            'G' => grid.push(Piece::Goblin(STARTING_HP)),
            'E' => grid.push(Piece::Elf(STARTING_HP)),
            '\n' => {
                if size_x.is_none() {
                    size_x = Some(x);
                } else {
                    assert_eq!(size_x.unwrap(), x);
                }
                x = 0;
            }
            _ => panic!("invalid input character '{}'", c),
        }
        if c != '\n' {
            x += 1;
        }
    }
    Board {
        grid,
        size_x: size_x.unwrap(),
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, piece) in self.grid.iter().enumerate() {
            let c = match piece {
                Piece::Open => '.',
                Piece::Wall => '#',
                Piece::Goblin(_) => 'G',
                Piece::Elf(_) => 'E',
            };
            write!(f, "{}", c);
            if i % self.size_x == self.size_x - 1 {
                writeln!(f);
            }
        }
        Ok(())
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

        let cases = vec![TestCase {
            input: 0,
            expected: 0,
        }];

        for case in cases {
            assert_eq!(case.expected, case.input);
        }
    }
}
