use clap::Parser;
use poem::{listener::TcpListener, Route, Server};
use poem_openapi::OpenApiService;

use mos_6502_disassembler::{Api, Frontend};

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    bind_address: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    tracing_subscriber::fmt().init();

    let endpoints = OpenApiService::new((Api, Frontend), "Api", "1.0");

    let ui = endpoints.swagger_ui();

    let _ = Server::new(TcpListener::bind(args.bind_address))
        .run(Route::new().nest("/", endpoints).nest("/swagger", ui))
        .await;
}
