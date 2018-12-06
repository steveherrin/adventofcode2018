#![allow(unused_doc_comments)]
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();
    let task = &args[1];
    let filename = &args[2];

    let points: Vec<Point> = fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .map(|s| s.parse::<Point>())
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap()) // I feel like there should be an idiom for this and the line above
        .collect();

    if task == "area" {
        println!("{}", max_interior_area(&points));
    } else if task == "proximal" {
        println!("{}", proximal_area(10000, &points));
    } else {
        panic!("Don't know how to '{}'", task);
    }
}

fn max_interior_area(points: &[Point]) -> i32 {
    /// compute the maximum nearest-neighbor area that isn't infinite
    let x_lo = points.iter().map(|p| p.x).min().unwrap();
    let x_hi = points.iter().map(|p| p.x).max().unwrap();

    let y_lo = points.iter().map(|p| p.y).min().unwrap();
    let y_hi = points.iter().map(|p| p.y).max().unwrap();

    let mut area_around: HashMap<Point, i32> = HashMap::new();
    let mut uncontained: HashSet<Point> = HashSet::new();

    // scan a grid that contains all points
    for y in y_lo..=y_hi {
        for x in x_lo..=x_hi {
            // find the nearest neighbors to the grid spot
            let p = Point::new(x, y);
            let mut nearest_neighbors: Vec<(i32, Point)> = points
                .iter()
                .map(|other| (p.distance(*other), *other))
                .collect();

            // throw away ties
            nearest_neighbors.sort_by_key(|(d, _other)| *d);
            let (distance, nn) = nearest_neighbors[0];
            let next_distance = nearest_neighbors[1].0;
            if distance != next_distance {
                let area = area_around.entry(nn).or_insert(0);
                *area += 1; // the grid point now adds to the area
            }

            // if a point is the nearest neighbor to an edge, it
            // will be a nearest neighbor out to infinity, so exclude it
            if y == y_lo || y == y_hi || x == x_lo || x == x_hi {
                uncontained.insert(nn);
            }
        }
    }
    // of all the regions, find the largest one that doesn't go to infinity
    area_around
        .iter()
        .filter(|(p, _area)| !uncontained.contains(p))
        .map(|(_p, area)| *area)
        .max()
        .unwrap()
}

fn proximal_area(max_distance: i32, points: &[Point]) -> i32 {
    /// compute the area of the region inside which the sum distance to all points is under a max
    let x_lo = points.iter().map(|p| p.x).min().unwrap();
    let x_hi = points.iter().map(|p| p.x).max().unwrap();

    let y_lo = points.iter().map(|p| p.y).min().unwrap();
    let y_hi = points.iter().map(|p| p.y).max().unwrap();

    // Since we're using Manattan distance, computing the distance
    // to all points and summing them at every point is not particularly
    // efficient.
    // Better would be to keep track of sorted x and y coordinates of the points.
    // If we move +1 x, then the distance to all points left increases by 1,
    // and the distance to all points right decreases by 1, and so the
    // total distance changes by the difference.
    // That said, this is simple and seems to run fast enough.

    let mut area: i32 = 0;
    for y in y_lo..=y_hi {
        for x in x_lo..=x_hi {
            let p = Point::new(x, y);
            let total_distance: i32 = points.iter().map(|other| p.distance(*other)).sum();
            if total_distance < max_distance {
                area += 1;
                if y == y_lo || y == y_hi || x == x_lo || x == x_hi {
                    panic!("The proximal region might extend past the bounding box of the points");
                }
            }
        }
    }
    area
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance(self, other: Point) -> i32 {
        (other.x - self.x).abs() + (other.y - self.y).abs()
    }

    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fields = s.split(',');
        let x = fields.next().unwrap_or("").trim().parse::<i32>()?;
        let y = fields.next().unwrap_or("").trim().parse::<i32>()?;
        Ok(Self { x, y })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        struct TestCase {
            input: String,
            output: Point,
        }

        let cases: Vec<TestCase> = vec![
            TestCase {
                input: String::from("123, 456"),
                output: Point::new(123, 456),
            },
            TestCase {
                input: String::from("1,4"),
                output: Point::new(1, 4),
            },
        ];

        for ref case in &cases[..] {
            assert_eq!(case.output, case.input.parse::<Point>().unwrap());
        }
    }

    #[test]
    fn test_distance() {
        struct TestCase {
            input: (Point, Point),
            output: i32,
        }

        let cases: Vec<TestCase> = vec![
            TestCase {
                input: (Point::new(0, 0), Point::new(0, 0)),
                output: 0,
            },
            TestCase {
                input: (Point::new(0, 0), Point::new(1, 1)),
                output: 2,
            },
            TestCase {
                input: (Point::new(0, 0), Point::new(1, -1)),
                output: 2,
            },
            TestCase {
                input: (Point::new(-3, 5), Point::new(9, 7)),
                output: 14,
            },
        ];

        for ref case in &cases[..] {
            assert_eq!(case.output, Point::distance(case.input.0, case.input.1));
        }
    }

    #[test]
    fn test_largest_area() {
        struct TestCase {
            input: Vec<Point>,
            output: i32,
        }

        let cases: Vec<TestCase> = vec![TestCase {
            input: vec![
                Point::new(1, 1),
                Point::new(1, 6),
                Point::new(8, 3),
                Point::new(3, 4),
                Point::new(5, 5),
                Point::new(8, 9),
            ],
            output: 17,
        }];

        for ref case in &cases[..] {
            assert_eq!(case.output, max_interior_area(&case.input));
        }
    }

    #[test]
    fn test_proximate_area() {
        struct TestCase {
            points: Vec<Point>,
            max_distance: i32,
            output: i32,
        }

        let cases: Vec<TestCase> = vec![TestCase {
            points: vec![
                Point::new(1, 1),
                Point::new(1, 6),
                Point::new(8, 3),
                Point::new(3, 4),
                Point::new(5, 5),
                Point::new(8, 9),
            ],
            max_distance: 32,
            output: 16,
        }];

        for ref case in &cases[..] {
            assert_eq!(case.output, proximal_area(case.max_distance, &case.points));
        }
    }
}
