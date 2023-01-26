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
    let (eorzean_hour, eorzean_minute, eorzean_second) =
        eorzean::get_eorzean_time(current_time.timestamp());

    let timestamp = Timestamp {
        utc: current_time.to_string(),
        et: current_time.with_timezone(&New_York).to_string(),
        pt: current_time.with_timezone(&Los_Angeles).to_string(),
        eorzea: format!(
            "{:02}:{:02}:{:02}",
            eorzean_hour, eorzean_minute, eorzean_second
        ),
    };

    (StatusCode::OK, Json(timestamp))
}
