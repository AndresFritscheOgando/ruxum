use utoipa::OpenApi;
use crate::handlers::health::HealthResponse;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::health::health_check,
    ),
    components(
        schemas(HealthResponse)
    ),
    tags(
        (name = "health", description = "Health check endpoints")
    )
)]
pub struct ApiDoc;
