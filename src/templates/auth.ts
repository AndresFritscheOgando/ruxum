import { ScaffoldConfig } from "../types";

export function buildAuthMod(_config: ScaffoldConfig): string {
  return `pub mod middleware;\n`;
}

export function buildAuthMiddlewareRs(_config: ScaffoldConfig): string {
  return `use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::{errors::AppError, AppState};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn encode_token(secret: &str, sub: &str, exp: usize) -> Result<String, AppError> {
    encode(
        &Header::default(),
        &Claims { sub: sub.to_string(), exp },
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::InternalServerError(e.to_string()))
}

pub fn decode_token(secret: &str, token: &str) -> Result<Claims, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| AppError::Unauthorized)
}

pub struct AuthUser(pub Claims);

#[axum::async_trait]
impl FromRequestParts<Arc<AppState>> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| AppError::Unauthorized)?;

        let secret = state.config.jwt_secret.as_deref().unwrap_or("");
        let claims = decode_token(secret, bearer.token())?;
        Ok(AuthUser(claims))
    }
}
`;
}
