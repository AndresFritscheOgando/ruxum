use axum::{
    async_trait,
    extract::{FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::de::DeserializeOwned;
use serde_json::json;
use validator::Validate;

/// A wrapper around `Json` that validates the deserialized data before accepting it.
/// This extractor deserializes JSON from the request body and then runs validation.
/// If validation fails, it returns a 400 Bad Request with validation error details.
///
/// # Example
///
/// ```rust
/// use validator::Validate;
/// use serde::Deserialize;
/// use axum::routing::post;
///
/// #[derive(Deserialize, Validate)]
/// pub struct CreateUserRequest {
///     #[validate(length(min = 1, max = 100))]
///     pub name: String,
///     #[validate(email)]
///     pub email: String,
/// }
///
/// async fn create_user(ValidatedJson(payload): ValidatedJson<CreateUserRequest>) {
///     // payload is guaranteed to be valid
/// }
/// ```
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = ValidationError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(payload) = Json::<T>::from_request(req, state)
            .await
            .map_err(|err| ValidationError::JsonError(err.to_string()))?;

        payload.validate()?;

        Ok(ValidatedJson(payload))
    }
}

/// Custom error response for validation failures
#[derive(Debug)]
pub enum ValidationError {
    JsonError(String),
    ValidationFailed(validator::ValidationErrors),
}

impl IntoResponse for ValidationError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ValidationError::JsonError(err) => (
                StatusCode::BAD_REQUEST,
                json!({ "error": "Invalid JSON", "details": err }),
            ),
            ValidationError::ValidationFailed(errors) => {
                let mut details = std::collections::HashMap::new();
                for (field, errors) in errors.field_errors() {
                    let messages: Vec<String> = errors
                        .iter()
                        .filter_map(|e| e.message.as_ref().map(|m| m.to_string()))
                        .collect();
                    details.insert(field.to_string(), messages);
                }
                (
                    StatusCode::BAD_REQUEST,
                    json!({ "error": "Validation failed", "details": details }),
                )
            }
        };

        (status, Json(error_message)).into_response()
    }
}

impl From<validator::ValidationErrors> for ValidationError {
    fn from(errors: validator::ValidationErrors) -> Self {
        ValidationError::ValidationFailed(errors)
    }
}
