use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Serialize};
use solana_sdk::{
    bs58,
    signature::{Keypair, Signature, Signer},
    system_program,
};
use spl_token::{
    instruction::{initialize_mint, mint_to},
    solana_program::{instruction::Instruction, pubkey::Pubkey},
};
use std::str::FromStr;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
#[serde(untagged)]
pub enum ApiResponse<T> {
    Success { success: bool, data: T },
    Error { success: bool, error: String },
}
#[derive(Debug, Serialize, ToSchema)]
pub struct KeypairData {
    pub pubkey: String,
    pub secret: String,
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

#[derive(Debug, Deserialize, ToSchema)]
pub struct MintTokenRequest {
    pub mint: String,
    pub destination: String,
    pub authority: String,
    pub amount: u64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TokenInstructionResponse {
    pub program_id: String,
    pub accounts: Vec<AccountMetaInfo>,
    pub instruction_data: String,
}

#[utoipa::path(
    post,
    path = "/token/mint",
    request_body = MintTokenRequest,
    responses(
        (status = 200, description = "Created SPL mint-to instruction", body = ApiResponse<TokenInstructionResponse>),
        (status = 400, description = "Invalid request")
    )
)]
pub async fn mint_token(Json(payload): Json<MintTokenRequest>) -> impl IntoResponse {
    let result: Result<ApiResponse<TokenInstructionResponse>, String> = (|| {
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

        Ok(ApiResponse::Success {
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
            Json(ApiResponse::<()>::Error {
                success: false,
                error: err,
            }),
        )
            .into_response(),
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SignMessageRequest {
    pub message: String,
    pub secret: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SignMessageResponse {
    pub signature: String,
    pub public_key: String,
    pub message: String,
}
#[utoipa::path(
    post,
    path = "/message/sign",
    request_body = SignMessageRequest,
    responses(
        (status = 200, description = "Message signed successfully", body = ApiResponse<SignMessageResponse>),
        (status = 400, description = "Invalid input")
    )
)]
pub async fn sign_message(Json(payload): Json<SignMessageRequest>) -> impl IntoResponse {
    let result: Result<ApiResponse<SignMessageResponse>, String> = (|| {
        if payload.message.trim().is_empty() || payload.secret.trim().is_empty() {
            return Err("Missing required fields".to_string());
        }

        // Decode base58 secret key (64 bytes expected)
        let secret_bytes = bs58::decode(&payload.secret)
            .into_vec()
            .map_err(|_| "Invalid base58-encoded secret key")?;

        let keypair = Keypair::from_bytes(&secret_bytes)
            .map_err(|_| "Invalid secret key format (must be 64 bytes)")?;

        let signature = keypair.sign_message(payload.message.as_bytes());
        let signature_base64 = general_purpose::STANDARD.encode(signature.as_ref());

        Ok(ApiResponse::Success {
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
            Json(ApiResponse::<()>::Error {
                success: false,
                error,
            }),
        )
            .into_response(),
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct VerifyMessageRequest {
    pub message: String,
    pub signature: String,
    pub pubkey: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct VerifyMessageResponse {
    pub valid: bool,
    pub message: String,
    pub pubkey: String,
}

#[utoipa::path(
    post,
    path = "/message/verify",
    request_body = VerifyMessageRequest,
    responses(
        (status = 200, description = "Verification result", body = ApiResponse<VerifyMessageResponse>),
        (status = 400, description = "Invalid input")
    )
)]
pub async fn verify_message(Json(payload): Json<VerifyMessageRequest>) -> impl IntoResponse {
    let result: Result<ApiResponse<VerifyMessageResponse>, String> = (|| {
        let pubkey = Pubkey::from_str(&payload.pubkey).map_err(|_| "Invalid public key")?;

        let signature_bytes = general_purpose::STANDARD
            .decode(&payload.signature)
            .map_err(|_| "Invalid base64 signature")?;

        let signature = Signature::try_from(signature_bytes.as_slice())
            .map_err(|_| "Failed to parse signature")?;

        let is_valid = signature.verify(pubkey.as_ref(), payload.message.as_bytes());

        Ok(ApiResponse::Success {
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
            Json(ApiResponse::<()>::Error {
                success: false,
                error: err,
            }),
        )
            .into_response(),
    }
}
