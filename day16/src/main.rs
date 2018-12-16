use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let task = &args[1];
    let filename = &args[2];

    if task == "something" {
        println!("do something");
    } else {
        panic!("Don't know how to '{}'", task);
    }
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
}
