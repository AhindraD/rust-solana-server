use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateTokenRequest {
    pub mint: String,
    pub mint_authority: String,
    pub decimals: u8,
}
#[derive(Debug, Deserialize, ToSchema)]
pub struct MintTokenRequest {
    pub mint: String,
    pub destination: String,
    pub authority: String,
    pub amount: u64,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SignMessageRequest {
    pub message: String,
    pub secret: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct VerifyMessageRequest {
    pub message: String,
    pub signature: String,
    pub pubkey: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SendSolRequest {
    pub from: String,
    pub to: String,
    pub lamports: u64,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SendTokenRequest {
    pub destination: String,
    pub mint: String,
    pub owner: String,
    pub amount: u64,
}
