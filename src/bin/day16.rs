use aoc2018::{dispatch, Result};
use lazy_static::lazy_static;
use regex::{CaptureMatches, Captures, Regex};
use std::collections::{HashMap, HashSet};

fn main() {
    dispatch(&part1, &part2)
}

type RegType = i32;
type Op = [RegType; 4];
type Registers = [RegType; 4];

#[derive(Debug)]
struct Input {
    before: Op,
    after: Op,
    op: Op,
}

struct InputWalker<'r, 't> {
    caps: CaptureMatches<'r, 't>,
}

impl<'r, 't> InputWalker<'r, 't> {
    fn new<'s: 't + 'r>(s: &'s str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"Before: \[(\d+), (\d+), (\d+), (\d+)\]
(\d+) (\d+) (\d+) (\d+)
After:  \[(\d+), (\d+), (\d+), (\d+)\]"
            )
            .unwrap();
        }

        let caps = RE.captures_iter(s);
        InputWalker { caps }
    }
}

impl<'r, 't> Iterator for InputWalker<'r, 't> {
    type Item = Input;

    fn next(&mut self) -> Option<Input> {
        if let Some(caps) = self.caps.next() {
            fn get_cap_int(caps: &Captures, pos: usize) -> i32 {
                caps[pos].parse().unwrap()
            }
            Some(Input {
                before: [
                    get_cap_int(&caps, 1),
                    get_cap_int(&caps, 2),
                    get_cap_int(&caps, 3),
                    get_cap_int(&caps, 4),
                ],
                op: [
                    get_cap_int(&caps, 5),
                    get_cap_int(&caps, 6),
                    get_cap_int(&caps, 7),
                    get_cap_int(&caps, 8),
                ],
                after: [
                    get_cap_int(&caps, 9),
                    get_cap_int(&caps, 10),
                    get_cap_int(&caps, 11),
                    get_cap_int(&caps, 12),
                ],
            })
        } else {
            None
        }
    }
}

struct OpWalker<'r, 't> {
    caps: CaptureMatches<'r, 't>,
}

impl<'r, 't> OpWalker<'r, 't> {
    fn new<'s: 't + 'r>(s: &'s str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+) (\d+) (\d+) (\d+)").unwrap();
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
            Some([
                get_cap_int(&caps, 1),
                get_cap_int(&caps, 2),
                get_cap_int(&caps, 3),
                get_cap_int(&caps, 4),
            ])
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Cpu {
    registers: Registers,
}

impl Cpu {
    fn new(registers: Registers) -> Self {
        Cpu { registers }
    }
    fn get(&self, register: RegType) -> RegType {
        self.registers[register as usize]
    }
    fn set(&mut self, register: RegType, value: RegType) {
        self.registers[register as usize] = value;
    }
    fn compare(&self, other: Registers) -> bool {
        self.registers == other
    }
    fn dispatch(&mut self, op: &str, a: RegType, b: RegType, c: RegType) {
        match op {
            "addr" => self.addr(a, b, c),
            "addi" => self.addi(a, b, c),
            "mulr" => self.mulr(a, b, c),
            "muli" => self.muli(a, b, c),
            "banr" => self.banr(a, b, c),
            "bani" => self.bani(a, b, c),
            "borr" => self.borr(a, b, c),
            "bori" => self.bori(a, b, c),
            "setr" => self.setr(a, b, c),
            "seti" => self.seti(a, b, c),
            "gtir" => self.gtir(a, b, c),
            "gtri" => self.gtri(a, b, c),
            "gtrr" => self.gtrr(a, b, c),
            "eqir" => self.eqir(a, b, c),
            "eqri" => self.eqri(a, b, c),
            "eqrr" => self.eqrr(a, b, c),
            _ => panic!("invalid op code"),
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

fn try_all(before: Registers, after: Registers, op: Op) -> Vec<String> {
    let mut result = vec![];
    let funcs = [
        "addr", "addi", "mulr", "muli", "banr", "bani", "borr", "bori", "setr", "seti", "gtir",
        "gtri", "gtrr", "eqir", "eqri", "eqrr",
    ];

    for name in funcs.iter() {
        let mut cpu = Cpu::new(before);
        cpu.dispatch(name, op[1], op[2], op[3]);
        if cpu.compare(after) {
            result.push((*name).into());
        }
    }

    result
}

fn part1(input: &str) -> Result<i32> {
    let mut count = 0;
    for input in InputWalker::new(input) {
        // println!("{:?}", input);
        if try_all(input.before, input.after, input.op).len() >= 3 {
            count += 1;
        }
    }
    Ok(count)
}

fn part2(input_str: &str) -> Result<i32> {
    let mut possible_ops: HashMap<i32, HashSet<String>> = HashMap::new();
    let mut op_inputs = 0;
    for input in InputWalker::new(&input_str) {
        op_inputs += 1;
        let opcode = input.op[0];
        let current_matches: HashSet<String> = try_all(input.before, input.after, input.op)
            .into_iter()
            .collect();
        let entry = possible_ops
            .entry(opcode)
            .or_insert(current_matches.clone());
        *entry = entry.intersection(&current_matches).cloned().collect();
    }

    let mut ops = HashMap::new();
    let mut total = 0;
    while total < 16 {
        let mut found = None;
        for (key, possible) in possible_ops.iter() {
            if possible.len() == 1 {
                let op = possible.iter().next().unwrap();
                ops.insert(key.clone(), op.clone());
                found = Some(op.clone());
                total += 1;
                break;
            }
        }
        let found: String = found.unwrap().to_string();
        for value in possible_ops.values_mut() {
            (*value).remove(&found);
        }
    }
    println!("{:?}", ops);
    let mut cpu = Cpu::new([0, 0, 0, 0]);
    let mut count = 0;
    for op in OpWalker::new(&input_str).skip(op_inputs) {
        count += 1;
        let op_name = ops.get(&op[0]).unwrap();
        cpu.dispatch(op_name, op[1], op[2], op[3]);
    }
    println!("{}", count);
    Ok(cpu.get(0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut cpu = Cpu {
            registers: [1, 2, 0, 0],
        };
        cpu.addr(0, 1, 2);
        assert_eq!(cpu.get(2), 3)
    }

    #[test]
    fn test_try() {
        assert_eq!(
            try_all([3, 2, 1, 1], [3, 2, 2, 1], [9, 2, 1, 2]),
            vec!["addi", "mulr", "seti"]
        );
    }

    #[test]
    fn test_part1() -> Result<()> {
        Ok(assert_eq!(part1("")?, 0))
    }
}
