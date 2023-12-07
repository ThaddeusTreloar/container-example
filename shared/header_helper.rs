use axum::http::HeaderMap;

use crate::prelude::generate_trace_id;

pub const LOGID_HEADER: &str = "logid";

pub fn set_header_logid(headers: &mut HeaderMap, logid: String) {
    headers.insert(LOGID_HEADER, logid.parse().unwrap());
}
pub fn get_logid_blocking(headers: &HeaderMap) -> String {
    headers.get(LOGID_HEADER)
        .and_then(|value| value.to_str().ok())
        .map(String::from)
        .unwrap_or(generate_trace_id())
}

pub async fn get_logid(headers: HeaderMap) -> String {
    get_header(headers, LOGID_HEADER).await.unwrap_or(generate_trace_id())
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