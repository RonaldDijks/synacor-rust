#![allow(dead_code)]

use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

const MEM_SIZE: usize = 32768;
const NUM_REGISTERS: usize = 8;

#[derive(Debug)]
pub enum Operation {
    Halt,
    Jump(Operand),
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
            6 => Operation::Jump(self.parse_operand()),
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

    pub fn execute(&mut self) {
        loop {
            let op = self.parse_operation();
            match op {
                Operation::Halt => break,
                Operation::Jump(a) => {
                    self.pc = self.get_operand(a);
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
