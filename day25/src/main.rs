use std::collections::HashMap;
use std::env;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();
    let task = &args[1];
    let filename = &args[2];

    let points = fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.parse::<Point>())
        .collect::<Result<Vec<Point>, _>>()
        .expect("Couldn't read input");

    if task == "constellations" {
        println!("{}", n_constellations(&points));
    } else {
        panic!("Don't know how to '{}'", task);
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
    t: i64,
}

impl Point {
    #[allow(dead_code)]
    fn new(x: i64, y: i64, z: i64, t: i64) -> Point {
        Point { x, y, z, t }
    }
    fn distance(&self, other: &Point) -> i64 {
        (self.x - other.x).abs()
            + (self.y - other.y).abs()
            + (self.z - other.z).abs()
            + (self.t - other.t).abs()
    }
}

impl FromStr for Point {
    type Err = ParseIntError;

    #[allow(clippy::many_single_char_names)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split(',');
        let x = parts.next().unwrap_or("").parse::<i64>()?;
        let y = parts.next().unwrap_or("").parse::<i64>()?;
        let z = parts.next().unwrap_or("").parse::<i64>()?;
        let t = parts.next().unwrap_or("").parse::<i64>()?;
        Ok(Point { x, y, z, t })
    }
}

fn point_not_in_any_constellation<'a>(
    points: &'a [Point],
    constellations: &HashMap<&Point, i64>,
) -> Option<&'a Point> {
    points.iter().find(|p| !constellations.contains_key(p))
}

const CONSTELLATION_DISTANCE: i64 = 3;

fn n_constellations(points: &[Point]) -> i64 {
    let mut constellations: HashMap<&Point, i64> = HashMap::new();
    let mut n_constellations = 0;

    // Repeatedly pick a point that has no constellation and assign it one,
    // then find all its neighbors within range and assign them the same
    // constellation, and repeat with their neighbors
    while let Some(first_point) = point_not_in_any_constellation(points, &constellations) {
        let mut stack = vec![first_point];
        while let Some(point) = stack.pop() {
            constellations.insert(point, n_constellations);

            for other in points {
                if point.distance(other) <= CONSTELLATION_DISTANCE
                    && !constellations.contains_key(other)
                {
                    stack.push(other);
                }
            }
        }
        n_constellations += 1
    }
    n_constellations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        struct TestCase {
            input: Vec<Point>,
            expected: i64,
        }

        let cases = vec![
            TestCase {
                input: vec![
                    Point::new(0, 0, 0, 0),
                    Point::new(3, 0, 0, 0),
                    Point::new(0, 3, 0, 0),
                    Point::new(0, 0, 3, 0),
                    Point::new(0, 0, 0, 3),
                    Point::new(0, 0, 0, 6),
                    Point::new(9, 0, 0, 0),
                    Point::new(12, 0, 0, 0),
                ],
                expected: 2,
            },
            TestCase {
                input: vec![
                    Point::new(-1, 2, 2, 0),
                    Point::new(0, 0, 2, -2),
                    Point::new(0, 0, 0, -2),
                    Point::new(-1, 2, 0, 0),
                    Point::new(-2, -2, -2, 2),
                    Point::new(3, 0, 2, -1),
                    Point::new(-1, 3, 2, 2),
                    Point::new(-1, 0, -1, 0),
                    Point::new(0, 2, 1, -2),
                    Point::new(3, 0, 0, 0),
                ],
                expected: 4,
            },
            TestCase {
                input: vec![
                    Point::new(1, -1, 0, 1),
                    Point::new(2, 0, -1, 0),
                    Point::new(3, 2, -1, 0),
                    Point::new(0, 0, 3, 1),
                    Point::new(0, 0, -1, -1),
                    Point::new(2, 3, -2, 0),
                    Point::new(-2, 2, 0, 0),
                    Point::new(2, -2, 0, 0),
                    Point::new(2, -2, 0, -1),
                    Point::new(3, 2, 0, 2),
                ],
                expected: 3,
            },
            TestCase {
                input: vec![
                    Point::new(1, -1, -1, -2),
                    Point::new(-2, -2, 0, 1),
                    Point::new(0, 2, 1, 3),
                    Point::new(-2, 3, -2, 1),
                    Point::new(0, 2, 3, -2),
                    Point::new(-1, -1, 1, -2),
                    Point::new(0, -2, -1, 0),
                    Point::new(-2, 2, 3, -1),
                    Point::new(1, 2, 2, 0),
                    Point::new(-1, -2, 0, -2),
                ],
                expected: 8,
            },
        ];

        for case in cases {
            assert_eq!(case.expected, n_constellations(&case.input));
        }
    }
}
