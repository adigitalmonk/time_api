use axum::{http::StatusCode, response::IntoResponse};

pub async fn controller() -> impl IntoResponse {
    (StatusCode::OK, "Hello.".to_owned())
}
