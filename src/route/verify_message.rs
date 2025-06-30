use crate::state::{ErrorResponse, SuccessResponse, VerifyMessageRequest, VerifyMessageResponse};
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use base64::{Engine as _, engine::general_purpose};
use solana_sdk::signature::Signature;
use spl_token::solana_program::pubkey::Pubkey;
use std::str::FromStr;

#[utoipa::path(
    post,
    path = "/message/verify",
    request_body = VerifyMessageRequest,
    responses(
        (status = 200, description = "Verification result", body = SuccessResponse<VerifyMessageResponse>),
        (status = 400, description = "Invalid input", body = ErrorResponse)
    )
)]
pub async fn verify_message(Json(payload): Json<VerifyMessageRequest>) -> impl IntoResponse {
    let result: Result<SuccessResponse<VerifyMessageResponse>, String> = (|| {
        let pubkey = Pubkey::from_str(&payload.pubkey).map_err(|_| "Invalid public key")?;

        let signature_bytes = general_purpose::STANDARD
            .decode(&payload.signature)
            .map_err(|_| "Invalid base64 signature")?;

        let signature = Signature::try_from(signature_bytes.as_slice())
            .map_err(|_| "Failed to parse signature")?;

        let is_valid = signature.verify(pubkey.as_ref(), payload.message.as_bytes());

        Ok(SuccessResponse {
            success: true,
            data: VerifyMessageResponse {
                valid: is_valid,
                message: payload.message.clone(),
                pubkey: payload.pubkey.clone(),
            },
        })
    })();

    match result {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: err,
            }),
        )
            .into_response(),
    }
}
