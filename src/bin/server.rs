use axum::{
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use clap::Parser;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tracing::info;

use mos_6502_disassembler::disassemble;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    bind_address: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    tracing_subscriber::fmt().init();

    let routes = Router::new().route("/", post(handler));

    let listener = TcpListener::bind(args.bind_address).await.unwrap();
    info!("{:<15} - {:?}\n", "LISTENING", listener.local_addr());
    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
struct Payload {
    data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Output {
    disassembly: Vec<String>,
}

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
