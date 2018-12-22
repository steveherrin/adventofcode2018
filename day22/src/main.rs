#![allow(unused_doc_comments)]
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let task = &args[1];

    if task == "risk" {
        let depth = args[2].parse::<u64>().expect("Bad depth");
        let target_x = args[3].parse::<usize>().expect("Bad target x");
        let target_y = args[4].parse::<usize>().expect("Bad target y");
        println!("{}", risk_level(depth, target_x, target_y));
    } else {
        panic!("Don't know how to '{}'", task);
    }
}

fn geo_index(x: usize, y: usize) -> u64 {
    if x == 0 && y == 0 {
        0
    } else if y == 0 {
        (x as u64 * 16807) % 20183
    } else if x == 0 {
        (y as u64 * 48271) % 20183
    } else {
        panic!("Compute another way");
    }
}

fn risk_level(depth: u64, target_x: usize, target_y: usize) -> u64 {
    /// Compute the total risk of the rectangle with corners and 0,0 and the target
    let size_x = target_x + 1;
    let size_y = target_y + 1;
    let mut cave: Vec<u64> = vec![0; size_x * size_y];
    for y in 0..size_y {
        for x in 0..size_x {
            let idx = x + y * size_x;
            if x == 0 || y == 0 {
                cave[idx] = geo_index(x, y);
            } else if x == target_x && y == target_y {
                cave[idx] = 0;
            } else {
                let left_erosion = cave[(x - 1) + y * size_x] + depth;
                let above_erosion = cave[x + (y - 1) * size_x] + depth;
                // since we're summing these, use (a * b) % c ≡ (a % c) * (b % c)
                cave[idx] = (left_erosion * above_erosion) % 20183;
            }
        }
    }

    // making use of (a + b) % c ≡ (a % c) + (b % c)
    cave.iter()
        .fold(0, |sum, gi| sum + ((gi + depth) % 20183) % 3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_risk() {
        let depth = 510;
        let target_x = 10;
        let target_y = 10;

        assert_eq!(114, risk_level(depth, target_x, target_y));
    }
}
