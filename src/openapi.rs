use crate::routes::{
    AccountMetaInfo, CreateTokenRequest, CreateTokenResponse, KeypairData, MintTokenRequest,
    TokenInstructionResponse,SignMessageRequest,SignMessageResponse,VerifyMessageRequest,VerifyMessageResponse,SendSolRequest,SendSolResponse,SendTokenRequest,SendTokenResponse,AccountInfo
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::generate_keypair,
        crate::routes::create_token,
        crate::routes::mint_token,
        crate::routes::sign_message,
        crate::routes::verify_message,
        crate::routes::send_sol,
        crate::routes::send_token,
    ),
    components(schemas(KeypairData,CreateTokenRequest,AccountMetaInfo,CreateTokenResponse,MintTokenRequest,TokenInstructionResponse,SignMessageRequest,SignMessageResponse,VerifyMessageRequest,VerifyMessageResponse,SendSolRequest,SendSolResponse,SendTokenRequest,SendTokenResponse,AccountInfo)),
    tags((name = "Solana API", description = "Solana openapi swagger UI"))
)]
pub struct ApiDoc;
