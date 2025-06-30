use serde::Serialize;
use utoipa::ToSchema;

// #[derive(Debug, Serialize, ToSchema)]
// pub enum ApiResponse<T> {
//     Success { success: bool, data: T },
//     Error { success: bool, error: String },
// }

#[derive(Debug, Serialize, ToSchema)]
pub struct SuccessResponse<T: ToSchema> {
    pub success: bool,
    pub data: T,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct KeypairData {
    pub pubkey: String,
    pub secret: String,
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

#[derive(Debug, Serialize, ToSchema)]
pub struct TokenInstructionResponse {
    pub program_id: String,
    pub accounts: Vec<AccountMetaInfo>,
    pub instruction_data: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SignMessageResponse {
    pub signature: String,
    pub public_key: String,
    pub message: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct VerifyMessageResponse {
    pub valid: bool,
    pub message: String,
    pub pubkey: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SendSolResponse {
    pub program_id: String,
    pub accounts: Vec<String>,
    pub instruction_data: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SendTokenResponse {
    pub program_id: String,
    pub accounts: Vec<AccountInfo>,
    pub instruction_data: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AccountInfo {
    pub pubkey: String,
    pub is_signer: bool,
}
