import { ScaffoldConfig } from "../types";

export function buildRouterRs(_config: ScaffoldConfig): string {
  return `use axum::{routing::get, Router};
use std::sync::Arc;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use crate::AppState;

pub fn app_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(crate::handlers::health::health_check))
        // If JWT selected, add a protected route example:
        // .route("/me", get(crate::handlers::user::me)
        //     .route_layer(axum::middleware::from_fn_with_state(
        //         state.clone(),
        //         crate::auth::middleware::require_auth,
        //     )))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
`;
}
