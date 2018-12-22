#![allow(unused_doc_comments)]
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fmt;

fn main() {
    let args: Vec<String> = env::args().collect();
    let task = &args[1];

    let depth = args[2].parse::<u64>().expect("Bad depth");
    let target_x = args[3].parse::<usize>().expect("Bad target x");
    let target_y = args[4].parse::<usize>().expect("Bad target y");

    if task == "risk" {
        println!("{}", risk_level(depth, target_x, target_y));
    } else if task == "draw" {
        let size_x = target_x + 6;
        let size_y = target_y + 6;
        let cave = Cave::new(depth, target_x, target_y, size_x, size_y);
        println!("{}", cave);
    } else if task == "path" {
        let size_x = target_x + 20;
        let size_y = target_y + 20;
        let cave = Cave::new(depth, target_x, target_y, size_x, size_y);
        println!("{}", min_time(&cave));
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

/// Compute the total risk of the rectangle with corners and 0,0 and the target
fn risk_level(depth: u64, target_x: usize, target_y: usize) -> u64 {
    let cave = Cave::new(depth, target_x, target_y, target_x + 1, target_y + 1);
    cave.caves
        .iter()
        .map(|ct| match ct {
            CaveType::Rocky => 0,
            CaveType::Wet => 1,
            CaveType::Narrow => 2,
        })
        .sum()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum CaveType {
    Rocky,
    Wet,
    Narrow,
}

#[derive(Debug)]
struct Cave {
    caves: Vec<CaveType>,
    size_x: usize,
    size_y: usize,
    target_x: usize,
    target_y: usize,
}

impl fmt::Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.size_y {
            for x in 0..self.size_x {
                let c = match self.caves[x + y * self.size_x] {
                    CaveType::Rocky => ".",
                    CaveType::Wet => "=",
                    CaveType::Narrow => "|",
                };
                write!(f, "{}", c)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl Cave {
    fn new(depth: u64, target_x: usize, target_y: usize, size_x: usize, size_y: usize) -> Cave {
        let mut geo_idx: Vec<u64> = vec![0; size_x * size_y];
        for y in 0..size_y {
            for x in 0..size_x {
                let idx = x + y * size_x;
                if x == 0 || y == 0 {
                    geo_idx[idx] = geo_index(x, y);
                } else if x == target_x && y == target_y {
                    geo_idx[idx] = 0;
                } else {
                    let left_erosion = geo_idx[(x - 1) + y * size_x] + depth;
                    let above_erosion = geo_idx[x + (y - 1) * size_x] + depth;
                    geo_idx[idx] = (left_erosion * above_erosion) % 20183;
                }
            }
        }
        let caves = geo_idx
            .iter()
            .map(|gi| match ((gi + depth) % 20183) % 3 {
                0 => CaveType::Rocky,
                1 => CaveType::Wet,
                2 => CaveType::Narrow,
                _ => unreachable!(),
            })
            .collect::<Vec<CaveType>>();
        Cave {
            caves,
            size_x,
            size_y,
            target_x,
            target_y,
        }
    }

    fn at(&self, x: usize, y: usize) -> CaveType {
        let idx = x + y * self.size_x;
        self.caves[idx]
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Tool {
    Neither,
    Torch,
    Gear,
}

fn valid_tool(tool: Tool, cave: CaveType) -> bool {
    match (cave, tool) {
        (CaveType::Rocky, Tool::Torch) => true,
        (CaveType::Rocky, Tool::Gear) => true,
        (CaveType::Wet, Tool::Neither) => true,
        (CaveType::Wet, Tool::Gear) => true,
        (CaveType::Narrow, Tool::Neither) => true,
        (CaveType::Narrow, Tool::Torch) => true,
        (_, _) => false,
    }
}

/// Find the minimum time to get through the cave
/// This is basically the assembly line dynamic programming problem
fn min_time(cave: &Cave) -> u32 {
    let mut min_time: HashMap<(usize, usize, Tool), u32> = HashMap::new();
    min_time.insert((cave.target_x, cave.target_y, Tool::Torch), 0);
    let mut queue: VecDeque<(usize, usize, Tool)> = VecDeque::new();
    queue.push_back((cave.target_x, cave.target_y, Tool::Torch));
    while let Some((x, y, tool)) = queue.pop_front() {
        // we should've already made an entry for it
        let t = *min_time
            .get(&(x, y, tool))
            .expect("Visiting an unexpected space");

        // For every space we can move to, or every tool we can switch to
        // see how time time to move/switch compares to the best time
        // we've already found to get there.
        // If the time is better, then revisit, because it's now faster
        // to visit its neighbors, and so on.
        if x > 0 && valid_tool(tool, cave.at(x - 1, y)) {
            let other_t = min_time.entry((x - 1, y, tool)).or_insert(u32::max_value());
            if t + 1 < *other_t {
                *other_t = t + 1;
                queue.push_back((x - 1, y, tool));
            }
        }
        if x + 1 < cave.size_x && valid_tool(tool, cave.at(x + 1, y)) {
            let other_t = min_time.entry((x + 1, y, tool)).or_insert(u32::max_value());
            if t + 1 < *other_t {
                *other_t = t + 1;
                queue.push_back((x + 1, y, tool));
            }
        }
        if y > 0 && valid_tool(tool, cave.at(x, y - 1)) {
            let other_t = min_time.entry((x, y - 1, tool)).or_insert(u32::max_value());
            if t + 1 < *other_t {
                *other_t = t + 1;
                queue.push_back((x, y - 1, tool));
            }
        }
        if y + 1 < cave.size_y && valid_tool(tool, cave.at(x, y + 1)) {
            let other_t = min_time.entry((x, y + 1, tool)).or_insert(u32::max_value());
            if t + 1 < *other_t {
                *other_t = t + 1;
                queue.push_back((x, y + 1, tool));
            }
        }
        for new_tool in [Tool::Neither, Tool::Torch, Tool::Gear].iter() {
            if valid_tool(*new_tool, cave.at(x, y)) {
                let other_t = min_time
                    .entry((x, y, *new_tool))
                    .or_insert(u32::max_value());
                if t + 7 < *other_t {
                    *other_t = t + 7;
                    queue.push_back((x, y, *new_tool));
                }
            }
        }
    }
    *min_time
        .get(&(0, 0, Tool::Torch))
        .expect("Couldn't find a path")
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

    #[test]
    fn test_min_time() {
        let depth = 510;
        let target_x = 10;
        let target_y = 10;
        let size_x = target_x + 6;
        let size_y = target_y + 6;

        let cave = Cave::new(depth, target_x, target_y, size_x, size_y);

        assert_eq!(45, min_time(&cave));
    }
}
