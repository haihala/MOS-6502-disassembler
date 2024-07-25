use axum::{
    routing::{get, post},
    Router,
};
use clap::Parser;
use tokio::net::TcpListener;
use tracing::info;

use mos_6502_disassembler::{front_page, json_handler, table, ApiDoc};
use utoipa::OpenApi;
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
        .route("/", get(front_page))
        .route("/table", get(table))
        .route("/json", post(json_handler))
        .merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", ApiDoc::openapi()));

    let listener = TcpListener::bind(args.bind_address).await.unwrap();
    info!("{:<15} - {:?}\n", "LISTENING", listener.local_addr());
    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();
}
