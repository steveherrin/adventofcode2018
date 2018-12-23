#[macro_use]
extern crate lazy_static;
extern crate regex;

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
        let best = best_spot(&bots);
        let d = best.0.abs() + best.1.abs() + best.2.abs();
        println!("best at ({}, {}, {})", best.0, best.1, best.2);
        println!("{}", d);
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

/// Find the number of bots in range of the stongest bot
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
        if bots.len() == 0 {
            Volume::new_zero()
        } else {
            let bot = &bots[0];
            let mut vol = Volume {
                x_lo: bot.x,
                x_hi: bot.x,
                y_lo: bot.y,
                y_hi: bot.y,
                z_lo: bot.z,
                z_hi: bot.z,
            };
            for bot in bots.iter().skip(1) {
                vol.x_lo = min(vol.x_lo, bot.x);
                vol.y_lo = min(vol.y_lo, bot.y);
                vol.z_lo = min(vol.z_lo, bot.z);
                vol.x_hi = max(vol.x_hi, bot.x);
                vol.y_hi = max(vol.y_hi, bot.y);
                vol.z_hi = max(vol.z_hi, bot.z);
            }
            vol
        }
    }
}

/// Number of bots in range of (x, y, z)
fn n_in_range(x: i64, y: i64, z: i64, bots: &[Bot]) -> usize {
    bots.iter()
        .filter(|b| b.distance_xyz(x, y, z) <= b.r)
        .count()
}

/// Find the best spot to stand amid the nanobots
/// I don't think this is a general solution; it relies on "most"
/// of the bots covering a similar volume, so that you can
/// binary search by looking at the number of points in range
/// of a given point.
fn best_spot(bots: &[Bot]) -> (i64, i64, i64) {
    // the maximum spot should be somewhere inside the bot volume
    let mut vol = Volume::containing_bots(bots);

    // figure out a step size that covers the whole volume for our search
    let mut range: i64 = 1;
    while range < vol.x_hi - vol.x_lo || range < vol.y_hi - vol.y_lo || range < vol.z_hi - vol.z_lo
    {
        range *= 2;
    }
    range /= 2; // don't skip over the full volume on the first step

    // keep track of our best so far
    let mut best_n = 0;
    let mut best_x = 0;
    let mut best_y = 0;
    let mut best_z = 0;
    loop {
        for x in (vol.x_lo..=vol.x_hi).step_by(range as usize) {
            for y in (vol.y_lo..=vol.y_hi).step_by(range as usize) {
                for z in (vol.z_lo..=vol.z_hi).step_by(range as usize) {
                    let n = n_in_range(x, y, z, bots);
                    if n >= best_n {
                        best_n = n;
                        best_x = x;
                        best_y = y;
                        best_z = z;
                    } else if n == best_n
                        && x.abs() + y.abs() + z.abs() < best_x.abs() + best_y.abs() + best_z.abs()
                    {
                        // in case of a tie, pick the one closest to the origin
                        best_x = x;
                        best_y = z;
                        best_z = z;
                    }
                }
            }
        }
        if range == 1 {
            // once we've gone through and found the best single point, we're done
            break;
        }
        // focus on the volume around the best point we've seen
        vol.x_lo = best_x - range;
        vol.x_hi = best_x + range;
        vol.y_lo = best_y - range;
        vol.y_hi = best_y + range;
        vol.z_lo = best_z - range;
        vol.z_hi = best_z + range;
        range /= 2;
    }
    (best_x, best_y, best_z)
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
        assert_eq!((12, 12, 12), best_spot(&bots));
    }
}
