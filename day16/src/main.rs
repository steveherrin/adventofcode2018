use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::num::ParseIntError;

fn main() {
    let args: Vec<String> = env::args().collect();
    let task = &args[1];
    let filename = &args[2];

    if task == "behaveslike" {
        let observations = read_behavior_input(filename).unwrap_or(vec![]);
        let mut n_gte_3 = 0;
        let n_tot = observations.len();
        for obs in observations {
            let n_behaves_like = behaves_like(&obs.instruction, &obs.before, &obs.after).len();
            if n_behaves_like >= 3 {
                n_gte_3 += 1;
            }
        }
        println!("processed {}; {} behave like 3 or more", n_tot, n_gte_3);
    } else if task == "opcodes" {
        let observations = read_behavior_input(filename).unwrap_or(vec![]);
        let pairs = determine_opcodes(&observations);
        for pair in pairs {
            println!("{}: {}", pair.0, pair.1);
        }
    } else {
        panic!("Don't know how to '{}'", task);
    }
}

#[derive(Debug)]
struct Observation {
    instruction: [usize; 4],
    before: [usize; 4],
    after: [usize; 4],
}

impl Observation {
    fn new(instruction: &[usize], before: &[usize], after: &[usize]) -> Observation {
        Observation {
            instruction: [
                instruction[0],
                instruction[1],
                instruction[2],
                instruction[3],
            ],
            before: [before[0], before[1], before[2], before[3]],
            after: [after[0], after[1], after[2], after[3]],
        }
    }
}

#[derive(Debug)]
enum ReadError {
    IO(std::io::Error),
    Number(ParseIntError),
    MissingNumbers,
}

impl From<std::io::Error> for ReadError {
    fn from(err: std::io::Error) -> ReadError {
        ReadError::IO(err)
    }
}

impl From<ParseIntError> for ReadError {
    fn from(err: ParseIntError) -> ReadError {
        ReadError::Number(err)
    }
}

fn extract_numbers(line: &str) -> Result<Vec<usize>, std::num::ParseIntError> {
    let s = line.trim();
    let lo = s.find('[').map(|i| i + 1).unwrap_or(0);
    let hi = s.find(']').unwrap_or(s.len());
    s[lo..hi]
        .split(' ')
        .map(|c| c.trim_matches(',').trim().parse::<usize>())
        .collect::<Result<Vec<_>, _>>()
}

fn read_behavior_input(filename: &str) -> Result<Vec<Observation>, ReadError> {
    let f = File::open(filename)?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut observations: Vec<Observation> = Vec::new();

    loop {
        line.truncate(0);
        reader.read_line(&mut line)?;
        if !line.starts_with("Before:") {
            break;
        } // have hit the end of test cases
        let before = extract_numbers(&line)?;
        line.truncate(0);
        reader.read_line(&mut line)?;
        let instructions = extract_numbers(&line)?;
        line.truncate(0);
        reader.read_line(&mut line)?;
        let after = extract_numbers(&line)?;
        line.truncate(0);
        reader.read_line(&mut line)?;
        if instructions.len() != 4 || before.len() != 4 || after.len() != 4 {
            return Err(ReadError::MissingNumbers);
        }
        observations.push(Observation::new(&instructions, &before, &after));
    }
    Ok(observations)
}

#[derive(Debug)]
struct CPU {
    registers: [usize; 4],
}

impl CPU {
    fn new(r0: usize, r1: usize, r2: usize, r3: usize) -> CPU {
        CPU {
            registers: [r0, r1, r2, r3],
        }
    }
    fn process(&mut self, opcode: usize, a: usize, b: usize, c: usize) {
        match opcode {
            0 => self.addr(a, b, c),
            1 => self.addi(a, b, c),
            2 => self.mulr(a, b, c),
            3 => self.muli(a, b, c),
            4 => self.banr(a, b, c),
            5 => self.bani(a, b, c),
            6 => self.borr(a, b, c),
            7 => self.bori(a, b, c),
            8 => self.setr(a, b, c),
            9 => self.seti(a, b, c),
            10 => self.gtir(a, b, c),
            11 => self.gtri(a, b, c),
            12 => self.gtrr(a, b, c),
            13 => self.eqir(a, b, c),
            14 => self.eqri(a, b, c),
            15 => self.eqrr(a, b, c),
            _ => unreachable!(),
        }
    }
    fn addr(&mut self, a: usize, b: usize, c: usize) {
        self.registers[c] = self.registers[a] + self.registers[b];
    }
    fn addi(&mut self, a: usize, b: usize, c: usize) {
        self.registers[c] = self.registers[a] + b;
    }
    fn mulr(&mut self, a: usize, b: usize, c: usize) {
        self.registers[c] = self.registers[a] * self.registers[b];
    }
    fn muli(&mut self, a: usize, b: usize, c: usize) {
        self.registers[c] = self.registers[a] * b;
    }
    fn banr(&mut self, a: usize, b: usize, c: usize) {
        self.registers[c] = self.registers[a] & self.registers[b];
    }
    fn bani(&mut self, a: usize, b: usize, c: usize) {
        self.registers[c] = self.registers[a] & b;
    }
    fn borr(&mut self, a: usize, b: usize, c: usize) {
        self.registers[c] = self.registers[a] | self.registers[b];
    }
    fn bori(&mut self, a: usize, b: usize, c: usize) {
        self.registers[c] = self.registers[a] | b;
    }
    fn setr(&mut self, a: usize, _b: usize, c: usize) {
        self.registers[c] = self.registers[a];
    }
    fn seti(&mut self, a: usize, _b: usize, c: usize) {
        self.registers[c] = a;
    }
    fn gtir(&mut self, a: usize, b: usize, c: usize) {
        self.registers[c] = if a > self.registers[b] { 1 } else { 0 }
    }
    fn gtri(&mut self, a: usize, b: usize, c: usize) {
        self.registers[c] = if self.registers[a] > b { 1 } else { 0 }
    }
    fn gtrr(&mut self, a: usize, b: usize, c: usize) {
        self.registers[c] = if self.registers[a] > self.registers[b] {
            1
        } else {
            0
        }
    }
    fn eqir(&mut self, a: usize, b: usize, c: usize) {
        self.registers[c] = if a == self.registers[b] { 1 } else { 0 }
    }
    fn eqri(&mut self, a: usize, b: usize, c: usize) {
        self.registers[c] = if self.registers[a] == b { 1 } else { 0 }
    }
    fn eqrr(&mut self, a: usize, b: usize, c: usize) {
        self.registers[c] = if self.registers[a] == self.registers[b] {
            1
        } else {
            0
        }
    }
}

fn behaves_like(instruction: &[usize], before: &[usize], after: &[usize]) -> HashSet<usize> {
    let mut like: HashSet<usize> = HashSet::new();
    for opcode in 0..16 {
        let mut cpu = CPU::new(before[0], before[1], before[2], before[3]);
        cpu.process(opcode, instruction[1], instruction[2], instruction[3]);
        if cpu.registers == [after[0], after[1], after[2], after[3]] {
            like.insert(opcode);
        }
    }
    like
}

fn determine_opcodes(observations: &[Observation]) -> Vec<(usize, usize)> {
    let mut allowed: Vec<HashSet<usize>> = vec![HashSet::new(); 16];
    for their_opcode in 0..16 {
        for my_opcode in 0..16 {
            allowed[their_opcode].insert(my_opcode);
        }
    }
    for obs in observations {
        let like = behaves_like(&obs.instruction, &obs.before, &obs.after);
        let their_opcode = obs.instruction[0];
        let new_allowed = allowed[their_opcode].intersection(&like).cloned().collect();
        allowed[their_opcode] = new_allowed;
    }
    let mut pairs: Vec<(usize, usize)> = Vec::new();
    let mut did_work = true;
    while did_work {
        // go through, find ones with only one possibility,
        // record it, make sure no others can use that one,
        // and repeat until we've worked everything out
        did_work = false;
        for their_opcode in 0..16 {
            let n_possible = allowed[their_opcode].len();
            if n_possible == 1 {
                let my_opcode = allowed[their_opcode].iter().cloned().next().unwrap();
                pairs.push((their_opcode, my_opcode));
                for other_opcode in 0..16 {
                    did_work |= allowed[other_opcode].remove(&my_opcode);
                }
            }
        }
    }
    pairs.sort();
    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addr() {
        let mut cpu = CPU::new(1, 7, 0, 0);
        cpu.addr(0, 1, 2);
        assert_eq!([1, 7, 8, 0], cpu.registers);
    }
    #[test]
    fn test_addi() {
        let mut cpu = CPU::new(1, 7, 0, 0);
        cpu.addi(0, 1, 2);
        assert_eq!([1, 7, 2, 0], cpu.registers);
    }
    #[test]
    fn test_mulr() {
        let mut cpu = CPU::new(2, 7, 0, 0);
        cpu.mulr(0, 1, 2);
        assert_eq!([2, 7, 14, 0], cpu.registers);
    }
    #[test]
    fn test_muli() {
        let mut cpu = CPU::new(2, 7, 0, 0);
        cpu.muli(0, 3, 3);
        assert_eq!([2, 7, 0, 6], cpu.registers);
    }
    #[test]
    fn test_banr() {
        let mut cpu = CPU::new(2, 7, 0, 0);
        cpu.banr(0, 1, 2);
        assert_eq!([2, 7, 2, 0], cpu.registers);
    }
    #[test]
    fn test_bani() {
        let mut cpu = CPU::new(2, 7, 0, 0);
        cpu.bani(0, 10, 3);
        assert_eq!([2, 7, 0, 2], cpu.registers);
    }
    #[test]
    fn test_borr() {
        let mut cpu = CPU::new(2, 7, 0, 0);
        cpu.borr(0, 1, 2);
        assert_eq!([2, 7, 7, 0], cpu.registers);
    }
    #[test]
    fn test_bori() {
        let mut cpu = CPU::new(2, 7, 0, 0);
        cpu.bori(0, 1, 3);
        assert_eq!([2, 7, 0, 3], cpu.registers);
    }
    #[test]
    fn test_setr() {
        let mut cpu = CPU::new(2, 7, 0, 0);
        cpu.setr(0, 8, 1);
        assert_eq!([2, 2, 0, 0], cpu.registers);
    }
    #[test]
    fn test_seti() {
        let mut cpu = CPU::new(2, 7, 0, 0);
        cpu.seti(3, 8, 2);
        assert_eq!([2, 7, 3, 0], cpu.registers);
    }
    #[test]
    fn test_gtir() {
        let mut cpu = CPU::new(2, 7, 0, 1);
        cpu.gtir(3, 0, 2);
        assert_eq!([2, 7, 1, 1], cpu.registers);
        cpu.gtir(1, 0, 3);
        assert_eq!([2, 7, 1, 0], cpu.registers);
    }
    #[test]
    fn test_gtri() {
        let mut cpu = CPU::new(2, 7, 0, 1);
        cpu.gtri(1, 6, 2);
        assert_eq!([2, 7, 1, 1], cpu.registers);
        cpu.gtri(1, 9, 3);
        assert_eq!([2, 7, 1, 0], cpu.registers);
    }
    #[test]
    fn test_gtrr() {
        let mut cpu = CPU::new(2, 7, 0, 1);
        cpu.gtrr(1, 0, 2);
        assert_eq!([2, 7, 1, 1], cpu.registers);
        cpu.gtrr(0, 1, 3);
        assert_eq!([2, 7, 1, 0], cpu.registers);
    }
    #[test]
    fn test_eqir() {
        let mut cpu = CPU::new(2, 7, 0, 1);
        cpu.eqir(7, 1, 2);
        assert_eq!([2, 7, 1, 1], cpu.registers);
        cpu.eqir(3, 0, 3);
        assert_eq!([2, 7, 1, 0], cpu.registers);
    }
    #[test]
    fn test_eqri() {
        let mut cpu = CPU::new(2, 7, 0, 1);
        cpu.eqri(0, 2, 2);
        assert_eq!([2, 7, 1, 1], cpu.registers);
        cpu.eqri(1, 2, 3);
        assert_eq!([2, 7, 1, 0], cpu.registers);
    }
    #[test]
    fn test_eqrr() {
        let mut cpu = CPU::new(2, 2, 0, 1);
        cpu.eqrr(0, 1, 2);
        assert_eq!([2, 2, 1, 1], cpu.registers);
        cpu.eqrr(0, 2, 3);
        assert_eq!([2, 2, 1, 0], cpu.registers);
    }
    #[test]
    fn test_behaves_like() {
        let instruction = [9, 2, 1, 2];
        let before = [3, 2, 1, 1];
        let after = [3, 2, 2, 1];
        assert_eq!(3, behaves_like(&instruction, &before, &after).len());
    }
    #[test]
    fn test_extract_numbers() {
        assert_eq!(vec![1, 2, 3, 4], extract_numbers("1 2 3 4").unwrap());
        assert_eq!(vec![1, 2, 3, 4], extract_numbers("1 2 3 4\n").unwrap());
        assert_eq!(
            vec![0, 1, 0, 1],
            extract_numbers("Before: [0, 1, 0, 1]").unwrap()
        );
        assert_eq!(
            vec![0, 1, 0, 1],
            extract_numbers("Before: [0, 1, 0, 1]\n").unwrap()
        );
        assert_eq!(
            vec![1, 0, 1, 0],
            extract_numbers("After: [1, 0, 1, 0]").unwrap()
        );
        assert_eq!(
            vec![1, 0, 1, 0],
            extract_numbers("After: [1, 0, 1, 0]\n").unwrap()
        );
    }
}
