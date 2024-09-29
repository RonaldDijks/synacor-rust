# 🖥️ Synacor Challenge VM

This project implements a virtual machine (VM) for the Synacor Challenge. The VM is capable of running the provided binary according to the specified architecture.

## 📁 Project Structure

- `vm/src/main.rs`: The entry point of the program, responsible for loading the binary and executing the VM.
- `vm/src/lib.rs`: Contains the core implementation of the VM, including operations, memory management, and execution logic.
- `vm/Cargo.toml`: The Rust project configuration file.
- `challenge.bin`: The binary file to be executed by the VM (not included in the repository).

## 🛠️ Requirements

- Rust programming language (edition 2018 or later)
- Cargo (Rust's package manager)

## 📦 Dependencies

- `byteorder`: Used for reading little-endian binary data.

## 🚀 Building and Running

1. Ensure you have Rust and Cargo installed on your system.
2. Navigate to the `vm` directory.
3. Build the project:
   ```
   cargo build --release
   ```
4. Place the `challenge.bin` file in the parent directory of the `vm` folder.
5. Run the VM:
   ```
   cargo run --release
   ```

## 🏗️ Architecture Specifications

The VM implements the following specifications:

- 15-bit address space storing 16-bit values
- 8 registers
- Unbounded stack for 16-bit values
- All numbers are unsigned integers 0..32767 (15-bit)
- All math is modulo 32768

For detailed opcode listings and binary format, refer to the `arch-spec` file.

## 🔧 Implementation Details

- The VM supports all specified operations, including arithmetic, logic, memory access, and control flow.
- Input/output operations are handled through a console interface.
- The VM executes instructions until a `halt` operation is encountered or an error occurs.

## 📝 Notes

- This implementation is part of the Synacor Challenge. Codes found during execution can be submitted to the challenge website to track progress.
- The `challenge.bin` file is not included in this repository and should be obtained separately.

## 🙏 Acknowledgements

This project is based on the Synacor Challenge created by Eric Wastl.
