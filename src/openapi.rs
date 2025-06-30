use crate::routes::{AccountMetaInfo, CreateTokenRequest, CreateTokenResponse, KeypairData};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::generate_keypair,
        crate::routes::create_token
    ),
    components(schemas(KeypairData,CreateTokenRequest,AccountMetaInfo,CreateTokenResponse)),
    tags((name = "Solana API", description = "Solana openapi swagger UI"))
)]
pub struct ApiDoc;
