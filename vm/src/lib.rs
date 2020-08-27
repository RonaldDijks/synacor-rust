#![allow(dead_code)]

use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;
use std::ops::BitAnd;
use std::ops::BitOr;

const MEM_SIZE: usize = 32768;
const NUM_REGISTERS: usize = 8;
const MAX_NUM: u16 = 32768;

#[derive(Debug)]
pub enum Operation {
    Halt,
    Set(Operand, Operand),
    Push(Operand),
    Pop(Operand),
    Eq(Operand, Operand, Operand),
    Gt(Operand, Operand, Operand),
    Jump(Operand),
    Jt(Operand, Operand),
    Jf(Operand, Operand),
    Add(Operand, Operand, Operand),
    Mult(Operand, Operand, Operand),
    Mod(Operand, Operand, Operand),
    And(Operand, Operand, Operand),
    Or(Operand, Operand, Operand),
    Not(Operand, Operand),
    RMem(Operand, Operand),
    WMem(Operand, Operand),
    Call(Operand),
    Ret,
    Out(Operand),
    Noop,
}

#[derive(Debug)]
pub enum Operand {
    Literal(usize),
    Register(usize),
}

impl Operand {
    pub fn new(code: u16) -> Self {
        if code < 32768 {
            Operand::Literal(code as usize)
        } else if code < 32776 {
            Operand::Register((code % MAX_NUM) as usize)
        } else {
            panic!("Unexpected operand: {}", code)
        }
    }
}

pub struct VM {
    memory: [u16; MEM_SIZE],
    registers: [u16; NUM_REGISTERS],
    stack: Vec<u16>,
    pc: u16,
}

impl VM {
    pub fn new() -> Self {
        VM {
            memory: [0; MEM_SIZE],
            registers: [0; NUM_REGISTERS],
            stack: Vec::new(),
            pc: 0,
        }
    }

    pub fn load_memory(&mut self, program: Vec<u8>) {
        let mut cursor = Cursor::new(program);
        let mut i = 0;
        while let Ok(v) = cursor.read_u16::<LittleEndian>() {
            self.memory[i] = v;
            i += 1;
        }
    }

    pub fn parse_op(&mut self) -> Operand {
        let code = self.memory[self.pc as usize];
        self.pc += 1;
        Operand::new(code)
    }

    fn parse_operation(&mut self) -> Operation {
        let opcode = self.memory[self.pc as usize];
        self.pc += 1;
        match opcode {
            0 => Operation::Halt,
            1 => Operation::Set(self.parse_op(), self.parse_op()),
            2 => Operation::Push(self.parse_op()),
            3 => Operation::Pop(self.parse_op()),
            4 => Operation::Eq(self.parse_op(), self.parse_op(), self.parse_op()),
            5 => Operation::Gt(self.parse_op(), self.parse_op(), self.parse_op()),
            6 => Operation::Jump(self.parse_op()),
            7 => Operation::Jt(self.parse_op(), self.parse_op()),
            8 => Operation::Jf(self.parse_op(), self.parse_op()),
            9 => Operation::Add(self.parse_op(), self.parse_op(), self.parse_op()),
            10 => Operation::Mult(self.parse_op(), self.parse_op(), self.parse_op()),
            11 => Operation::Mod(self.parse_op(), self.parse_op(), self.parse_op()),
            12 => Operation::And(self.parse_op(), self.parse_op(), self.parse_op()),
            13 => Operation::Or(self.parse_op(), self.parse_op(), self.parse_op()),
            14 => Operation::Not(self.parse_op(), self.parse_op()),
            15 => Operation::RMem(self.parse_op(), self.parse_op()),
            16 => Operation::WMem(self.parse_op(), self.parse_op()),
            17 => Operation::Call(self.parse_op()),
            18 => Operation::Ret,
            19 => Operation::Out(self.parse_op()),
            21 => Operation::Noop,
            _ => panic!("Unexpected opcode: {}", opcode),
        }
    }

    fn read(&mut self, operand: Operand) -> u16 {
        match operand {
            Operand::Literal(n) => n as u16,
            Operand::Register(r) => self.registers[r],
        }
    }

    fn set(&mut self, address: Operand, value: u16) {
        match address {
            Operand::Literal(n) => self.memory[n] = value,
            Operand::Register(r) => self.registers[r] = value,
        }
    }

    pub fn execute(&mut self) {
        loop {
            let op = self.parse_operation();
            match op {
                Operation::Halt => break,
                Operation::Set(a, b) => {
                    let b = self.read(b);
                    self.set(a, b);
                }
                Operation::Push(a) => {
                    let a = self.read(a);
                    self.stack.push(a);
                }
                Operation::Pop(a) => match self.stack.pop() {
                    None => panic!(""),
                    Some(value) => self.set(a, value),
                },
                Operation::Eq(a, b, c) => {
                    let b = self.read(b);
                    let c = self.read(c);
                    let value = if b == c { 1 } else { 0 };
                    self.set(a, value);
                }
                Operation::Gt(a, b, c) => {
                    let b = self.read(b);
                    let c = self.read(c);
                    let value = if b > c { 1 } else { 0 };
                    self.set(a, value);
                }
                Operation::Jump(a) => {
                    self.pc = self.read(a);
                }
                Operation::Jt(a, b) => {
                    let a = self.read(a);
                    let b = self.read(b);
                    if a > 0 {
                        self.pc = b
                    }
                }
                Operation::Jf(a, b) => {
                    let a = self.read(a);
                    let b = self.read(b);
                    if a == 0 {
                        self.pc = b
                    }
                }
                Operation::Add(a, b, c) => {
                    let b = self.read(b);
                    let c = self.read(c);
                    let value = (b + c) % MAX_NUM;
                    self.set(a, value)
                }
                Operation::Mult(a, b, c) => {
                    let b = self.read(b);
                    let c = self.read(c);
                    let value = b.wrapping_mul(c) % MAX_NUM;
                    self.set(a, value)
                }
                Operation::Mod(a, b, c) => {
                    let b = self.read(b);
                    let c = self.read(c);
                    let value = (b % c) % MAX_NUM;
                    self.set(a, value)
                }
                Operation::And(a, b, c) => {
                    let b = self.read(b);
                    let c = self.read(c);
                    let value = (b.bitand(c)) % MAX_NUM;
                    self.set(a, value)
                }
                Operation::Or(a, b, c) => {
                    let b = self.read(b);
                    let c = self.read(c);
                    let value = (b.bitor(c)) % MAX_NUM;
                    self.set(a, value)
                }
                Operation::Not(a, b) => {
                    let b = self.read(b);
                    let value = (!b) % MAX_NUM;
                    self.set(a, value)
                }
                Operation::RMem(a, b) => {
                    let b = self.read(b);
                    let value = self.memory[b as usize];
                    self.set(a, value);
                }
                Operation::WMem(a, b) => {
                    let b = self.read(b);
                    let addr = self.read(a);
                    self.set(Operand::Literal(addr as usize), b);
                }
                Operation::Call(a) => {
                    let a = self.read(a);
                    self.stack.push(self.pc);
                    self.pc = a;
                }
                Operation::Ret => {
                    let address = self.stack.pop().unwrap();
                    self.pc = address;
                }
                Operation::Out(a) => {
                    let a = self.read(a) as u32;
                    let a = std::char::from_u32(a).unwrap();
                    print!("{}", a);
                }
                Operation::Noop => continue,
            }
        }
    }
}
