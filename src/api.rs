use crate::{disassemble, Instruction};
use poem_openapi::{payload::Json, ApiResponse, Object, OpenApi};
use serde::{Deserialize, Serialize};
use tracing::{event, instrument, Level};

#[derive(Debug, Serialize, Deserialize, Object)]
pub struct Input {
    bytes: Vec<u8>,
}

#[derive(Debug, PartialEq, ApiResponse)]
pub enum StructuredOutput {
    #[oai(status = 200)]
    Ok(Json<StructuredDisassembly>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Object, Clone)]
pub struct StructuredDisassembly {
    instructions: Vec<Instruction>,
}

#[derive(Debug, PartialEq, ApiResponse)]
pub enum FormattedOutput {
    #[oai(status = 200)]
    Ok(Json<FormattedDisassembly>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Object, Clone)]
pub struct FormattedDisassembly {
    instructions: Vec<String>,
}

#[derive(Debug)]
pub struct Api;

#[OpenApi(prefix_path = "/json")]
impl Api {
    #[instrument]
    #[oai(path = "/structured", method = "post")]
    pub async fn structured_handler(&self, payload: Json<Input>) -> StructuredOutput {
        event!(Level::INFO, "Structured disassembling Json");
        let instructions = disassemble(&payload.bytes);

        StructuredOutput::Ok(Json(StructuredDisassembly { instructions }))
    }

    #[instrument]
    #[oai(path = "/formatted", method = "post")]
    pub async fn formatted_handler(&self, payload: Json<Input>) -> FormattedOutput {
        event!(Level::INFO, "Formatted disassembling Json");
        let structured = disassemble(&payload.bytes);

        FormattedOutput::Ok(Json(FormattedDisassembly {
            instructions: structured
                .into_iter()
                .map(|instruction| instruction.to_string())
                .collect(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_structured_api() {
        let client = reqwest::Client::builder().build().unwrap();

        let payload = Input {
            bytes: vec![0xa9, 0xbd, 0xa0, 0xbd, 0x20, 0x28, 0xba],
        };

        let output = client
            .post("http://localhost:9999/json/structured")
            .json(&payload)
            .send()
            .await
            .unwrap()
            .json::<StructuredDisassembly>()
            .await
            .unwrap()
            .instructions;

        let expected: Vec<Instruction> = vec![
            Instruction {
                offset: 0,
                bytes: "A9 BD".into(),
                operation: "LDA".into(),
                address: "#$BD".into(),
            },
            Instruction {
                offset: 2,
                bytes: "A0 BD".into(),
                operation: "LDY".into(),
                address: "#$BD".into(),
            },
            Instruction {
                offset: 4,
                bytes: "20 28 BA".into(),
                operation: "JSR".into(),
                address: "$BA28".into(),
            },
        ];

        assert_eq!(expected, output);
    }

    #[tokio::test]
    async fn test_formatted_api() {
        let client = reqwest::Client::builder().build().unwrap();

        let payload = Input {
            bytes: vec![0xa9, 0xbd, 0xa0, 0xbd, 0x20, 0x28, 0xba],
        };

        let lines = client
            .post("http://localhost:9999/json/formatted")
            .json(&payload)
            .send()
            .await
            .unwrap()
            .json::<FormattedDisassembly>()
            .await
            .unwrap()
            .instructions;

        let expected: Vec<String> = [
            "0000   A9 BD         LDA #$BD",
            "0002   A0 BD         LDY #$BD",
            "0004   20 28 BA      JSR $BA28",
        ]
        .iter()
        .map(|&s| s.into())
        .collect();

        assert_eq!(expected, lines);
    }
}
