mod root;
mod times;

use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(super::routes::root::handler))
        .route("/times", get(super::routes::times::handler))
}
