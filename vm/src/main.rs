mod lib;

use std::io;
use std::io::prelude::*;

use std::fs::File;

fn main() -> Result<(), io::Error> {
    let mut vm = lib::VM::new();
    let mut program = Vec::new();
    File::open("../../challenge.bin")?.read_to_end(&mut program)?;
    vm.load_memory(program);
    Ok(())
}
