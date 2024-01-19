use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub type AppResult<T> = Result<T, AppError>;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("unable to read config: {0}")]
    Config(#[from] config::ConfigError),

    #[error("unset environment: {0}")]
    Env(#[from] std::env::VarError),

    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = (StatusCode::INTERNAL_SERVER_ERROR, self.to_string());

        let body = Json(json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}
