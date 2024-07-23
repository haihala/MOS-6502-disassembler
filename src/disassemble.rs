use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
enum Operation {
    ADC,
    AND,
    ASL,
    BBC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
    Unknown,
}

use Operation::*;

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if *self == Operation::Unknown {
            write!(f, "???")
        } else {
            write!(f, "{:?}", self)
        }
    }
}

#[derive(Debug)]
enum AddressMode {
    Accumulator,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Immediate,
    Implied,
    Indirect,
    XIndirect,
    IndirectY,
    Relative,
    Zeropage,
    ZeropageX,
    ZeropageY,
    Unknown,
}
impl AddressMode {
    fn format(&self, args: &[u8], offset: usize) -> String {
        // The entire instruction is passed in, index accordingly
        match self {
            Accumulator => " A".to_string(),
            Absolute => format!(" ${:0>2X}{:0>2X}", args[2], args[1]),
            AbsoluteX => format!(" ${:0>2X}{:0>2X},X", args[2], args[1]),
            AbsoluteY => format!(" ${:0>2X}{:0>2X},Y", args[2], args[1]),
            Immediate => format!(" #${:0>2X}", args[1]),
            Implied => String::new(),
            Indirect => format!(" (${:0>2X}{:0>2X})", args[2], args[1]),
            XIndirect => format!(" (${:0>2X},X)", args[1]),
            IndirectY => format!(" (${:0>2X}),Y", args[1]),
            Relative => format!(" ${:0>4X}", (offset + 2) as i16 + (args[1] as i8) as i16),
            Zeropage => format!(" ${:0>2X}", args[1]),
            ZeropageX => format!(" ${:0>2X},X", args[1]),
            ZeropageY => format!(" ${:0>2X},Y", args[1]),
            AddressMode::Unknown => {
                if args[0].is_ascii_alphabetic() || args[0].is_ascii_punctuation() {
                    // Ascii is a subset of utf-8
                    // args should only have one value for Undefined addresses
                    let char = String::from_utf8(args.to_vec()).unwrap();
                    format!("                ;%{:0>8b} '{}'", args[0], char)
                } else {
                    format!("                ;%{:0>8b}", args[0])
                }
            }
        }
    }

    fn length(&self) -> usize {
        match self {
            Accumulator | Implied | AddressMode::Unknown => 1,
            Immediate | Relative | Zeropage | ZeropageX | ZeropageY | XIndirect | IndirectY => 2,
            Absolute | AbsoluteX | AbsoluteY | Indirect => 3,
        }
    }
}
use AddressMode::*;

fn decode_opcode(value: u8) -> (Operation, AddressMode) {
    match value {
        // Goes through https://www.masswerk.at/6502/6502_instruction_set.html
        // Considered parsing this, but got carried away with vim macros.
        0x00 => (BRK, Implied),
        0x01 => (ORA, XIndirect),
        0x05 => (ORA, Zeropage),
        0x06 => (ASL, Zeropage),
        0x08 => (PHP, Implied),
        0x09 => (ORA, Immediate),
        0x0a => (ASL, Accumulator),
        0x0d => (ORA, Absolute),
        0x0e => (ASL, Absolute),

        0x10 => (BPL, Relative),
        0x11 => (ORA, IndirectY),
        0x15 => (ORA, ZeropageX),
        0x16 => (ASL, ZeropageX),
        0x18 => (CLC, Implied),
        0x19 => (ORA, AbsoluteY),
        0x1d => (ORA, AbsoluteX),
        0x1e => (ASL, AbsoluteX),

        0x20 => (JSR, Absolute),
        0x21 => (AND, XIndirect),
        0x24 => (BIT, Zeropage),
        0x25 => (AND, Zeropage),
        0x26 => (ROL, Zeropage),
        0x28 => (PLP, Implied),
        0x29 => (AND, Immediate),
        0x2a => (ROL, Accumulator),
        0x2c => (BIT, Absolute),
        0x2d => (AND, Absolute),
        0x2e => (ROL, Absolute),

        0x30 => (BMI, Relative),
        0x31 => (AND, IndirectY),
        0x35 => (AND, ZeropageX),
        0x36 => (ROL, ZeropageX),
        0x38 => (PLP, Implied),
        0x39 => (AND, AbsoluteY),
        0x3d => (AND, AbsoluteX),
        0x3e => (ROL, AbsoluteX),

        0x40 => (RTI, Implied),
        0x41 => (EOR, XIndirect),
        0x45 => (EOR, Zeropage),
        0x46 => (LSR, Zeropage),
        0x48 => (PHA, Implied),
        0x49 => (EOR, Immediate),
        0x4a => (LSR, Accumulator),
        0x4c => (JMP, Absolute),
        0x4d => (EOR, Absolute),
        0x4e => (LSR, Absolute),

        0x50 => (BVC, Relative),
        0x51 => (EOR, IndirectY),
        0x55 => (EOR, ZeropageX),
        0x56 => (LSR, ZeropageX),
        0x58 => (CLI, Implied),
        0x59 => (EOR, AbsoluteY),
        0x5d => (EOR, AbsoluteX),
        0x5e => (LSR, AbsoluteX),

        0x60 => (RTS, Implied),
        0x61 => (ADC, XIndirect),
        0x65 => (ADC, Zeropage),
        0x66 => (ROR, Zeropage),
        0x68 => (PLA, Implied),
        0x69 => (ADC, Immediate),
        0x6a => (ROR, Accumulator),
        0x6c => (JMP, Indirect),
        0x6d => (ADC, Absolute),
        0x6e => (ROR, Absolute),

        0x70 => (BVS, Relative),
        0x71 => (ADC, IndirectY),
        0x75 => (ADC, ZeropageX),
        0x76 => (ROR, ZeropageX),
        0x78 => (SEI, Implied),
        0x79 => (ADC, AbsoluteY),
        0x7d => (ADC, AbsoluteX),
        0x7e => (ROR, AbsoluteX),

        0x81 => (STA, XIndirect),
        0x84 => (STY, Zeropage),
        0x85 => (STA, Zeropage),
        0x86 => (STX, Zeropage),
        0x88 => (DEY, Implied),
        0x8a => (TXA, Implied),
        0x8c => (STY, Absolute),
        0x8d => (STA, Absolute),
        0x8e => (STX, Absolute),

        0x90 => (BBC, Relative),
        0x91 => (STA, IndirectY),
        0x94 => (STY, ZeropageX),
        0x95 => (STA, ZeropageX),
        0x96 => (STX, ZeropageY),
        0x98 => (TYA, Implied),
        0x99 => (STA, AbsoluteX),
        0x9a => (TXS, Implied),
        0x9d => (STA, AbsoluteX),

        0xa0 => (LDY, Immediate),
        0xa1 => (LDA, XIndirect),
        0xa2 => (LDX, Immediate),
        0xa4 => (LDY, Zeropage),
        0xa5 => (LDA, Zeropage),
        0xa6 => (LDX, Zeropage),
        0xa8 => (TAY, Implied),
        0xa9 => (LDA, Immediate),
        0xaa => (TAX, Implied),
        0xac => (LDY, Absolute),
        0xad => (LDA, Absolute),
        0xae => (LDX, Absolute),

        0xb0 => (BCS, Relative),
        0xb1 => (LDA, IndirectY),
        0xb4 => (LDY, ZeropageX),
        0xb5 => (LDA, ZeropageX),
        0xb6 => (LDX, ZeropageY),
        0xb8 => (CLV, Implied),
        0xb9 => (LDA, AbsoluteY),
        0xba => (TSX, Implied),
        0xbc => (LDY, AbsoluteX),
        0xbd => (LDA, AbsoluteX),
        0xbe => (LDX, AbsoluteY),

        0xc0 => (CPY, Immediate),
        0xc1 => (CMP, XIndirect),
        0xc4 => (CPY, Zeropage),
        0xc5 => (CMP, Zeropage),
        0xc6 => (DEC, Zeropage),
        0xc8 => (INY, Implied),
        0xc9 => (CMP, Immediate),
        0xca => (DEX, Implied),
        0xcc => (CPY, Absolute),
        0xcd => (CMP, Absolute),
        0xce => (DEC, Absolute),

        0xd0 => (BNE, Relative),
        0xd1 => (CMP, IndirectY),
        0xd5 => (CMP, Zeropage),
        0xd6 => (DEC, ZeropageX),
        0xd8 => (CLD, Implied),
        0xd9 => (CMP, AbsoluteY),
        0xdd => (CMP, AbsoluteX),
        0xde => (DEC, AbsoluteX),

        0xe0 => (CPX, Immediate),
        0xe1 => (SBC, XIndirect),
        0xe4 => (CPX, Zeropage),
        0xe5 => (SBC, Zeropage),
        0xe6 => (INC, Zeropage),
        0xe8 => (INX, Implied),
        0xe9 => (SBC, Immediate),
        0xea => (NOP, Implied),
        0xec => (CPX, Absolute),
        0xed => (SBC, Absolute),
        0xee => (INC, Absolute),

        0xf0 => (BEQ, Relative),
        0xf1 => (SBC, IndirectY),
        0xf5 => (SBC, ZeropageX),
        0xf6 => (INC, ZeropageX),
        0xf8 => (SED, Implied),
        0xf9 => (SBC, AbsoluteY),
        0xfd => (SBC, AbsoluteX),
        0xfe => (INC, AbsoluteX),

        _ => (Operation::Unknown, AddressMode::Unknown),
    }
}

#[derive(Debug)]
struct Instruction {
    operation: Operation,
    address_mode: AddressMode,
    raw_bytes: Vec<u8>,
    offset: usize,
}

impl Instruction {
    fn new(offset: usize, token: u8) -> Self {
        let (operation, address_mode) = decode_opcode(token);

        Instruction {
            offset,
            operation,
            address_mode,
            raw_bytes: vec![token],
        }
    }

    fn add(&mut self, token: u8) {
        self.raw_bytes.push(token);
    }

    fn is_satisfied(&self) -> bool {
        self.address_mode.length() == self.raw_bytes.len()
    }
}
impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string_bytes: Vec<String> = self
            .raw_bytes
            .iter()
            .map(|byte| format!("{:0>2X}", byte))
            .collect();

        write!(
            f,
            "{:04X}   {: <8}      {}{}",
            self.offset,
            string_bytes.join(" "),
            self.operation,
            self.address_mode.format(&self.raw_bytes, self.offset),
        )
    }
}

pub fn disassemble(data: Vec<u8>) -> Vec<String> {
    data.into_iter()
        .enumerate()
        .fold(vec![], |mut acc: Vec<Instruction>, (offset, token)| {
            match acc.last_mut() {
                Some(last) if !last.is_satisfied() => last.add(token),
                _ => acc.push(Instruction::new(offset, token)),
            };

            acc
        })
        .into_iter()
        .map(|row| row.to_string())
        .collect()
}

#[cfg(test)]
mod test {
    use std::{fs, io::BufRead};

    use crate::disassemble;

    #[test]
    fn test_binary_one() {
        test_example_bin("test1");
    }

    #[test]
    fn test_binary_two() {
        test_example_bin("test2");
    }

    fn test_example_bin(case: &'static str) {
        let input = fs::read(format!("test-bin/{}.bin", case)).unwrap();
        // Examples are from https://www.masswerk.at/6502/disassembler.html
        // Set output format to "verbose - no symbols"
        // Removed first and last lines
        let expected = fs::read(format!("test-bin/{}.example", case)).unwrap();

        for (line, (output, exp)) in disassemble(input)
            .into_iter()
            .zip(expected.lines())
            .enumerate()
        {
            assert_eq!(output, exp.unwrap(), "Line {}", line);
        }
    }
}
