use crate::state::{ErrorResponse, KeypairData, SuccessResponse};
use axum::{extract::Json, http::StatusCode, response::IntoResponse};

use solana_sdk::{
    bs58,
    signature::{Keypair, Signer},
};

#[utoipa::path(
    post,
    path = "/keypair",
    responses(
        (status = 200, description = "Generated new keypair.", body = SuccessResponse<KeypairData>),
        (status = 400, description = "Keypair generation failed !", body = ErrorResponse)
    )
)]
pub async fn generate_keypair() -> impl IntoResponse {
    let result: Result<_, &'static str> = (|| {
        let keypair = Keypair::new();
        let pubkey = keypair.pubkey().to_string();
        let secret = bs58::encode(keypair.to_bytes()).into_string();
        Ok(SuccessResponse {
            success: true,
            data: KeypairData { pubkey, secret },
        })
    })();

    match result {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: err.to_string(),
            }),
        )
            .into_response(),
    }
}
