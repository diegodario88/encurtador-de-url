use axum::{http::StatusCode, response::IntoResponse};

pub async fn health_check() -> impl IntoResponse {
    return (StatusCode::OK, "Service is healthy");
}
