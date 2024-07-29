use crate::{disassemble, Instruction};
use poem_openapi::{payload::Json, ApiResponse, Object, OpenApi};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Object)]
pub struct Input {
    bytes: Vec<u8>,
}

#[derive(Debug, PartialEq, ApiResponse)]
pub enum Output {
    #[oai(status = 200)]
    Ok(Json<Disassembly>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Object, Clone)]
pub struct Disassembly {
    lines: Vec<String>,
    structured: Vec<Instruction>,
}

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/json", method = "post")]
    pub async fn json_handler(&self, payload: Json<Input>) -> Output {
        let structured = disassemble(&payload.bytes);

        Output::Ok(Json(Disassembly {
            lines: structured.iter().map(|i| i.to_string()).collect(),
            structured,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_api_with_example() {
        const URL: &str = "http://localhost:9999/json";
        let client = reqwest::Client::builder().build().unwrap();

        let payload = Input {
            bytes: vec![0xa9, 0xbd, 0xa0, 0xbd, 0x20, 0x28, 0xba],
        };

        let lines = client
            .post(URL)
            .json(&payload)
            .send()
            .await
            .unwrap()
            .json::<Disassembly>()
            .await
            .unwrap()
            .lines;

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
