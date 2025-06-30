use crate::routes::{
    AccountMetaInfo, CreateTokenRequest, CreateTokenResponse, KeypairData, MintTokenRequest,
    TokenInstructionResponse,SignMessageRequest,SignMessageResponse,VerifyMessageRequest,VerifyMessageResponse,SendSolRequest,SendSolResponse
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
        crate::routes::send_sol
    ),
    components(schemas(KeypairData,CreateTokenRequest,AccountMetaInfo,CreateTokenResponse,MintTokenRequest,TokenInstructionResponse,SignMessageRequest,SignMessageResponse,VerifyMessageRequest,VerifyMessageResponse,SendSolRequest,SendSolResponse)),
    tags((name = "Solana API", description = "Solana openapi swagger UI"))
)]
pub struct ApiDoc;
