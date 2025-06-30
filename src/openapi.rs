use crate::routes::{
    AccountMetaInfo, CreateTokenRequest, CreateTokenResponse, KeypairData, MintTokenRequest,
    TokenInstructionResponse,SignMessageRequest,SignMessageResponse
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::generate_keypair,
        crate::routes::create_token,
        crate::routes::mint_token,
        crate::routes::sign_message
    ),
    components(schemas(KeypairData,CreateTokenRequest,AccountMetaInfo,CreateTokenResponse,MintTokenRequest,TokenInstructionResponse,SignMessageRequest,SignMessageResponse)),
    tags((name = "Solana API", description = "Solana openapi swagger UI"))
)]
pub struct ApiDoc;
