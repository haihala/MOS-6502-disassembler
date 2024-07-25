use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

use crate::disassemble;

#[derive(OpenApi)]
#[openapi(paths(json_handler), components(schemas(Input, Output)))]
pub struct ApiDoc;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Input {
    // Vec<u8> is a string (octet stream), https://github.com/juhaku/utoipa/issues/570
    #[schema(value_type = Vec<u32>, example = json!([169, 189, 160, 189, 32, 40, 186]))]
    bytes: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct Output {
    disassembly: Vec<String>,
}

#[utoipa::path(
    post,
    path = "/json",
    request_body = Input,
    responses(
        (
            status = 200,
            description = "Successful decompillation, outputs list of lines",
            body = Output
        ),
        (
            status = 422,
            description = "Invalid input, can't parse input as 8 bit numbers"
        ),
    )
)]
pub async fn json_handler(Json(payload): Json<Input>) -> Response {
    let res = Output {
        disassembly: disassemble(payload.bytes),
    };
    Json(res).into_response()
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
                "0000   A9 BD         LDA #$BD",
                "0002   A0 BD         LDY #$BD",
                "0004   20 28 BA      JSR $BA28",
            ]
            .iter()
            .map(|&s| s.into())
            .collect(),
        };
        assert_eq!(expected, res);
    }
}
