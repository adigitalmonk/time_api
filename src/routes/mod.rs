use axum::{routing::get, Router};

mod root;
mod times;

pub fn build() -> Router {
    Router::new()
        .route("/", get(root::controller))
        .route("/times", get(times::controller))
}
