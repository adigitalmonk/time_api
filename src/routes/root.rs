use axum::{extract::Query, http::StatusCode, response::IntoResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GreetUser {
    name: Option<String>,
}

pub async fn handler(Query(payload): Query<GreetUser>) -> impl IntoResponse {
    payload.name.map_or_else(
        || (StatusCode::BAD_REQUEST, "Missing param: name".to_owned()),
        |name| {
            let formatted_name = if name.is_empty() {
                "no name.".to_owned()
            } else {
                title_case(&name)
            };
            (StatusCode::OK, format!("Hello, {formatted_name}."))
        },
    )
}

fn title_case(name: &str) -> String {
    name.split(' ')
        .fold(String::new(), |a, b| a + " " + &ucfirst(b))
        .trim()
        .to_string()
}

fn ucfirst(val: &str) -> String {
    val[0..1].to_uppercase() + &val[1..]
}
