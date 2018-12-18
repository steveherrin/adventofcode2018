#![allow(unused_doc_comments)]
#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;
use std::fmt;
use std::fs;
use std::num::ParseIntError;

use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let task = &args[1];
    let filename = &args[2];

    if task == "fill" {
        let mut ground = read_file(filename).expect("Couldn't read input");
        let source_idx = ground.xy_to_idx(500, 0);
        ground.source(source_idx);
        println!("{}", ground.count_water_touched());
    } else if task == "fillanddrain" {
        let mut ground = read_file(filename).expect("Couldn't read input");
        let source_idx = ground.xy_to_idx(500, 0);
        ground.source(source_idx);
        println!("{}", ground.count_water());
    } else {
        panic!("Don't know how to '{}'", task);
    }
}

#[derive(Debug)]
enum ReadError {
    Missing,
    Int(ParseIntError),
}

impl From<ParseIntError> for ReadError {
    fn from(err: ParseIntError) -> ReadError {
        ReadError::Int(err)
    }
}

fn parse_line(s: &str) -> Result<(usize, usize, usize, usize), ReadError> {
    let x_first = s.starts_with("x=");
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"[xy]=(?P<c>\d+), [xy]=(?P<lo>\d+)\.\.(?P<hi>\d+)\w*").unwrap();
    }
    match RE.captures(s) {
        Some(cap) => {
            let c = cap.name("c").map_or("", |m| m.as_str()).parse::<usize>()?;
            let lo = cap.name("lo").map_or("", |m| m.as_str()).parse::<usize>()?;
            let hi = cap.name("hi").map_or("", |m| m.as_str()).parse::<usize>()?;
            if x_first {
                Ok((c, c, lo, hi))
            } else {
                Ok((lo, hi, c, c))
            }
        }
        None => Err(ReadError::Missing),
    }
}

fn read_file(filename: &str) -> Result<Ground, ReadError> {
    let clays: Vec<(usize, usize, usize, usize)> = fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(parse_line)
        .collect::<Result<Vec<_>, _>>()?;
    let x_max = clays.iter().map(|clay| clay.1).max().unwrap_or(0);
    let y_max = clays.iter().map(|clay| clay.3).max().unwrap_or(0);
    let mut ground = Ground::new(x_max + 2, y_max + 1);
    for clay in clays {
        ground.add_clay(clay.0, clay.1, clay.2, clay.3);
    }
    Ok(ground)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Square {
    Sand,
    Clay,
    Water,
    Flow,
}

#[derive(Debug)]
struct Ground {
    squares: Vec<Square>,
    size_x: usize,
    size_y: usize,
    min_y: usize,
    min_x: usize,
}

impl fmt::Display for Ground {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, piece) in self.squares.iter().enumerate() {
            let c = match piece {
                Square::Sand => '.',
                Square::Clay => '#',
                Square::Water => '~',
                Square::Flow => '|',
            };
            if i % self.size_x >= (self.min_x - 1) {
                write!(f, "{}", c);
            }
            if i % self.size_x == self.size_x - 1 {
                writeln!(f);
            }
        }
        Ok(())
    }
}

impl Ground {
    fn new(size_x: usize, size_y: usize) -> Ground {
        Ground {
            squares: vec![Square::Sand; size_x * size_y],
            size_x,
            size_y,
            min_y: size_y - 1,
            min_x: size_x - 1,
        }
    }

    fn idx_to_xy(&self, i: usize) -> (usize, usize) {
        (i % self.size_x, i / self.size_x)
    }

    fn xy_to_idx(&self, x: usize, y: usize) -> usize {
        x + y * self.size_x
    }

    fn find_boundary(&self, idx: usize, d: isize) -> usize {
        let (x_u, y) = self.idx_to_xy(idx);
        let mut x = x_u as isize;
        loop {
            let below = self.xy_to_idx(x as usize, y + 1);
            let adjacent = self.xy_to_idx((x + d) as usize, y);
            let x_in_range = (x + d) >= 0 && ((x + d) as usize) < self.size_x;
            if (y + 1 < self.size_y)
                && self.squares[below] != Square::Clay
                && self.squares[below] != Square::Water
            {
                return self.xy_to_idx(x as usize, y);
            } else if x_in_range && self.squares[adjacent] == Square::Clay {
                return adjacent;
            }
            x += d;
        }
    }

    fn find_boundary_left(&self, x: usize, y: usize) -> usize {
        let idx = self.xy_to_idx(x, y);
        self.find_boundary(idx, -1)
    }

    fn find_boundary_right(&self, x: usize, y: usize) -> usize {
        let idx = self.xy_to_idx(x, y);
        self.find_boundary(idx, 1)
    }

    fn source(&mut self, source_idx: usize) {
        let mut to_pour = vec![source_idx];
        while !to_pour.is_empty() {
            let pour_idx = to_pour.pop().unwrap();
            let spillovers = self.pour(pour_idx);
            to_pour.extend(spillovers);
        }
    }

    fn pour(&mut self, pour_idx: usize) -> Vec<usize> {
        /// Pour water down from the given index, return any new sources (from spillover)
        let (x, mut y) = self.idx_to_xy(pour_idx);
        let mut need_to_fill = false;
        loop {
            let idx = self.xy_to_idx(x, y);
            self.squares[idx] = Square::Flow;
            // if we fall off the bottom, or hit something, stop
            if y >= self.size_y - 1 {
                break;
            } else {
                let below = self.xy_to_idx(x, y + 1);
                if self.squares[below] == Square::Clay || self.squares[below] == Square::Water {
                    need_to_fill = true;
                    break;
                }
                y += 1;
            }
        }
        let mut new_sources: Vec<usize> = vec![];
        while need_to_fill {
            let b_l = self.find_boundary_left(x, y);
            let b_r = self.find_boundary_right(x, y);
            let s_l = self.squares[b_l];
            let s_r = self.squares[b_r];
            match (s_l, s_r) {
                (Square::Clay, Square::Clay) => {
                    self.fill_water(b_l, b_r);
                    y -= 1;
                }
                (Square::Sand, Square::Clay) => {
                    self.flow_water(b_l, b_r);
                    new_sources.push(b_l);
                    need_to_fill = false;
                }
                (Square::Clay, Square::Sand) => {
                    self.flow_water(b_l, b_r);
                    new_sources.push(b_r);
                    need_to_fill = false;
                }
                (Square::Sand, Square::Sand) => {
                    self.flow_water(b_l, b_r);
                    new_sources.push(b_l);
                    new_sources.push(b_r);
                    need_to_fill = false;
                }
                (Square::Flow, Square::Sand) => {
                    // see the pouring onto edge test
                    self.flow_water(b_l, b_r);
                    new_sources.push(b_r);
                    need_to_fill = false;
                }
                (Square::Sand, Square::Flow) => {
                    self.flow_water(b_l, b_r);
                    new_sources.push(b_l);
                    need_to_fill = false;
                }
                (_, _) => {
                    need_to_fill = false;
                } // ran into water that's already been handled
            }
        }
        new_sources
    }

    fn count_water_touched(&self) -> usize {
        self.squares
            .iter()
            .filter(|&s| *s == Square::Water || *s == Square::Flow)
            .count()
            - self.min_y
    }

    fn count_water(&self) -> usize {
        self.squares.iter().filter(|&s| *s == Square::Water).count()
    }

    fn add_clay(&mut self, x_lo: usize, x_hi: usize, y_lo: usize, y_hi: usize) {
        for y in y_lo..=y_hi {
            if y < self.min_y {
                self.min_y = y;
            }
            for x in x_lo..=x_hi {
                if x < self.min_x {
                    self.min_x = x;
                }
                let idx = self.xy_to_idx(x, y);
                self.squares[idx] = Square::Clay;
            }
        }
    }

    fn fill(&mut self, substance: Square, idx_lo: usize, idx_hi: usize) {
        let (x_lo, y) = self.idx_to_xy(idx_lo);
        let (x_hi, y_hi) = self.idx_to_xy(idx_hi);
        assert_eq!(y, y_hi);
        for x in x_lo..=x_hi {
            let idx = self.xy_to_idx(x, y);
            if self.squares[idx] != Square::Clay {
                self.squares[idx] = substance;
            }
        }
    }

    fn fill_water(&mut self, idx_lo: usize, idx_hi: usize) {
        self.fill(Square::Water, idx_lo, idx_hi)
    }

    fn flow_water(&mut self, idx_lo: usize, idx_hi: usize) {
        self.fill(Square::Flow, idx_lo, idx_hi)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_boundary() {
        let ground = Ground {
            squares: vec![
                Square::Clay,
                Square::Sand,
                Square::Sand,
                Square::Sand,
                Square::Sand,
                Square::Clay,
                Square::Clay,
                Square::Clay,
                Square::Clay,
                Square::Sand,
            ],
            size_x: 5,
            size_y: 2,
            min_x: 0,
            min_y: 0,
        };
        assert_eq!(0, ground.find_boundary(2, -1));
        assert_eq!(4, ground.find_boundary(2, 1));
    }

    #[test]
    fn test_pour_no_clay() {
        let mut ground_1 = Ground::new(1, 1);
        assert_eq!(vec![] as Vec<usize>, ground_1.pour(0));
        assert_eq!(vec![Square::Flow], ground_1.squares);

        let mut ground_9 = Ground::new(3, 3);
        assert_eq!(vec![] as Vec<usize>, ground_9.pour(1));
        assert_eq!(
            vec![
                Square::Sand,
                Square::Flow,
                Square::Sand,
                Square::Sand,
                Square::Flow,
                Square::Sand,
                Square::Sand,
                Square::Flow,
                Square::Sand
            ],
            ground_9.squares
        );
    }

    #[test]
    fn test_pour_cup() {
        let mut ground = Ground::new(5, 4);
        ground.add_clay(1, 1, 1, 3);
        ground.add_clay(3, 3, 1, 3);
        ground.add_clay(1, 3, 3, 3);
        assert_eq!(vec![0, 4], ground.pour(2));
        assert_eq!(
            vec![
                Square::Flow,
                Square::Flow,
                Square::Flow,
                Square::Flow,
                Square::Flow,
                Square::Sand,
                Square::Clay,
                Square::Water,
                Square::Clay,
                Square::Sand,
                Square::Sand,
                Square::Clay,
                Square::Water,
                Square::Clay,
                Square::Sand,
                Square::Sand,
                Square::Clay,
                Square::Clay,
                Square::Clay,
                Square::Sand,
            ],
            ground.squares
        );
    }

    #[test]
    fn test_pour_onto_edge() {
        let mut ground = Ground::new(6, 5);
        ground.add_clay(2, 2, 2, 4);
        ground.add_clay(4, 4, 2, 4);
        ground.add_clay(2, 4, 4, 4);
        ground.source(2);
        assert_eq!(
            vec![
                Square::Sand,
                Square::Sand,
                Square::Flow,
                Square::Sand,
                Square::Sand,
                Square::Sand,
                Square::Sand,
                Square::Flow,
                Square::Flow,
                Square::Flow,
                Square::Flow,
                Square::Flow,
                Square::Sand,
                Square::Flow,
                Square::Clay,
                Square::Water,
                Square::Clay,
                Square::Flow,
                Square::Sand,
                Square::Flow,
                Square::Clay,
                Square::Water,
                Square::Clay,
                Square::Flow,
                Square::Sand,
                Square::Flow,
                Square::Clay,
                Square::Clay,
                Square::Clay,
                Square::Flow,
            ],
            ground.squares
        );
    }

    #[test]
    fn test_count_water_touched() {
        let mut ground = Ground {
            squares: vec![Square::Sand; 196],
            size_x: 14,
            size_y: 14,
            min_x: 0,
            min_y: 1,
        };
        ground.add_clay(1, 1, 2, 7);
        ground.add_clay(0, 7, 7, 7);
        ground.add_clay(7, 7, 3, 7);
        ground.add_clay(4, 4, 2, 4);
        ground.add_clay(12, 12, 1, 2);
        ground.add_clay(4, 4, 10, 13);
        ground.add_clay(10, 10, 10, 13);
        ground.add_clay(4, 10, 13, 13);
        ground.source(6);
        assert_eq!(57, ground.count_water_touched());
    }

    #[test]
    fn test_count_water() {
        let mut ground = Ground {
            squares: vec![Square::Sand; 196],
            size_x: 14,
            size_y: 14,
            min_x: 0,
            min_y: 1,
        };
        ground.add_clay(1, 1, 2, 7);
        ground.add_clay(0, 7, 7, 7);
        ground.add_clay(7, 7, 3, 7);
        ground.add_clay(4, 4, 2, 4);
        ground.add_clay(12, 12, 1, 2);
        ground.add_clay(4, 4, 10, 13);
        ground.add_clay(10, 10, 10, 13);
        ground.add_clay(4, 10, 13, 13);
        ground.source(6);
        assert_eq!(29, ground.count_water());
    }
}
