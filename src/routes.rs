use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Serialize};
use solana_sdk::{
    bs58,
    signature::{Keypair, Signer},
    system_program,
};
use spl_token::{
    instruction::initialize_mint,
    solana_program::{instruction::Instruction, pubkey::Pubkey},
};
use std::str::FromStr;
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

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateTokenRequest {
    pub mint: String,
    pub mint_authority: String,
    pub decimals: u8,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AccountMetaInfo {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateTokenResponse {
    pub program_id: String,
    pub accounts: Vec<AccountMetaInfo>,
    pub instruction_data: String,
}

#[utoipa::path(
    post,
    path = "/token/create",
    request_body = CreateTokenRequest,
    responses(
        (status = 200, description = "Created SPL token mint instruction...", body = ApiResponse<CreateTokenResponse>),
        (status = 400, description = "Invalid pubkey or request!")
    )
)]
pub async fn create_token(Json(payload): Json<CreateTokenRequest>) -> impl IntoResponse {
    let result: Result<ApiResponse<CreateTokenResponse>, String> = (|| {
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

        Ok(ApiResponse::Success {
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
            Json(ApiResponse::<()>::Error {
                success: false,
                error,
            }),
        )
            .into_response(),
    }
}
