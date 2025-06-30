use crate::routes::{KeypairData};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::generate_keypair
    ),
    components(schemas(KeypairData)),
    tags((name = "Solana API", description = "Solana balance and echo service"))
)]
pub struct ApiDoc;
