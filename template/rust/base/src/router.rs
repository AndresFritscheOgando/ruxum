use axum::{routing::get, Router};
use std::sync::Arc;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use crate::AppState;

pub fn app_router(state: Arc<AppState>) -> Router {
    let cors = build_cors_layer();

    Router::new()
        .route("/health", get(crate::handlers::health::health_check))
        // If JWT selected, add a protected route example:
        // .route("/me", get(crate::handlers::user::me)
        //     .route_layer(axum::middleware::from_fn_with_state(
        //         state.clone(),
        //         crate::auth::middleware::require_auth,
        //     )))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

fn build_cors_layer() -> CorsLayer {
    // Load allowed origins from ALLOWED_ORIGINS env var (comma-separated)
    // Example: ALLOWED_ORIGINS=https://example.com,https://app.example.com
    if let Ok(origins_str) = std::env::var("ALLOWED_ORIGINS") {
        let origins: Vec<&str> = origins_str.split(',').map(|s| s.trim()).collect();

        if !origins.is_empty() && !origins[0].is_empty() {
            // Build a restricted CORS layer from specific origins
            let mut cors = CorsLayer::permissive();
            for origin in origins {
                if let Ok(parsed) = origin.parse() {
                    cors = cors.allow_origin(parsed);
                } else if !origin.is_empty() {
                    tracing::warn!("Invalid CORS origin in ALLOWED_ORIGINS: {}", origin);
                }
            }
            tracing::info!("CORS configured: restricted to {} origin(s)", origins.len());
            return cors;
        }
    }

    // Fallback to permissive (dev mode)
    tracing::warn!("⚠️  CORS: ALLOWED_ORIGINS not set. Allowing all origins. In production, set ALLOWED_ORIGINS=https://yourdomain.com");
    CorsLayer::permissive()
}
