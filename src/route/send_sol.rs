use crate::state::{ErrorResponse, SendSolRequest, SendSolResponse, SuccessResponse};
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use base64::{Engine as _, engine::general_purpose};
use std::str::FromStr;

#[utoipa::path(
    post,
    path = "/send/sol",
    request_body = SendSolRequest,
    responses(
        (status = 200, description = "SOL transfer instruction", body = SuccessResponse<SendSolResponse>),
        (status = 400, description = "Invalid input", body = ErrorResponse)
    )
)]
pub async fn send_sol(Json(payload): Json<SendSolRequest>) -> impl IntoResponse {
    let result: Result<SuccessResponse<SendSolResponse>, String> = (|| {
        if payload.lamports == 0 {
            return Err("Lamports must be greater than 0".to_string());
        }

        let from_pubkey = solana_sdk::pubkey::Pubkey::from_str(&payload.from)
            .map_err(|_| "Invalid 'from' pubkey")?;
        let to_pubkey =
            solana_sdk::pubkey::Pubkey::from_str(&payload.to).map_err(|_| "Invalid 'to' pubkey")?;

        let instruction =
            solana_sdk::system_instruction::transfer(&from_pubkey, &to_pubkey, payload.lamports);

        Ok(SuccessResponse {
            success: true,
            data: SendSolResponse {
                program_id: instruction.program_id.to_string(),
                accounts: instruction
                    .accounts
                    .iter()
                    .map(|a| a.pubkey.to_string())
                    .collect(),
                instruction_data: general_purpose::STANDARD.encode(&instruction.data),
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
