# Tools Programmer Homework

Disassembler for the MOS 6502 microprocessor family. Used
https://www.masswerk.at/6502/6502_instruction_set.html as a reference.

Used https://github.com/sagiegurari/cargo-make to manage tasks. The project has
the following tasks available:

- clean: Runs cargo clean
- build: Runs cargo build
- test: Runs cargo test
- format: Check formatting with rustfmt
- clippy: Check code with clippy
- validate: Runs build, test, format, and clippy tasks

Run any of them with `cargo make <task>`. You need to install cargo-make.
