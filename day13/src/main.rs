use std::cmp::Ordering;
use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let task = &args[1];
    let filename = &args[2];

    let mut file = File::open(filename).expect("Couldn't open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Couldn't read from file");

    let (mut tracks, mut carts) = parse_tracks(&contents);

    if task == "collide" {
        let mut t = 0u64;
        loop {
            let r = tracks.tick(&carts);
            t += 1;
            match r {
                TickResult::Success(new_carts) => { carts = new_carts },
                TickResult::Collision((x, y)) => {
                    println!("Collision at t={}, at {},{}", t, x, y);
                    break;
                }
            }
        }
    } else {
        panic!("Don't know how to '{}'", task);
    }
}

fn parse_tracks(s: &str) -> (Tracks, Vec<Cart>) {
    let mut n_carts = 0u32;
    let mut n_x: Option<usize> = None;
    let mut grid: Vec<Track> = Vec::with_capacity(s.len());
    let mut carts : Vec<Cart> = Vec::new();

    let mut x = 0;
    let mut y = 0;
    for c in s.chars() {
        match c {
            '|' => grid.push(Track::V),
            '-' => grid.push(Track::H),
            '/' => grid.push(Track::RC),
            '\\' => grid.push(Track::LC),
            '+' => grid.push(Track::I),
            ' ' => grid.push(Track::N),
            '\n' => {
                if n_x.is_none() {
                    n_x = Some(x);
                } else {
                    assert_eq!(n_x.unwrap(), x);
                }
                y += 1;
                x = 0;
            },
            'v' => {
                grid.push(Track::V);
                carts.push(Cart::new(n_carts, x, y, Direction::Down));
                n_carts += 1;
            },
            '>' => {
                grid.push(Track::H);
                carts.push(Cart::new(n_carts, x, y, Direction::Right));
                n_carts += 1;
            },
            '^' => {
                grid.push(Track::V);
                carts.push(Cart::new(n_carts, x, y, Direction::Up));
                n_carts += 1;
            },
            '<' => {
                grid.push(Track::H);
                carts.push(Cart::new(n_carts, x, y, Direction::Left));
                n_carts += 1;
            },
            _ => panic!("invalid input character '{}'", c),
        }
        if c != '\n' {
            x += 1;
        }
    }
    (Tracks { grid, n_x: n_x.unwrap() }, carts)
}

// Next step is to read in the tracks from input
// Observed that > < are always on horizontal tracks,
// and ^ v are always on vertical tracks, so can read
// the input cell by cell
// Should just need to read it in and run it

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Track {
    V,  // vertical line '|'
    H,  // horizontal line '-'
    RC, // rightward-sloping curve '/'
    LC, // leftward-sloping curve '\'
    I,  // intersection '+'
    N,  // nothing ' '
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Turn {
    Left,
    Right,
    Straight,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Cart {
    id: u32,
    x: usize,
    y: usize,
    d: Direction,
    next_turn: Turn,
}

impl Ord for Cart {
    fn cmp(&self, other: &Cart) -> Ordering {
        if self.y < other.y {
            Ordering::Less
        } else if self.y > other.y {
            Ordering::Greater
        } else if self.x < other.x {
            Ordering::Less
        } else if self.x > other.x {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Cart) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Cart {
    fn new(id: u32, x: usize, y: usize, d: Direction) -> Cart {
        Cart { id, x, y, d, next_turn: Turn::Left }
    }

    fn intersection_turn(&mut self) -> Direction {
        let d = match (&self.next_turn, &self.d) {
            (Turn::Straight, _) => self.d,
            (Turn::Left, Direction::Left) => Direction::Down,
            (Turn::Left, Direction::Right) => Direction::Up,
            (Turn::Left, Direction::Up) => Direction::Left,
            (Turn::Left, Direction::Down) => Direction::Right,
            (Turn::Right, Direction::Left) => Direction::Up,
            (Turn::Right, Direction::Right) => Direction::Down,
            (Turn::Right, Direction::Up) => Direction::Right,
            (Turn::Right, Direction::Down) => Direction::Left,
        };
        self.d = d;
        let next_turn = match self.next_turn {
            Turn::Left => Turn::Straight,
            Turn::Straight => Turn::Right,
            Turn::Right => Turn::Left,
        };
        self.next_turn = next_turn;
        d
    }

    fn move_to(&mut self, x: usize, y: usize, track: Track) {
        let old_d = self.d;
        let d = match (track, &old_d) {
            (Track::V, _) => old_d,
            (Track::H, _) => old_d,
            (Track::RC, Direction::Up) => Direction::Right,
            (Track::RC, Direction::Down) => Direction::Left,
            (Track::RC, Direction::Right) => Direction::Up,
            (Track::RC, Direction::Left) => Direction::Down,
            (Track::LC, Direction::Up) => Direction::Left,
            (Track::LC, Direction::Down) => Direction::Right,
            (Track::LC, Direction::Right) => Direction::Down,
            (Track::LC, Direction::Left) => Direction::Up,
            (Track::I, _) => self.intersection_turn(),
            (Track::N, _) => panic!("Cart off the rails at {},{}", x, y),
        };

        self.x = x;
        self.y = y;
        self.d = d;
    }
}

struct Tracks {
    grid: Vec<Track>,
    n_x: usize,
}

enum TickResult {
    Success(Vec<Cart>),
    Collision((usize, usize)),
}

impl Tracks {
    fn at(&self, x: usize, y: usize) -> Track {
        let idx = x + y * self.n_x;
        self.grid[idx]
    }

    fn tick(&mut self, carts: &[Cart]) -> TickResult {
        let mut new_carts: Vec<Cart> = carts.iter().cloned().collect();
        new_carts.sort();
        let mut carts_at: VecDeque<(usize, usize)> = new_carts.iter().map(|c| (c.x, c.y)).collect();

        for mut cart in &mut new_carts {
            let (new_x, new_y) = match cart.d {
                Direction::Up => (cart.x, cart.y - 1),
                Direction::Down => (cart.x, cart.y + 1),
                Direction::Right => (cart.x + 1, cart.y),
                Direction::Left => (cart.x - 1, cart.y),
            };

            carts_at.pop_front();
            for (x, y) in &carts_at {
                if *x == new_x && *y == new_y {
                    return TickResult::Collision((*x, *y));
                }
            }
            carts_at.push_back((new_x, new_y));

            let track = self.at(new_x, new_y);
            cart.move_to(new_x, new_y, track);
        }
        TickResult::Success(new_carts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_plants() {
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
