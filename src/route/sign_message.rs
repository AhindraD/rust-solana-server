use crate::state::{ErrorResponse, SignMessageRequest, SignMessageResponse, SuccessResponse};
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use base64::{Engine as _, engine::general_purpose};
use solana_sdk::{
    bs58,
    signature::{Keypair, Signer},
};

#[utoipa::path(
    post,
    path = "/message/sign",
    request_body = SignMessageRequest,
    responses(
        (status = 200, description = "Message signed successfully", body = SuccessResponse<SignMessageResponse>),
        (status = 400, description = "Invalid input", body = ErrorResponse)
    )
)]
pub async fn sign_message(Json(payload): Json<SignMessageRequest>) -> impl IntoResponse {
    let result: Result<SuccessResponse<SignMessageResponse>, String> = (|| {
        if payload.message.trim().is_empty() || payload.secret.trim().is_empty() {
            return Err("Missing required fields".to_string());
        }

        let secret_bytes = bs58::decode(&payload.secret)
            .into_vec()
            .map_err(|_| "Invalid base58-encoded secret key")?;

        let keypair = Keypair::from_bytes(&secret_bytes)
            .map_err(|_| "Invalid secret key format (must be 64 bytes)")?;

        let signature = keypair.sign_message(payload.message.as_bytes());
        let signature_base64 = general_purpose::STANDARD.encode(signature.as_ref());

        Ok(SuccessResponse {
            success: true,
            data: SignMessageResponse {
                signature: signature_base64,
                public_key: keypair.pubkey().to_string(),
                message: payload.message.clone(),
            },
        })
    })();

    match result {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(error) => (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error,
            }),
        )
            .into_response(),
    }
}
