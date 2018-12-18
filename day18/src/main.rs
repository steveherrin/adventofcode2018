#![allow(unused_doc_comments)]
use std::env;
use std::fmt;
use std::fs::File;
use std::io::Read;

#[macro_use]
extern crate itertools;

fn main() {
    let args: Vec<String> = env::args().collect();
    let task = &args[1];
    let filename = &args[2];

    let mut file = File::open(filename).expect("Couldn't open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Couldn't read from file");

    let mut board = read_board(&contents);

    if task == "resources" {
        for _ in 0..10 {
            board = board.evolve();
        }
        let n_tree = board.count_acre(Acre::Tree);
        let n_yard = board.count_acre(Acre::Yard);
        println!(
            "{} trees and {} lumberyards = {}",
            n_tree,
            n_yard,
            n_tree * n_yard
        );
    } else {
        panic!("Don't know how to '{}'", task);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Acre {
    Open,
    Tree,
    Yard,
}

#[derive(Debug, PartialEq, Eq)]
struct Board {
    acres: Vec<Acre>,
    size_x: usize,
    size_y: usize,
}

fn read_board(s: &str) -> Board {
    /// Read in a board from a string
    let mut row_length: Option<usize> = None;
    let mut acres: Vec<Acre> = Vec::with_capacity(s.len());

    let mut x = 0;
    for c in s.chars() {
        match c {
            '.' => acres.push(Acre::Open),
            '|' => acres.push(Acre::Tree),
            '#' => acres.push(Acre::Yard),
            '\n' => {
                if row_length.is_none() {
                    row_length = Some(x);
                } else {
                    assert_eq!(row_length.unwrap(), x);
                }
                x = 0;
            }
            _ => panic!("invalid input character '{}'", c),
        }
        if c != '\n' {
            x += 1;
        }
    }
    let size_x = row_length.unwrap();
    let size_y = acres.len() / size_x;
    Board {
        acres,
        size_x,
        size_y,
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, piece) in self.acres.iter().enumerate() {
            let c = match piece {
                Acre::Open => '.',
                Acre::Tree => '|',
                Acre::Yard => '#',
            };
            write!(f, "{}", c);
            if i % self.size_x == self.size_x - 1 {
                writeln!(f);
            }
        }
        Ok(())
    }
}

impl Board {
    fn new(size_x: usize, size_y: usize) -> Board {
        Board {
            acres: vec![Acre::Open; size_x * size_y],
            size_x,
            size_y,
        }
    }
    fn idx_to_xy(&self, i: usize) -> (i32, i32) {
        ((i % self.size_x) as i32, (i / self.size_x) as i32)
    }

    fn xy_to_idx(&self, x: i32, y: i32) -> usize {
        (x as usize) + (y as usize) * self.size_x
    }

    fn neighbors(&self, idx: usize) -> Vec<usize> {
        let (x, y) = self.idx_to_xy(idx);
        iproduct!((x - 1)..=(x + 1), (y - 1)..=(y + 1))
            .filter(|(i, _j)| *i >= 0)
            .filter(|(i, _j)| (*i as usize) < self.size_x)
            .filter(|(_i, j)| *j >= 0)
            .filter(|(_i, j)| (*j as usize) < self.size_y)
            .filter(|(i, j)| *i != x || *j != y)
            .map(|(x, y)| self.xy_to_idx(x, y))
            .collect()
    }

    fn count_acre(&self, acre: Acre) -> usize {
        self.acres.iter().filter(|&a| *a == acre).count()
    }

    fn evolve(&self) -> Board {
        let acres: Vec<Acre> = self
            .acres
            .iter()
            .enumerate()
            .map(|(idx, acre)| {
                (
                    acre,
                    self.neighbors(idx)
                        .iter()
                        .map(|n| self.acres[*n])
                        .collect::<Vec<_>>(),
                )
            }).map(|(acre, neighbors)| evolve_acre(*acre, &neighbors))
            .collect();
        Board {
            acres,
            size_x: self.size_x,
            size_y: self.size_y,
        }
    }
}

fn evolve_acre(acre: Acre, neighbors: &[Acre]) -> Acre {
    match acre {
        Acre::Open => {
            if neighbors.iter().filter(|&n| *n == Acre::Tree).count() >= 3 {
                Acre::Tree
            } else {
                Acre::Open
            }
        }
        Acre::Tree => {
            if neighbors.iter().filter(|&n| *n == Acre::Yard).count() >= 3 {
                Acre::Yard
            } else {
                Acre::Tree
            }
        }
        Acre::Yard => {
            let n_yard = neighbors.iter().filter(|&n| *n == Acre::Yard).count();
            let n_tree = neighbors.iter().filter(|&n| *n == Acre::Tree).count();
            if n_yard >= 1 && n_tree >= 1 {
                Acre::Yard
            } else {
                Acre::Open
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbors() {
        let board = Board::new(3, 3);
        assert_eq!(vec![3, 1, 4], board.neighbors(0));
        assert_eq!(vec![0, 3, 4, 2, 5], board.neighbors(1));
        assert_eq!(vec![1, 4, 5], board.neighbors(2));
        assert_eq!(vec![0, 6, 1, 4, 7], board.neighbors(3));
        assert_eq!(vec![0, 3, 6, 1, 7, 2, 5, 8], board.neighbors(4));
        assert_eq!(vec![1, 4, 7, 2, 8], board.neighbors(5));
        assert_eq!(vec![3, 4, 7], board.neighbors(6));
        assert_eq!(vec![3, 6, 4, 5, 8], board.neighbors(7));
        assert_eq!(vec![4, 7, 5], board.neighbors(8));
    }

    #[test]
    fn test_evolve() {
        let mut board = read_board(
            ".#.#...|#.\n\
             .....#|##|\n\
             .|..|...#.\n\
             ..|#.....#\n\
             #.#|||#|#|\n\
             ...#.||...\n\
             .|....|...\n\
             ||...#|.#|\n\
             |.||||..|.\n\
             ...#.|..|.\n",
        );
        for _ in 0..10 {
            board = board.evolve();
        }
        assert_eq!(37, board.count_acre(Acre::Tree));
        assert_eq!(31, board.count_acre(Acre::Yard));
    }
}
