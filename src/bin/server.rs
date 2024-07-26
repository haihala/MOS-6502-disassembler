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

    let service = OpenApiService::new((Api, Frontend), "Api", "1.0")
        .server(format!("http://{}", args.bind_address));

    let ui = service.swagger_ui();

    let _ = Server::new(TcpListener::bind(args.bind_address))
        .run(Route::new().nest("/", service).nest("/swagger", ui))
        .await;
}
