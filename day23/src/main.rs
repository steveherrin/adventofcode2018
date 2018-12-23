#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let task = &args[1];
    let filename = &args[2];

    let bots: Vec<Bot> = fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<Bot>().unwrap())
        .collect();

    if task == "strongest" {
        println!("{}", strongest_range(&bots));
    } else if task == "best" {
        println!("{}", best_spot(&bots));
    } else {
        panic!("Don't know how to '{}'", task);
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Bot {
    x: i64,
    y: i64,
    z: i64,
    r: i64,
}

#[derive(Debug)]
enum ParseBotError {
    Regex,
    Number(ParseIntError),
}

impl From<ParseIntError> for ParseBotError {
    fn from(err: ParseIntError) -> ParseBotError {
        ParseBotError::Number(err)
    }
}

impl FromStr for Bot {
    type Err = ParseBotError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"pos=<\s*(?P<x>-?\d+),\s*(?P<y>-?\d+),\s*(?P<z>-?\d+)>, r=(?P<r>-?\d+)"
            )
            .unwrap();
        }
        match RE.captures(s) {
            Some(parts) => {
                let x = parts.name("x").map_or("", |m| m.as_str()).parse::<i64>()?;
                let y = parts.name("y").map_or("", |m| m.as_str()).parse::<i64>()?;
                let z = parts.name("z").map_or("", |m| m.as_str()).parse::<i64>()?;
                let r = parts.name("r").map_or("", |m| m.as_str()).parse::<i64>()?;
                Ok(Bot { x, y, z, r })
            }
            None => Err(ParseBotError::Regex),
        }
    }
}

impl Bot {
    fn distance(&self, other: &Bot) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
    fn distance_xyz(&self, x: i64, y: i64, z: i64) -> i64 {
        (self.x - x).abs() + (self.y - y).abs() + (self.z - z).abs()
    }
}

/// Given a list of points and ranges, compute the interval(s) where they overlap most
fn find_max_overlaps(pos_range: &[(i64, i64)]) -> Vec<(i64, i64)> {
    // first work how how many ranges start and end at each position
    let mut n_start: HashMap<i64, i64> = HashMap::new();
    let mut n_end: HashMap<i64, i64> = HashMap::new();
    for (p, r) in pos_range {
        let ds = n_start.entry(p - r).or_insert(0);
        *ds += 1;
        let de = n_end.entry(p + r).or_insert(0);
        *de -= 1;
    }

    // merge those together, sort so we look at starts before ends
    let mut deltas = n_start.drain().collect::<Vec<(i64, i64)>>();
    deltas.append(&mut n_end.drain().collect::<Vec<(i64, i64)>>());
    deltas.sort_by_key(|(x, d)| (*x, -*d));

    // first pass through and figure out the largest number we have in an overlap
    let mut n_max_overlap = 0;
    let mut csum = 0;
    for (_x, delta) in &deltas {
        csum += delta;
        n_max_overlap = max(csum, n_max_overlap);
    }

    // then find all ranges that have that amount of overlap
    let mut max_overlaps: Vec<(i64, i64)> = Vec::new();
    let mut csum = 0;
    let mut start: Option<i64> = None;
    for (x, delta) in &deltas {
        csum += delta;
        if csum == n_max_overlap {
            if let None = start {
                start = Some(*x);
            }
        } else {
            if let Some(s) = start {
                max_overlaps.push((s, *x));
            }
            start = None;
        }
    }
    max_overlaps
}

fn strongest_range(bots: &[Bot]) -> usize {
    let strongest = bots.iter().max_by_key(|b| b.r).expect("No bots");
    bots.iter()
        .filter(|&b| strongest.distance(b) <= strongest.r)
        .count()
}

#[derive(Debug)]
struct Volume {
    x_lo: i64,
    x_hi: i64,
    y_lo: i64,
    y_hi: i64,
    z_lo: i64,
    z_hi: i64,
}

impl Volume {
    fn n_points(&self) -> i64 {
        (self.x_hi - self.x_lo + 1) * (self.y_hi - self.y_lo + 1) * (self.z_hi - self.z_lo + 1)
    }
}

#[derive(Debug)]
struct PointsOfInterest {
    xs: Vec<i64>,
    ys: Vec<i64>,
    zs: Vec<i64>,
}

impl PointsOfInterest {
    fn new() -> PointsOfInterest {
        PointsOfInterest {
            xs: Vec::new(),
            ys: Vec::new(),
            zs: Vec::new(),
        }
    }
    fn n_points(&self) -> usize {
        self.xs.len() * self.ys.len() * self.zs.len()
    }
}

fn find_search_points(bots: &[Bot]) -> PointsOfInterest {
    let xs = bots.iter().map(|b| (b.x, b.r)).collect::<Vec<(i64, i64)>>();
    let x_overlaps = find_max_overlaps(&xs);
    let ys = bots.iter().map(|b| (b.y, b.r)).collect::<Vec<(i64, i64)>>();
    let y_overlaps = find_max_overlaps(&ys);
    let zs = bots.iter().map(|b| (b.z, b.r)).collect::<Vec<(i64, i64)>>();
    let z_overlaps = find_max_overlaps(&zs);

    println!("{:?}", x_overlaps);
    println!("{:?}", y_overlaps);
    println!("{:?}", z_overlaps);

    let mut pois = PointsOfInterest::new();
    for (x, r) in &xs {
        if x_overlaps
            .iter()
            .any(|(x_lo, x_hi)| *x_lo <= x - r && x - r <= *x_hi)
        {
            pois.xs.push(*x - *r);
        }
        if x_overlaps
            .iter()
            .any(|(x_lo, x_hi)| *x_lo <= x + r && x + r <= *x_hi)
        {
            pois.xs.push(*x + *r);
        }
    }
    for (y, r) in &ys {
        if y_overlaps
            .iter()
            .any(|(y_lo, y_hi)| *y_lo <= y - r && y - r <= *y_hi)
        {
            pois.ys.push(*y - *r);
        }
        if y_overlaps
            .iter()
            .any(|(y_lo, y_hi)| *y_lo <= y + r && y + r <= *y_hi)
        {
            pois.ys.push(*y + *r);
        }
    }
    for (z, r) in &zs {
        if z_overlaps
            .iter()
            .any(|(z_lo, z_hi)| *z_lo <= z - r && z - r <= *z_hi)
        {
            pois.zs.push(*z - *r);
        }
        if z_overlaps
            .iter()
            .any(|(z_lo, z_hi)| *z_lo <= z + r && z + r <= *z_hi)
        {
            pois.zs.push(*z + *r);
        }
    }

    println!("{:?}", pois);
    pois
}

fn n_in_range(x: i64, y: i64, z: i64, bots: &[Bot]) -> usize {
    bots.iter()
        .filter(|b| b.distance_xyz(x, y, z) <= b.r)
        .count()
}

fn best_in_volume(bots: &[Bot], volume: &Volume) -> (usize, i64) {
    let mut best_n = 0;
    let mut best_d = i64::max_value();
    for x in volume.x_lo..=volume.x_hi {
        for y in volume.y_lo..=volume.y_hi {
            for z in volume.z_lo..=volume.z_hi {
                let n = n_in_range(x, y, z, bots);
                //println!("    ({}, {}, {}) {}", x, y, z, n);
                if n > best_n {
                    best_n = n;
                    best_d = x.abs() + y.abs() + z.abs();
                } else if n == best_n {
                    let d = x.abs() + y.abs() + z.abs();
                    best_d = min(d, best_d); // if a tie, pick the one nearest the origin
                }
            }
        }
    }
    (best_n, best_d)
}

fn best_point(bots: &[Bot], pois: &PointsOfInterest) -> (usize, i64) {
    let mut best_n = 0;
    let mut best_d = i64::max_value();
    for x in &pois.xs {
        for y in &pois.ys {
            for z in &pois.zs {
                let n = n_in_range(*x, *y, *z, bots);
                //println!("    ({}, {}, {}) {}", x, y, z, n);
                if n > best_n {
                    best_n = n;
                    best_d = x.abs() + y.abs() + z.abs();
                } else if n == best_n {
                    let d = x.abs() + y.abs() + z.abs();
                    best_d = min(d, best_d); // if a tie, pick the one nearest the origin
                }
            }
        }
    }
    (best_n, best_d)
}

fn best_spot(bots: &[Bot]) -> i64 {
    let pois = find_search_points(bots);
    println!("{} points", pois.n_points());
    let (n, d) = best_point(bots, &pois);

    println!("best was {} at {}", n, d);
    d
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strongest() {
        let bots = vec![
            "pos=<0,0,0>, r=4".parse::<Bot>().unwrap(),
            "pos=<1,0,0>, r=1".parse::<Bot>().unwrap(),
            "pos=<4,0,0>, r=3".parse::<Bot>().unwrap(),
            "pos=<0,2,0>, r=1".parse::<Bot>().unwrap(),
            "pos=<0,5,0>, r=3".parse::<Bot>().unwrap(),
            "pos=<0,0,3>, r=1".parse::<Bot>().unwrap(),
            "pos=<1,1,1>, r=1".parse::<Bot>().unwrap(),
            "pos=<1,1,2>, r=1".parse::<Bot>().unwrap(),
            "pos=<1,3,1>, r=1".parse::<Bot>().unwrap(),
        ];
        assert_eq!(7, strongest_range(&bots));
    }

    #[test]
    fn test_best_spot() {
        let bots = vec![
            "pos=<10,12,12>, r=2".parse::<Bot>().unwrap(),
            "pos=<12,14,12>, r=2".parse::<Bot>().unwrap(),
            "pos=<16,12,12>, r=4".parse::<Bot>().unwrap(),
            "pos=<14,14,14>, r=6".parse::<Bot>().unwrap(),
            "pos=<50,50,50>, r=200".parse::<Bot>().unwrap(),
            "pos=<10,10,10>, r=5".parse::<Bot>().unwrap(),
        ];
        assert_eq!(36, best_spot(&bots));
    }

    #[test]
    fn test_max_overlap() {
        let pos_ranges = vec![
            (10, 2),
            (12, 2),
            (16, 4),
            (14, 6),
            (50, 200),
            (10, 5),
            (17, 2),
            (18, 1),
            (19, 1),
        ];
        let overlaps = find_max_overlaps(&pos_ranges);
        assert_eq!(vec![(12, 12), (18, 19)], overlaps);
    }
}
