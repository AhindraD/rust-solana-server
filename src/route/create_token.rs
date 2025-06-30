use crate::state::{
    AccountMetaInfo, CreateTokenRequest, CreateTokenResponse, ErrorResponse, SuccessResponse,
};
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use base64::{Engine as _, engine::general_purpose};

use spl_token::{instruction::initialize_mint, solana_program::pubkey::Pubkey};
use std::str::FromStr;

#[utoipa::path(
    post,
    path = "/token/create",
    request_body = CreateTokenRequest,
    responses(
        (status = 200, description = "Created SPL token mint instruction.", body = SuccessResponse<CreateTokenResponse>),
        (status = 400, description = "Invalid pubkey or request!", body = ErrorResponse)
    )
)]
pub async fn create_token(Json(payload): Json<CreateTokenRequest>) -> impl IntoResponse {
    let result: Result<SuccessResponse<CreateTokenResponse>, String> = (|| {
        let mint = Pubkey::from_str(&payload.mint).map_err(|_| "Invalid mint public key")?;
        let mint_authority = Pubkey::from_str(&payload.mint_authority)
            .map_err(|_| "Invalid mint authority public key")?;

        let token_program = spl_token::ID;

        let instruction = initialize_mint(
            &token_program,
            &mint,
            &mint_authority,
            None,
            payload.decimals,
        )
        .map_err(|e| format!("Failed to build instruction: {e}"))?;

        let encoded_data = general_purpose::STANDARD.encode(&instruction.data);
        let accounts = instruction
            .accounts
            .into_iter()
            .map(|meta| AccountMetaInfo {
                pubkey: meta.pubkey.to_string(),
                is_signer: meta.is_signer,
                is_writable: meta.is_writable,
            })
            .collect();

        Ok(SuccessResponse {
            success: true,
            data: CreateTokenResponse {
                program_id: instruction.program_id.to_string(),
                accounts,
                instruction_data: encoded_data,
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
