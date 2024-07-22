use std::fmt::Display;

use axum::{
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let routes = Router::new().route("/", post(handler));

    let addr = format!("127.0.0.1:{}", 9999);
    let listener = TcpListener::bind(addr).await.unwrap();
    info!("{:<15} - {:?}\n", "LISTENING", listener.local_addr());
    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
struct Payload {
    data: Vec<u8>,
}

#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
enum Instruction {
    BRK,
    LDA,
    LDY,
    JSR,
}
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Output {
    disassembly: Vec<String>,
}

async fn handler(Json(payload): Json<Payload>) -> Response {
    let Payload { data } = payload;
    let res = disassemble(data);
    Json(res).into_response()
}

fn disassemble(_data: Vec<u8>) -> Output {
    Output {
        disassembly: _data
            .into_iter()
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
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_api_with_example() {
        const URL: &str = "http://localhost:9999/";
        let client = reqwest::Client::builder().build().unwrap();

        let payload = Payload {
            data: vec![0xa9, 0xbd, 0xa0, 0xbd, 0x20, 0x28, 0xba],
        };

        let res: Output = client
            .post(URL)
            .json(&payload)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        let expected: Output = Output {
            disassembly: [
                "0x0000 a9 bd        LDA #$bd",
                "0x0002 a0 bd        LDY #$bd",
                "0x0004 20 28 ba     JSR $ba28",
            ]
            .iter()
            .map(|&s| s.into())
            .collect(),
        };
        assert_eq!(expected, res);
    }
}
