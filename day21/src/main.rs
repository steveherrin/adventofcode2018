#![allow(unused_doc_comments)]
use std::env;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();
    let task = &args[1];
    let filename = &args[2];

    let (ipointer_idx, instructions) = read_program_input(filename).expect("Bad program");

    if task == "findhalt" {
        let mut cpu = CPU::new();
        let reg0_val = cpu.run_breakpoint(ipointer_idx, &instructions);
        println!("{}", reg0_val);
        cpu = CPU::new_state([reg0_val, 0, 0, 0, 0, 0]);
        println!("halted after {}", cpu.run(ipointer_idx, &instructions));
    } else {
        panic!("Don't know how to '{}'", task);
    }
}

#[derive(Debug)]
enum ReadError {
    IO(std::io::Error),
    BadInstruction(ParseInstructionError),
    BadInstructionPointer,
}

impl From<std::io::Error> for ReadError {
    fn from(err: std::io::Error) -> ReadError {
        ReadError::IO(err)
    }
}

impl From<ParseInstructionError> for ReadError {
    fn from(err: ParseInstructionError) -> ReadError {
        ReadError::BadInstruction(err)
    }
}

fn read_program_input(filename: &str) -> Result<(usize, Vec<Instruction>), ReadError> {
    /// Parse the input file, return the instruction pointer index and the instructions
    let f = File::open(filename)?;
    let reader = BufReader::new(f);
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut maybe_ipointer: Option<usize> = None; // Option so we can error if it's not provided at all

    for maybe_line in reader.lines() {
        let line = maybe_line?;
        if line.starts_with("#ip ") {
            maybe_ipointer = line[4..].parse::<usize>().ok();
        } else {
            let instruction = line.parse::<Instruction>()?;
            instructions.push(instruction);
        }
    }
    match maybe_ipointer {
        None => Err(ReadError::BadInstructionPointer),
        Some(ipointer) => Ok((ipointer, instructions)),
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Instruction {
    opcode: Opcode,
    i0: usize,
    i1: usize,
    o0: usize,
}

#[derive(Debug)]
enum ParseInstructionError {
    BadOpcode,
    MissingFields,
    Parse(ParseIntError),
}

impl From<ParseIntError> for ParseInstructionError {
    fn from(err: ParseIntError) -> ParseInstructionError {
        ParseInstructionError::Parse(err)
    }
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields: Vec<&str> = s.split_whitespace().collect();
        if fields.len() != 4 {
            return Err(ParseInstructionError::MissingFields);
        }
        let i0 = fields[1].parse::<usize>()?;
        let i1 = fields[2].parse::<usize>()?;
        let o0 = fields[3].parse::<usize>()?;
        let maybe_opcode = match fields[0] {
            "addr" => Some(Opcode::Addr),
            "addi" => Some(Opcode::Addi),
            "mulr" => Some(Opcode::Mulr),
            "muli" => Some(Opcode::Muli),
            "banr" => Some(Opcode::Banr),
            "bani" => Some(Opcode::Bani),
            "borr" => Some(Opcode::Borr),
            "bori" => Some(Opcode::Bori),
            "setr" => Some(Opcode::Setr),
            "seti" => Some(Opcode::Seti),
            "gtir" => Some(Opcode::Gtir),
            "gtri" => Some(Opcode::Gtri),
            "gtrr" => Some(Opcode::Gtrr),
            "eqir" => Some(Opcode::Eqir),
            "eqri" => Some(Opcode::Eqri),
            "eqrr" => Some(Opcode::Eqrr),
            _ => None,
        };
        match maybe_opcode {
            Some(opcode) => Ok(Instruction { opcode, i0, i1, o0 }),
            None => Err(ParseInstructionError::BadOpcode),
        }
    }
}

const N_REGISTERS: usize = 6;

#[derive(Debug, Clone)]
struct CPU {
    registers: [usize; N_REGISTERS],
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}, {}, {}, {}, {}, {}]",
            self.registers[0],
            self.registers[1],
            self.registers[2],
            self.registers[3],
            self.registers[4],
            self.registers[5]
        )?;
        Ok(())
    }
}

impl CPU {
    fn new() -> CPU {
        CPU {
            registers: [0; N_REGISTERS],
        }
    }
    fn new_state(registers: [usize; N_REGISTERS]) -> CPU {
        CPU { registers }
    }
    fn run(&mut self, ipointer_idx: usize, instructions: &[Instruction]) -> usize {
        let mut i = 0;
        loop {
            let inst_idx = self.registers[ipointer_idx];
            let instruction = instructions[inst_idx];
            self.process(&instruction);
            if self.registers[ipointer_idx] + 1 < instructions.len() {
                self.registers[ipointer_idx] += 1;
            } else {
                break;
            }
            i += 1;
        }
        i
    }
    /*
    fn run_and_inspect(&mut self, ipointer_idx: usize, instructions: &[Instruction]) {
        let mut last0 = 1;
        let mut last_state = self.clone();
        loop {
            last_state = self.clone();
            let inst_idx = self.registers[ipointer_idx];
            let instruction = instructions[inst_idx];
            self.process(&instruction);
            if self.registers[ipointer_idx] + 1 < instructions.len() {
                self.registers[ipointer_idx] += 1;
            } else {
                break;
            }
            if last0 != self.registers[0] {
                last0 = self.registers[0];
                println!("{} -> {}", last_state, self);
            }
        }
    }
    */

    fn run_breakpoint(&mut self, ipointer_idx: usize, instructions: &[Instruction]) -> usize {
        loop {
            let inst_idx = self.registers[ipointer_idx];
            let instruction = instructions[inst_idx];
            if instruction.opcode == Opcode::Eqrr {
                // by inspection, this is the only time we use register 0
                // it's an equality check against registers 4, so that's
                // an obvious candidate for when it halts
                return self.registers[4];
            }
            self.process(&instruction);
            if self.registers[ipointer_idx] + 1 >= instructions.len() {
                panic!("never used register 0");
            }
            self.registers[ipointer_idx] += 1;
        }
    }

    fn process(&mut self, instruction: &Instruction) {
        match instruction.opcode {
            Opcode::Addr => self.addr(instruction.i0, instruction.i1, instruction.o0),
            Opcode::Addi => self.addi(instruction.i0, instruction.i1, instruction.o0),
            Opcode::Mulr => self.mulr(instruction.i0, instruction.i1, instruction.o0),
            Opcode::Muli => self.muli(instruction.i0, instruction.i1, instruction.o0),
            Opcode::Banr => self.banr(instruction.i0, instruction.i1, instruction.o0),
            Opcode::Bani => self.bani(instruction.i0, instruction.i1, instruction.o0),
            Opcode::Borr => self.borr(instruction.i0, instruction.i1, instruction.o0),
            Opcode::Bori => self.bori(instruction.i0, instruction.i1, instruction.o0),
            Opcode::Setr => self.setr(instruction.i0, instruction.i1, instruction.o0),
            Opcode::Seti => self.seti(instruction.i0, instruction.i1, instruction.o0),
            Opcode::Gtir => self.gtir(instruction.i0, instruction.i1, instruction.o0),
            Opcode::Gtri => self.gtri(instruction.i0, instruction.i1, instruction.o0),
            Opcode::Gtrr => self.gtrr(instruction.i0, instruction.i1, instruction.o0),
            Opcode::Eqir => self.eqir(instruction.i0, instruction.i1, instruction.o0),
            Opcode::Eqri => self.eqri(instruction.i0, instruction.i1, instruction.o0),
            Opcode::Eqrr => self.eqrr(instruction.i0, instruction.i1, instruction.o0),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addr() {
        let mut cpu = CPU::new_state([1, 7, 0, 0, 0, 0]);
        cpu.addr(0, 1, 2);
        assert_eq!([1, 7, 8, 0, 0, 0], cpu.registers);
    }
    #[test]
    fn test_addi() {
        let mut cpu = CPU::new_state([1, 7, 0, 0, 0, 0]);
        cpu.addi(0, 1, 2);
        assert_eq!([1, 7, 2, 0, 0, 0], cpu.registers);
    }
    #[test]
    fn test_mulr() {
        let mut cpu = CPU::new_state([2, 7, 0, 0, 0, 0]);
        cpu.mulr(0, 1, 2);
        assert_eq!([2, 7, 14, 0, 0, 0], cpu.registers);
    }
    #[test]
    fn test_muli() {
        let mut cpu = CPU::new_state([2, 7, 0, 0, 0, 0]);
        cpu.muli(0, 3, 3);
        assert_eq!([2, 7, 0, 6, 0, 0], cpu.registers);
    }
    #[test]
    fn test_banr() {
        let mut cpu = CPU::new_state([2, 7, 0, 0, 0, 0]);
        cpu.banr(0, 1, 2);
        assert_eq!([2, 7, 2, 0, 0, 0], cpu.registers);
    }
    #[test]
    fn test_bani() {
        let mut cpu = CPU::new_state([2, 7, 0, 0, 0, 0]);
        cpu.bani(0, 10, 3);
        assert_eq!([2, 7, 0, 2, 0, 0], cpu.registers);
    }
    #[test]
    fn test_borr() {
        let mut cpu = CPU::new_state([2, 7, 0, 0, 0, 0]);
        cpu.borr(0, 1, 2);
        assert_eq!([2, 7, 7, 0, 0, 0], cpu.registers);
    }
    #[test]
    fn test_bori() {
        let mut cpu = CPU::new_state([2, 7, 0, 0, 0, 0]);
        cpu.bori(0, 1, 3);
        assert_eq!([2, 7, 0, 3, 0, 0], cpu.registers);
    }
    #[test]
    fn test_setr() {
        let mut cpu = CPU::new_state([2, 7, 0, 0, 0, 0]);
        cpu.setr(0, 8, 1);
        assert_eq!([2, 2, 0, 0, 0, 0], cpu.registers);
    }
    #[test]
    fn test_seti() {
        let mut cpu = CPU::new_state([2, 7, 0, 0, 0, 0]);
        cpu.seti(3, 8, 2);
        assert_eq!([2, 7, 3, 0, 0, 0], cpu.registers);
    }
    #[test]
    fn test_gtir() {
        let mut cpu = CPU::new_state([2, 7, 0, 1, 0, 0]);
        cpu.gtir(3, 0, 2);
        assert_eq!([2, 7, 1, 1, 0, 0], cpu.registers);
        cpu.gtir(1, 0, 3);
        assert_eq!([2, 7, 1, 0, 0, 0], cpu.registers);
    }
    #[test]
    fn test_gtri() {
        let mut cpu = CPU::new_state([2, 7, 0, 1, 0, 0]);
        cpu.gtri(1, 6, 2);
        assert_eq!([2, 7, 1, 1, 0, 0], cpu.registers);
        cpu.gtri(1, 9, 3);
        assert_eq!([2, 7, 1, 0, 0, 0], cpu.registers);
    }
    #[test]
    fn test_gtrr() {
        let mut cpu = CPU::new_state([2, 7, 0, 1, 0, 0]);
        cpu.gtrr(1, 0, 2);
        assert_eq!([2, 7, 1, 1, 0, 0], cpu.registers);
        cpu.gtrr(0, 1, 3);
        assert_eq!([2, 7, 1, 0, 0, 0], cpu.registers);
    }
    #[test]
    fn test_eqir() {
        let mut cpu = CPU::new_state([2, 7, 0, 1, 0, 0]);
        cpu.eqir(7, 1, 2);
        assert_eq!([2, 7, 1, 1, 0, 0], cpu.registers);
        cpu.eqir(3, 0, 3);
        assert_eq!([2, 7, 1, 0, 0, 0], cpu.registers);
    }
    #[test]
    fn test_eqri() {
        let mut cpu = CPU::new_state([2, 7, 0, 1, 0, 0]);
        cpu.eqri(0, 2, 2);
        assert_eq!([2, 7, 1, 1, 0, 0], cpu.registers);
        cpu.eqri(1, 2, 3);
        assert_eq!([2, 7, 1, 0, 0, 0], cpu.registers);
    }
    #[test]
    fn test_eqrr() {
        let mut cpu = CPU::new_state([2, 2, 0, 1, 0, 0]);
        cpu.eqrr(0, 1, 2);
        assert_eq!([2, 2, 1, 1, 0, 0], cpu.registers);
        cpu.eqrr(0, 2, 3);
        assert_eq!([2, 2, 1, 0, 0, 0], cpu.registers);
    }
    #[test]
    fn test_run() {
        let ipointer = 0usize;
        let instructions = vec![
            Instruction {
                opcode: Opcode::Seti,
                i0: 5,
                i1: 0,
                o0: 1,
            },
            Instruction {
                opcode: Opcode::Seti,
                i0: 6,
                i1: 0,
                o0: 2,
            },
            Instruction {
                opcode: Opcode::Addi,
                i0: 0,
                i1: 1,
                o0: 0,
            },
            Instruction {
                opcode: Opcode::Addr,
                i0: 1,
                i1: 2,
                o0: 3,
            },
            Instruction {
                opcode: Opcode::Setr,
                i0: 1,
                i1: 0,
                o0: 0,
            },
            Instruction {
                opcode: Opcode::Seti,
                i0: 8,
                i1: 0,
                o0: 4,
            },
            Instruction {
                opcode: Opcode::Seti,
                i0: 9,
                i1: 0,
                o0: 5,
            },
        ];
        let mut cpu = CPU::new();
        cpu.run(ipointer, &instructions);
        assert_eq!([6, 5, 6, 0, 0, 9], cpu.registers);
    }
}
