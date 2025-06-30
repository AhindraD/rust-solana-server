use crate::state::{
    AccountMetaInfo, ErrorResponse, MintTokenRequest, SuccessResponse, TokenInstructionResponse,
};
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use base64::{Engine as _, engine::general_purpose};
use spl_token::{instruction::mint_to, solana_program::pubkey::Pubkey};
use std::str::FromStr;

#[utoipa::path(
    post,
    path = "/token/mint",
    request_body = MintTokenRequest,
    responses(
        (status = 200, description = "Created SPL mint-to instruction", body = SuccessResponse<TokenInstructionResponse>),
        (status = 400, description = "Invalid request", body = ErrorResponse)
    )
)]
pub async fn mint_token(Json(payload): Json<MintTokenRequest>) -> impl IntoResponse {
    let result: Result<SuccessResponse<TokenInstructionResponse>, String> = (|| {
        let mint = Pubkey::from_str(&payload.mint).map_err(|_| "Invalid mint pubkey")?;
        let dest =
            Pubkey::from_str(&payload.destination).map_err(|_| "Invalid destination pubkey")?;
        let authority =
            Pubkey::from_str(&payload.authority).map_err(|_| "Invalid authority pubkey")?;

        let instruction = mint_to(
            &spl_token::ID,
            &mint,
            &dest,
            &authority,
            &[],
            payload.amount,
        )
        .map_err(|e| format!("Failed to build instruction: {e}"))?;

        let accounts = instruction
            .accounts
            .into_iter()
            .map(|meta| AccountMetaInfo {
                pubkey: meta.pubkey.to_string(),
                is_signer: meta.is_signer,
                is_writable: meta.is_writable,
            })
            .collect();

        let encoded_data = general_purpose::STANDARD.encode(&instruction.data);

        Ok(SuccessResponse {
            success: true,
            data: TokenInstructionResponse {
                program_id: instruction.program_id.to_string(),
                accounts,
                instruction_data: encoded_data,
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
