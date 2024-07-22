#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
enum Instruction {
    BRK,
    LDA,
    LDY,
    JSR,
}
use std::fmt::Display;

use Instruction::*;

impl Instruction {
    fn length(&self) -> usize {
        match self {
            BRK => 1,
            LDA | LDY => 2,
            JSR => 3,
        }
    }

    fn format(&self, args: &[u8]) -> String {
        match self {
            BRK => "BRK".to_string(),
            LDA => format!("LDA #${:x}", args[1]),
            LDY => format!("LDY #${:x}", args[1]),
            JSR => format!("JSR ${:x}{:x}", args[2], args[1]),
        }
    }
}

impl From<u8> for Instruction {
    fn from(value: u8) -> Self {
        match value {
            0x00 => BRK,
            0xa9 => LDA,
            0xa0 => LDY,
            0x20 => JSR,
            new => {
                dbg!(new);
                todo!()
            }
        }
    }
}

#[derive(Debug)]
struct Row {
    instruction: Instruction,
    raw: Vec<u8>,
    offset: usize,
}

impl Row {
    fn new(offset: usize, token: u8) -> Self {
        Row {
            offset,
            instruction: token.into(),
            raw: vec![token],
        }
    }

    fn add(&mut self, token: u8) {
        self.raw.push(token);
    }

    fn is_satisfied(&self) -> bool {
        self.instruction.length() == self.raw.len()
    }
}
impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string_bytes: Vec<String> = self.raw.iter().map(|byte| format!("{:x}", byte)).collect();

        f.write_fmt(format_args!(
            "{:#06x} {: <8}     {}",
            self.offset,
            string_bytes.join(" "),
            self.instruction.format(&self.raw),
        ))
    }
}

pub fn disassemble(data: Vec<u8>) -> Vec<String> {
    data.into_iter()
        .enumerate()
        .fold(vec![], |mut acc: Vec<Row>, (offset, token)| {
            match acc.last_mut() {
                Some(last) if !last.is_satisfied() => last.add(token),
                _ => acc.push(Row::new(offset, token)),
            };

            acc
        })
        .into_iter()
        .map(|row| row.to_string())
        .collect()
}
