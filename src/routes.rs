use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    bs58,
    signature::{Keypair, Signer},
};
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct KeypairData {
    pub pubkey: String,
    pub secret: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(untagged)]
pub enum ApiResponse<T> {
    Success { success: bool, data: T },
    Error { success: bool, error: String },
}

#[utoipa::path(
    post,
    path = "/keypair",
    responses(
        (status = 200, description = "Generated new keypair...", body = ApiResponse<KeypairData>),
        (status = 400, description = "Keypair generation failed !")
    )
)]
pub async fn generate_keypair() -> impl IntoResponse {
    let result: Result<_, &'static str> = (|| {
        let keypair = Keypair::new();
        let pubkey = keypair.pubkey().to_string();
        let secret = bs58::encode(keypair.to_bytes()).into_string();
        Ok(ApiResponse::Success {
            success: true,
            data: KeypairData { pubkey, secret },
        })
    })();

    match result {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<()>::Error {
                success: false,
                error: err.to_string(),
            }),
        )
            .into_response(),
    }
}
