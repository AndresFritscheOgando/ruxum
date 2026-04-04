use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::fmt;
use thiserror::Error;

/// Application-level errors
///
/// Errors are split into two categories:
/// 1. **Expected/User Errors** (4xx status codes):
///    - `NotFound` - Resource doesn't exist
///    - `Unauthorized` - Authentication required/failed
///    - `Forbidden` - Authorization failed
///    - `BadRequest` - Client sent invalid data
///    - `ValidationError` - Data failed validation rules
///    - `Conflict` - Resource state conflict (e.g., duplicate)
///
/// 2. **System Failures** (5xx status codes):
///    - `DatabaseError` - Database operation failed
///    - `ExternalServiceError` - External API/service failed
///    - `InternalServerError` - Unexpected internal failure
///
/// System failures log the full error context but return generic messages to clients.
#[derive(Debug, Error)]
pub enum AppError {
    /// Resource not found (404)
    #[error("Resource not found")]
    NotFound,

    /// Unauthorized - authentication required or failed (401)
    #[error("Unauthorized")]
    Unauthorized,

    /// Forbidden - authenticated but lacks permission (403)
    #[error("Forbidden")]
    Forbidden,

    /// Bad request - client error (400)
    #[error("Bad request: {0}")]
    BadRequest(String),

    /// Validation failed (400)
    #[error("Validation failed")]
    ValidationError(String),

    /// Resource already exists or conflicts with existing state (409)
    #[error("Conflict: {0}")]
    Conflict(String),

    /// Database operation failed (500)
    /// Wraps the underlying error but logs it internally
    #[error("Database error")]
    DatabaseError(#[from] Box<dyn std::error::Error + Send + Sync>),

    /// External service/API call failed (503 or 500)
    /// Wraps the underlying error but logs it internally
    #[error("External service error: {0}")]
    ExternalServiceError(String),

    /// Unexpected internal error (500)
    /// Wraps the underlying error and logs the full context
    #[error("Internal server error")]
    InternalServerError(#[from] anyhow::Error),
}

impl AppError {
    /// Create a validation error with details
    pub fn validation(msg: impl Into<String>) -> Self {
        AppError::ValidationError(msg.into())
    }

    /// Create a bad request error with details
    pub fn bad_request(msg: impl Into<String>) -> Self {
        AppError::BadRequest(msg.into())
    }

    /// Create a conflict error with details
    pub fn conflict(msg: impl Into<String>) -> Self {
        AppError::Conflict(msg.into())
    }

    /// Create a database error and log the context
    pub fn database(err: impl std::error::Error + Send + Sync + 'static) -> Self {
        let msg = format!("Database error: {}", err);
        tracing::error!(error = %err, "Database operation failed");
        AppError::DatabaseError(Box::new(err))
    }

    /// Create an external service error and log the context
    pub fn external_service(msg: impl Into<String>) -> Self {
        let msg_str = msg.into();
        tracing::warn!(service_error = %msg_str, "External service call failed");
        AppError::ExternalServiceError(msg_str)
    }

    /// Get the HTTP status code for this error
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::Forbidden => StatusCode::FORBIDDEN,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ExternalServiceError(_) => StatusCode::SERVICE_UNAVAILABLE,
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// Get the user-facing error message (never reveals internal details)
    fn user_message(&self) -> String {
        match self {
            // User errors: show the actual message
            AppError::NotFound => "Not found".to_string(),
            AppError::Unauthorized => "Unauthorized".to_string(),
            AppError::Forbidden => "Forbidden".to_string(),
            AppError::BadRequest(msg) => format!("Bad request: {}", msg),
            AppError::ValidationError(msg) => format!("Validation error: {}", msg),
            AppError::Conflict(msg) => format!("Conflict: {}", msg),

            // System errors: generic message (details logged internally)
            AppError::DatabaseError(_) => "An error occurred processing your request".to_string(),
            AppError::ExternalServiceError(_) => {
                "Service temporarily unavailable".to_string()
            }
            AppError::InternalServerError(_) => "An unexpected error occurred".to_string(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Log system errors with full context
        match &self {
            AppError::DatabaseError(err) => {
                tracing::error!(error = ?err, "Database operation failed - returning 500");
            }
            AppError::ExternalServiceError(msg) => {
                tracing::warn!(service_error = %msg, "External service failed - returning 503");
            }
            AppError::InternalServerError(err) => {
                tracing::error!(error = ?err, "Internal server error - returning 500");
            }
            // User errors don't need special logging - they're expected
            _ => {}
        }

        let status = self.status_code();
        let message = self.user_message();

        (
            status,
            Json(json!({
                "error": message,
                "status": status.as_u16(),
            })),
        )
            .into_response()
    }
}

/// Convert from `anyhow::Error` to `AppError`
/// Use this when an unexpected error occurs that should be treated as an internal server error
impl From<Box<dyn std::error::Error + Send + Sync>> for AppError {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        tracing::error!(error = ?err, "Converting unknown error to AppError");
        AppError::InternalServerError(anyhow::anyhow!("{}", err))
    }
}
