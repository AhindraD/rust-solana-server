use crate::routes::{
    AccountInfo, AccountMetaInfo, CreateTokenRequest, CreateTokenResponse, ErrorResponse,
    KeypairData, MintTokenRequest, SendSolRequest, SendSolResponse, SendTokenRequest,
    SendTokenResponse, SignMessageRequest, SignMessageResponse, SuccessResponse,
    TokenInstructionResponse, VerifyMessageRequest, VerifyMessageResponse,
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
    components(schemas(KeypairData,CreateTokenRequest,AccountMetaInfo,CreateTokenResponse,MintTokenRequest,TokenInstructionResponse,SignMessageRequest,SignMessageResponse,VerifyMessageRequest,VerifyMessageResponse,SendSolRequest,SendSolResponse,SendTokenRequest,SendTokenResponse,AccountInfo,    SuccessResponse<KeypairData>,
        SuccessResponse<CreateTokenResponse>,
        SuccessResponse<SignMessageResponse>,
        SuccessResponse<TokenInstructionResponse>,
        SuccessResponse<SendSolResponse>,
        SuccessResponse<SendTokenResponse>,
        SuccessResponse<VerifyMessageResponse>,ErrorResponse)),
    tags((name = "Solana API", description = "Solana openapi swagger UI"))
)]
pub struct ApiDoc;
