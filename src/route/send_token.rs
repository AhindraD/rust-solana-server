use crate::state::{
    AccountInfo, ErrorResponse, SendTokenRequest, SendTokenResponse, SuccessResponse,
};
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use base64::{Engine as _, engine::general_purpose};

use spl_token::{instruction::transfer, solana_program::pubkey::Pubkey};
use std::str::FromStr;

#[utoipa::path(
    post,
    path = "/send/token",
    request_body = SendTokenRequest,
    responses(
        (status = 200, description = "SPL Token transfer instruction", body = SuccessResponse<SendTokenResponse>),
        (status = 400, description = "Invalid input", body = ErrorResponse)
    )
)]
pub async fn send_token(Json(payload): Json<SendTokenRequest>) -> impl IntoResponse {
    let result: Result<SuccessResponse<SendTokenResponse>, String> = (|| {
        let destination =
            Pubkey::from_str(&payload.destination).map_err(|_| "Invalid destination pubkey")?;
        let mint = Pubkey::from_str(&payload.mint).map_err(|_| "Invalid mint pubkey")?;
        let owner = Pubkey::from_str(&payload.owner).map_err(|_| "Invalid owner pubkey")?;

        let instruction = transfer(
            &spl_token::ID,
            &mint,
            &destination,
            &owner,
            &[],
            payload.amount,
        )
        .map_err(|e| format!("Failed to build instruction: {e}"))?;

        let accounts = instruction
            .accounts
            .iter()
            .map(|a| AccountInfo {
                pubkey: a.pubkey.to_string(),
                is_signer: a.is_signer,
            })
            .collect();

        let instruction_data = general_purpose::STANDARD.encode(&instruction.data);

        Ok(SuccessResponse {
            success: true,
            data: SendTokenResponse {
                program_id: instruction.program_id.to_string(),
                accounts,
                instruction_data,
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
