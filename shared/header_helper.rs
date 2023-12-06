use axum::http::HeaderMap;

use crate::prelude::generate_trace_id;




pub async fn get_logid(headers: HeaderMap) -> String {
    get_header(headers, "logid").await.unwrap_or(generate_trace_id())
}

pub async fn get_header(headers: HeaderMap, key: &str) -> Option<String> {
    match headers.get(key) {
        Some(value) => {
            match value.to_str() {
                Ok(value) => Some(String::from(value)),
                Err(_) => None,
            }
        }
        None => None,
    }

}