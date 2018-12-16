#![allow(unused_doc_comments)]

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

    if task == "combat" {
        let (n_rounds, hp_left) = board.combat();
        println!("Combat over after t={}", n_rounds);
        println!("HP left: {}", hp_left);
        println!(
            "{} * {} = {}",
            n_rounds,
            hp_left,
            n_rounds * (hp_left as u32)
        );
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
    fn is_creature(self) -> bool {
        match self {
            Piece::Elf(_) => true,
            Piece::Goblin(_) => true,
            _ => false,
        }
    }
    fn is_enemy(self, other: Piece) -> bool {
        match (self, other) {
            (Piece::Elf(_), Piece::Goblin(_)) => true,
            (Piece::Goblin(_), Piece::Elf(_)) => true,
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
    /// Read in a board from a string
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

enum MoveResult {
    AlreadyDead,
    MovedTo(usize),
    Stuck(usize),
    NoTargets,
}

impl Board {
    fn piece_locations(&self) -> Vec<usize> {
        /// locations of creature pieces
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
        /// get all the neighbors "in range" of the location
        /// respects the max board dimensions, and will
        /// return the neighbors in "reading" order
        let (x, y) = self.idx_to_xy(loc);
        let mut neighbors: Vec<usize> = Vec::with_capacity(4);
        // the order results in the final vector being reading order
        if y > 0 {
            neighbors.push(self.xy_to_idx(x, y - 1));
        }
        if x > 0 {
            neighbors.push(self.xy_to_idx(x - 1, y));
        }
        if x < self.size_x - 1 {
            neighbors.push(self.xy_to_idx(x + 1, y));
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
        let (x0, y0) = self.idx_to_xy(from);
        let (x1, y1) = self.idx_to_xy(to);
        let dx = if x0 > x1 { x0 - x1 } else { x1 - x0 };
        let dy = if y0 > y1 { y0 - y1 } else { y1 - y0 };
        dx + dy
    }

    fn find_path(&self, from: usize, to: usize) -> Option<Vec<usize>> {
        /// Find a path from one location to another
        /// Returns an Option, with None reflecting no path available
        /// Path returned will include both the start and end ocations
        let mut path: Vec<usize> = Vec::new();
        path.push(from);
        if from == to {
            // trivial case
            return Some(path);
        }

        let mut min_distances: Vec<usize> = vec![usize::max_value(); self.grid.len()];
        min_distances[to] = 0;
        let mut search_stack: Vec<(usize, usize)> = vec![(to, 0)];
        while !search_stack.is_empty() {
            let (loc, d) = search_stack.pop().unwrap();
            for neighbor in self.in_range(loc) {
                if (d + 1) < min_distances[neighbor] && self.grid[neighbor].is_open() {
                    min_distances[neighbor] = d + 1;
                    search_stack.push((neighbor, d + 1));
                }
            }
        }

        let mut probe_loc = from;
        while probe_loc != to {
            let neighbors = self.in_range(probe_loc);
            let next_step = neighbors.iter().min_by_key(|&l| min_distances[*l]);
            if next_step.is_some()
                && min_distances[**next_step.as_ref().unwrap()] < usize::max_value()
            {
                probe_loc = *next_step.unwrap();
                path.push(probe_loc);
            } else {
                return None;
            }
        }
        Some(path)
    }

    fn move_piece(&mut self, piece_loc: usize) -> MoveResult {
        /// Move the piece to where its AI wants to go.
        /// which is along the shortest path that gets it
        /// in range of a target
        /// Returns an option
        let piece = self.grid[piece_loc];
        if !piece.is_creature() {
            return MoveResult::AlreadyDead;
        }
        let target_locs = match piece {
            Piece::Elf(_) => self.goblin_locations(),
            Piece::Goblin(_) => self.elf_locations(),
            _ => unreachable!(),
        };
        if target_locs.is_empty() {
            return MoveResult::NoTargets;
        }
        let in_range_locs: Vec<usize> = target_locs
            .iter()
            .map(|&t| self.in_range(t))
            .flatten()
            .filter(|&loc| loc == piece_loc || self.grid[loc].is_open())
            .collect();
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
            if path_to_closest.is_some() {
                let p = path_to_closest.as_ref().unwrap();
                assert!(p.len() >= 2);
                self.grid.swap(p[0], p[1]); // always moving to open space, and leaving behind open space
                return MoveResult::MovedTo(p[1]);
            }
        }
        return MoveResult::Stuck(piece_loc); // it couldn't find a path
    }

    fn attack_with(&mut self, piece_loc: usize) {
        /// Have the creature at the location attack
        /// It will pick the weakest enemy and attack it
        let piece = self.grid[piece_loc];
        if !piece.is_creature() {
            return; // it already died, probably
        }
        let neighbors = self.in_range(piece_loc);
        let maybe_weakest = neighbors
            .iter()
            .filter(|&neighbor| piece.is_enemy(self.grid[*neighbor]))
            .min_by_key(|&neighbor| match self.grid[*neighbor] {
                Piece::Goblin(hp) => hp,
                Piece::Elf(hp) => hp,
                _ => unreachable!(),
            });
        if maybe_weakest.is_some() {
            let weakest_loc = maybe_weakest.unwrap();
            let weakest_piece = self.grid[*weakest_loc];
            self.grid[*weakest_loc] = match weakest_piece {
                Piece::Goblin(hp) => {
                    let new_hp = hp - ATTACK_POWER;
                    if new_hp <= 0 {
                        Piece::Open
                    } else {
                        Piece::Goblin(new_hp)
                    }
                }
                Piece::Elf(hp) => {
                    let new_hp = hp - ATTACK_POWER;
                    if new_hp <= 0 {
                        Piece::Open
                    } else {
                        Piece::Elf(new_hp)
                    }
                }
                _ => unreachable!(),
            };
        }
    }

    fn round(&mut self) -> bool {
        /// Run a round of combat
        /// Return indicates whether or not the turn fully completed
        let piece_locs = self.piece_locations();
        for piece_loc in piece_locs {
            let new_loc = self.move_piece(piece_loc);
            match new_loc {
                MoveResult::Stuck(l) => self.attack_with(l),
                MoveResult::MovedTo(l) => self.attack_with(l),
                MoveResult::AlreadyDead => (),
                MoveResult::NoTargets => {
                    return false;
                } // end immediately if no targets
            }
        }
        true
    }

    fn combat_over(&self) -> bool {
        self.elf_locations().is_empty() || self.goblin_locations().is_empty()
    }

    fn sum_hp_left(&self) -> i32 {
        self.grid
            .iter()
            .map(|p| match p {
                Piece::Elf(hp) => *hp,
                Piece::Goblin(hp) => *hp,
                _ => 0,
            }).sum()
    }

    fn combat(&mut self) -> (u32, i32) {
        let mut t = 0;
        while !self.combat_over() {
            let completed = self.round();
            if completed {
                t += 1;
            }
        }
        let hp_left = self.sum_hp_left();

        (t, hp_left)
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
            TestCase {
                xy0: (2, 1),
                xy1: (1, 2),
                d: 2,
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
            let piece_locs = board.piece_locations();
            for piece_loc in piece_locs {
                board.move_piece(piece_loc);
            }
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

    #[test]
    fn test_attack() {
        let mut board = Board {
            grid: vec![
                Piece::Goblin(9),
                Piece::Open,
                Piece::Open,
                Piece::Open,
                Piece::Open,
                Piece::Open,
                Piece::Open,
                Piece::Goblin(4),
                Piece::Open,
                Piece::Open,
                Piece::Open,
                Piece::Open,
                Piece::Elf(2),
                Piece::Goblin(2),
                Piece::Open,
                Piece::Open,
                Piece::Open,
                Piece::Goblin(2),
                Piece::Open,
                Piece::Open,
                Piece::Open,
                Piece::Open,
                Piece::Open,
                Piece::Goblin(1),
                Piece::Open,
            ],
            size_x: 5,
            size_y: 5,
        };
        board.attack_with(12);
        assert!(board.grid[13].is_open());
        assert_eq!(Piece::Elf(2), board.grid[12]);
    }

    #[test]
    fn test_combat() {
        struct TestCase {
            board: Board,
            n_rounds: u32,
            hp_left: i32,
        }

        let cases = vec![
            TestCase {
                board: read_board(
                    "#######\n\
                     #.G...#\n\
                     #...EG#\n\
                     #.#.#G#\n\
                     #..G#E#\n\
                     #.....#\n\
                     #######\n",
                ),
                n_rounds: 47,
                hp_left: 590,
            },
            TestCase {
                board: read_board(
                    "#######\n\
                     #G..#E#\n\
                     #E#E.E#\n\
                     #G.##.#\n\
                     #...#E#\n\
                     #...E.#\n\
                     #######\n",
                ),
                n_rounds: 37,
                hp_left: 982,
            },
            TestCase {
                board: read_board(
                    "#######\n\
                     #E..EG#\n\
                     #.#G.E#\n\
                     #E.##E#\n\
                     #G..#.#\n\
                     #..E#.#\n\
                     #######\n",
                ),
                n_rounds: 46,
                hp_left: 859,
            },
            TestCase {
                board: read_board(
                    "#######\n\
                     #E.G#.#\n\
                     #.#G..#\n\
                     #G.#.G#\n\
                     #G..#.#\n\
                     #...E.#\n\
                     #######\n",
                ),
                n_rounds: 35,
                hp_left: 793,
            },
            TestCase {
                board: read_board(
                    "#######\n\
                     #.E...#\n\
                     #.#..G#\n\
                     #.###.#\n\
                     #E#G#G#\n\
                     #...#G#\n\
                     #######\n",
                ),
                n_rounds: 54,
                hp_left: 536,
            },
            TestCase {
                board: read_board(
                    "#########\n\
                     #G......#\n\
                     #.E.#...#\n\
                     #..##..G#\n\
                     #...##..#\n\
                     #...#...#\n\
                     #.G...G.#\n\
                     #.....G.#\n\
                     #########\n",
                ),
                n_rounds: 20,
                hp_left: 937,
            },
        ];

        for mut case in cases {
            assert_eq!((case.n_rounds, case.hp_left), case.board.combat())
        }
    }
}
