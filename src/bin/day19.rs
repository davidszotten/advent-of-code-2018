use aoc2018::{dispatch, Result};
use failure::{err_msg, Error};
use lazy_static::lazy_static;
use regex::{CaptureMatches, Captures, Regex};
use std::str::FromStr;

fn main() {
    dispatch(&part1, &part2)
}

type RegType = i32;
type Registers = [RegType; 6];

#[derive(Debug, Clone, Copy)]
enum OpType {
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

impl FromStr for OpType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        use self::OpType::*;
        match s {
            "addr" => Ok(Addr),
            "addi" => Ok(Addi),
            "mulr" => Ok(Mulr),
            "muli" => Ok(Muli),
            "banr" => Ok(Banr),
            "bani" => Ok(Bani),
            "borr" => Ok(Borr),
            "bori" => Ok(Bori),
            "setr" => Ok(Setr),
            "seti" => Ok(Seti),
            "gtir" => Ok(Gtir),
            "gtri" => Ok(Gtri),
            "gtrr" => Ok(Gtrr),
            "eqir" => Ok(Eqir),
            "eqri" => Ok(Eqri),
            "eqrr" => Ok(Eqrr),
            _ => Err(err_msg("parse fail")),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Op {
    op_type: OpType,
    a: RegType,
    b: RegType,
    c: RegType,
}

struct OpWalker<'r, 't> {
    caps: CaptureMatches<'r, 't>,
}

impl<'r, 't> OpWalker<'r, 't> {
    fn new<'s: 't + 'r>(s: &'s str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\w{4}) (\d+) (\d+) (\d+)").unwrap();
        }

        let caps = RE.captures_iter(s);
        OpWalker { caps }
    }
}

impl<'r, 't> Iterator for OpWalker<'r, 't> {
    type Item = Op;

    fn next(&mut self) -> Option<Op> {
        if let Some(caps) = self.caps.next() {
            fn get_cap_int(caps: &Captures, pos: usize) -> i32 {
                caps[pos].parse().unwrap()
            }
            Some(Op {
                op_type: caps[1].parse().unwrap(),
                a: get_cap_int(&caps, 2),
                b: get_cap_int(&caps, 3),
                c: get_cap_int(&caps, 4),
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
struct Cpu {
    pc: RegType,
    pc_register: usize,
    registers: Registers,
    program: Vec<Op>,
}

impl Cpu {
    fn new(pc_register: usize, registers: Registers, program: Vec<Op>) -> Self {
        Cpu {
            pc: 0,
            pc_register,
            registers,
            program,
        }
    }
    fn get(&self, register: RegType) -> RegType {
        self.registers[register as usize]
    }
    fn set(&mut self, register: RegType, value: RegType) {
        self.registers[register as usize] = value;
    }
    // fn compare(&self, other: Registers) -> bool {
    //     self.registers == other
    // }

    fn run(&mut self) {
        loop {
            // println!("{:?}", self.registers);
            let instruction_idx = self.get(self.pc_register as i32);
            if instruction_idx < 0 {
                break;
            }
            if let Some(&op) = self.program.get(instruction_idx as usize) {
                // println!("{:?}", op);
                self.dispatch(&op.op_type, op.a, op.b, op.c);
                let pc = self.get(self.pc_register as i32) + 1;
                if pc < 0 || pc >= self.program.len() as i32 {
                    break;
                }
                self.set(self.pc_register as i32, pc);
                // println!("{:?}\n", self.registers);
            } else {
                break;
            }
        }
    }

    fn dispatch(&mut self, op: &OpType, a: RegType, b: RegType, c: RegType) {
        use self::OpType::*;
        match op {
            Addr => self.addr(a, b, c),
            Addi => self.addi(a, b, c),
            Mulr => self.mulr(a, b, c),
            Muli => self.muli(a, b, c),
            Banr => self.banr(a, b, c),
            Bani => self.bani(a, b, c),
            Borr => self.borr(a, b, c),
            Bori => self.bori(a, b, c),
            Setr => self.setr(a, b, c),
            Seti => self.seti(a, b, c),
            Gtir => self.gtir(a, b, c),
            Gtri => self.gtri(a, b, c),
            Gtrr => self.gtrr(a, b, c),
            Eqir => self.eqir(a, b, c),
            Eqri => self.eqri(a, b, c),
            Eqrr => self.eqrr(a, b, c),
        }
    }

    fn addr(&mut self, a: RegType, b: RegType, c: RegType) {
        self.set(c, self.get(a) + self.get(b));
    }
    fn addi(&mut self, a: RegType, b: RegType, c: RegType) {
        self.set(c, self.get(a) + b);
    }
    fn mulr(&mut self, a: RegType, b: RegType, c: RegType) {
        self.set(c, self.get(a) * self.get(b));
    }
    fn muli(&mut self, a: RegType, b: RegType, c: RegType) {
        self.set(c, self.get(a) * b);
    }
    fn banr(&mut self, a: RegType, b: RegType, c: RegType) {
        self.set(c, self.get(a) & self.get(b));
    }
    fn bani(&mut self, a: RegType, b: RegType, c: RegType) {
        self.set(c, self.get(a) & b);
    }
    fn borr(&mut self, a: RegType, b: RegType, c: RegType) {
        self.set(c, self.get(a) | self.get(b));
    }
    fn bori(&mut self, a: RegType, b: RegType, c: RegType) {
        self.set(c, self.get(a) | b);
    }
    fn setr(&mut self, a: RegType, _: RegType, c: RegType) {
        self.set(c, self.get(a));
    }
    fn seti(&mut self, a: RegType, _: RegType, c: RegType) {
        self.set(c, a);
    }
    fn _gt(&mut self, a: RegType, b: RegType, c: RegType) {
        self.set(c, if a > b { 1 } else { 0 })
    }
    fn gtir(&mut self, a: RegType, b: RegType, c: RegType) {
        self._gt(a, self.get(b), c);
    }
    fn gtri(&mut self, a: RegType, b: RegType, c: RegType) {
        self._gt(self.get(a), b, c);
    }
    fn gtrr(&mut self, a: RegType, b: RegType, c: RegType) {
        self._gt(self.get(a), self.get(b), c);
    }

    fn _eq(&mut self, a: RegType, b: RegType, c: RegType) {
        self.set(c, if a == b { 1 } else { 0 })
    }
    fn eqir(&mut self, a: RegType, b: RegType, c: RegType) {
        self._eq(a, self.get(b), c);
    }
    fn eqri(&mut self, a: RegType, b: RegType, c: RegType) {
        self._eq(self.get(a), b, c);
    }
    fn eqrr(&mut self, a: RegType, b: RegType, c: RegType) {
        self._eq(self.get(a), self.get(b), c);
    }
}

fn part1(input: &str) -> Result<i32> {
    let (pc_info, program) = input.split_at(5);
    let pc_register = pc_info
        .split_whitespace()
        .filter_map(|p| p.parse::<usize>().ok())
        .nth(0)
        .unwrap();
    let program = OpWalker::new(program).collect();
    let mut cpu = Cpu::new(pc_register, [0, 0, 0, 0, 0, 0], program);
    cpu.run();
    Ok(cpu.get(0))
}

fn part2(input: &str) -> Result<i32> {
    let (pc_info, program) = input.split_at(5);
    let pc_register = pc_info
        .split_whitespace()
        .filter_map(|p| p.parse::<usize>().ok())
        .nth(0)
        .unwrap();
    let program = OpWalker::new(program).collect();
    let mut cpu = Cpu::new(pc_register, [1, 0, 0, 0, 0, 0], program);
    cpu.run();
    Ok(cpu.get(0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        Ok(assert_eq!(
            part1(
                "#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5"
            )?,
            6
        ))
    }
}
