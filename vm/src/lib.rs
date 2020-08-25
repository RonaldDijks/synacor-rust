#![allow(dead_code)]

use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

const MEM_SIZE: usize = 32768;
const NUM_REGISTERS: usize = 8;

#[derive(Debug)]
pub enum Operation {
    Halt,
    Jump(u16),
    Out(char),
    Noop,
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

    fn next_operation(&mut self) -> Operation {
        let opcode = self.memory[self.pc as usize];
        match opcode {
            0 => {
                self.pc += 1;
                Operation::Halt
            }
            6 => {
                self.pc += 1;
                let address = self.memory[self.pc as usize];
                Operation::Jump(address)
            }
            19 => {
                self.pc += 1;
                let charcode = self.memory[self.pc as usize] as u32;
                let character = std::char::from_u32(charcode)
                    .map(Operation::Out)
                    .unwrap();
                self.pc += 1;
                character
            }
            21 => {
                self.pc += 1;
                Operation::Noop
            }
            _ => panic!("Unexpected opcode: {}", opcode),
        }
    }

    pub fn execute(&mut self) {
        loop {
            let op = self.next_operation();
            match op {
                Operation::Halt => {
                    break;
                }
                Operation::Jump(address) => {
                    self.pc = address;
                }
                Operation::Out(c) => {
                    print!("{}", c);
                }
                Operation::Noop => {
                    continue;
                }
            }
        }
    }
}
