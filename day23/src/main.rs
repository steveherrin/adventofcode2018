#[macro_use]
extern crate lazy_static;
extern crate priority_queue;
extern crate regex;

use priority_queue::PriorityQueue;
use std::cmp::max;
use std::cmp::min;
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

fn strongest_range(bots: &[Bot]) -> usize {
    let strongest = bots.iter().max_by_key(|b| b.r).expect("No bots");
    bots.iter()
        .filter(|&b| strongest.distance(b) <= strongest.r)
        .count()
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Volume {
    x_lo: i64,
    x_hi: i64,
    y_lo: i64,
    y_hi: i64,
    z_lo: i64,
    z_hi: i64,
}

impl Volume {
    fn new(x_lo: i64, x_hi: i64, y_lo: i64, y_hi: i64, z_lo: i64, z_hi: i64) -> Volume {
        Volume {
            x_lo,
            x_hi,
            y_lo,
            y_hi,
            z_lo,
            z_hi,
        }
    }
    fn new_zero() -> Volume {
        Volume::new(0, 0, 0, 0, 0, 0)
    }
    fn containing_bots(bots: &[Bot]) -> Volume {
        let mut vol = Volume::new_zero();
        for bot in bots {
            vol.x_lo = min(vol.x_lo, bot.x - bot.r);
            vol.y_lo = min(vol.y_lo, bot.y - bot.r);
            vol.z_lo = min(vol.z_lo, bot.z - bot.r);
            vol.x_hi = max(vol.x_hi, bot.x + bot.r);
            vol.y_hi = max(vol.y_hi, bot.y + bot.r);
            vol.z_hi = max(vol.z_hi, bot.z + bot.r);
        }
        vol
    }
    fn n_in_range(&self, bots: &[Bot]) -> usize {
        bots.iter()
            .filter(|b| b.x - b.r <= self.x_hi && self.x_lo <= b.x + b.r)
            .filter(|b| b.y - b.r <= self.y_hi && self.y_lo <= b.y + b.r)
            .filter(|b| b.z - b.r <= self.z_hi && self.z_lo <= b.z + b.r)
            .count()
    }
    fn is_point(&self) -> bool {
        self.x_lo == self.x_hi && self.y_lo == self.y_hi && self.z_lo == self.z_hi
    }
    fn min_corner_distance(&self) -> i64 {
        min(self.x_lo.abs(), self.x_hi.abs())
            + min(self.y_lo.abs(), self.y_hi.abs())
            + min(self.z_lo.abs(), self.z_hi.abs())
    }
    fn octants(&self) -> Vec<Volume> {
        let x_mid = (self.x_lo + self.x_hi) / 2;
        let y_mid = (self.y_lo + self.y_hi) / 2;
        let z_mid = (self.z_lo + self.z_hi) / 2;
        let mut octants = vec![
            Volume::new(self.x_lo, x_mid, self.y_lo, y_mid, self.z_lo, z_mid),
            Volume::new(self.x_lo, x_mid, self.y_lo, y_mid, z_mid + 1, self.z_hi),
            Volume::new(self.x_lo, x_mid, y_mid + 1, self.y_hi, self.z_lo, z_mid),
            Volume::new(self.x_lo, x_mid, y_mid + 1, self.y_hi, z_mid + 1, self.z_hi),
            Volume::new(x_mid + 1, self.x_hi, self.y_lo, y_mid, self.z_lo, z_mid),
            Volume::new(x_mid + 1, self.x_hi, self.y_lo, y_mid, z_mid + 1, self.z_hi),
            Volume::new(x_mid + 1, self.x_hi, y_mid + 1, self.y_hi, self.z_lo, z_mid),
            Volume::new(
                x_mid + 1,
                self.x_hi,
                y_mid + 1,
                self.y_hi,
                z_mid + 1,
                self.z_hi,
            ),
        ];
        // by adding 1 to ensure non-overlapping octants,
        // we might have created some invalid ones; strip them out
        octants.retain(|v| v.x_lo <= v.x_hi && v.y_lo <= v.y_hi && v.z_lo <= v.z_hi);
        //println!("before: {:?}", octants);
        // don't duplicate itself
        octants.retain(|v| {
            v.x_lo != self.x_lo
                || v.x_hi != self.x_hi
                || v.y_lo != self.y_lo
                || v.y_hi != self.y_hi
                || v.z_lo != self.z_lo
                || v.z_hi != self.z_hi
        });
        //println!("after: {:?}", octants);
        octants
    }
}

fn n_in_range(x: i64, y: i64, z: i64, bots: &[Bot]) -> usize {
    bots.iter()
        .filter(|b| b.distance_xyz(x, y, z) <= b.r)
        .count()
}

fn best_spot(bots: &[Bot]) -> i64 {
    let mut queue: PriorityQueue<Volume, usize> = PriorityQueue::new();
    let full_volume = Volume::containing_bots(bots);
    let full_n = full_volume.n_in_range(bots);
    queue.push(full_volume, full_n);
    let mut i = 0;
    let mut best_n = 0;
    let mut best: Vec<Volume> = Vec::new();
    while let Some((vol, n)) = queue.pop() {
        if i % 100000 == 0 {
            println!("queue {}", queue.len());
            //println!("{} {:?}", n, vol);
        }
        i += 1;
        for oct in vol.octants() {
            let oct_n = oct.n_in_range(bots);
            if oct.is_point() && oct_n > best_n {
                best_n = oct_n;
            }
            //println!("  {} {:?}", oct_n, oct);
            if oct_n >= best_n {
                queue.push(oct, oct_n);
            }
        }
        if n >= best_n && vol.is_point() {
            best.push(vol);
        }
    }
    best.sort_by_key(|v| {
        (
            n_in_range(v.x_lo, v.y_lo, v.z_lo, bots),
            -v.min_corner_distance(),
        )
    });
    best.last().expect("No best spot").min_corner_distance()
}

/*
fn best_spot(bots: &[Bot]) -> i64 {
    let pois = find_search_points(bots);
    println!("{} points", pois.n_points());
    let (n, d) = best_point(bots, &pois);

    println!("best was {} at {}", n, d);
    d
}
*/

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
}
