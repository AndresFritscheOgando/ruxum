import { ScaffoldConfig } from "../types";

export function buildHealthRs(_config: ScaffoldConfig): string {
  return `use axum::Json;
use serde_json::{json, Value};

pub async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION")
    }))
}
`;
}

export function buildHandlersMod(_config: ScaffoldConfig): string {
  return `pub mod health;\n`;
}
