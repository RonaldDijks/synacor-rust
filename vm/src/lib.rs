#![allow(dead_code)]

use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

const MEM_SIZE: usize = 32768;
const NUM_REGISTERS: usize = 8;

#[derive(Debug)]
pub enum Operation {
    Halt,
    Set(Operand, Operand),
    Push(Operand),
    Eq(Operand, Operand, Operand),
    Jump(Operand),
    Jt(Operand, Operand),
    Jf(Operand, Operand),
    Add(Operand, Operand, Operand),
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
        if code < 32767 {
            Operand::Literal(code as usize)
        } else if code < 32776 {
            Operand::Register((code % 32768) as usize)
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

    pub fn parse_operand(&mut self) -> Operand {
        let code = self.memory[self.pc as usize];
        self.pc += 1;
        Operand::new(code)
    }

    fn parse_operation(&mut self) -> Operation {
        let opcode = self.memory[self.pc as usize];
        self.pc += 1;
        match opcode {
            0 => Operation::Halt,
            1 => Operation::Set(self.parse_operand(), self.parse_operand()),
            2 => Operation::Push(self.parse_operand()),
            4 => Operation::Eq(
                self.parse_operand(),
                self.parse_operand(),
                self.parse_operand(),
            ),
            6 => Operation::Jump(self.parse_operand()),
            7 => Operation::Jt(self.parse_operand(), self.parse_operand()),
            8 => Operation::Jf(self.parse_operand(), self.parse_operand()),
            9 => Operation::Add(
                self.parse_operand(),
                self.parse_operand(),
                self.parse_operand(),
            ),
            19 => Operation::Out(self.parse_operand()),
            21 => Operation::Noop,
            _ => panic!("Unexpected opcode: {}", opcode),
        }
    }

    fn get_operand(&mut self, operand: Operand) -> u16 {
        match operand {
            Operand::Literal(n) => n as u16,
            Operand::Register(r) => self.registers[r],
        }
    }

    fn set_value(&mut self, address: Operand, value: u16) {
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
                    let b = self.get_operand(b);
                    self.set_value(a, b);
                }
                Operation::Push(a) => {
                    let a = self.get_operand(a);
                    self.stack.push(a);
                }
                Operation::Eq(a, b, c) => {
                    let b = self.get_operand(b);
                    let c = self.get_operand(c);
                    let value = if b == c { 1 } else { 0 };
                    self.set_value(a, value);
                }
                Operation::Jump(a) => {
                    self.pc = self.get_operand(a);
                }
                Operation::Jt(a, b) => {
                    let a = self.get_operand(a);
                    let b = self.get_operand(b);
                    if a > 0 {
                        self.pc = b
                    }
                }
                Operation::Jf(a, b) => {
                    let a = self.get_operand(a);
                    let b = self.get_operand(b);
                    if a == 0 {
                        self.pc = b
                    }
                }
                Operation::Add(a, b, c) => {
                    let b = self.get_operand(b);
                    let c = self.get_operand(c);
                    let value = (b + c) % 32768;
                    self.set_value(a, value)
                }
                Operation::Out(a) => {
                    let a = self.get_operand(a) as u32;
                    let a = std::char::from_u32(a).unwrap();
                    print!("{}", a);
                }
                Operation::Noop => continue,
            }
        }
    }
}
