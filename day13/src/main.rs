use std::cmp::Ordering;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let task = &args[1];
    let filename = &args[2];

    if task == "a" {
        println!("do something");
    } else {
        panic!("Don't know how to '{}'", task);
    }
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
            Turn::Right => Turn::Straight,
        };
        self.next_turn = next_turn;
        d
    }

    fn crash(&self, other: &Cart) -> bool {
        self.id != other.id && self.x == other.x && self.y == other.y
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
    n_y: usize,
}

enum TickResult {
    Success(Vec<Cart>),
    Collision((usize, usize)),
}

impl Tracks {
    fn idx(x: usize, y: usize) -> usize {
        0
    }

    fn at(&self, x: usize, y: usize) -> Track {
        Track::N
    }

    fn tick(&mut self, carts: &[Cart]) -> TickResult {
        let mut new_carts: Vec<Cart> = carts.iter().cloned().collect();
        new_carts.sort();

        for mut cart in &mut new_carts {
            let (new_x, new_y) = match cart.d {
                Direction::Up => (cart.x, cart.y + 1),
                Direction::Down => (cart.x, cart.y - 1),
                Direction::Right => (cart.x + 1, cart.y),
                Direction::Left => (cart.x - 1, cart.y),
            };

            let track = self.at(new_x, new_y);
            cart.move_to(new_x, new_y, track);
            if carts.iter().any(|other| cart.crash(other)) {
                return TickResult::Collision((new_x, new_y));
            }
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
