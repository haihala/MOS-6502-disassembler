use axum::{
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use clap::Parser;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tracing::info;
use utoipa::{OpenApi, ToSchema};

use mos_6502_disassembler::disassemble;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    bind_address: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    tracing_subscriber::fmt().init();

    let routes = Router::new()
        .route("/", post(handler))
        .merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", ApiDoc::openapi()));

    let listener = TcpListener::bind(args.bind_address).await.unwrap();
    info!("{:<15} - {:?}\n", "LISTENING", listener.local_addr());
    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();
}

#[derive(OpenApi)]
#[openapi(paths(handler), components(schemas(Payload, Output)))]
struct ApiDoc;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct Payload {
    // Vec<u8> is a string (octet stream), https://github.com/juhaku/utoipa/issues/570
    #[schema(value_type = Vec<u32>)]
    data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, ToSchema)]
struct Output {
    disassembly: Vec<String>,
}

#[utoipa::path(
    post,
    path = "/",
    request_body = Payload,
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
async fn handler(Json(payload): Json<Payload>) -> Response {
    let Payload { data } = payload;
    let res = Output {
        disassembly: disassemble(data),
    };
    Json(res).into_response()
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
