mod openapi;
pub mod route;
pub mod state;

use axum::{Router, routing::post};
use dotenv::dotenv;
use openapi::ApiDoc;
use route::{
    create_token, generate_keypair, mint_token, send_sol, send_token, sign_message, verify_message,
};
use std::net::SocketAddr;
use tracing_subscriber;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap_or(3000);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Server running at http://{}", addr);

    let app = Router::new()
        .route("/keypair", post(generate_keypair))
        .route("/token/create", post(create_token))
        .route("/token/mint", post(mint_token))
        .route("/message/sign", post(sign_message))
        .route("/message/verify", post(verify_message))
        .route("/sol/send", post(send_sol))
        .route("/token/send", post(send_token))
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
