use axum::{http::StatusCode, response::IntoResponse, Json};

use chrono::Utc;
use chrono_tz::America::Los_Angeles;
use chrono_tz::America::New_York;
use serde::Serialize;

mod eorzean;

#[derive(Serialize)]
struct Timestamp {
    utc: String,
    et: String,
    pt: String,
    eorzea: String,
}

pub async fn controller() -> impl IntoResponse {
    let current_time = Utc::now();
    let eorzean_time = eorzean::from_datetime(current_time);

    let timestamp = Timestamp {
        utc: current_time.to_string(),
        et: current_time.with_timezone(&New_York).to_string(),
        pt: current_time.with_timezone(&Los_Angeles).to_string(),
        eorzea: eorzean_time,
    };

    (StatusCode::OK, Json(timestamp))
}
