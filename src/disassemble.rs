use std::fmt::Display;

use poem_openapi::Object;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize)]
#[allow(clippy::upper_case_acronyms)]
enum Operation {
    ADC,
    AND,
    ASL,
    BCC,
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
    SEC,
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
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
use AddressMode::*;
impl AddressMode {
    fn format(&self, formatted: &[String], raw: &[u8], offset: usize) -> String {
        if formatted.len() != self.length() {
            // Input is faulty and missing bytes
            return String::from("*Missing operands*");
        }

        match self {
            Accumulator => String::from("A"),
            Absolute => format!("${}{}", formatted[2], formatted[1]),
            AbsoluteX => format!("${}{},X", formatted[2], formatted[1]),
            AbsoluteY => format!("${}{},Y", formatted[2], formatted[1]),
            Immediate => format!("#${}", formatted[1]),
            Implied => String::new(),
            Indirect => format!("(${}{})", formatted[2], formatted[1]),
            XIndirect => format!("(${},X)", formatted[1]),
            IndirectY => format!("(${}),Y", formatted[1]),
            Relative => {
                // MOS-6502 can only handle addresses up to 2^16
                // For the sake of convenience, this disassembler handles bigger binaries
                let addr = (offset + 2) as isize + (raw[1] as i8) as isize;

                format!("${:04X}", addr as u16)
            }
            Zeropage => format!("${}", formatted[1]),
            ZeropageX => format!("${},X", formatted[1]),
            ZeropageY => format!("${},Y", formatted[1]),
            AddressMode::Unknown => {
                let is_ascii_symbol = (32..126).contains(&raw[0]);
                if is_ascii_symbol {
                    format!(";%{:0>8b} '{}'", raw[0], raw[0] as char)
                } else {
                    format!(";%{:0>8b}", raw[0])
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

fn decode_opcode(value: u8) -> (Operation, AddressMode) {
    match value {
        // From https://www.masswerk.at/6502/6502_instruction_set.html
        // Validated with a binary that contains one of each byte as an instruction
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
        0x38 => (SEC, Implied),
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

        0x90 => (BCC, Relative),
        0x91 => (STA, IndirectY),
        0x94 => (STY, ZeropageX),
        0x95 => (STA, ZeropageX),
        0x96 => (STX, ZeropageY),
        0x98 => (TYA, Implied),
        0x99 => (STA, AbsoluteY),
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
        0xd5 => (CMP, ZeropageX),
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

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
struct InstructionBuilder {
    operation: Operation,
    address_mode: AddressMode,
    raw_bytes: Vec<u8>,
    offset: usize,
}

impl InstructionBuilder {
    fn new(offset: usize, token: u8) -> Self {
        let (operation, address_mode) = decode_opcode(token);

        InstructionBuilder {
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Object, Clone)]
pub struct Instruction {
    pub offset: usize,
    pub bytes: String,
    pub operation: String,
    pub address: String,
}

impl From<InstructionBuilder> for Instruction {
    fn from(value: InstructionBuilder) -> Self {
        let formatted_bytes: Vec<String> = value
            .raw_bytes
            .iter()
            .map(|byte| format!("{:0>2X}", byte))
            .collect();

        Instruction {
            offset: value.offset,
            bytes: formatted_bytes.join(" "),
            address: value
                .address_mode
                .format(&formatted_bytes, &value.raw_bytes, value.offset),
            operation: value.operation.to_string(),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let base = format!("{:04X}   {: <8}", self.offset, self.bytes);

        let opcode = if self.address.is_empty() {
            self.operation.to_string()
        } else if self.operation == "???" {
            format!("???                {}", self.address)
        } else {
            format!("{} {}", self.operation, self.address)
        };

        write!(f, "{}      {}", base, opcode)
    }
}

pub fn disassemble(bytes: &[u8]) -> Vec<Instruction> {
    bytes
        .iter()
        .enumerate()
        .fold(
            vec![],
            |mut acc: Vec<InstructionBuilder>, (offset, token)| {
                match acc.last_mut() {
                    Some(last) if !last.is_satisfied() => last.add(*token),
                    _ => acc.push(InstructionBuilder::new(offset, *token)),
                };

                acc
            },
        )
        .into_iter()
        .map(Instruction::from)
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

    #[test]
    fn test_mega_binary() {
        // This is a special binary that has been generated with a python script
        // It has all of the legal opcodes with all operands as 'FF'
        // Illegal opcodes have no operands
        test_example_bin("mega");
    }

    fn test_example_bin(case: &'static str) {
        let input = fs::read(format!("test-bin/{}.bin", case)).unwrap();
        // Used https://www.masswerk.at/6502/disassembler.html as a reference
        // Set output format to "verbose - no symbols", this changes indentation
        // Removed first and last lines, "* = $0000" and ".END" respectively
        let expected = fs::read(format!("test-bin/{}.example", case)).unwrap();

        for (line, (output, exp)) in disassemble(&input)
            .into_iter()
            .zip(expected.lines())
            .enumerate()
        {
            assert_eq!(output.to_string(), exp.unwrap(), "Line {}", line);
        }
    }
}
