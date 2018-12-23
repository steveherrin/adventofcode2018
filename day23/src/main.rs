#[macro_use]
extern crate lazy_static;
extern crate regex;

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

fn strongest_range(bots: &[Bot]) -> usize {
    let strongest = bots.iter().max_by_key(|b| b.r).expect("No bots");
    bots.iter()
        .filter(|&b| strongest.distance(b) <= strongest.r)
        .count()
}

impl Bot {
    fn distance(&self, other: &Bot) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
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
}
