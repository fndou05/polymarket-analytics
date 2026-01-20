use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::fmt;

#[allow(dead_code)]
#[derive(Debug)]
pub enum ApiError {
    GammaFetch(reqwest::Error),
    GammaDeserialize(reqwest::Error),
    TransformFailed,
    CacheUnavailable,
    NotFound,
}

impl ApiError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            ApiError::GammaFetch(_) => StatusCode::BAD_GATEWAY,
            ApiError::GammaDeserialize(_) => StatusCode::BAD_GATEWAY,
            ApiError::TransformFailed => StatusCode::UNPROCESSABLE_ENTITY,
            ApiError::CacheUnavailable => StatusCode::SERVICE_UNAVAILABLE,
            ApiError::NotFound => StatusCode::NOT_FOUND,
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        self.status_code().into_response()
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum CacheError {
    Redis(redis::RedisError),
    Serialization(serde_json::Error),
    Internal(String),
}

impl fmt::Display for CacheError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for CacheError {}
