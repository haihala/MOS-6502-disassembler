use std::fs;

use clap::Parser;
use mos_6502_disassembler::disassemble;

#[derive(Debug, Parser)]
struct Args {
    files: Vec<String>,
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    for file in args.files {
        let input = fs::read(file.clone()).expect("to be able to open file");

        if args.verbose {
            println!("Disassembly of {}:", file);
        }

        for line in disassemble(input) {
            println!("{}", line);
        }

        if args.verbose {
            println!();
        }
    }
}
