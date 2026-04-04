// Example of a validated request handler
// Add this to your handlers/mod.rs:
// pub mod example; (if creating new file)

use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::validation::ValidatedJson;
use crate::errors::AppError;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateItemRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,

    #[validate(length(min = 1, max = 500))]
    pub description: Option<String>,

    #[validate(range(min = 0.0))]
    pub price: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct ItemResponse {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub price: Option<f64>,
}

/// POST /items
/// Creates a new item with validated request data
pub async fn create_item(
    ValidatedJson(payload): ValidatedJson<CreateItemRequest>,
) -> Result<(StatusCode, Json<ItemResponse>), AppError> {
    // At this point, payload is guaranteed to be valid
    // - name is 1-100 chars
    // - description is optional but 1-500 chars if present
    // - price is optional but non-negative if present

    let item = ItemResponse {
        id: 1, // In real app, would be from database
        name: payload.name,
        description: payload.description,
        price: payload.price,
    };

    Ok((StatusCode::CREATED, Json(item)))
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateItemRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,

    #[validate(length(min = 1, max = 500))]
    pub description: Option<String>,

    #[validate(range(min = 0.0))]
    pub price: Option<f64>,
}

/// PATCH /items/:id
/// Updates an item with optional validated fields
pub async fn update_item(
    axum::extract::Path(id): axum::extract::Path<u64>,
    ValidatedJson(payload): ValidatedJson<UpdateItemRequest>,
) -> Result<Json<ItemResponse>, AppError> {
    // payload.name, description, and price are all optional
    // but if provided, they must pass validation

    let item = ItemResponse {
        id,
        name: payload.name.unwrap_or_else(|| "Item".to_string()),
        description: payload.description,
        price: payload.price,
    };

    Ok(Json(item))
}
