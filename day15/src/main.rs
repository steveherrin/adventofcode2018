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

    if task == "round" {
        board.round();
        print!("{}", board);
        board.round();
        print!("{}", board);
        board.round();
        print!("{}", board);
    } else {
        panic!("Don't know how to '{}'", task);
    }
}

static STARTING_HP: i32 = 200;
static ATTACK_POWER: i32 = 3;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Piece {
    Open,
    Wall,
    Elf(i32),
    Goblin(i32),
}

impl Piece {
    fn is_goblin(self) -> bool {
        match self {
            Piece::Goblin(_) => true,
            _ => false,
        }
    }
    fn is_elf(self) -> bool {
        match self {
            Piece::Elf(_) => true,
            _ => false,
        }
    }
    fn is_open(self) -> bool {
        match self {
            Piece::Open => true,
            _ => false,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Board {
    grid: Vec<Piece>,
    size_x: usize,
    size_y: usize,
}

fn read_board(s: &str) -> Board {
    let mut row_length: Option<usize> = None;
    let mut grid: Vec<Piece> = Vec::with_capacity(s.len());

    let mut x = 0;
    for c in s.chars() {
        match c {
            '.' => grid.push(Piece::Open),
            '#' => grid.push(Piece::Wall),
            'E' => grid.push(Piece::Elf(STARTING_HP)),
            'G' => grid.push(Piece::Goblin(STARTING_HP)),
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
    let size_y = grid.len() / size_x;
    Board {
        grid,
        size_x,
        size_y,
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, piece) in self.grid.iter().enumerate() {
            let c = match piece {
                Piece::Open => '.',
                Piece::Wall => '#',
                Piece::Elf(_) => 'E',
                Piece::Goblin(_) => 'G',
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
    fn piece_locations(&self) -> Vec<usize> {
        self.grid
            .iter()
            .enumerate()
            .filter_map(|(i, p)| match p {
                Piece::Elf(_) => Some(i),
                Piece::Goblin(_) => Some(i),
                _ => None,
            }).collect()
    }

    fn goblin_locations(&self) -> Vec<usize> {
        self.piece_locations()
            .iter()
            .filter(|&i| self.grid[*i].is_goblin())
            .cloned()
            .collect()
    }
    fn elf_locations(&self) -> Vec<usize> {
        self.piece_locations()
            .iter()
            .filter(|&i| self.grid[*i].is_elf())
            .cloned()
            .collect()
    }
    fn in_range(&self, loc: usize) -> Vec<usize> {
        let (x, y) = self.idx_to_xy(loc);
        let mut neighbors: Vec<usize> = Vec::with_capacity(4);
        // the order results in the final vector being reading order
        if y > 0 {
            neighbors.push(self.xy_to_idx(x, y - 1));
        }
        if x < self.size_x - 1 {
            neighbors.push(self.xy_to_idx(x + 1, y));
        }
        if x > 0 {
            neighbors.push(self.xy_to_idx(x - 1, y));
        }
        if y < self.size_y - 1 {
            neighbors.push(self.xy_to_idx(x, y + 1));
        }
        neighbors
    }

    fn idx_to_xy(&self, i: usize) -> (usize, usize) {
        (i % self.size_x, i / self.size_x)
    }

    fn xy_to_idx(&self, x: usize, y: usize) -> usize {
        x + y * self.size_x
    }

    fn distance(&self, from: usize, to: usize) -> usize {
        let diff = if from >= to { from - to } else { to - from };
        (diff % self.size_x) + (diff / self.size_x)
    }

    fn find_path(&self, from: usize, to: usize) -> Option<Vec<usize>> {
        //println!("  pathfinding {} to {}", from, to);
        let mut path: Vec<usize> = Vec::new();
        path.push(from);
        if from == to {
            // trivial case
            return Some(path);
        }

        let mut best_path: Option<Vec<usize>> = None;
        let mut to_visit: Vec<Vec<usize>> = Vec::new();
        to_visit.push(self.in_range(from));

        'outer: while !path.is_empty() {
            let d = path.len() - 1;
            // if d is greater than our best path, we can't do better, so backtrack
            while d < best_path.as_ref().map_or(usize::max_value(), |p| p.len())
                && !to_visit[d].is_empty()
            {
                let next = to_visit[d].pop().unwrap();
                if next == to {
                    if d < best_path.as_ref().map_or(usize::max_value(), |p| p.len()) {
                        let mut temp_path = path.clone();
                        temp_path.push(next);
                        best_path = Some(temp_path);
                    }
                } else if !path.contains(&next) && self.grid[next].is_open() {
                    path.push(next);
                    to_visit.push(self.in_range(next));
                    continue 'outer;
                }
            }
            to_visit.pop();
            path.pop();
        }
        best_path
    }

    fn round(&mut self) {
        let piece_locs = self.piece_locations();
        for piece_loc in piece_locs {
            let piece = self.grid[piece_loc];
            //println!("working on a {:?} at {}", piece, piece_loc);
            let target_locs = match piece {
                Piece::Elf(_) => self.goblin_locations(),
                Piece::Goblin(_) => self.elf_locations(),
                _ => unreachable!(),
            };
            //println!("  targets at {:?}", target_locs);
            if target_locs.is_empty() {
                break; // if no targets remain, combat is over
            }
            let in_range_locs: Vec<usize> = target_locs
                .iter()
                .map(|&t| self.in_range(t))
                .flatten()
                .filter(|&loc| loc == piece_loc || self.grid[loc].is_open())
                .collect();
            //println!("  want to move to {:?}", in_range_locs);
            if !in_range_locs.is_empty() && !in_range_locs.contains(&piece_loc) {
                // move if it isn't already in range of a piece
                let paths: Vec<Option<Vec<usize>>> = in_range_locs
                    .iter()
                    .map(|dest| self.find_path(piece_loc, *dest))
                    .collect();
                let path_to_closest = paths
                    .iter()
                    .min_by_key(|path| path.as_ref().map_or(usize::max_value(), |p| p.len()))
                    .unwrap();
                //println!("  following path {:?}", path_to_closest);
                if path_to_closest.is_some() {
                    let p = path_to_closest.as_ref().unwrap();
                    assert!(p.len() >= 2);
                    self.grid.swap(p[0], p[1]); // always moving to open space, and leaving behind open space
                                                //println!("  moving from {} to {}", p[0], p[1]);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinates() {
        let board = Board {
            grid: vec![Piece::Open; 35],
            size_x: 7,
            size_y: 5,
        };
        struct TestCase {
            xy: (usize, usize),
            idx: usize,
        }

        let cases = vec![
            TestCase { xy: (0, 0), idx: 0 },
            TestCase { xy: (0, 1), idx: 7 },
            TestCase { xy: (1, 0), idx: 1 },
            TestCase {
                xy: (2, 3),
                idx: 23,
            },
            TestCase {
                xy: (6, 4),
                idx: 34,
            },
        ];

        for case in cases {
            assert_eq!(board.idx_to_xy(case.idx), case.xy);
            assert_eq!(board.xy_to_idx(case.xy.0, case.xy.1), case.idx);
        }
    }

    #[test]
    fn test_distance() {
        let board = Board {
            grid: vec![Piece::Open; 35],
            size_x: 7,
            size_y: 5,
        };
        struct TestCase {
            xy0: (usize, usize),
            xy1: (usize, usize),
            d: usize,
        }

        let cases = vec![
            TestCase {
                xy0: (0, 0),
                xy1: (6, 4),
                d: 10,
            },
            TestCase {
                xy0: (1, 1),
                xy1: (3, 3),
                d: 4,
            },
            TestCase {
                xy0: (2, 1),
                xy1: (2, 4),
                d: 3,
            },
            TestCase {
                xy0: (0, 3),
                xy1: (4, 3),
                d: 4,
            },
        ];

        for case in cases {
            let (x0, y0) = case.xy0;
            let loc0 = board.xy_to_idx(x0, y0);
            let (x1, y1) = case.xy1;
            let loc1 = board.xy_to_idx(x1, y1);
            assert_eq!(case.d, board.distance(loc0, loc1));
            assert_eq!(case.d, board.distance(loc1, loc0));
        }
    }

    #[test]
    fn test_move() {
        let mut board = read_board(
            "#########\n\
             #G..G..G#\n\
             #.......#\n\
             #.......#\n\
             #G..E..G#\n\
             #.......#\n\
             #.......#\n\
             #G..G..G#\n\
             #########\n",
        );
        let expecteds = vec![
            read_board(
                "#########\n\
                 #.G...G.#\n\
                 #...G...#\n\
                 #...E..G#\n\
                 #.G.....#\n\
                 #.......#\n\
                 #G..G..G#\n\
                 #.......#\n\
                 #########\n",
            ),
            read_board(
                "#########\n\
                 #..G.G..#\n\
                 #...G...#\n\
                 #.G.E.G.#\n\
                 #.......#\n\
                 #G..G..G#\n\
                 #.......#\n\
                 #.......#\n\
                 #########\n",
            ),
            read_board(
                "#########\n\
                 #.......#\n\
                 #..GGG..#\n\
                 #..GEG..#\n\
                 #G..G...#\n\
                 #......G#\n\
                 #.......#\n\
                 #.......#\n\
                 #########\n",
            ),
        ];
        for expected in expecteds {
            board.round();
            assert_eq!(expected, board);
        }
    }
    #[test]
    fn test_pathfinding() {
        let board = read_board(
            "#######\n\
             #.....#\n\
             #.E#G.#\n\
             #.....#\n\
             #..#..#\n\
             ###.###\n",
        );
        assert_eq!(Some(vec![16, 9, 10, 11, 18]), board.find_path(16, 18));
        assert_eq!(Some(vec![16, 9]), board.find_path(16, 9));
        assert_eq!(None, board.find_path(24, 38));
        assert_eq!(Some(vec![33]), board.find_path(33, 33));
    }
}
