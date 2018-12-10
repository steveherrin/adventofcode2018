#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::VecDeque;
use std::env;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let task = &args[1];
    let filename = &args[2];

    let particles: Vec<Particle> = fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<Particle>().unwrap())
        .collect();
    if task == "message" {
        let sky = Sky::new(particles);

        let mut t = 0;
        let mut best_t = 0;
        let mut best_size = sky.y_size(0);
        let mut last_sizes: VecDeque<i64> = VecDeque::new();
        loop {
            // Assume that the message is when the points are
            // most compact vertically, and that they will
            // diverge after that
            t += 1;
            let size = sky.y_size(t);
            if size < best_size {
                best_size = size;
                best_t = t;
            }

            // Look at the last few time slices to make
            // sure we're really diverging and not just
            // at a local minimum.
            last_sizes.push_back(size);
            while last_sizes.len() > 3 {
                last_sizes.pop_front();
            }
            if last_sizes.iter().all(|sz| sz > &best_size) {
                break;
            }
        }
        let message = sky.to_str(best_t);
        println!("{}", message)
    } else {
        panic!("Don't know how to '{}'", task);
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Particle {
    x: i64,
    y: i64,
    v_x: i64,
    v_y: i64,
}

impl Particle {
    fn at(&self, t: i64) -> Particle {
        Particle {
            x: self.x + t * self.v_x,
            y: self.y + t * self.v_y,
            v_x: self.v_x,
            v_y: self.v_y,
        }
    }
}

#[derive(Debug)]
enum ParseParticleError {
    Regex,
    Number(ParseIntError),
}

impl From<ParseIntError> for ParseParticleError {
    fn from(err: ParseIntError) -> ParseParticleError {
        ParseParticleError::Number(err)
    }
}

impl FromStr for Particle {
    type Err = ParseParticleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"position=<\s*(?P<x>-?\d+),\s*(?P<y>-?\d+)> velocity=<\s*(?P<v_x>-?\d+),\s*(?P<v_y>-?\d+)>").unwrap();
        }
        match RE.captures(s) {
            Some(parts) => {
                let x = parts.name("x").map_or("", |m| m.as_str()).parse::<i64>()?;
                let y = parts.name("y").map_or("", |m| m.as_str()).parse::<i64>()?;
                let v_x = parts
                    .name("v_x")
                    .map_or("", |m| m.as_str())
                    .parse::<i64>()?;
                let v_y = parts
                    .name("v_y")
                    .map_or("", |m| m.as_str())
                    .parse::<i64>()?;
                Ok(Particle { x, y, v_x, v_y })
            }
            None => Err(ParseParticleError::Regex),
        }
    }
}

struct Sky {
    particles: Vec<Particle>,
}

impl Sky {
        fn new(particles: Vec<Particle>) -> Self {
        Self { particles }
    }
    fn x_min(&self, t: i64) -> i64 {
        self.particles.iter().map(|p| p.x + t * p.v_x).min().unwrap_or(0)
    }
    fn x_max(&self, t: i64) -> i64 {
        self.particles.iter().map(|p| p.x + t * p.v_x).max().unwrap_or(0)
    }
    fn y_min(&self, t: i64) -> i64 {
        self.particles.iter().map(|p| p.y + t * p.v_y).min().unwrap_or(0)
    }
    fn y_max(&self, t: i64) -> i64 {
        self.particles.iter().map(|p| p.y + t * p.v_y).max().unwrap_or(0)
    }
    fn x_size(&self, t: i64) -> i64 {
        self.x_max(t) - self.x_min(t) + 1
    }
    fn y_size(&self, t: i64) -> i64 {
        self.y_max(t) - self.y_min(t) + 1
    }
    fn to_str(&self, t: i64) -> String {
        let x_size = self.x_size(t);
        let y_size = self.y_size(t);
        let offset = self.x_min(t) + x_size * self.y_min(t);
        let mut chars: Vec<char> = vec!['.'; (x_size * y_size) as usize];
        for particle in self.particles.iter().map(|p| p.at(t)) {
            let idx = (-offset + particle.x + (particle.y * x_size)) as usize;
            chars[idx] = '#';
        }
        let lines = (0..y_size as usize)
            .map(|i| chars.iter()
                 .skip(i * x_size as usize)
                 .take(x_size as usize)
                 .chain(['\n'].iter())
                 .collect::<String>())
            .collect::<Vec<String>>()
            .join("");
        lines
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        struct TestCase {
            input: String,
            output: Particle,
        }

        let cases: Vec<TestCase> = vec![
            TestCase {
                input: String::from("position=< 9,  1> velocity=< 0,  2>"),
                output: Particle {
                    x: 9,
                    y: 1,
                    v_x: 0,
                    v_y: 2,
                },
            },
            TestCase {
                input: String::from("position=<-3, -6> velocity=<-1, -1>"),
                output: Particle {
                    x: -3,
                    y: -6,
                    v_x: -1,
                    v_y: -1,
                },
            },
            TestCase {
                input: String::from("position=< 42772, -21149> velocity=<-4,  2>"),
                output: Particle {
                    x: 42772,
                    y: -21149,
                    v_x: -4,
                    v_y: 2,
                },
            },
        ];

        for ref case in &cases[..] {
            assert_eq!(case.output, case.input.parse::<Particle>().unwrap());
        }
    }

    #[test]
    fn test_sky() {
        struct TestCase {
            particles: Vec<Particle>,
            t: i64,
            output: String,
        }

        let cases: Vec<TestCase> = vec![
            TestCase {
                particles: vec![
                    Particle { x: 9, y: 1, v_x: 0, v_y: 2 },
                ],
                t: 1,
                output: String::from("#\n"),
            },
            TestCase {
                particles: vec![
                    Particle { x: 1, y: 0, v_x: 0, v_y: 1 },
                    Particle { x: -1, y: 0, v_x: 0, v_y: -1 },
                ],
                t: 1,
                output: String::from("#..\n...\n..#\n"),
            },
            TestCase {
                particles: vec![
                    Particle { x: 9, y: 1, v_x: 0, v_y: 2 },
                    Particle { x: 7, y: 0, v_x: -1, v_y: 0 },
                    Particle { x: 3, y: -2, v_x: -1, v_y: 1 },
                    Particle { x: 6, y: 10, v_x: -2, v_y: -1 },
                    Particle { x: 2, y: -4, v_x: 2, v_y: 2 },
                    Particle { x: -6, y: 10, v_x: 2, v_y: -2 },
                    Particle { x: 1, y: 8, v_x: 1, v_y: -1 },
                    Particle { x: 1, y: 7, v_x: 1, v_y: 0 },
                    Particle { x: -3, y: 11, v_x: 1, v_y: -2 },
                    Particle { x: 7, y: 6, v_x: -1, v_y: -1 },
                    Particle { x: -2, y: 3, v_x: 1, v_y: 0 },
                    Particle { x: -4, y: 3, v_x: 2, v_y: 0 },
                    Particle { x: 10, y: -3, v_x: -1, v_y: 1 },
                    Particle { x: 5, y: 11, v_x: 1, v_y: -2 },
                    Particle { x: 4, y: 7, v_x: 0, v_y: -1 },
                    Particle { x: 8, y: -2, v_x: 0, v_y: 1 },
                    Particle { x: 15, y: 0, v_x: -2, v_y: 0 },
                    Particle { x: 1, y: 6, v_x: 1, v_y: 0 },
                    Particle { x: 8, y: 9, v_x: 0, v_y: -1 },
                    Particle { x: 3, y: 3, v_x: -1, v_y: 1 },
                    Particle { x: 0, y: 5, v_x: 0, v_y: -1 },
                    Particle { x: -2, y: 2, v_x: 2, v_y: 0 },
                    Particle { x: 5, y: -2, v_x: 1, v_y: 2 },
                    Particle { x: 1, y: 4, v_x: 2, v_y: 1 },
                    Particle { x: -2, y: 7, v_x: 2, v_y: -2 },
                    Particle { x: 3, y: 6, v_x: -1, v_y: -1 },
                    Particle { x: 5, y: 0, v_x: 1, v_y: 0 },
                    Particle { x: -6, y: 0, v_x: 2, v_y: 0 },
                    Particle { x: 5, y: 9, v_x: 1, v_y: -2 },
                    Particle { x: 14, y: 7, v_x: -2, v_y: 0 },
                    Particle { x: -3, y: 6, v_x: 2, v_y: -1 },
                ],
                t: 3,
                output: String::from("#...#..###\n\
                                      #...#...#.\n\
                                      #...#...#.\n\
                                      #####...#.\n\
                                      #...#...#.\n\
                                      #...#...#.\n\
                                      #...#...#.\n\
                                      #...#..###\n"),
            },
        ];

        for case in cases {
            let sky = Sky::new(case.particles);
            assert_eq!(case.output, sky.to_str(case.t));
        }
    }
}
