use aoc2018::{dispatch, Result};
use failure::{err_msg, Error};
use lazy_static::lazy_static;
use regex::{CaptureMatches, Captures, Regex};
use std::fmt;
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

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::OpType::*;
        let name = match self.op_type {
            Addr => "addr",
            Addi => "addi",
            Mulr => "mulr",
            Muli => "muli",
            Banr => "banr",
            Bani => "bani",
            Borr => "borr",
            Bori => "bori",
            Setr => "setr",
            Seti => "seti",
            Gtir => "gtir",
            Gtri => "gtri",
            Gtrr => "gtrr",
            Eqir => "eqir",
            Eqri => "eqri",
            Eqrr => "eqrr",
        };
        write!(f, "{}({:02}, {:02}, {:02})", name, self.a, self.b, self.c)
    }
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

    fn from_input(input: &str, registers: Registers) -> Self {
        let (pc_info, program) = input.split_at(5);
        let pc_register = pc_info
            .split_whitespace()
            .filter_map(|p| p.parse::<usize>().ok())
            .nth(0)
            .unwrap();
        let program = OpWalker::new(program).collect();
        Cpu::new(pc_register, registers, program)
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
            let instruction_idx = self.get(self.pc_register as i32);
            if instruction_idx < 0 {
                break;
            }
            println!("{}: {:?}", instruction_idx, self.registers);
            if let Some(&op) = self.program.get(instruction_idx as usize) {
                self.dispatch(&op.op_type, op.a, op.b, op.c);
                let pc = self.get(self.pc_register as i32) + 1;
                if pc < 0 || pc >= self.program.len() as i32 {
                    break;
                }
                self.set(self.pc_register as i32, pc);
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
    let mut cpu = Cpu::from_input(input, [29395, 0, 0, 0, 0, 0]);
    // let mut cpu = Cpu::from_input(input, [13522479, 0, 0, 0, 0, 0]);
    cpu.run();
    Ok(cpu.get(0))
}

fn fast(r0: i32, mut max_loops: i32) -> Option<i32> {
    let mut r1 = 0;
    let mut r2;
    let mut r3 = 0;
    let mut r5 = 0;
    // #ip 4
    // 00 - 04: self test

    // 05: seti 0 0 2 : r2 = 0
    r2 = 0;
    // 06: bori 2 65536 5 : r5 = r2 | (2^16)
    while r2 != r0 {
        max_loops -= 1;
        if max_loops < 0 {
            return None;
        }
        // println!("loop2");
        r5 = r2 | 65536;
        // 07: seti 5234604 6 2 : r2 = 5234604
        r2 = 5234604;
        // 08: bani 5 255 3 : r3 = r5 & 255
        loop {
            max_loops -= 1;
            if max_loops < 0 {
                return None;
            }
            r3 = r5 & 255;
            // 09: addr 2 3 2 : r2 = r2 + r3
            r2 = r2 + r3;
            // 10: bani 2 16777215 2 : r2 = r2 & 16777215 (2^24-1)
            r2 = r2 & 16777215;
            // 11: muli 2 65899 2 : r2 = r2 * 65899
            r2 = r2 * 65899;
            // 12: bani 2 16777215 2 : r2 = r2 & (2^24-1)
            r2 = r2 & 16777215;
            // 13: gtir 256 5 3 : r3 = (256 > r5) : if r5 < 256 then jmp 28 else jmp 17
            // println!("13: {} {} {} {} {} {}", r0, r1, r2, r3, "_", r5);
            if r5 >= 256 {
                // not read: r3 = 0;

                r3 = r5 / 256;
                r1 = 1;

                // 26: setr 3 4 5 : r5 = r3
                // println!("26: {} {} {} {} {} {}", r0, r1, r2, r3, "_", r5);
                r5 = r3;
            // 27: seti 7 8 4 : jmp 8
            } else {
                r3 = 1;
                break;
            }
        }
        // 28: eqrr 2 0 3 : r3 = (r2 == r0) : if r2 == r0 then end
        // println!("28: {} {} {} {} {} {}", r0, r1, r2, r3, "_", r5);
        // if r2 == r0 {
        //     break;
        // } else {
        //     // not read: r3 = 0;
        // }
        // 29: addr 3 4 4 : r4 = r4 + r3

        // 30: seti 5 6 4 : jmp 6
    }
    // println!("loops remaining: {}", max_loops);
    // println!("done: {} {} {} {} {} {}", r0, r1, r2, r3, "_", r5);
    // println!("r0: {}, loops remaining: {}", r0, max_loops);
    Some(max_loops)
}

fn part2(_input: &str) -> Result<i32> {
    // let mut cpu = Cpu::from_input(input, [13522479, 0, 0, 0, 0, 0]);
    // cpu.run();
    // println!("{:?}", fast(13522479, 1000));
    // return Ok(0);
    // fast(13522479);

    let mut fewest_remaining = 100_000;
    let mut best_r0 = 0;

    let mut reg0 = 1;
    Ok(loop {
        if reg0 % 1000_000 == 0 {
            println!("{}", reg0);
        }
        if let Some(remaining) = fast(reg0, 100_000) {
            if remaining < fewest_remaining {
                fewest_remaining = remaining;
                best_r0 = reg0;
                println!("{}: {}", reg0, remaining);
            }
            // break reg0;
            // println!("{}", reg0);
        }
        reg0 += 1;
        if reg0 == 16_777_215 {
            println!("probably done");
        }
        if reg0 > 235224800 {
            break -1;
        }
    })
    // println!("{:?}", res);
    // Ok(0)
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
