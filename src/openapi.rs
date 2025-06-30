use crate::state::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::route::generate_keypair,
        crate::route::create_token,
        crate::route::mint_token,
        crate::route::sign_message,
        crate::route::verify_message,
        crate::route::send_sol,
        crate::route::send_token,
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
