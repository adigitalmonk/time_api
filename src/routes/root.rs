use axum::{extract::Query, http::StatusCode, response::IntoResponse};
use serde::{de, Deserialize, Deserializer};
use std::{fmt, str::FromStr};

#[derive(Deserialize)]
pub struct GreetUser {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    name: Option<String>,
}

// https://github.com/tokio-rs/axum/blob/4a5dc4391cc94640eca5eab61de7d5fb508b0145/examples/query-params-with-empty-strings/src/main.rs#L43
// Without this, it'll error on an empty string
fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}

pub async fn controller(Query(payload): Query<GreetUser>) -> impl IntoResponse {
    match payload.name {
        Some(name) => {
            let name = title_case(&name);
            (StatusCode::OK, format!("Hello, {name}."))
        }
        _ => (StatusCode::BAD_REQUEST, "Missing param: name".to_owned()),
    }
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
